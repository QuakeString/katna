// SPDX-License-Identifier: GPL-3.0-or-later

//! The sign-in and refresh flows against a local fake OAuth server; no
//! real provider is ever asked.

use std::{
    io::{Read, Write},
    net::{TcpListener as StdListener, TcpStream as StdStream},
    sync::{Arc, Mutex},
    thread,
};

use super::*;

/// `{"email":"ada@gmail.com","name":"Ada","picture":"https://lh3.example/a.png"}`
const ID_PAYLOAD: &str = "eyJlbWFpbCI6ImFkYUBnbWFpbC5jb20iLCJuYW1lIjoiQWRhIiwicGljdHVyZSI6Imh0dHBzOi8vbGgzLmV4YW1wbGUvYS5wbmcifQ";

/// A token endpoint that gives `answers` in turn, one per connection, and
/// keeps the form bodies it was sent.
struct FakeServer {
    url: String,
    forms: Arc<Mutex<Vec<String>>>,
}

fn fake_server(answers: Vec<(u16, String)>) -> FakeServer {
    let listener = StdListener::bind("127.0.0.1:0").unwrap();
    let url = format!(
        "http://127.0.0.1:{}/token",
        listener.local_addr().unwrap().port()
    );
    let forms = Arc::new(Mutex::new(Vec::new()));
    let seen = forms.clone();
    thread::spawn(move || {
        for (status, body) in answers {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_http(&mut stream);
            let form = request.split("\r\n\r\n").nth(1).unwrap_or_default();
            seen.lock().unwrap().push(form.to_owned());
            let answer = format!(
                "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\n\
                 Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(answer.as_bytes()).unwrap();
        }
    });
    FakeServer { url, forms }
}

/// Reads a request with its `Content-Length` body.
fn read_http(stream: &mut StdStream) -> String {
    let mut data = Vec::new();
    let mut buf = [0; 4096];
    loop {
        let n = stream.read(&mut buf).unwrap();
        data.extend_from_slice(&buf[..n]);
        let text = String::from_utf8_lossy(&data).into_owned();
        if let Some(end) = text.find("\r\n\r\n") {
            let length = text[..end]
                .lines()
                .find_map(|l| {
                    l.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .map(|v| v.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);
            if data.len() >= end + 4 + length {
                return text;
            }
        }
        if n == 0 {
            return String::from_utf8_lossy(&data).into_owned();
        }
    }
}

fn provider(kind: OAuthProvider, token_url: &str) -> Provider {
    Provider {
        kind,
        auth_url: "https://accounts.test/auth".into(),
        token_url: token_url.into(),
        client_id: "katna-test".into(),
        client_secret: if kind == OAuthProvider::Google {
            "not-secret".into()
        } else {
            String::new()
        },
        scope: "https://mail.test/ openid email".into(),
        redirect_host: "127.0.0.1",
        tls: Tls::insecure_for_local_tests(),
    }
}

fn param(query: &str, name: &str) -> Option<String> {
    query
        .split(['?', '&'])
        .filter_map(|pair| pair.split_once('='))
        .find(|(key, _)| *key == name)
        .map(|(_, value)| unescape(value))
}

/// What a browser does after the provider's page: a GET to the redirect.
fn browse(redirect: &str, query: &str) -> String {
    let authority = redirect
        .strip_prefix("http://")
        .unwrap()
        .trim_end_matches('/');
    let mut stream = StdStream::connect(authority).unwrap();
    write!(stream, "GET /{query} HTTP/1.1\r\nHost: {authority}\r\n\r\n").unwrap();
    let mut answer = String::new();
    stream.read_to_string(&mut answer).unwrap();
    answer
}

fn token_json(refresh: Option<&str>, id_token: bool) -> String {
    let mut json =
        String::from(r#"{"access_token":"at-1","expires_in":3599,"token_type":"Bearer""#);
    if let Some(refresh) = refresh {
        json.push_str(&format!(r#","refresh_token":"{refresh}""#));
    }
    if id_token {
        json.push_str(&format!(r#","id_token":"e30.{ID_PAYLOAD}.c2ln""#));
    }
    json.push('}');
    json
}

fn pages() -> Pages {
    Pages {
        signed_in: "Signed in <ok>".into(),
        failed: "Not signed in".into(),
    }
}

#[test]
fn pkce_challenge_is_s256() {
    // Computed independently: base64url(SHA-256(verifier)).
    assert_eq!(
        challenge("dBjftJeZ4CVP-mJ92K9uQn0OBxJgwG4AjhD9GS-8Fhw"),
        "oMJ6JtxiPIO_tvwifXbDoxkhcM381t_x-c1JV0PcCyM"
    );
    let a = random(48).unwrap();
    assert_eq!(a.len(), 64);
    assert_ne!(a, random(48).unwrap());
}

#[test]
fn reads_who_signed_in() {
    let id = identity(&format!("e30.{ID_PAYLOAD}.c2ln")).unwrap();
    assert_eq!(id.email, "ada@gmail.com");
    assert_eq!(id.name, "Ada");
    assert_eq!(id.picture, "https://lh3.example/a.png");
    // Microsoft: the address may only be the user name.
    let payload = URL_SAFE_NO_PAD
        .encode(r#"{"preferred_username":"kay@outlook.com","picture":"http://x/y"}"#);
    let id = identity(&format!("e30.{payload}.")).unwrap();
    assert_eq!(id.email, "kay@outlook.com");
    assert_eq!(id.picture, "", "only https pictures");
    assert!(identity("garbage").is_none());
}

#[test]
fn query_values_are_decoded() {
    assert_eq!(unescape("4%2F0Ab_x+y%zz"), "4/0Ab_x y%zz");
}

#[test]
fn servers_and_hosts() {
    let (imap, smtp) = Provider::servers(OAuthProvider::Google, "ada@gmail.com");
    assert_eq!((imap.host.as_str(), imap.port), ("imap.gmail.com", 993));
    assert_eq!((smtp.host.as_str(), smtp.port), ("smtp.gmail.com", 465));
    assert_eq!(
        Provider::for_imap_host("imap.gmail.com"),
        Some(OAuthProvider::Google)
    );
    assert_eq!(
        Provider::for_imap_host("Outlook.Office365.com."),
        Some(OAuthProvider::Microsoft)
    );
    assert_eq!(Provider::for_imap_host("evilgmail.com"), None);
    assert_eq!(Provider::for_imap_host("imap.fastmail.com"), None);
}

#[test]
fn sign_in_with_pkce_and_loopback_redirect() {
    let server = fake_server(vec![(200, token_json(Some("rt-1"), true))]);
    let provider = provider(OAuthProvider::Google, &server.url);
    smol::block_on(async {
        let sign_in = SignIn::start(&provider, "ada@gmail.com").await.unwrap();
        let url = sign_in.url().to_owned();
        assert!(url.starts_with("https://accounts.test/auth?response_type=code&"));
        assert_eq!(param(&url, "client_id").unwrap(), "katna-test");
        assert_eq!(param(&url, "login_hint").unwrap(), "ada@gmail.com");
        assert_eq!(param(&url, "access_type").unwrap(), "offline");
        assert_eq!(param(&url, "code_challenge_method").unwrap(), "S256");
        let redirect = param(&url, "redirect_uri").unwrap();
        assert!(redirect.starts_with("http://127.0.0.1:"), "{redirect}");
        let state = param(&url, "state").unwrap();
        let challenge = param(&url, "code_challenge").unwrap();

        let browser = thread::spawn(move || {
            // Stray requests are turned away and the wait goes on.
            assert!(browse(&redirect, "favicon.ico").contains("404"));
            assert!(browse(&redirect, "?code=forged&state=wrong").contains("400"));
            browse(&redirect, &format!("?state={state}&code=4%2F0Ab&scope=x"))
        });
        let grant = sign_in.finish(&provider, &pages()).await.unwrap();
        let page = browser.join().unwrap();
        assert!(page.starts_with("HTTP/1.1 200"));
        assert!(page.contains("Signed in &lt;ok&gt;"), "{page}");

        assert_eq!(grant.access_token, "at-1");
        assert_eq!(grant.refresh_token.as_deref(), Some("rt-1"));
        assert_eq!(grant.expires_in, Duration::from_secs(3599));
        assert_eq!(grant.identity.unwrap().email, "ada@gmail.com");

        let form = server.forms.lock().unwrap()[0].clone();
        assert_eq!(param(&form, "grant_type").unwrap(), "authorization_code");
        assert_eq!(param(&form, "code").unwrap(), "4/0Ab");
        assert_eq!(param(&form, "client_secret").unwrap(), "not-secret");
        assert!(
            param(&form, "redirect_uri")
                .unwrap()
                .starts_with("http://127.0.0.1:")
        );
        let verifier = param(&form, "code_verifier").unwrap();
        assert_eq!(super::challenge(&verifier), challenge);
    });
}

#[test]
fn a_refusal_in_the_browser_ends_the_sign_in() {
    let provider = provider(OAuthProvider::Microsoft, "http://127.0.0.1:9/token");
    smol::block_on(async {
        let sign_in = SignIn::start(&provider, "").await.unwrap();
        let url = sign_in.url().to_owned();
        assert!(param(&url, "login_hint").is_none());
        assert!(param(&url, "access_type").is_none());
        let redirect = param(&url, "redirect_uri").unwrap();
        let state = param(&url, "state").unwrap();
        let browser = thread::spawn(move || {
            browse(&redirect, &format!("?error=access_denied&state={state}"))
        });
        let err = sign_in.finish(&provider, &pages()).await.unwrap_err();
        assert!(matches!(err, Error::Auth(_)), "{err}");
        assert!(browser.join().unwrap().contains("Not signed in"));
    });
}

#[test]
fn no_refresh_token_is_an_error() {
    let server = fake_server(vec![(200, token_json(None, true))]);
    let provider = provider(OAuthProvider::Google, &server.url);
    smol::block_on(async {
        let sign_in = SignIn::start(&provider, "").await.unwrap();
        let url = sign_in.url().to_owned();
        let redirect = param(&url, "redirect_uri").unwrap();
        let state = param(&url, "state").unwrap();
        thread::spawn(move || browse(&redirect, &format!("?code=c&state={state}")));
        let err = sign_in.finish(&provider, &pages()).await.unwrap_err();
        assert!(err.to_string().contains("no refresh token"), "{err}");
    });
}

#[test]
fn token_source_refreshes_caches_and_saves_rotated_tokens() {
    let server = fake_server(vec![(200, token_json(Some("rt-2"), false))]);
    let rotated = Arc::new(Mutex::new(Vec::new()));
    let saved = rotated.clone();
    let tokens = TokenSource::new(
        provider(OAuthProvider::Microsoft, &server.url),
        "rt-1".into(),
        Some(Box::new(move |token| saved.lock().unwrap().push(token))),
    );
    smol::block_on(async {
        assert_eq!(tokens.access_token().await.unwrap(), "at-1");
        // Cached: the server would not answer a second request.
        assert_eq!(tokens.access_token().await.unwrap(), "at-1");
    });
    assert_eq!(*rotated.lock().unwrap(), ["rt-2"]);
    let form = server.forms.lock().unwrap()[0].clone();
    assert_eq!(param(&form, "grant_type").unwrap(), "refresh_token");
    assert_eq!(param(&form, "refresh_token").unwrap(), "rt-1");
    assert_eq!(
        param(&form, "scope").unwrap(),
        "https://mail.test/ openid email"
    );
    assert!(param(&form, "client_secret").is_none());
    assert!(tokens.forget_access_token());
    assert!(!tokens.forget_access_token());
}

#[test]
fn a_revoked_grant_asks_to_sign_in_again() {
    let server = fake_server(vec![
        (
            400,
            r#"{"error":"invalid_grant","error_description":"Token has been expired or revoked."}"#
                .into(),
        ),
        (503, "busy".into()),
    ]);
    let tokens = TokenSource::new(
        provider(OAuthProvider::Google, &server.url),
        "rt".into(),
        None,
    );
    smol::block_on(async {
        let err = tokens.access_token().await.unwrap_err();
        assert!(
            matches!(&err, Error::Auth(m) if m.contains("sign in again")),
            "{err}"
        );
        // A server in trouble is not a reason to sign in again.
        let err = tokens.access_token().await.unwrap_err();
        assert!(matches!(err, Error::Protocol(_)), "{err}");
    });
}

#[test]
fn only_loopback_http() {
    let tls = Tls::insecure_for_local_tests();
    smol::block_on(async {
        for url in ["http://example.org:80/token", "http://127.0.0.1/token"] {
            let err = http::post_form(url, &[], &tls, Duration::from_secs(1))
                .await
                .unwrap_err();
            assert!(err.to_string().contains("only https"), "{url}: {err}");
        }
    });
}

#[test]
fn a_cancelled_refresh_does_not_block_the_next() {
    use futures_lite::FutureExt;

    let listener = StdListener::bind("127.0.0.1:0").unwrap();
    let url = format!(
        "http://127.0.0.1:{}/token",
        listener.local_addr().unwrap().port()
    );
    thread::spawn(move || {
        // The first refresh is never answered; the second is.
        let (first, _) = listener.accept().unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        read_http(&mut stream);
        let body = token_json(None, false);
        let answer = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\
             Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(answer.as_bytes()).unwrap();
        drop(first);
    });
    let tokens = TokenSource::new(provider(OAuthProvider::Google, &url), "rt".into(), None);
    let within = |limit: Duration| async move {
        async_io::Timer::after(limit).await;
        Err(Error::Timeout(limit))
    };
    smol::block_on(async {
        // Dropped mid-refresh, as a stopped worker's would be.
        let cancelled = tokens
            .access_token()
            .or(within(Duration::from_millis(300)))
            .await;
        assert!(matches!(cancelled, Err(Error::Timeout(_))), "{cancelled:?}");
        let token = tokens
            .access_token()
            .or(within(Duration::from_secs(10)))
            .await
            .unwrap();
        assert_eq!(token, "at-1");
    });
}

#[test]
fn a_prefilled_token_is_used_first() {
    let tokens = TokenSource::new(
        provider(OAuthProvider::Google, "http://127.0.0.1:9/t"),
        "rt".into(),
        None,
    )
    .with_access_token("fresh".into(), Duration::from_secs(3600));
    assert_eq!(smol::block_on(tokens.access_token()).unwrap(), "fresh");
}

/// An IMAP server that knows XOAUTH2 and takes only the token `good`.
fn fake_imap(good: &'static str) -> (u16, Arc<Mutex<Vec<String>>>) {
    let listener = StdListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let logins = Arc::new(Mutex::new(Vec::new()));
    let seen = logins.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let seen = seen.clone();
            thread::spawn(move || {
                stream
                    .write_all(b"* OK [CAPABILITY IMAP4rev1 SASL-IR AUTH=XOAUTH2] ready\r\n")
                    .unwrap();
                let mut reader = std::io::BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                while std::io::BufRead::read_line(&mut reader, &mut line).unwrap_or(0) > 0 {
                    let mut words = line.trim_end().splitn(3, ' ');
                    let tag = words.next().unwrap_or("*").to_owned();
                    let command = words.next().unwrap_or("").to_ascii_uppercase();
                    let rest = words.next().unwrap_or("").to_owned();
                    let answer = match command.as_str() {
                        "AUTHENTICATE" => {
                            let b64 = rest.split(' ').nth(1).unwrap_or("");
                            let decoded = String::from_utf8(
                                base64::engine::general_purpose::STANDARD
                                    .decode(b64)
                                    .unwrap(),
                            )
                            .unwrap();
                            seen.lock().unwrap().push(decoded.clone());
                            if decoded
                                == format!("user=ada@gmail.com\x01auth=Bearer {good}\x01\x01")
                            {
                                format!("{tag} OK [CAPABILITY IMAP4rev1] signed in\r\n")
                            } else {
                                format!("{tag} NO [AUTHENTICATIONFAILED] Invalid credentials\r\n")
                            }
                        }
                        "CAPABILITY" => format!("* CAPABILITY IMAP4rev1\r\n{tag} OK done\r\n"),
                        "ID" => format!("* ID NIL\r\n{tag} OK done\r\n"),
                        "LOGOUT" => format!("* BYE bye\r\n{tag} OK done\r\n"),
                        _ => format!("{tag} BAD unknown\r\n"),
                    };
                    if stream.write_all(answer.as_bytes()).is_err() || command == "LOGOUT" {
                        break;
                    }
                    line.clear();
                }
            });
        }
    });
    (port, logins)
}

#[test]
fn imap_logs_in_with_xoauth2_and_renews_a_refused_token() {
    use crate::{Credentials, Endpoint, MailBackend, Security as Sec, imap::ImapBackend};

    let (port, logins) = fake_imap("at-1");
    // The cached token is stale; the refresh brings `at-1`.
    let server = fake_server(vec![(200, token_json(None, false))]);
    let tokens = Arc::new(
        TokenSource::new(
            provider(OAuthProvider::Google, &server.url),
            "rt".into(),
            None,
        )
        .with_access_token("stale".into(), Duration::from_secs(3600)),
    );
    let creds = Credentials::oauth2("ada@gmail.com", tokens);
    let endpoint = Endpoint::new("127.0.0.1", port, Sec::Plain);
    smol::block_on(async {
        let backend = ImapBackend::connect(&endpoint, &creds, Tls::insecure_for_local_tests())
            .await
            .unwrap();
        let _ = backend.logout().await;
    });
    let logins = logins.lock().unwrap();
    assert_eq!(logins.len(), 2, "{logins:?}");
    assert!(logins[0].contains("Bearer stale"));
    assert!(logins[1].contains("Bearer at-1"));
}
