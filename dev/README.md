# Local test servers

`compose.yaml` starts the mail and calendar servers Katna is developed and
tested against, and seeds them with test accounts and sample data:

| Service | What it is for |
|---|---|
| Stalwart | IMAP, JMAP, SMTP, CalDAV, CardDAV and Sieve in one server |
| Dovecot | The most common real-world IMAP server (CONDSTORE, QRESYNC, NOTIFY) |
| Radicale | Simple CalDAV/CardDAV reference server |
| Mailpit | Catches all outgoing mail, so nothing leaves your machine |

This is for local testing only: passwords are fixed, TLS certificates are
self-signed, and all ports listen on `127.0.0.1` only.

## Start and stop

With Docker:

```sh
cd dev
docker compose up -d           # start everything; the seed service runs once
docker compose logs seed       # should end with "seed: done"
docker compose down            # stop, keep the data
docker compose down -v         # stop and delete all data (fresh seed next time)
```

With Podman, use `podman compose` (or `podman-compose`) with the same
arguments.

The `seed` service creates the accounts and loads the sample data, then
exits. It is safe to run again (`docker compose run --rm seed`); anything
that already exists is left alone.

## Accounts

Every server has the same two users, both with the password `katna-dev`:

- `alice@katna.test`: has the sample mail, events and contacts
- `bob@katna.test`: empty

Dovecot accepts any user name with that password. Stalwart's admin login
is `admin` / `katna-dev`, for its web UI at <http://127.0.0.1:18080/admin>
(Stalwart downloads the UI from GitHub on first start).

## Ports

The addresses below use the default ports; see [Changing ports](#changing-ports).

| Server | Protocol | Address |
|---|---|---|
| Stalwart | IMAP (TLS) | `127.0.0.1:10993` |
| | SMTP submission (TLS) | `127.0.0.1:10465` |
| | SMTP (STARTTLS) | `127.0.0.1:10025` |
| | ManageSieve | `127.0.0.1:14190` |
| | JMAP | `http://127.0.0.1:18080/jmap/session` |
| | CalDAV | `http://127.0.0.1:18080/dav/cal/alice%40katna.test/` |
| | CardDAV | `http://127.0.0.1:18080/dav/card/alice%40katna.test/` |
| | HTTPS (all of the above) | `https://127.0.0.1:18443` |
| Dovecot | IMAP (plain or STARTTLS) | `127.0.0.1:20143` |
| | IMAP (TLS) | `127.0.0.1:20993` |
| | Submission (plain or STARTTLS) | `127.0.0.1:20587` |
| | ManageSieve | `127.0.0.1:24190` |
| Radicale | CalDAV and CardDAV | `http://127.0.0.1:5232/alice%40katna.test/` |
| Mailpit | Web UI and API | <http://127.0.0.1:8025> |
| | SMTP (no auth) | `127.0.0.1:1025` |

Stalwart has no plain-text IMAP port here: its listeners only change after
a restart, so the setup keeps Stalwart's defaults (TLS only).

Mail that Stalwart or Dovecot would send to another domain goes to Mailpit
instead. The seed script sends one such message through each server, so
Mailpit starts with two messages.

### Changing ports

Every host port can be changed with an environment variable, for example
when another service already uses it. Set the variable in your shell or put
it in `dev/.env`, which Compose reads automatically and git ignores:

```sh
# dev/.env
KATNA_STALWART_HTTP_PORT=38080
```

| Variable | Default |
|---|---|
| `KATNA_STALWART_IMAPS_PORT` | 10993 |
| `KATNA_STALWART_SUBMISSIONS_PORT` | 10465 |
| `KATNA_STALWART_SMTP_PORT` | 10025 |
| `KATNA_STALWART_SIEVE_PORT` | 14190 |
| `KATNA_STALWART_HTTP_PORT` | 18080 |
| `KATNA_STALWART_HTTPS_PORT` | 18443 |
| `KATNA_DOVECOT_IMAP_PORT` | 20143 |
| `KATNA_DOVECOT_IMAPS_PORT` | 20993 |
| `KATNA_DOVECOT_SUBMISSION_PORT` | 20587 |
| `KATNA_DOVECOT_SIEVE_PORT` | 24190 |
| `KATNA_RADICALE_PORT` | 5232 |
| `KATNA_MAILPIT_SMTP_PORT` | 1025 |
| `KATNA_MAILPIT_HTTP_PORT` | 8025 |

The seed service talks to the servers inside the Compose network, so it is
not affected by these settings.

## Integration tests

`katna-sync` has tests that run against these servers. They are ignored by
default because they need the servers up:

```sh
cargo test -p katna-sync --test dev_servers -- --ignored --test-threads 1
```

They honour the `KATNA_*_PORT` variables above and only write to folders
named `katna-test-…` and to alice's inbox.

## Sample data

Everything lives in `seed/` and is loaded into alice's account on every
server:

- `seed/mail/<folder>/*.eml`: plain text, HTML with quoted-printable and
  non-ASCII text, an attachment, a reply thread with a missing parent, and
  a meeting invitation. Each folder name becomes a mailbox.
- `seed/calendar/*.ics`: a weekly recurring event with a time zone, an
  excluded date and one moved instance; a multi-day all-day event; an event
  with an alarm; a task. Loaded into a calendar named `katna-seed`.
- `seed/contacts/*.vcf`: vCard 3.0 and 4.0 contacts. Loaded into an address
  book named `katna-seed-contacts`.

To add data, drop a file into the right folder and run
`docker compose down -v && docker compose up -d`.

## Stalwart notes

Stalwart 0.16 keeps its settings in its database instead of a config file,
and on a blank install it starts a web setup wizard. `stalwart/config.json`
only names the database, and `STALWART_RECOVERY_ADMIN` gives a fixed admin
login, so the wizard is skipped. `seed/seed.py` then creates the domain,
accounts and dev settings through Stalwart's JMAP management API.
