// SPDX-License-Identifier: GPL-3.0-or-later

//! Our side of the sans-I/O contract: TCP via `async-net` (the `async-io`
//! reactor, which runs on its own thread and so works under any executor,
//! GPUI's included) and TLS via `futures-rustls`.

use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
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

/// How long a single read may block outside IDLE before the connection is
/// considered dead.
pub const READ_TIMEOUT: Duration = Duration::from_secs(120);

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
}

impl Conn {
    pub fn new(tls: Tls) -> Self {
        Self {
            stream: None,
            host: String::new(),
            tls,
            buf: vec![0; 64 * 1024],
        }
    }

    pub async fn connect_tcp(&mut self, host: &str, port: u16) -> Result<()> {
        let tcp = TcpStream::connect((host, port)).await?;
        tcp.set_nodelay(true)?;
        self.host = host.to_owned();
        self.stream = Some(Stream::Plain(tcp));
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
        let tls = self.tls.connector.connect(name, tcp).await?;
        self.stream = Some(Stream::Tls(Box::new(tls)));
        Ok(())
    }

    pub async fn write_all(&mut self, bytes: &[u8]) -> Result<()> {
        let stream = self.stream_mut()?;
        stream.write_all(bytes).await?;
        stream.flush().await?;
        Ok(())
    }

    /// Reads what is available. Returns an empty slice at EOF, which is what
    /// the Pimalaya coroutines expect.
    pub async fn read(&mut self) -> Result<&[u8]> {
        self.read_timeout(READ_TIMEOUT)
            .await?
            .ok_or(Error::Timeout(READ_TIMEOUT))
    }

    /// Like [`Self::read`] but returns `None` when `timeout` passes first.
    /// Dropping a pending read is safe: rustls and `async-io` only consume
    /// bytes when the read completes.
    pub async fn read_timeout(&mut self, timeout: Duration) -> Result<Option<&[u8]>> {
        let Self { stream, buf, .. } = self;
        let stream = stream.as_mut().ok_or_else(not_connected)?;
        let read = async { Some(stream.read(buf).await) };
        let timer = async {
            Timer::after(timeout).await;
            None
        };
        match read.or(timer).await {
            Some(n) => Ok(Some(&buf[..n?])),
            None => Ok(None),
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.close().await?;
        }
        Ok(())
    }

    fn stream_mut(&mut self) -> Result<&mut Stream> {
        self.stream.as_mut().ok_or_else(not_connected)
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
