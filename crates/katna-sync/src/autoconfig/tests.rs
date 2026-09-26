// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::*;

/// A network made of maps; records every URL and probe.
#[derive(Default)]
struct FakeNet {
    files: HashMap<String, &'static str>,
    srv: HashMap<String, Vec<dns::Srv>>,
    mx: HashMap<String, Vec<dns::Mx>>,
    /// `host:port` that greet.
    open: Vec<String>,
    asked: Arc<Mutex<Vec<String>>>,
}

impl Network for FakeNet {
    fn ispdb(&self) -> &str {
        "https://ispdb.test/v1.1/"
    }

    async fn fetch(&self, url: String) -> Option<Vec<u8>> {
        self.asked.lock().unwrap().push(url.clone());
        self.files.get(&url).map(|body| body.as_bytes().to_vec())
    }

    async fn srv(&self, name: String) -> Vec<dns::Srv> {
        self.srv.get(&name).cloned().unwrap_or_default()
    }

    async fn mx(&self, name: String) -> Vec<dns::Mx> {
        self.mx.get(&name).cloned().unwrap_or_default()
    }

    async fn probe(&self, server: Server, _protocol: Protocol) -> bool {
        let at = format!("{}:{}", server.host, server.port);
        self.asked.lock().unwrap().push(format!("probe {at}"));
        self.open.contains(&at)
    }
}

const CONFIG: &str = r#"<?xml version="1.0"?>
<clientConfig version="1.1">
  <emailProvider id="example.org">
    <domain>example.org</domain>
    <incomingServer type="pop3">
      <hostname>pop.example.org</hostname><port>995</port>
      <socketType>SSL</socketType><username>%EMAILADDRESS%</username>
      <authentication>password-cleartext</authentication>
    </incomingServer>
    <incomingServer type="imap">
      <hostname>imap.example.org</hostname><port>143</port>
      <socketType>STARTTLS</socketType><username>%EMAILLOCALPART%</username>
      <authentication>password-cleartext</authentication>
    </incomingServer>
    <incomingServer type="imap">
      <hostname>IMAPS.%EMAILDOMAIN%</hostname><port>993</port>
      <socketType>SSL</socketType><username>%EMAILLOCALPART%</username>
      <authentication>password-cleartext</authentication>
    </incomingServer>
    <outgoingServer type="smtp">
      <hostname>smtp.example.org</hostname><port>587</port>
      <socketType>STARTTLS</socketType><username>%EMAILADDRESS%</username>
      <authentication>password-encrypted</authentication>
    </outgoingServer>
  </emailProvider>
</clientConfig>"#;

const OAUTH_ONLY: &str = r#"<clientConfig><emailProvider id="x">
    <incomingServer type="imap">
      <hostname>imap.x.org</hostname><port>993</port><socketType>SSL</socketType>
      <authentication>OAuth2</authentication>
    </incomingServer></emailProvider></clientConfig>"#;

fn discover(net: &FakeNet, address: &str) -> std::result::Result<Discovered, String> {
    smol::block_on(discover_with(net, address))
}

fn provider_url() -> String {
    "https://autoconfig.example.org/mail/config-v1.1.xml?emailaddress=ada%2Bx%40example.org"
        .to_owned()
}

#[test]
fn built_in_providers_need_no_network() {
    let net = FakeNet::default();
    let found = discover(&net, "Ada@Gmail.com").unwrap();
    assert_eq!(found.source, Source::BuiltIn);
    assert_eq!(
        (
            found.imap.host.as_str(),
            found.imap.port,
            found.imap.username.as_str()
        ),
        ("imap.gmail.com", 993, "Ada@Gmail.com")
    );
    let icloud = discover(&net, "ada@icloud.com").unwrap();
    assert_eq!(icloud.imap.username, "ada");
    assert_eq!(icloud.smtp.unwrap().port, 587);
    assert!(net.asked.lock().unwrap().is_empty());
    assert!(discover(&net, "no-at-sign").is_err());
}

#[test]
fn provider_file_beats_the_ispdb() {
    let mut net = FakeNet::default();
    net.files.insert(provider_url(), CONFIG);
    net.files
        .insert("https://ispdb.test/v1.1/example.org".into(), OAUTH_ONLY);
    let found = discover(&net, "ada+x@example.org").unwrap();
    assert_eq!(found.source, Source::Provider);
    assert_eq!(
        found.imap,
        server("imaps.example.org", 993, Security::Tls, "ada+x")
    );
    assert_eq!(
        found.smtp.unwrap(),
        server(
            "smtp.example.org",
            587,
            Security::StartTls,
            "ada+x@example.org"
        )
    );
    // All three files were asked for at once.
    assert_eq!(net.asked.lock().unwrap().len(), 3);
}

#[test]
fn ispdb_then_srv_then_mx_then_guess() {
    let mut net = FakeNet::default();
    net.files
        .insert("https://ispdb.test/v1.1/example.org".into(), CONFIG);
    assert_eq!(
        discover(&net, "ada@example.org").unwrap().source,
        Source::Ispdb
    );

    let mut net = FakeNet::default();
    net.srv.insert(
        "_imap._tcp.example.org".into(),
        vec![dns::Srv {
            priority: 0,
            weight: 0,
            port: 143,
            target: "mail.example.org".into(),
        }],
    );
    net.srv.insert(
        "_submissions._tcp.example.org".into(),
        vec![dns::Srv {
            priority: 0,
            weight: 0,
            port: 465,
            target: "out.example.org".into(),
        }],
    );
    let found = discover(&net, "ada@example.org").unwrap();
    assert_eq!(found.source, Source::DnsSrv);
    assert_eq!(
        found.imap,
        server(
            "mail.example.org",
            143,
            Security::StartTls,
            "ada@example.org"
        )
    );
    assert_eq!(found.smtp.unwrap().security, Security::Tls);

    // A hosted domain: the MX names the provider.
    let mut net = FakeNet::default();
    net.mx.insert(
        "shop.example".into(),
        vec![dns::Mx {
            preference: 10,
            exchange: "in1-smtp.messagingengine.com".into(),
        }],
    );
    net.files
        .insert("https://ispdb.test/v1.1/messagingengine.com".into(), CONFIG);
    let found = discover(&net, "ada@shop.example").unwrap();
    assert_eq!(found.source, Source::Mx);
    assert_eq!(found.imap.username, "ada");

    let net = FakeNet {
        open: vec!["mail.example.net:993".into(), "smtp.example.net:587".into()],
        ..FakeNet::default()
    };
    let found = discover(&net, "ada@example.net").unwrap();
    assert_eq!(found.source, Source::Guess);
    assert_eq!(found.imap.host, "mail.example.net");
    assert_eq!(found.smtp.unwrap().security, Security::StartTls);
    let probes = net
        .asked
        .lock()
        .unwrap()
        .iter()
        .filter(|a| a.starts_with("probe"))
        .count();
    assert_eq!(probes, 8);
}

#[test]
fn nothing_found_says_why() {
    let net = FakeNet::default();
    let err = discover(&net, "ada@nowhere.example").unwrap_err();
    assert!(err.contains("give them by hand"), "{err}");

    let mut net = FakeNet::default();
    net.files
        .insert("https://ispdb.test/v1.1/x.org".into(), OAUTH_ONLY);
    let err = discover(&net, "ada@x.org").unwrap_err();
    assert!(err.contains("OAuth2"), "{err}");
}

#[test]
fn config_files_are_checked() {
    let user = User {
        address: "a@b.org",
        local: "a",
        domain: "b.org",
    };
    assert!(parse_config(b"<clientConfig/>", &user).is_err());
    assert!(parse_config(b"not xml", &user).is_err());
    let bad_host = CONFIG.replace("IMAPS.%EMAILDOMAIN%", "evil host/");
    let (imap, _) = parse_config(bad_host.as_bytes(), &user).unwrap().unwrap();
    assert_eq!(imap.host, "imap.example.org", "the bad entry is skipped");
}

#[test]
fn base_domains() {
    assert_eq!(base_domain("aspmx.l.google.com."), "google.com");
    assert_eq!(base_domain("mx1.mail.example.co.uk"), "example.co.uk");
    assert_eq!(base_domain("example.org"), "example.org");
}

#[test]
fn probes_read_the_greeting() {
    use futures_lite::{AsyncWriteExt, FutureExt};
    smol::block_on(async {
        let imap = async_net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let smtp = async_net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let (imap_port, smtp_port) = (
            imap.local_addr().unwrap().port(),
            smtp.local_addr().unwrap().port(),
        );
        let greet = |listener: async_net::TcpListener, line: &'static [u8]| async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                stream.write_all(line).await.unwrap();
            }
        };
        let net = Discovery {
            ispdb: ISPDB.into(),
            resolvers: Vec::new(),
            tls: Tls::insecure_for_local_tests(),
            timeout: Duration::from_secs(2),
        };
        let at = |port| server("127.0.0.1", port, Security::StartTls, "a");
        let checks = async {
            assert!(net.probe(at(imap_port), Protocol::Imap).await);
            assert!(!net.probe(at(imap_port), Protocol::Smtp).await);
            assert!(net.probe(at(smtp_port), Protocol::Smtp).await);
            let closed = std::net::TcpListener::bind("127.0.0.1:0")
                .unwrap()
                .local_addr()
                .unwrap()
                .port();
            assert!(!net.probe(at(closed), Protocol::Imap).await);
        };
        checks
            .or(greet(imap, b"* OK IMAP4rev1 ready\r\n"))
            .or(greet(smtp, b"220 mail.example.org ESMTP\r\n"))
            .await;
    });
}
