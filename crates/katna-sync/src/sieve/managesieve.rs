// SPDX-License-Identifier: GPL-3.0-or-later

//! A ManageSieve client (RFC 5804): logs in like the account's IMAP
//! connection (SASL PLAIN, or XOAUTH2 / OAUTHBEARER with its access
//! token) after STARTTLS, lists, reads and writes scripts, and makes one
//! active. Only what Katna's rules need.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use crate::backend::Login;
use crate::{Credentials, Error, Result, Security, net::Conn, net::Tls};

/// ManageSieve's port.
pub const PORT: u16 = 4190;

/// Longest answer read, in bytes.
const MAX_ANSWER: usize = 4 * 1024 * 1024;

/// What the server says it has.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Capabilities {
    pub implementation: String,
    /// SASL mechanisms, uppercase.
    pub sasl: Vec<String>,
    /// Sieve extensions, lowercase.
    pub sieve: Vec<String>,
    pub starttls: bool,
}

/// A word of an answer.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Atom(String),
    /// A quoted string or a literal.
    Text(String),
    /// A response code, `(…)`, without the parentheses.
    Code(String),
}

/// How an answer ended.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    Ok,
    No,
    Bye,
}

/// An answer: its data lines and how it ended, with the server's words.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Answer {
    lines: Vec<Vec<Token>>,
    status: Status,
    code: Option<String>,
    text: String,
}

/// A logged-in ManageSieve session.
pub struct ManageSieve {
    conn: Conn,
    /// Bytes read, not yet parsed.
    buf: Vec<u8>,
    capabilities: Capabilities,
}

impl ManageSieve {
    /// Connects to `host`:`port`, upgrades with STARTTLS unless
    /// `security` is [`Security::Plain`] (local test servers), and logs
    /// in. A refused access token is renewed once.
    pub async fn connect(
        host: &str,
        port: u16,
        security: Security,
        creds: &Credentials,
        tls: Tls,
    ) -> Result<Self> {
        match Self::connect_once(host, port, security, creds, tls.clone()).await {
            Err(Error::Auth(_)) if creds.retry_after_refusal() => {
                Self::connect_once(host, port, security, creds, tls).await
            }
            result => result,
        }
    }

    async fn connect_once(
        host: &str,
        port: u16,
        security: Security,
        creds: &Credentials,
        tls: Tls,
    ) -> Result<Self> {
        let mut conn = Conn::new(tls);
        conn.connect_tcp(host, port).await?;
        let mut session = Self {
            conn,
            buf: Vec::new(),
            capabilities: Capabilities::default(),
        };
        let greeting = session.answer().await?;
        session.capabilities = capabilities(&greeting)?;
        if security != Security::Plain {
            if !session.capabilities.starttls {
                return Err(Error::Tls(format!(
                    "{host}:{port} offers ManageSieve without STARTTLS"
                )));
            }
            session.command(b"STARTTLS\r\n").await?;
            session.buf.clear();
            session.conn.upgrade_tls().await?;
            // RFC 5804 §2.2: the capabilities again, now over TLS.
            let again = session.answer().await?;
            session.capabilities = capabilities(&again)?;
        }
        session.authenticate(creds).await?;
        Ok(session)
    }

    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    async fn authenticate(&mut self, creds: &Credentials) -> Result<()> {
        let has = |name: &str| self.capabilities.sasl.iter().any(|m| m == name);
        let (mechanism, response) = match creds.login().await? {
            Login::Password(password) => {
                if !has("PLAIN") {
                    return Err(Error::Auth("the Sieve server takes no password".into()));
                }
                ("PLAIN", format!("\0{}\0{password}", creds.user))
            }
            Login::Token(token) if has("XOAUTH2") => (
                "XOAUTH2",
                format!("user={}\x01auth=Bearer {token}\x01\x01", creds.user),
            ),
            Login::Token(token) if has("OAUTHBEARER") => (
                "OAUTHBEARER",
                format!("n,a={},\x01auth=Bearer {token}\x01\x01", creds.user),
            ),
            Login::Token(_) => {
                return Err(Error::Auth("the Sieve server takes no OAuth2 token".into()));
            }
        };
        let command = format!(
            "AUTHENTICATE {} {}\r\n",
            quote(mechanism),
            quote(&STANDARD.encode(response))
        );
        self.conn.write_all(command.as_bytes()).await?;
        loop {
            let line = self.line().await?;
            match status_of(&line) {
                Some((Status::Ok, ..)) => return Ok(()),
                Some((_, _, text)) => {
                    return Err(Error::Auth(if text.is_empty() {
                        "the Sieve server refused the login".into()
                    } else {
                        format!("the Sieve server refused the login: {text}")
                    }));
                }
                // A challenge: an OAuth2 error in JSON. "*" gives up.
                None => self.conn.write_all(b"\"*\"\r\n").await?,
            }
        }
    }

    /// The scripts, each with whether it is the active one.
    pub async fn list(&mut self) -> Result<Vec<(String, bool)>> {
        let answer = self.command(b"LISTSCRIPTS\r\n").await?;
        Ok(answer
            .lines
            .iter()
            .filter_map(|line| match line.as_slice() {
                [Token::Text(name), rest @ ..] => Some((
                    name.clone(),
                    rest.iter()
                        .any(|t| matches!(t, Token::Atom(a) if a.eq_ignore_ascii_case("ACTIVE"))),
                )),
                _ => None,
            })
            .collect())
    }

    /// Script `name`'s text.
    pub async fn get(&mut self, name: &str) -> Result<String> {
        let answer = self
            .command(format!("GETSCRIPT {}\r\n", quote(name)).as_bytes())
            .await?;
        answer
            .lines
            .iter()
            .find_map(|line| match line.as_slice() {
                [Token::Text(text)] => Some(text.clone()),
                _ => None,
            })
            .ok_or_else(|| Error::Protocol(format!("the Sieve server sent no script {name:?}")))
    }

    /// Writes script `name`; the server checks it first and refuses one
    /// it can't run.
    pub async fn put(&mut self, name: &str, script: &str) -> Result<()> {
        let mut command =
            format!("PUTSCRIPT {} {{{}+}}\r\n", quote(name), script.len()).into_bytes();
        command.extend_from_slice(script.as_bytes());
        command.extend_from_slice(b"\r\n");
        self.command(&command).await.map(drop)
    }

    /// Makes script `name` the one that runs.
    pub async fn set_active(&mut self, name: &str) -> Result<()> {
        self.command(format!("SETACTIVE {}\r\n", quote(name)).as_bytes())
            .await
            .map(drop)
    }

    pub async fn logout(mut self) -> Result<()> {
        let _ = self.command(b"LOGOUT\r\n").await;
        self.conn.close().await
    }

    /// Sends `command` and reads its answer; `NO` is
    /// [`Error::Rejected`] with the server's words.
    async fn command(&mut self, command: &[u8]) -> Result<Answer> {
        self.conn.write_all(command).await?;
        let answer = self.answer().await?;
        match answer.status {
            Status::Ok => Ok(answer),
            Status::No => Err(Error::Rejected(
                match (answer.text.is_empty(), answer.code) {
                    (true, None) => "the Sieve server refused".into(),
                    (true, Some(code)) => format!("the Sieve server refused ({code})"),
                    (false, _) => answer.text,
                },
            )),
            Status::Bye => Err(Error::Closed(format!(
                "the Sieve server hung up: {}",
                answer.text
            ))),
        }
    }

    /// Lines up to the one that ends the answer.
    async fn answer(&mut self) -> Result<Answer> {
        let mut lines = Vec::new();
        loop {
            let line = self.line().await?;
            if let Some((status, code, text)) = status_of(&line) {
                return Ok(Answer {
                    lines,
                    status,
                    code,
                    text,
                });
            }
            lines.push(line);
        }
    }

    /// The next line, reading more as needed.
    async fn line(&mut self) -> Result<Vec<Token>> {
        loop {
            match parse_line(&self.buf)? {
                Some((tokens, used)) => {
                    self.buf.drain(..used);
                    return Ok(tokens);
                }
                None => {
                    if self.buf.len() > MAX_ANSWER {
                        return Err(Error::Protocol(
                            "the Sieve server's answer is too long".into(),
                        ));
                    }
                    let read = self.conn.read().await?;
                    if read.is_empty() {
                        return Err(Error::Closed(
                            "the Sieve server closed the connection".into(),
                        ));
                    }
                    self.buf.extend_from_slice(read);
                }
            }
        }
    }
}

/// The capabilities in a greeting (or the answer to `CAPABILITY`).
fn capabilities(answer: &Answer) -> Result<Capabilities> {
    if answer.status != Status::Ok {
        return Err(Error::Rejected(format!(
            "the Sieve server is not ready: {}",
            answer.text
        )));
    }
    let mut caps = Capabilities::default();
    for line in &answer.lines {
        let (Some(Token::Text(name)), value) = (line.first(), line.get(1)) else {
            continue;
        };
        let value = match value {
            Some(Token::Text(value)) => value.as_str(),
            _ => "",
        };
        match name.to_ascii_uppercase().as_str() {
            "IMPLEMENTATION" => caps.implementation = value.to_owned(),
            "SASL" => {
                caps.sasl = value
                    .split_whitespace()
                    .map(str::to_ascii_uppercase)
                    .collect()
            }
            "SIEVE" => {
                caps.sieve = value
                    .split_whitespace()
                    .map(str::to_ascii_lowercase)
                    .collect()
            }
            "STARTTLS" => caps.starttls = true,
            _ => {}
        }
    }
    Ok(caps)
}

/// Whether a line ends an answer: its status, response code and words.
fn status_of(line: &[Token]) -> Option<(Status, Option<String>, String)> {
    let Some(Token::Atom(word)) = line.first() else {
        return None;
    };
    let status = match word.to_ascii_uppercase().as_str() {
        "OK" => Status::Ok,
        "NO" => Status::No,
        "BYE" => Status::Bye,
        _ => return None,
    };
    let mut code = None;
    let mut text = String::new();
    for token in &line[1..] {
        match token {
            Token::Code(c) => code = Some(c.clone()),
            Token::Text(t) => text = t.clone(),
            Token::Atom(_) => {}
        }
    }
    Some((status, code, text))
}

/// One line at the start of `buf`, with how many bytes it took; `None`
/// when it hasn't all arrived.
fn parse_line(buf: &[u8]) -> Result<Option<(Vec<Token>, usize)>> {
    let bad = |what: &str| Error::Protocol(format!("the Sieve server sent {what}"));
    let mut tokens = Vec::new();
    let mut i = 0;
    loop {
        let Some(&c) = buf.get(i) else {
            return Ok(None);
        };
        match c {
            b' ' => i += 1,
            b'\r' => {
                if buf.len() < i + 2 {
                    return Ok(None);
                }
                if buf[i + 1] != b'\n' {
                    return Err(bad("a bare CR"));
                }
                return Ok(Some((tokens, i + 2)));
            }
            b'\n' => return Ok(Some((tokens, i + 1))),
            b'"' => {
                let mut text = Vec::new();
                i += 1;
                loop {
                    match buf.get(i) {
                        None => return Ok(None),
                        Some(b'\\') => {
                            let Some(&next) = buf.get(i + 1) else {
                                return Ok(None);
                            };
                            text.push(next);
                            i += 2;
                        }
                        Some(b'"') => {
                            i += 1;
                            break;
                        }
                        Some(b'\r' | b'\n') => return Err(bad("a line break in a quoted string")),
                        Some(&b) => {
                            text.push(b);
                            i += 1;
                        }
                    }
                }
                tokens.push(Token::Text(String::from_utf8_lossy(&text).into_owned()));
            }
            b'{' => {
                let Some(close) = buf[i..].iter().position(|&b| b == b'}') else {
                    return Ok(None);
                };
                let digits = std::str::from_utf8(&buf[i + 1..i + close])
                    .map_err(|_| bad("a broken literal"))?
                    .trim_end_matches('+');
                let len: usize = digits.parse().map_err(|_| bad("a broken literal"))?;
                if len > MAX_ANSWER {
                    return Err(bad("a literal too long"));
                }
                let start = i + close + 1;
                if buf.len() < start + 2 {
                    return Ok(None);
                }
                if &buf[start..start + 2] != b"\r\n" {
                    return Err(bad("a literal without its line break"));
                }
                let end = start + 2 + len;
                if buf.len() < end {
                    return Ok(None);
                }
                tokens.push(Token::Text(
                    String::from_utf8_lossy(&buf[start + 2..end]).into_owned(),
                ));
                i = end;
            }
            b'(' => {
                let Some(close) = buf[i..].iter().position(|&b| b == b')') else {
                    return Ok(None);
                };
                tokens.push(Token::Code(
                    String::from_utf8_lossy(&buf[i + 1..i + close]).into_owned(),
                ));
                i += close + 1;
            }
            _ => {
                let len = buf[i..]
                    .iter()
                    .position(|&b| matches!(b, b' ' | b'\r' | b'\n'))
                    .unwrap_or(buf.len() - i);
                if i + len == buf.len() {
                    return Ok(None);
                }
                tokens.push(Token::Atom(
                    String::from_utf8_lossy(&buf[i..i + len]).into_owned(),
                ));
                i += len;
            }
        }
    }
}

/// A ManageSieve quoted string.
fn quote(text: &str) -> String {
    super::quote(text)
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn reads_strings_literals_and_codes() {
        let data = b"\"SIEVE\" \"fileinto regex\"\r\n{11}\r\nif true {}\n\r\nNO (QUOTA/MAXSIZE) \"Too big\"\r\n";
        let (first, used) = parse_line(data).unwrap().unwrap();
        assert_eq!(
            first,
            [
                Token::Text("SIEVE".into()),
                Token::Text("fileinto regex".into())
            ]
        );
        let (second, more) = parse_line(&data[used..]).unwrap().unwrap();
        assert_eq!(second, [Token::Text("if true {}\n".into())]);
        let (third, _) = parse_line(&data[used + more..]).unwrap().unwrap();
        assert_eq!(
            status_of(&third),
            Some((Status::No, Some("QUOTA/MAXSIZE".into()), "Too big".into()))
        );
        // Not all there yet.
        assert_eq!(parse_line(b"\"SIEVE\" \"file").unwrap(), None);
        assert_eq!(parse_line(b"{11}\r\nif tr").unwrap(), None);
        assert_eq!(parse_line(b"OK").unwrap(), None);
        assert!(parse_line(b"\"a\rb\"\r\n").is_err());
    }

    /// Its port, the commands it got and its scripts (name, text, active).
    type Fake = (
        u16,
        Arc<Mutex<Vec<String>>>,
        Arc<Mutex<Vec<(String, String, bool)>>>,
    );

    /// A ManageSieve server on the loopback that takes `alice` / `secret`,
    /// keeps scripts and logs every command.
    fn fake_server() -> Fake {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let log = Arc::new(Mutex::new(Vec::new()));
        let scripts = Arc::new(Mutex::new(vec![(
            "old".to_owned(),
            "keep;".to_owned(),
            true,
        )]));
        let (seen, kept) = (log.clone(), scripts.clone());
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let mut out = stream.try_clone().unwrap();
                let mut reader = BufReader::new(stream);
                out.write_all(
                    b"\"IMPLEMENTATION\" \"Fake\"\r\n\"SASL\" \"PLAIN\"\r\n\
                      \"SIEVE\" \"fileinto imap4flags include\"\r\nOK \"Ready\"\r\n",
                )
                .unwrap();
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap() == 0 {
                        break;
                    }
                    let line = line.trim_end().to_owned();
                    seen.lock().unwrap().push(line.clone());
                    let mut scripts = kept.lock().unwrap();
                    let answer: String = if let Some(rest) =
                        line.strip_prefix("AUTHENTICATE \"PLAIN\" ")
                    {
                        let wanted = STANDARD.encode("\0alice\0secret");
                        if rest.trim_matches('"') == wanted {
                            "OK\r\n".into()
                        } else {
                            "NO \"Bad password\"\r\n".into()
                        }
                    } else if line == "LISTSCRIPTS" {
                        let mut text: String = scripts
                            .iter()
                            .map(|(n, _, a)| {
                                format!("\"{n}\"{}\r\n", if *a { " ACTIVE" } else { "" })
                            })
                            .collect();
                        text.push_str("OK\r\n");
                        text
                    } else if let Some(rest) = line.strip_prefix("PUTSCRIPT ") {
                        let (name, size) = rest.split_once(' ').unwrap();
                        let size: usize = size.trim_matches(['{', '}', '+']).parse().unwrap();
                        let mut body = vec![0; size + 2];
                        reader.read_exact(&mut body).unwrap();
                        let text = String::from_utf8(body[..size].to_vec()).unwrap();
                        let name = name.trim_matches('"').to_owned();
                        if text.contains("vacation") {
                            "NO \"line 1: unknown command vacation\"\r\n".into()
                        } else {
                            scripts.retain(|(n, ..)| *n != name);
                            scripts.push((name, text, false));
                            "OK\r\n".into()
                        }
                    } else if let Some(name) = line.strip_prefix("GETSCRIPT ") {
                        let name = name.trim_matches('"');
                        match scripts.iter().find(|(n, ..)| n == name) {
                            Some((_, text, _)) => format!("{{{}}}\r\n{text}\r\nOK\r\n", text.len()),
                            None => "NO (NONEXISTENT) \"No script\"\r\n".into(),
                        }
                    } else if let Some(name) = line.strip_prefix("SETACTIVE ") {
                        let name = name.trim_matches('"');
                        for script in scripts.iter_mut() {
                            script.2 = script.0 == name;
                        }
                        "OK\r\n".into()
                    } else if line == "LOGOUT" {
                        out.write_all(b"OK \"Bye\"\r\n").unwrap();
                        break;
                    } else {
                        "NO \"Unknown\"\r\n".into()
                    };
                    drop(scripts);
                    out.write_all(answer.as_bytes()).unwrap();
                }
            }
        });
        (port, log, scripts)
    }

    #[test]
    fn logs_in_writes_and_activates_a_script() {
        let (port, log, scripts) = fake_server();
        smol::block_on(async {
            let creds = Credentials::new("alice", "secret");
            let tls = Tls::insecure_for_local_tests();
            let mut sieve = ManageSieve::connect("127.0.0.1", port, Security::Plain, &creds, tls)
                .await
                .unwrap();
            assert_eq!(
                sieve.capabilities().sieve,
                ["fileinto", "imap4flags", "include"]
            );
            assert_eq!(sieve.list().await.unwrap(), [("old".to_owned(), true)]);
            let script = "require \"fileinto\";\r\nif header :contains \"subject\" \"x\" {\n    fileinto \"Bills\";\n}\n";
            sieve.put("katna", script).await.unwrap();
            assert_eq!(sieve.get("katna").await.unwrap(), script);
            sieve.set_active("katna").await.unwrap();
            assert_eq!(
                sieve.list().await.unwrap(),
                [("old".to_owned(), false), ("katna".to_owned(), true)]
            );
            // The server's own check refuses what it can't run.
            match sieve.put("katna", "vacation \"x\";").await {
                Err(Error::Rejected(why)) => assert!(why.contains("vacation"), "{why}"),
                other => panic!("{other:?}"),
            }
            assert!(matches!(sieve.get("gone").await, Err(Error::Rejected(_))));
            sieve.logout().await.unwrap();

            let wrong = Credentials::new("alice", "wrong");
            let tls = Tls::insecure_for_local_tests();
            assert!(matches!(
                ManageSieve::connect("127.0.0.1", port, Security::Plain, &wrong, tls).await,
                Err(Error::Auth(_))
            ));
            // A server without STARTTLS is only for plain test setups.
            let tls = Tls::insecure_for_local_tests();
            assert!(matches!(
                ManageSieve::connect("127.0.0.1", port, Security::StartTls, &creds, tls).await,
                Err(Error::Tls(_))
            ));
        });
        let log = log.lock().unwrap();
        assert!(log.iter().any(|l| l.starts_with("PUTSCRIPT \"katna\" {")));
        assert_eq!(scripts.lock().unwrap().len(), 2, "the old script is kept");
    }
}
