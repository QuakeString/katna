# Katna Server

The server behind Katna's features that need one. Installs register once
and use their token for every feature; `/api/v1/` has room for more, and
periodic work runs in the server's own tasks (today: deleting old records).
The first feature is open and click tracking for mail sent with Katna
(`docs/ARCHITECTURE.md` §16.1). Tracking is off unless the sender turns it
on for a message.

For each recipient of a tracked message the daemon asks for a random
128-bit ID and puts two kinds of address into that recipient's copy:

- `https://<tracking domain>/o/<id>.png`: a transparent 1×1 picture.
  Fetching it records an **open**.
- `https://<tracking domain>/l/<id>/<n>`: the message's link number `n`.
  Following it records a **click** and redirects (a plain `302`, so
  recipients never see a page in between) to the link's real address,
  which the daemon stored with the ID.

The server redirects only to addresses stored that way, but anyone with a
confirmed Katna account can store one, so a `/l/` link can point anywhere
an account holder chooses: the server can be misused as a redirect. That
is kept small by the daily limits per account (5000 tracked copies, 16 MiB
of links) and, once a host is seen abused, by
`KATNA_SERVER_BLOCKED_HOSTS`: a list of hosts (each with its subdomains)
the server refuses to store links to and no longer redirects to, even for
links stored earlier (those answer `404`).

Opens and clicks always get the picture or the redirect, but only so many
are recorded: 300 per hour per client network (an IPv4 address or an IPv6
/64) and 20 per hour per tracking ID. Past that the fetch is served and
not recorded, so a flood of fetches neither fills the database nor floods
the sender with events.

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

Translation keeps nothing: the text and the translation pass through to
LibreTranslate and back, and neither is logged (only a failed request's
status is).
## Katna accounts

Every server feature needs a Katna account, much like a Mailspring ID.
Someone creates one in Katna Mail (Settings > Subscription) with an
email address and a password of its own; it is never a mail password, and
mail logins never reach this server. The server mails a six-digit code to
confirm the address, and features work once it is confirmed. Each install
signed in to the account is one of its devices; the device list can sign
any of them out.

- Passwords are stored as Argon2id hashes, worked on one per CPU at a
  time (at least two); a request that waits more than 10 seconds for its
  turn is answered `503 busy`.
- Codes work for 30 minutes and for 5 wrong guesses, and are stored hashed.
  An account also gets at most 10 wrong guesses in 24 hours over all its
  codes, counted in the database: asking for a new code or restarting the
  server does not give more. After that even the right code is refused
  until the day is over (`429` when confirming the address; the same
  `wrong_code` as any other failure when resetting the password).
- Sign-in attempts are limited per address (10 per 15 minutes) and per
  client address (60 per 15 minutes); codes to 5 mails per address per
  hour.
- `reset` answers `202` at once for every well-formed address, whether or
  not it has an account and whatever becomes of the mail (a failure is
  only logged), and `reset/confirm` answers an unknown address the same as
  a wrong code. `POST /api/v1/account` still answers `409` for an address
  that has an account, so that route tells whether an address has one;
  answering as if created would need a mail warning the owner, which the
  server does not send yet.
- Changing or resetting the password signs the other devices out.
- Deleting the account deletes its devices and all their data.
- Accounts whose address is never confirmed are deleted after a week.

There are no plans or payments yet.

### Mail for account codes

The server sends the codes through an SMTP relay you choose (your mail
provider's SMTP server, or a service such as Postmark, Mailgun or Amazon
SES). Set in `.env`:

```sh
KATNA_SERVER_SMTP_HOST=smtppro.zoho.in      # your provider's SMTP server
KATNA_SERVER_SMTP_PORT=465                  # 465 (TLS) or 587 (STARTTLS)
KATNA_SERVER_SMTP_USERNAME=no-reply@example.com
KATNA_SERVER_SMTP_PASSWORD=app-password
KATNA_SERVER_MAIL_FROM=                     # empty: the username
```

Instead of the four lines, `KATNA_SERVER_SMTP_URL` can hold them as one
URL (`smtps://user:password@host:465`, with `@` and `/` in the user or
password written `%40`, `%2F`); it wins when both are set.

The sender's domain needs the relay's SPF and DKIM records, or the codes
land in spam. Without an SMTP host the server refuses to start, unless
`KATNA_SERVER_DEV_MAILER=log` is set: then the codes are only written to
the server's log (`docker compose logs server`), which is fine for trying
it out alone but never where others sign up, since anyone who can read the
log could reset any account.

## API for the daemon

| Request | Does |
|---|---|
| `POST /api/v1/installs` | Registers an install; returns `{"install", "token"}`. 10 per address per hour. |
| `DELETE /api/v1/installs/me` | Deletes the install and all its data. |
| `POST /api/v1/account` `{"email", "password", "device"}` | Creates an account, signs this install in, mails a code. `409` if the address has one. |
| `POST /api/v1/account/verify` `{"code"}` | Confirms the address. |
| `POST /api/v1/account/verify/resend` | Mails a new code. |
| `POST /api/v1/account/sign-in` `{"email", "password", "device"}` | Signs this install in; returns `{"email", "verified", "created_at"}`. |
| `POST /api/v1/account/sign-out` | Signs this install out. |
| `GET /api/v1/account` | `{"email", "verified", "created_at"}`. |
| `GET /api/v1/account/devices` | `[{"id", "name", "signed_in_at", "last_seen", "this"}]`. |
| `DELETE /api/v1/account/devices/<id>` | Signs that device out. |
| `POST /api/v1/account/password` `{"current", "new"}` | Changes the password; signs the other devices out. |
| `POST /api/v1/account/reset` `{"email"}` | Mails a reset code; `202` at once, the same for unknown addresses. |
| `POST /api/v1/account/reset/confirm` `{"email", "code", "password", "device"}` | New password; signs this install in and the others out. |
| `POST /api/v1/account/delete` `{"password"}` | Deletes the account, its devices and their data. |
| `POST /api/v1/tracks` `{"count": n, "links": [...]}` | `n` (1–100) new IDs sharing the links; returns `{"ids": [...]}`. 5000 per account per day, and at most 256 KiB of links per request and 16 MiB per account per day. |
| `DELETE /api/v1/tracks/<id>` | Deletes one ID and its events. |
| `GET /api/v1/events` | Server-sent events (`event: track`) after `Last-Event-ID` or `?after=`: `{"seq", "id", "kind": "open"\|"click", "link", "source", "at"}` (`at` in ms). At most 4 open per install (`429` past that). A stream ends when its install is signed out, deleted or its password changed elsewhere (checked at once, and every minute). |
| `GET /api/v1/languages` | LibreTranslate's languages: `[{"code", "name", "targets"}]`. |
| `POST /api/v1/translate` `{"q", "source", "target"}` | Plain text translated by LibreTranslate: `{"translatedText"}`. `source` may be `auto`. 2000 requests per account per day; `503` when `KATNA_SERVER_TRANSLATE_CONCURRENCY` (8) requests are already being translated. |
| `POST /api/v1/detect` `{"q"}` | The language of a text, as LibreTranslate answers. |
| `POST /api/v1/ai/rephrase` `{"text", "tone", "instruction"}` | Katna AI rewrites the text: `{"text", "plan": {"kind": "trial"\|"paid", "days_left"}}`. 30 days free from the first use, then `402` (`pay`) until paid; `429` past the account's monthly cap (`KATNA_SERVER_AI_ACCOUNT_CAP_USD`), everyone's (`_BUDGET_USD`) or 300 an hour; `503` when no AI service is set; `502` (`upstream`) when it and the fallback fail. |
| `GET /admin` | The admin page (only with `KATNA_SERVER_ADMIN_EMAILS`; 404 otherwise): Katna AI's service, model, limits and use. Its calls under `/admin/api/` take a session cookie from signing in with the admin password (set with `katna-server admin-password`, not a Katna account) and a mailed code, and an `X-Katna-Admin: 1` header. |
| `POST /api/v1/ai/complete` `{"before", "answered"}` | The rest of the sentence, the same way; `""` when there is too little to go on. |
| `GET /healthz` | `ok` when the database answers. |

All but the first need `Authorization: Bearer <token>`. The tracking
and translation routes (`/api/v1/tracks`, `/api/v1/events`,
`/api/v1/languages`, `/api/v1/translate`, `/api/v1/detect`, `/api/v1/ai/…`) also need the install signed in
to an account with a confirmed address. Errors are
`{"error": "…", "code": "…"}`; `code` is `unknown_install` (401, register
again), `sign_in` or `not_verified` (403), `wrong_password` (401),
`exists` (409), `bad_email`, `short_password`, `long_password`,
`wrong_code`, `code_expired`, `bad_request` (400), `pay` (402), `not_found`, `too_many` (429),
`mail_failed` or `upstream` (502), `busy` (503) or `server`.

## Running it

It runs as four containers: Katna Server, PostgreSQL, Caddy, which gets
the TLS certificate, and LibreTranslate for automatic translation.
LibreTranslate needs about 4 GB of memory with the default languages; set
`KATNA_TRANSLATE_LANGUAGES` in `.env` to load fewer. Three networks keep
them apart: only Katna Server reaches PostgreSQL (on an internal network)
and LibreTranslate, LibreTranslate cannot reach PostgreSQL, and only Caddy
has published ports. Images are pinned to versions (PostgreSQL 17, Caddy
2, LibreTranslate `v1.9.6`, set `KATNA_TRANSLATE_VERSION` to change it).

Caddy adds `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`,
`Referrer-Policy: no-referrer`, `X-Frame-Options: DENY` and a
Content-Security-Policy that allows nothing but the about page's own
style; it drops its `Server` header, turns its admin API off, refuses
request bodies over 1 MB and times out slow clients (10 s for headers,
30 s for a body, 2 minutes idle; no write timeout, as event streams stay
open).

1. DNS for the tracking domain (a separate domain from the one you send
   mail from, for example `server.katna.invenia.in`):
   - `A` record: the server's IPv4 address.
   - `AAAA` record: its IPv6 address, if it has one.
   - Optional `CAA` record `0 issue "letsencrypt.org"`.
2. Ports 80 and 443 open to the internet (80 is used for the certificate).
3. On the server:

   ```sh
   cd server/katna-server
   cp env.example .env      # set KATNA_TRACKING_DOMAIN, POSTGRES_PASSWORD
                            # and the SMTP relay (see "Mail for account codes")
   docker compose up -d     # or: docker compose up -d --build
   curl https://server.katna.invenia.in/healthz
   ```

The image is `ghcr.io/quakestring/katna-server`, built by CI from `main`;
`--build` builds it from this checkout instead.

Settings (environment): `DATABASE_URL`, `KATNA_SERVER_LISTEN`
(`0.0.0.0:8080`), `KATNA_SERVER_TRUST_FORWARDED` (read the client address
from the proxy's `X-Forwarded-For`; only behind a proxy),
`KATNA_SERVER_RETENTION_DAYS` (180), `KATNA_SERVER_DAILY_LIMIT` (5000),
`KATNA_SERVER_INSTALLS_PER_HOUR` (10), `KATNA_SERVER_SMTP_HOST`, `_PORT`
(465), `_USERNAME`, `_PASSWORD` or `KATNA_SERVER_SMTP_URL`,
`KATNA_SERVER_MAIL_FROM` (the username), `KATNA_SERVER_DEV_MAILER` (`log`
to write codes to the log when there is no SMTP relay; local testing
only), `KATNA_SERVER_TRANSLATE_URL` (LibreTranslate, `http://` on the
internal network; empty turns translation off),
`KATNA_SERVER_TRANSLATIONS_PER_DAY` (2000),
`KATNA_SERVER_TRANSLATE_CONCURRENCY` (8), `KATNA_SERVER_BLOCKED_HOSTS`
(hosts, split by commas or spaces, that links may not go to),
`KATNA_SERVER_AI_KEY` (Katna AI's service key; empty turns it off),
`_AI_PROVIDER` (`gemini`), `_AI_MODEL` (the service's usual one),
`_AI_BASE` (its API address; `http://` only on an internal network),
the same four as `KATNA_SERVER_AI_FALLBACK_*` for the service tried when
the first fails, `_AI_TRIAL_DAYS` (30), `_AI_ACCOUNT_CAP_USD` (1.00 a
month), `_AI_BUDGET_USD` (50 a month for all; 0 turns it off),
`_AI_PRICE_IN_USD` and `_AI_PRICE_OUT_USD` (0.10 and 0.40 per million
tokens, Gemini 2.5 Flash-Lite's), `_AI_PER_HOUR` (300),
`KATNA_SERVER_AI_<SERVICE>_KEY` for `GEMINI`, `OPENAI`, `ANTHROPIC`,
`MISTRAL`, `DEEPSEEK`, `OPENROUTER` and `OTHER` (with
`KATNA_SERVER_AI_OTHER_BASE`): the services the admin page may choose
among; what it saves wins over the `_AI_*` lines above,
`KATNA_SERVER_ADMIN_EMAILS` (who may open `/admin`; empty turns the page
off), `RUST_LOG`.

The admin page is for whoever runs the server; Katna accounts can never
open it. Set each admin's password on the server, asked twice and stored
as a hash (run it again to change it):

```sh
docker compose exec -it server katna-server admin-password [address]
```

Then sign in at `https://<tracking domain>/admin` with that address and
password, and the code mailed to the address.
Neither the text sent to Katna AI nor its answer is logged or kept; only
the number of requests and their cost per account and month.

The in-memory limits (per address, per install, per network) each hold at
most 100,000 keys; when one is full of keys still inside their window it
refuses new keys until old ones expire, rather than growing. A request
waits at most 5 seconds for a database connection.

Run one server process: events are numbered and streamed in order within
the process.

## Tests

```sh
cargo test -p katna-server                       # without a database
KATNA_SERVER_TEST_DATABASE_URL=postgres://katna:katna@localhost/katna_test \
  cargo test -p katna-server -- --include-ignored
```

CI's `server` job runs the second against a PostgreSQL service container.
