# Katna Server

Open and click tracking for mail sent with Katna
(`docs/ARCHITECTURE.md` §16.1). Tracking is off unless the sender turns it
on for a message.

For each recipient of a tracked message the daemon asks for a random
128-bit ID and puts two kinds of address into that recipient's copy:

- `https://<tracking domain>/o/<id>.png`: a transparent 1×1 picture.
  Fetching it records an **open**.
- `https://<tracking domain>/l/<id>/<n>`: the message's link number `n`.
  Following it records a **click** and redirects to the link's real
  address, which the daemon stored with the ID. The server redirects only
  to addresses stored that way, so it can never be used as an open
  redirect.

Each event is labelled `person`, `apple_proxy` (Apple Mail Privacy
Protection fetches pictures on delivery: "maybe opened") or `scanner`
(security scanners, `HEAD` requests, fetches seconds after sending), and
streamed to the install that created the ID.

## What it stores

Random IDs, link addresses, event times and those labels. Never subjects,
recipients or message content; never IP addresses or user agents (they are
read only to pick the label). Everything is deleted after
`KATNA_SERVER_RETENTION_DAYS` (180), and an install can delete all its data
at once. `/` shows recipients a short page saying this.

An EU legal review (GDPR, ePrivacy) is still open before tracking is sold
to anyone.

## API for the daemon

| Request | Does |
|---|---|
| `POST /api/v1/installs` | Registers an install; returns `{"install", "token"}`. 10 per address per hour. |
| `DELETE /api/v1/installs/me` | Deletes the install and all its data. |
| `POST /api/v1/tracks` `{"count": n, "links": [...]}` | `n` (1–100) new IDs sharing the links; returns `{"ids": [...]}`. 5000 per install per day. |
| `DELETE /api/v1/tracks/<id>` | Deletes one ID and its events. |
| `GET /api/v1/events` | Server-sent events (`event: track`) after `Last-Event-ID` or `?after=`: `{"seq", "id", "kind": "open"\|"click", "link", "source", "at"}` (`at` in ms). |
| `GET /healthz` | `ok` when the database answers. |

All but the first need `Authorization: Bearer <token>`.

## Running it

It runs as three containers: Katna Server, PostgreSQL and Caddy, which
gets the TLS certificate.

1. DNS for the tracking domain (a separate domain from the one you send
   mail from, for example `t.katna.invenia.in`):
   - `A` record: the server's IPv4 address.
   - `AAAA` record: its IPv6 address, if it has one.
   - Optional `CAA` record `0 issue "letsencrypt.org"`.
2. Ports 80 and 443 open to the internet (80 is used for the certificate).
3. On the server:

   ```sh
   cd server/katna-server
   cp env.example .env      # set KATNA_TRACKING_DOMAIN and POSTGRES_PASSWORD
   docker compose up -d     # or: docker compose up -d --build
   curl https://t.katna.invenia.in/healthz
   ```

The image is `ghcr.io/quakestring/katna-server`, built by CI from `main`;
`--build` builds it from this checkout instead.

Settings (environment): `DATABASE_URL`, `KATNA_SERVER_LISTEN`
(`0.0.0.0:8080`), `KATNA_SERVER_TRUST_FORWARDED` (read the client address
from the proxy's `X-Forwarded-For`; only behind a proxy),
`KATNA_SERVER_RETENTION_DAYS` (180), `KATNA_SERVER_DAILY_LIMIT` (5000),
`KATNA_SERVER_INSTALLS_PER_HOUR` (10), `RUST_LOG`.

Run one server process: events are numbered and streamed in order within
the process.

## Tests

```sh
cargo test -p katna-server                       # without a database
KATNA_SERVER_TEST_DATABASE_URL=postgres://katna:katna@localhost/katna_test \
  cargo test -p katna-server -- --include-ignored
```

CI's `server` job runs the second against a PostgreSQL service container.
