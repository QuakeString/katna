// SPDX-License-Identifier: GPL-3.0-or-later

//! SRV, MX and TXT lookups over UDP to the system's resolver
//! (`/etc/resolv.conf`, or the network adapters' settings on Windows).
//! Small on purpose: one question, no caching, no DNSSEC. The answers only pick which servers to try; the login then
//! goes over TLS to the name found.

use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use async_net::UdpSocket;
use futures_lite::FutureExt;

/// Record types we ask for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Mx = 15,
    Txt = 16,
    Srv = 33,
}

/// An SRV record: where a service runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Srv {
    pub priority: u16,
    pub weight: u16,
    pub port: u16,
    /// Without the final dot. `.` in the record ("not offered") is left out.
    pub target: String,
}

/// An MX record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mx {
    pub preference: u16,
    pub exchange: String,
}

/// Name servers from `/etc/resolv.conf`, or none.
#[cfg(not(windows))]
pub fn system_resolvers() -> Vec<SocketAddr> {
    let Ok(conf) = std::fs::read_to_string("/etc/resolv.conf") else {
        return Vec::new();
    };
    conf.lines()
        .filter_map(|line| line.trim().strip_prefix("nameserver"))
        .filter_map(|rest| name_servers(rest).next())
        .map(|ip| SocketAddr::new(ip, 53))
        .collect()
}

/// Name servers of the network adapters: those set by hand first, then
/// those from DHCP. Windows has no `/etc/resolv.conf`.
#[cfg(windows)]
pub fn system_resolvers() -> Vec<SocketAddr> {
    use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

    let machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut found: Vec<IpAddr> = Vec::new();
    for value in ["NameServer", "DhcpNameServer"] {
        for stack in ["Tcpip", "Tcpip6"] {
            let path = format!(r"SYSTEM\CurrentControlSet\Services\{stack}\Parameters\Interfaces");
            let Ok(interfaces) = machine.open_subkey(path) else {
                continue;
            };
            for name in interfaces.enum_keys().flatten() {
                let Ok(list) = interfaces
                    .open_subkey(&name)
                    .and_then(|key| key.get_value::<String, _>(value))
                else {
                    continue;
                };
                for ip in name_servers(&list) {
                    if !found.contains(&ip) {
                        found.push(ip);
                    }
                }
            }
        }
    }
    found
        .into_iter()
        .map(|ip| SocketAddr::new(ip, 53))
        .collect()
}

/// Addresses in a list split by spaces or commas; a `%zone` is dropped.
fn name_servers(list: &str) -> impl Iterator<Item = IpAddr> + '_ {
    list.split([' ', ',', '\t'])
        .filter_map(|word| word.split('%').next()?.parse::<IpAddr>().ok())
        .filter(|ip| !ip.is_unspecified())
}

/// SRV records of `name`, best first (lowest priority, then highest weight).
pub async fn srv(resolvers: &[SocketAddr], name: &str, timeout: Duration) -> Vec<Srv> {
    let Some(answer) = ask(resolvers, name, Kind::Srv, timeout).await else {
        return Vec::new();
    };
    let mut records: Vec<Srv> = answers(&answer, Kind::Srv)
        .into_iter()
        .filter_map(|(offset, data)| {
            let (target, _) = read_name(&answer, offset + 6)?;
            (!target.is_empty()).then(|| Srv {
                priority: u16::from_be_bytes([data[0], data[1]]),
                weight: u16::from_be_bytes([data[2], data[3]]),
                port: u16::from_be_bytes([data[4], data[5]]),
                target,
            })
        })
        .collect();
    records.sort_by_key(|r| (r.priority, std::cmp::Reverse(r.weight)));
    records
}

/// MX records of `name`, best first.
pub async fn mx(resolvers: &[SocketAddr], name: &str, timeout: Duration) -> Vec<Mx> {
    let Some(answer) = ask(resolvers, name, Kind::Mx, timeout).await else {
        return Vec::new();
    };
    let mut records: Vec<Mx> = answers(&answer, Kind::Mx)
        .into_iter()
        .filter_map(|(offset, data)| {
            let (exchange, _) = read_name(&answer, offset + 2)?;
            Some(Mx {
                preference: u16::from_be_bytes([data[0], data[1]]),
                exchange,
            })
        })
        .collect();
    records.sort_by_key(|r| r.preference);
    records
}

/// TXT records of `name`, each with its strings joined.
pub async fn txt(resolvers: &[SocketAddr], name: &str, timeout: Duration) -> Vec<String> {
    let Some(answer) = ask(resolvers, name, Kind::Txt, timeout).await else {
        return Vec::new();
    };
    answers(&answer, Kind::Txt)
        .into_iter()
        .map(|(_, mut data)| {
            let mut text = String::new();
            while let Some((&len, rest)) = data.split_first() {
                let len = usize::from(len).min(rest.len());
                text.push_str(&String::from_utf8_lossy(&rest[..len]));
                data = &rest[len..];
            }
            text
        })
        .collect()
}

/// Sends the question to each resolver in turn; the first answer wins.
async fn ask(
    resolvers: &[SocketAddr],
    name: &str,
    kind: Kind,
    timeout: Duration,
) -> Option<Vec<u8>> {
    let id = random_id()?;
    let query = question(id, name, kind)?;
    for resolver in resolvers {
        let attempt = async {
            let local: SocketAddr = if resolver.is_ipv4() {
                "0.0.0.0:0".parse().ok()?
            } else {
                "[::]:0".parse().ok()?
            };
            let socket = UdpSocket::bind(local).await.ok()?;
            socket.connect(resolver).await.ok()?;
            socket.send(&query).await.ok()?;
            let mut buf = vec![0; 4096];
            loop {
                let n = socket.recv(&mut buf).await.ok()?;
                let reply = &buf[..n];
                // Another packet on the port, or not our answer: keep waiting.
                if answers_query(reply, &query) {
                    return Some(reply.to_vec());
                }
            }
        };
        let reply = attempt
            .or(async {
                async_io::Timer::after(timeout).await;
                None
            })
            .await;
        match reply {
            // NXDOMAIN and other errors are answers too: nothing there.
            Some(reply) if reply[3] & 0x0f == 0 => return Some(reply),
            Some(_) => return None,
            None => tracing::debug!(%resolver, name, "no DNS answer"),
        }
    }
    None
}

/// A transaction ID nobody off the path can guess, so a forged answer
/// has to be sent to every one of 65 536 IDs (and the random source port)
/// before the real one arrives.
fn random_id() -> Option<u16> {
    use ring::rand::SecureRandom;
    let mut id = [0u8; 2];
    ring::rand::SystemRandom::new().fill(&mut id).ok()?;
    Some(u16::from_be_bytes(id))
}

/// Whether `reply` answers `query`: the same ID, the answer bit set, one
/// question, and that question the same name (in any case), type and
/// class as asked.
fn answers_query(reply: &[u8], query: &[u8]) -> bool {
    let asked = &query[12..];
    reply.len() >= 12
        && reply[..2] == query[..2]
        && reply[2] & 0x80 != 0
        && reply[4..6] == [0, 1]
        && reply
            .get(12..12 + asked.len())
            .is_some_and(|echoed| echoed.eq_ignore_ascii_case(asked))
}

/// A query packet for one question, recursion desired.
fn question(id: u16, name: &str, kind: Kind) -> Option<Vec<u8>> {
    let mut packet = Vec::with_capacity(name.len() + 18);
    packet.extend_from_slice(&id.to_be_bytes());
    packet.extend_from_slice(&[0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0]);
    for label in name.trim_end_matches('.').split('.') {
        if label.is_empty() || label.len() > 63 {
            return None;
        }
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
    packet.extend_from_slice(&(kind as u16).to_be_bytes());
    packet.extend_from_slice(&[0, 1]);
    Some(packet)
}

/// The records of type `kind` in the answer section, as (offset of the
/// data, data).
fn answers(packet: &[u8], kind: Kind) -> Vec<(usize, &[u8])> {
    let mut found = Vec::new();
    let count = |at: usize| {
        packet
            .get(at..at + 2)
            .map(|b| u16::from_be_bytes([b[0], b[1]]))
    };
    let (Some(questions), Some(records)) = (count(4), count(6)) else {
        return found;
    };
    let mut at = 12;
    for _ in 0..questions {
        let Some((_, end)) = read_name(packet, at) else {
            return found;
        };
        at = end + 4;
    }
    for _ in 0..records {
        let Some((_, end)) = read_name(packet, at) else {
            return found;
        };
        let Some(fixed) = packet.get(end..end + 10) else {
            return found;
        };
        let rtype = u16::from_be_bytes([fixed[0], fixed[1]]);
        let length = usize::from(u16::from_be_bytes([fixed[8], fixed[9]]));
        let data_at = end + 10;
        let Some(data) = packet.get(data_at..data_at + length) else {
            return found;
        };
        let long_enough = match kind {
            Kind::Srv => length >= 7,
            Kind::Mx => length >= 3,
            Kind::Txt => length >= 1,
        };
        if rtype == kind as u16 && long_enough {
            found.push((data_at, data));
        }
        at = data_at + length;
    }
    found
}

/// Reads a possibly compressed name at `at`. Returns it without the final
/// dot, and where the name ends in the packet.
fn read_name(packet: &[u8], mut at: usize) -> Option<(String, usize)> {
    let mut name = String::new();
    let mut end = None;
    // Guards against pointer loops.
    for _ in 0..128 {
        let len = *packet.get(at)?;
        match len {
            0 => {
                return Some((name, end.unwrap_or(at + 1)));
            }
            l if l & 0xc0 == 0xc0 => {
                let pointer = usize::from(u16::from_be_bytes([l & 0x3f, *packet.get(at + 1)?]));
                end.get_or_insert(at + 2);
                at = pointer;
            }
            l if l < 64 => {
                let label = packet.get(at + 1..at + 1 + usize::from(l))?;
                if !name.is_empty() {
                    name.push('.');
                }
                name.push_str(&String::from_utf8_lossy(label).to_ascii_lowercase());
                at += 1 + usize::from(l);
            }
            _ => return None,
        }
    }
    None
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn name_server_lists() {
        let ips: Vec<IpAddr> =
            name_servers(" 192.168.1.1,8.8.8.8 fe80::1%12\t0.0.0.0 bogus").collect();
        assert_eq!(
            ips,
            [
                "192.168.1.1".parse::<IpAddr>().unwrap(),
                "8.8.8.8".parse().unwrap(),
                "fe80::1".parse().unwrap(),
            ]
        );
    }

    /// A reply to `query` with the given records (type, data), each named
    /// by a pointer to the question.
    pub(crate) fn reply(query: &[u8], records: &[(Kind, Vec<u8>)]) -> Vec<u8> {
        let mut packet = query.to_vec();
        packet[2] |= 0x80;
        packet[6..8].copy_from_slice(&(records.len() as u16).to_be_bytes());
        for (kind, data) in records {
            packet.extend_from_slice(&[0xc0, 12]);
            packet.extend_from_slice(&(*kind as u16).to_be_bytes());
            packet.extend_from_slice(&[0, 1, 0, 0, 1, 0]);
            packet.extend_from_slice(&(data.len() as u16).to_be_bytes());
            packet.extend_from_slice(data);
        }
        packet
    }

    /// SRV record data with an uncompressed target.
    pub(crate) fn srv_data(priority: u16, weight: u16, port: u16, target: &str) -> Vec<u8> {
        let mut data = Vec::new();
        for n in [priority, weight, port] {
            data.extend_from_slice(&n.to_be_bytes());
        }
        for label in target.split('.').filter(|l| !l.is_empty()) {
            data.push(label.len() as u8);
            data.extend_from_slice(label.as_bytes());
        }
        data.push(0);
        data
    }

    #[test]
    fn parses_srv_and_mx_answers() {
        let query = question(7, "_imaps._tcp.Example.org.", Kind::Srv).unwrap();
        assert_eq!(read_name(&query, 12).unwrap().0, "_imaps._tcp.example.org");
        let packet = reply(
            &query,
            &[
                (Kind::Srv, srv_data(10, 0, 993, "b.example.org")),
                (Kind::Srv, srv_data(0, 5, 993, "a.example.org")),
                (Kind::Srv, srv_data(0, 0, 0, ".")),
            ],
        );
        let found = answers(&packet, Kind::Srv);
        assert_eq!(found.len(), 3);
        let (offset, _) = found[1];
        assert_eq!(read_name(&packet, offset + 6).unwrap().0, "a.example.org");

        // A compressed MX exchange pointing into the question name.
        let query = question(8, "example.org", Kind::Mx).unwrap();
        let mut data = vec![0, 10, 2, b'm', b'x', 0xc0, 12];
        let packet = reply(&query, &[(Kind::Mx, std::mem::take(&mut data))]);
        let (offset, _) = answers(&packet, Kind::Mx)[0];
        assert_eq!(read_name(&packet, offset + 2).unwrap().0, "mx.example.org");
    }

    #[test]
    fn replies_must_echo_the_question() {
        let query = question(0x1234, "_imaps._tcp.example.org", Kind::Srv).unwrap();
        let good = reply(
            &query,
            &[(Kind::Srv, srv_data(0, 0, 993, "imap.example.org"))],
        );
        assert!(answers_query(&good, &query));
        // Resolvers may echo the name in another case.
        let mut upper = good.clone();
        upper[13..19].make_ascii_uppercase();
        assert!(answers_query(&upper, &query));

        let mut wrong_id = good.clone();
        wrong_id[1] ^= 1;
        assert!(!answers_query(&wrong_id, &query));
        let mut not_answer = good.clone();
        not_answer[2] &= 0x7f;
        assert!(!answers_query(&not_answer, &query));
        let other_name = question(0x1234, "_imaps._tcp.evil.example", Kind::Srv).unwrap();
        let spoofed = reply(
            &other_name,
            &[(Kind::Srv, srv_data(0, 0, 993, "imap.evil.example"))],
        );
        assert!(!answers_query(&spoofed, &query));
        let other_type = question(0x1234, "_imaps._tcp.example.org", Kind::Txt).unwrap();
        assert!(!answers_query(&reply(&other_type, &[]), &query));
        let mut other_class = good.clone();
        let class_at = query.len() - 1;
        other_class[class_at] = 3;
        assert!(!answers_query(&other_class, &query));
        assert!(!answers_query(&good[..12], &query));
    }

    #[test]
    fn transaction_ids_are_random() {
        let ids: std::collections::HashSet<u16> = (0..64).filter_map(|_| random_id()).collect();
        assert!(ids.len() > 32, "{ids:?}");
    }

    #[test]
    fn rejects_loops_and_bad_names() {
        let mut packet = vec![0; 12];
        packet.extend_from_slice(&[0xc0, 12]);
        assert!(read_name(&packet, 12).is_none());
        assert!(question(1, "a..b", Kind::Mx).is_none());
    }

    #[test]
    fn lookups_against_a_local_resolver() {
        smol::block_on(async {
            let server = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();
            let serve = async {
                let mut buf = [0; 512];
                loop {
                    let (n, from) = server.recv_from(&mut buf).await.unwrap();
                    let query = &buf[..n];
                    let records = match query[n - 3] {
                        33 => vec![
                            (Kind::Srv, srv_data(10, 0, 143, "slow.example.org")),
                            (Kind::Srv, srv_data(0, 1, 993, "imap.example.org")),
                        ],
                        _ => vec![(Kind::Mx, {
                            let mut d = vec![0, 5];
                            d.extend_from_slice(&srv_data(0, 0, 0, "mx.mail.example")[6..]);
                            d
                        })],
                    };
                    server.send_to(&reply(query, &records), from).await.unwrap();
                }
            };
            let lookups = async {
                let timeout = Duration::from_secs(2);
                let found = srv(&[addr], "_imaps._tcp.example.org", timeout).await;
                assert_eq!(found[0].target, "imap.example.org");
                assert_eq!(found[1].port, 143);
                let found = mx(&[addr], "example.org", timeout).await;
                assert_eq!(found[0].exchange, "mx.mail.example");
            };
            lookups.or(serve).await;
        });
    }
}
