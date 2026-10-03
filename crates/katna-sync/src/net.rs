// SPDX-License-Identifier: GPL-3.0-or-later

//! Our side of the sans-I/O contract: TCP from `async-net` and TLS from
//! `futures-rustls` (rustls only, ring provider). The `async-io` reactor runs
//! on its own thread, so this works under any executor.

use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
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
    /// Certificates are not checked ([`Tls::insecure_for_local_tests`]).
    insecure: bool,
}

impl Tls {
    /// Verifies certificates against the system trust store.
    pub fn system() -> Result<Self> {
        let provider = Arc::new(ring::default_provider());
        let config = ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .map_err(|e| Error::Tls(format!("TLS setup: {e}")))?
            .with_platform_verifier()
            .map_err(|e| Error::Tls(format!("TLS setup: {e}")))?
            .with_no_client_auth();
        Ok(Self::from_config(config, false))
    }

    /// Accepts any certificate: for self-signed test servers, such as the
    /// ones in `dev/`. An account's "accept invalid certificates" setting
    /// (D-Bus `ServerSpec.accept_invalid_certs`, `katnactl --insecure`)
    /// selects it too, so it only works for servers on this computer or
    /// the local network: [`Conn`] refuses the handshake when the address
    /// it connected to is public ([`is_internal`]), so the password never
    /// goes unprotected across the internet.
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
        Self::from_config(config, true)
    }

    fn from_config(config: ClientConfig, insecure: bool) -> Self {
        Self {
            connector: TlsConnector::from(Arc::new(config)),
            insecure,
        }
    }
}

/// Which addresses a [`Conn`] may connect to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Reach {
    /// Any address: mail servers, autoconfig, OAuth2 and Katna Server,
    /// which the user or Katna chose (and tests run on this computer).
    #[default]
    Any,
    /// Public addresses only ([`is_internal`] ones are refused): for URLs
    /// that mail, DNS or web pages name, such as remote images and sender
    /// pictures, so a message cannot make the daemon reach this computer
    /// or its network. The name is resolved once and the connection goes
    /// to the address that was checked, so DNS rebinding cannot swap it.
    Public,
}

/// Whether `ip` is on this computer or a local, private or special
/// network rather than the public internet: unspecified, loopback,
/// private (RFC 1918), shared (CGNAT, RFC 6598), link-local, unique local
/// (ULA), site-local, multicast, broadcast, documentation, benchmarking
/// and reserved ranges, including IPv4 addresses inside IPv6 (mapped,
/// compatible, NAT64 and 6to4).
pub fn is_internal(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => internal_v4(v4),
        IpAddr::V6(v6) => internal_v6(v6),
    }
}

fn internal_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_multicast()
        || ip.is_broadcast()
        || ip.is_documentation()
        || a == 0
        // Shared address space (CGNAT), 100.64.0.0/10.
        || (a == 100 && (64..128).contains(&b))
        // IETF protocol assignments, 192.0.0.0/24.
        || (a == 192 && b == 0 && c == 0)
        // Benchmarking, 198.18.0.0/15.
        || (a == 198 && (18..20).contains(&b))
        // Reserved, 240.0.0.0/4.
        || a >= 240
}

fn internal_v6(ip: Ipv6Addr) -> bool {
    let segments = ip.segments();
    let embedded = |high: u16, low: u16| {
        let [h1, h2] = high.to_be_bytes();
        let [l1, l2] = low.to_be_bytes();
        internal_v4(Ipv4Addr::new(h1, h2, l1, l2))
    };
    if let Some(v4) = ip.to_ipv4_mapped() {
        return internal_v4(v4);
    }
    ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_multicast()
        // Unique local, fc00::/7.
        || (segments[0] & 0xfe00) == 0xfc00
        // Link-local, fe80::/10, and the old site-local, fec0::/10.
        || (segments[0] & 0xffc0) == 0xfe80
        || (segments[0] & 0xffc0) == 0xfec0
        // Documentation, 2001:db8::/32.
        || (segments[0] == 0x2001 && segments[1] == 0x0db8)
        // IPv4-compatible, ::a.b.c.d (deprecated).
        || (segments[..6].iter().all(|s| *s == 0) && embedded(segments[6], segments[7]))
        // NAT64, 64:ff9b::/96.
        || (segments[..6] == [0x64, 0xff9b, 0, 0, 0, 0] && embedded(segments[6], segments[7]))
        // 6to4, 2002::/16, with the IPv4 address in the next 32 bits.
        || (segments[0] == 0x2002 && embedded(segments[1], segments[2]))
}

/// The addresses of `host` that `reach` allows, in the resolver's order.
async fn addresses(host: &str, port: u16, reach: Reach) -> Result<Vec<SocketAddr>> {
    // An IPv6 literal may come in brackets, as in a URL.
    let bare = host.trim_start_matches('[').trim_end_matches(']');
    let all = match bare.parse::<IpAddr>() {
        Ok(ip) => vec![SocketAddr::new(ip, port)],
        Err(_) => async_net::resolve((bare, port)).await?,
    };
    if reach == Reach::Any {
        return Ok(all);
    }
    let public: Vec<SocketAddr> = all
        .iter()
        .copied()
        .filter(|addr| !is_internal(addr.ip()))
        .collect();
    if public.is_empty() && !all.is_empty() {
        return Err(Error::Protocol(format!(
            "{host} is on this computer or a local network; it is not fetched"
        )));
    }
    Ok(public)
}

/// `host`, with `port` when it isn't HTTPS's, for people.
fn shown(host: &str, port: u16) -> String {
    if port == 443 {
        host.to_owned()
    } else {
        format!("{host} port {port}")
    }
}

/// Connects to the first address of `host` that `reach` allows and that
/// answers.
async fn connect_to(host: &str, port: u16, reach: Reach) -> Result<TcpStream> {
    let mut last = None;
    for addr in addresses(host, port, reach).await? {
        match TcpStream::connect(addr).await {
            Ok(tcp) => return Ok(tcp),
            Err(err) => last = Some(err),
        }
    }
    Err(Error::Io(last.unwrap_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("{host}: no address found"))
    })))
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
    reach: Reach,
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
            reach: Reach::Any,
            buf: vec![0; 64 * 1024],
            filled: 0,
            handed_out: false,
        }
    }

    /// A connection that may only go where `reach` allows.
    pub fn with_reach(tls: Tls, reach: Reach) -> Self {
        Self {
            reach,
            ..Self::new(tls)
        }
    }

    pub async fn connect_tcp(&mut self, host: &str, port: u16) -> Result<()> {
        let reach = self.reach;
        let tcp = with_timeout(CONNECT_TIMEOUT, connect_to(host, port, reach))
            .await
            .map_err(|err| match err {
                Error::Timeout(limit) => Error::Unreachable(format!(
                    "{} did not take a connection within {}s",
                    shown(host, port),
                    limit.as_secs()
                )),
                err => err,
            })?;
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
        if self.tls.insecure {
            // The address actually connected to, so a name cannot resolve
            // to a local address for a check and a public one after it.
            check_unverified_peer(&self.host, tcp.peer_addr()?.ip())?;
        }
        let name = ServerName::try_from(self.host.clone())
            .map_err(|e| Error::Tls(format!("{}: not a server name: {e}", self.host)))?;
        let connector = self.tls.connector.clone();
        let tls = with_timeout(CONNECT_TIMEOUT, async {
            connector
                .connect(name, tcp)
                .await
                .map_err(|err| tls_error(&self.host, err))
        })
        .await
        .map_err(|err| match err {
            Error::Timeout(limit) => Error::Unreachable(format!(
                "{} took a connection but did not finish the secure handshake within {}s",
                self.host,
                limit.as_secs()
            )),
            err => err,
        })
        .inspect_err(|err| {
            // Sites a message names (pictures, images) are not the user's
            // servers: not worth a warning.
            if let Error::Tls(error) = err {
                match self.reach {
                    Reach::Any => tracing::warn!(host = self.host, %error, "TLS handshake failed"),
                    Reach::Public => {
                        tracing::debug!(host = self.host, %error, "TLS handshake failed")
                    }
                }
            }
        })?;
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

/// Refuses to skip certificate checks for a server at a public address.
fn check_unverified_peer(host: &str, ip: IpAddr) -> Result<()> {
    if is_internal(ip) {
        return Ok(());
    }
    Err(Error::Tls(format!(
        "{host} ({ip}) is not on this computer or the local network, so its \
         certificate must be valid: turn off \"accept invalid certificates\" \
         for this account"
    )))
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
/// A failed handshake with `host`, which it names: people see it as the
/// account's reason line, and the log shows which server it was.
fn tls_error(host: &str, err: io::Error) -> Error {
    if err.kind() != io::ErrorKind::InvalidData {
        return Error::Io(err);
    }
    let error = match err
        .get_ref()
        .and_then(|e| e.downcast_ref::<rustls::Error>())
    {
        Some(rustls::Error::InvalidCertificate(problem)) => certificate_problem(host, problem),
        _ => format!("the secure connection to {host} failed: {err}"),
    };
    Error::Tls(error)
}

/// What is wrong with `host`'s certificate, and who can fix it.
fn certificate_problem(host: &str, problem: &rustls::CertificateError) -> String {
    use rustls::CertificateError as E;
    let day = |time: &UnixTime| {
        jiff::Timestamp::from_second(time.as_secs() as i64)
            .map(|t| t.strftime("%Y-%m-%d").to_string())
            .unwrap_or_default()
    };
    match problem {
        E::ExpiredContext { not_after, .. } => format!(
            "{host}'s security certificate expired on {} (UTC); the provider has to renew it",
            day(not_after)
        ),
        E::Expired => {
            format!("{host}'s security certificate has expired; the provider has to renew it")
        }
        E::NotValidYet | E::NotValidYetContext { .. } => format!(
            "{host}'s security certificate is not valid yet; check this computer's date and time"
        ),
        E::NotValidForName | E::NotValidForNameContext { .. } => format!(
            "{host}'s security certificate is for another name; check the server name in the \
             account's settings"
        ),
        E::UnknownIssuer => format!(
            "{host}'s security certificate is not from an authority this computer trusts; \
             the provider has to fix it"
        ),
        E::Revoked => {
            format!("{host}'s security certificate was revoked; the provider has to replace it")
        }
        other => format!("{host}'s security certificate was refused ({other:?})"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_addresses() {
        for ip in [
            "0.0.0.0",
            "127.0.0.1",
            "127.8.9.10",
            "10.0.0.1",
            "172.16.5.4",
            "172.31.255.255",
            "192.168.1.1",
            "169.254.169.254",
            "100.64.0.1",
            "100.127.255.254",
            "192.0.0.8",
            "198.18.0.1",
            "224.0.0.251",
            "255.255.255.255",
            "240.0.0.1",
            "::",
            "::1",
            "fc00::1",
            "fd12:3456::1",
            "fe80::1",
            "fec0::1",
            "ff02::1",
            "2001:db8::1",
            "::ffff:127.0.0.1",
            "::ffff:10.0.0.1",
            "::127.0.0.1",
            "64:ff9b::a9fe:a9fe",
            "2002:c0a8:0101::1",
        ] {
            assert!(is_internal(ip.parse().unwrap()), "{ip}");
        }
        for ip in [
            "1.1.1.1",
            "8.8.8.8",
            "100.63.255.255",
            "100.128.0.1",
            "172.32.0.1",
            "193.0.0.1",
            "2606:4700:4700::1111",
            "::ffff:8.8.8.8",
            "64:ff9b::808:808",
            "2002:0808:0808::1",
        ] {
            assert!(!is_internal(ip.parse().unwrap()), "{ip}");
        }
    }

    #[test]
    fn public_reach_refuses_local_servers() {
        smol::block_on(async {
            let listener = async_net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();
            let tls = Tls::system().unwrap();
            let mut conn = Conn::with_reach(tls.clone(), Reach::Public);
            let err = conn.connect_tcp("127.0.0.1", port).await.unwrap_err();
            assert!(err.to_string().contains("local network"), "{err}");
            let err = conn.connect_tcp("localhost", port).await.unwrap_err();
            assert!(err.to_string().contains("local network"), "{err}");
            // The default reaches it, as tests and local servers need.
            Conn::new(tls).connect_tcp("127.0.0.1", port).await.unwrap();
        });
    }

    #[test]
    fn refused_certificates_name_the_server_and_who_fixes_it() {
        use rustls::CertificateError as E;
        // 2026-09-14 08:24:52 UTC, as in a real report.
        let expired = E::ExpiredContext {
            time: UnixTime::since_unix_epoch(Duration::from_secs(1_790_796_501)),
            not_after: UnixTime::since_unix_epoch(Duration::from_secs(1_789_374_292)),
        };
        assert_eq!(
            certificate_problem("imap.example.org", &expired),
            "imap.example.org's security certificate expired on 2026-09-14 (UTC); \
             the provider has to renew it"
        );
        let other_name = certificate_problem("mail.example.org", &E::NotValidForName);
        assert!(other_name.starts_with("mail.example.org's"), "{other_name}");
        assert!(other_name.contains("server name"), "{other_name}");
        let wrapped = io::Error::new(
            io::ErrorKind::InvalidData,
            rustls::Error::InvalidCertificate(E::UnknownIssuer),
        );
        let err = tls_error("smtp.example.org", wrapped);
        assert!(matches!(err, Error::Tls(_)), "{err}");
        assert!(
            err.to_string()
                .starts_with("smtp.example.org's security certificate"),
            "{err}"
        );
    }

    #[test]
    fn unchecked_certificates_only_for_local_servers() {
        assert!(check_unverified_peer("localhost", "127.0.0.1".parse().unwrap()).is_ok());
        assert!(check_unverified_peer("nas", "192.168.1.20".parse().unwrap()).is_ok());
        assert!(check_unverified_peer("nas", "fd00::20".parse().unwrap()).is_ok());
        let err = check_unverified_peer("imap.example.org", "93.184.215.14".parse().unwrap())
            .unwrap_err();
        assert!(matches!(err, Error::Tls(_)), "{err}");
        assert!(
            err.to_string().contains("accept invalid certificates"),
            "{err}"
        );
    }
}
