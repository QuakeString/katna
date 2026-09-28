// SPDX-License-Identifier: GPL-3.0-or-later

//! Our side of the sans-I/O contract: TCP from `async-net` and TLS from
//! `futures-rustls` (rustls only, ring provider). The `async-io` reactor runs
//! on its own thread, so this works under any executor.

use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use async_io::Timer;
use async_net::TcpStream;
use futures_lite::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, FutureExt};
use futures_rustls::{
    TlsConnector,
    client::TlsStream,
    rustls::{
        self, ClientConfig, DigitallySignedStruct, SignatureScheme,
        client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
        crypto::{CryptoProvider, ring},
        pki_types::{CertificateDer, ServerName, UnixTime},
    },
};
use rustls_platform_verifier::BuilderVerifierExt;

use crate::{Error, Result};

/// How long connecting (TCP and the TLS handshake) may take.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// How long a read may block while we expect an answer. Waiting for pushed
/// changes (IDLE) uses its own, longer limit.
pub const READ_TIMEOUT: Duration = Duration::from_secs(120);

/// Most bytes a server may send without ending a line, and the most one
/// POP3 answer may hold. IMAP literals stop at 64 MiB (io-imap), so this
/// only stops a server that never ends its line from using up the memory.
pub const MAX_READ: usize = 256 * 1024 * 1024;

/// TLS settings shared by all connections of the daemon.
#[derive(Clone)]
pub struct Tls {
    connector: TlsConnector,
}

impl Tls {
    /// Verifies certificates against the system trust store.
    pub fn system() -> Result<Self> {
        let provider = Arc::new(ring::default_provider());
        let config = ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .map_err(|e| Error::Tls(e.to_string()))?
            .with_platform_verifier()
            .map_err(|e| Error::Tls(e.to_string()))?
            .with_no_client_auth();
        Ok(Self::from_config(config))
    }

    /// Accepts any certificate. Only for the self-signed local test servers
    /// in `dev/`; never reachable from a user setting.
    ///
    /// # Panics
    ///
    /// Never in practice: ring supports rustls' default protocol versions.
    pub fn insecure_for_local_tests() -> Self {
        let provider = Arc::new(ring::default_provider());
        let config = ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .expect("ring supports the default protocol versions")
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAny(provider)))
            .with_no_client_auth();
        Self::from_config(config)
    }

    fn from_config(config: ClientConfig) -> Self {
        Self {
            connector: TlsConnector::from(Arc::new(config)),
        }
    }
}

enum Stream {
    Plain(TcpStream),
    Tls(Box<TlsStream<TcpStream>>),
}

/// One connection to a server, plain or TLS, with its read buffer.
pub struct Conn {
    stream: Option<Stream>,
    host: String,
    tls: Tls,
    buf: Vec<u8>,
    /// Bytes in `buf` not yet handed out. They survive a cancelled read.
    filled: usize,
    /// `buf[..filled]` was handed out; clear it on the next read.
    handed_out: bool,
}

impl Conn {
    pub fn new(tls: Tls) -> Self {
        Self {
            stream: None,
            host: String::new(),
            tls,
            buf: vec![0; 64 * 1024],
            filled: 0,
            handed_out: false,
        }
    }

    pub async fn connect_tcp(&mut self, host: &str, port: u16) -> Result<()> {
        let tcp = with_timeout(CONNECT_TIMEOUT, async {
            Ok(TcpStream::connect((host, port)).await?)
        })
        .await?;
        tcp.set_nodelay(true)?;
        tracing::debug!(host, port, "connected");
        self.host = host.to_owned();
        self.stream = Some(Stream::Plain(tcp));
        self.filled = 0;
        self.handed_out = false;
        Ok(())
    }

    pub async fn connect_tls(&mut self, host: &str, port: u16) -> Result<()> {
        self.connect_tcp(host, port).await?;
        self.upgrade_tls().await
    }

    /// Wraps the open plain connection in TLS (implicit TLS or STARTTLS).
    pub async fn upgrade_tls(&mut self) -> Result<()> {
        let Some(Stream::Plain(tcp)) = self.stream.take() else {
            return Err(Error::Tls("no plain connection to upgrade".into()));
        };
        let name =
            ServerName::try_from(self.host.clone()).map_err(|e| Error::Tls(e.to_string()))?;
        let connector = self.tls.connector.clone();
        let tls = with_timeout(CONNECT_TIMEOUT, async {
            connector.connect(name, tcp).await.map_err(tls_error)
        })
        .await?;
        tracing::debug!(host = self.host, "TLS established");
        self.stream = Some(Stream::Tls(Box::new(tls)));
        Ok(())
    }

    pub async fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        let stream = self.stream.as_mut().ok_or_else(not_connected)?;
        with_timeout(READ_TIMEOUT, async {
            stream.write_all(bytes).await?;
            stream.flush().await?;
            Ok(())
        })
        .await
    }

    /// Reads the next chunk. Returns an empty slice at EOF, which is what
    /// the Pimalaya coroutines expect.
    pub async fn read(&mut self) -> Result<&[u8]> {
        self.read_timeout(READ_TIMEOUT)
            .await?
            .ok_or(Error::Timeout(READ_TIMEOUT))
    }

    /// Like [`Self::read`], but returns `None` when nothing arrives within
    /// `timeout`.
    ///
    /// Always returns whole lines: when a read stops mid-line, it keeps
    /// reading until the data ends with a line break. io-imap 0.6 breaks
    /// APPEND when the server's `+` continuation arrives in two pieces (S2
    /// findings, problem 10), and Dovecot does split lines across TLS
    /// records. Every IMAP and SMTP response ends with CRLF, so this never
    /// waits for data the server is not about to send.
    ///
    /// Cancel-safe: if the future is dropped, bytes already read stay in the
    /// buffer and the next call returns them.
    pub async fn read_timeout(&mut self, timeout: Duration) -> Result<Option<&[u8]>> {
        if self.handed_out {
            self.filled = 0;
            self.handed_out = false;
        }
        let deadline = Instant::now() + timeout;
        let stream = self.stream.as_mut().ok_or_else(not_connected)?;
        loop {
            if self.filled == self.buf.len() {
                if self.filled >= MAX_READ {
                    return Err(Error::Protocol(format!(
                        "the server sent more than {} MiB without a line end",
                        MAX_READ >> 20
                    )));
                }
                self.buf.resize(self.buf.len() * 2, 0);
            }
            // Once a line has started, the rest must follow promptly.
            let (wait, started) = match self.filled {
                0 => (deadline.saturating_duration_since(Instant::now()), false),
                _ => (READ_TIMEOUT, true),
            };
            let read = async { Some(stream.read(&mut self.buf[self.filled..]).await) };
            let timer = async {
                Timer::after(wait).await;
                None
            };
            let n = match read.or(timer).await {
                Some(n) => n?,
                None if started => return Err(Error::Timeout(READ_TIMEOUT)),
                None => return Ok(None),
            };
            self.filled += n;
            // At EOF, hand over what we have; the next read returns empty.
            if n == 0 || self.buf[self.filled - 1] == b'\n' {
                self.handed_out = true;
                return Ok(Some(&self.buf[..self.filled]));
            }
        }
    }

    /// Reads whatever arrives next, without waiting for a whole line: for
    /// binary data such as a download. Returns an empty slice at EOF.
    pub async fn read_raw(&mut self) -> Result<&[u8]> {
        if self.handed_out {
            self.filled = 0;
            self.handed_out = false;
        }
        let stream = self.stream.as_mut().ok_or_else(not_connected)?;
        if self.filled == self.buf.len() {
            self.buf.resize(self.buf.len() * 2, 0);
        }
        let read = async { Some(stream.read(&mut self.buf[self.filled..]).await) };
        let timer = async {
            Timer::after(READ_TIMEOUT).await;
            None
        };
        let n = read.or(timer).await.ok_or(Error::Timeout(READ_TIMEOUT))??;
        self.filled += n;
        self.handed_out = true;
        Ok(&self.buf[..self.filled])
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.close().await?;
        }
        Ok(())
    }
}

async fn with_timeout<T>(
    timeout: Duration,
    fut: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    fut.or(async {
        Timer::after(timeout).await;
        Err(Error::Timeout(timeout))
    })
    .await
}

/// rustls reports certificate problems as `InvalidData` I/O errors; keep
/// them apart from network failures.
fn tls_error(err: io::Error) -> Error {
    match err.kind() {
        io::ErrorKind::InvalidData => Error::Tls(err.to_string()),
        _ => Error::Io(err),
    }
}

fn not_connected() -> Error {
    Error::Io(io::Error::new(io::ErrorKind::NotConnected, "not connected"))
}

impl AsyncRead for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Stream::Plain(s) => Pin::new(s).poll_read(cx, buf),
            Stream::Tls(s) => Pin::new(s.as_mut()).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Stream::Plain(s) => Pin::new(s).poll_write(cx, buf),
            Stream::Tls(s) => Pin::new(s.as_mut()).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Stream::Plain(s) => Pin::new(s).poll_flush(cx),
            Stream::Tls(s) => Pin::new(s.as_mut()).poll_flush(cx),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Stream::Plain(s) => Pin::new(s).poll_close(cx),
            Stream::Tls(s) => Pin::new(s.as_mut()).poll_close(cx),
        }
    }
}

#[derive(Debug)]
struct AcceptAny(Arc<CryptoProvider>);

impl ServerCertVerifier for AcceptAny {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}
