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
    // It stops there: guessed servers would refuse the password anyway.
    assert!(
        !net.asked
            .lock()
            .unwrap()
            .iter()
            .any(|a| a.starts_with("probe"))
    );

    // The same when the MX leads to an OAuth2-only provider.
    let mut net = FakeNet::default();
    net.mx.insert(
        "corp.example".into(),
        vec![dns::Mx {
            preference: 0,
            exchange: "corp-example.mail.protection.outlook.com".into(),
        }],
    );
    net.files
        .insert("https://ispdb.test/v1.1/outlook.com".into(), OAUTH_ONLY);
    let err = discover(&net, "ada@corp.example").unwrap_err();
    assert!(err.contains("OAuth2"), "{err}");
    assert!(
        !net.asked
            .lock()
            .unwrap()
            .iter()
            .any(|a| a.starts_with("probe"))
    );
}

#[test]
fn socket_type_tls_means_implicit_tls() {
    let user = User {
        address: "a@disroot.org",
        local: "a",
        domain: "disroot.org",
    };
    let config = CONFIG.replace(
        "<port>587</port>\n      <socketType>STARTTLS</socketType>",
        "<port>465</port>\n      <socketType>TLS</socketType>",
    );
    assert_ne!(config, CONFIG);
    let (_, smtp) = parse_config(config.as_bytes(), &user).unwrap().unwrap();
    let smtp = smtp.unwrap();
    assert_eq!((smtp.port, smtp.security), (465, Security::Tls));
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
fn cleartext_only_when_nothing_else_is_offered() {
    let user = User {
        address: "a@b.org",
        local: "a",
        domain: "b.org",
    };
    // The file lists cleartext first; the encrypted servers still win.
    let with_plain = CONFIG.replace(
        "<incomingServer type=\"imap\">",
        "<incomingServer type=\"imap\">\n      <hostname>plain.example.org</hostname>\
         <port>143</port><socketType>PLAIN</socketType>\
         <authentication>password-cleartext</authentication>\n    </incomingServer>\n    \
         <incomingServer type=\"imap\">",
    );
    let (imap, smtp) = parse_config(with_plain.as_bytes(), &user).unwrap().unwrap();
    assert_eq!(imap.security, Security::Tls);
    assert_eq!(smtp.unwrap().security, Security::StartTls);

    // Only cleartext: taken, as the provider says (the dialog shows it).
    let only_plain = r#"<clientConfig><emailProvider id="b.org">
        <incomingServer type="imap">
          <hostname>mail.b.org</hostname><port>143</port><socketType>PLAIN</socketType>
          <authentication>password-cleartext</authentication>
        </incomingServer></emailProvider></clientConfig>"#;
    let (imap, _) = parse_config(only_plain.as_bytes(), &user).unwrap().unwrap();
    assert_eq!(imap.security, Security::Plain);
}

#[test]
fn srv_targets_stay_in_the_domain() {
    let record = |target: &str| {
        vec![dns::Srv {
            priority: 0,
            weight: 0,
            port: 993,
            target: target.into(),
        }]
    };
    // A spoofed answer pointing elsewhere is ignored; guessing follows.
    let mut net = FakeNet::default();
    net.srv.insert(
        "_imaps._tcp.example.org".into(),
        record("imap.attacker.net"),
    );
    net.srv.insert(
        "_submissions._tcp.example.org".into(),
        record("example.org.attacker.net"),
    );
    assert!(discover(&net, "ada@example.org").is_err());

    // Later records under the domain are still used.
    let mut records = record("mail.attacker.net");
    records.extend(record("imap.example.org"));
    let mut net = FakeNet::default();
    net.srv.insert("_imaps._tcp.example.org".into(), records);
    let found = discover(&net, "ada@example.org").unwrap();
    assert_eq!(found.source, Source::DnsSrv);
    assert_eq!(found.imap.host, "imap.example.org");

    // The domain itself, and providers Katna knows, are fine.
    for target in ["example.org", "imap.gmail.com", "outlook.office365.com"] {
        let mut net = FakeNet::default();
        net.srv
            .insert("_imaps._tcp.example.org".into(), record(target));
        let found = discover(&net, "ada@example.org").unwrap();
        assert_eq!(found.imap.host, target);
    }
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

#[test]
fn google_and_microsoft_offer_oauth() {
    let net = FakeNet::default();
    let gmail = discover(&net, "ada@gmail.com").unwrap();
    assert_eq!(gmail.oauth, Some(OAuthProvider::Google));
    assert!(gmail.password, "Gmail also takes app passwords");

    let outlook = discover(&net, "ada@outlook.com").unwrap();
    assert_eq!(outlook.oauth, Some(OAuthProvider::Microsoft));
    assert!(!outlook.password);
    assert_eq!(outlook.imap.host, "outlook.office365.com");
    assert_eq!(outlook.imap.username, "ada@outlook.com");
    assert_eq!(outlook.smtp.unwrap().security, Security::StartTls);

    // A Microsoft 365 domain whose MX leads to an OAuth2-only ISPDB entry.
    let mut net = FakeNet::default();
    net.mx.insert(
        "corp.example".into(),
        vec![dns::Mx {
            preference: 0,
            exchange: "corp-example.mail.protection.outlook.com".into(),
        }],
    );
    let office = OAUTH_ONLY.replace("imap.x.org", "outlook.office365.com");
    net.files.insert(
        "https://ispdb.test/v1.1/outlook.com".into(),
        Box::leak(office.into_boxed_str()),
    );
    let found = discover(&net, "ada@corp.example").unwrap();
    assert_eq!(found.source, Source::Mx);
    assert_eq!(found.oauth, Some(OAuthProvider::Microsoft));
    assert!(!found.password);
    assert_eq!(found.imap.username, "ada@corp.example");
}
