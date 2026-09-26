# SPDX-License-Identifier: GPL-3.0-or-later
"""Seed the Katna dev servers with test accounts, mail, events and contacts.

Runs inside the `seed` service of dev/compose.yaml. Standard library only.
Safe to run again: anything that already exists is left alone.
"""

import base64
import email
import email.utils
import imaplib
import json
import os
import smtplib
import socket
import ssl
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from email.message import EmailMessage
from pathlib import Path

SEED = Path(__file__).resolve().parent
PASSWORD = os.environ.get("KATNA_DEV_PASSWORD", "katna-dev")
DOMAIN = "katna.test"
USERS = ["alice", "bob"]
# Mail, events and contacts go to this user; the others start empty.
SEED_USER = f"alice@{DOMAIN}"

STALWART_ADMIN = ("admin", PASSWORD)
STALWART_HTTP = "http://stalwart:8080"
RADICALE_HTTP = "http://radicale:5232"

CALENDAR = "katna-seed"
ADDRESSBOOK = "katna-seed-contacts"

# Self-signed certificates everywhere; this is a local test setup.
INSECURE_TLS = ssl.create_default_context()
INSECURE_TLS.check_hostname = False
INSECURE_TLS.verify_mode = ssl.CERT_NONE


def log(msg):
    print(f"seed: {msg}", flush=True)


def wait_for(host, port, timeout=120):
    deadline = time.monotonic() + timeout
    while True:
        try:
            with socket.create_connection((host, port), timeout=2):
                return
        except OSError:
            if time.monotonic() > deadline:
                sys.exit(f"seed: {host}:{port} did not come up in {timeout}s")
            time.sleep(1)


def basic_auth(user, password):
    token = base64.b64encode(f"{user}:{password}".encode()).decode()
    return f"Basic {token}"


def http(method, url, auth, body=None, headers=None):
    """Returns (status, body). HTTP errors are returned, not raised."""
    req = urllib.request.Request(url, data=body, method=method)
    req.add_header("Authorization", basic_auth(*auth))
    for key, value in (headers or {}).items():
        req.add_header(key, value)
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            return resp.status, resp.read()
    except urllib.error.HTTPError as err:
        return err.code, err.read()


# --- Stalwart --------------------------------------------------------------
#
# Stalwart 0.16 keeps all settings in its data store and manages them as
# JMAP objects (x:<Object>/get|set). compose.yaml pins a recovery admin
# (STALWART_RECOVERY_ADMIN) so we can log in without the web setup wizard.


def jmap(*calls):
    body = json.dumps(
        {
            "using": ["urn:ietf:params:jmap:core", "urn:stalwart:jmap"],
            "methodCalls": [[name, args, str(i)] for i, (name, args) in enumerate(calls)],
        }
    ).encode()
    status, raw = http(
        "POST",
        f"{STALWART_HTTP}/jmap",
        STALWART_ADMIN,
        body,
        {"Content-Type": "application/json"},
    )
    if status != 200:
        sys.exit(f"seed: Stalwart JMAP returned {status}: {raw[:500]!r}")
    responses = [r[1] for r in json.loads(raw)["methodResponses"]]
    for (name, _), resp in zip(calls, responses):
        failed = resp.get("notCreated") or resp.get("notUpdated") or resp.get("type")
        if failed:
            sys.exit(f"seed: Stalwart {name} failed: {json.dumps(resp)}")
    return responses


def setup_stalwart():
    wait_for("stalwart", 8080)
    (domains,) = jmap(("x:Domain/get", {"ids": None, "properties": ["name"]}))
    if any(d["name"] == DOMAIN for d in domains["list"]):
        log("Stalwart already set up")
        return

    log(f"Stalwart: creating domain {DOMAIN} and accounts")
    created, _ = jmap(
        (
            "x:Domain/set",
            {
                "create": {
                    "d": {
                        "name": DOMAIN,
                        "isEnabled": True,
                        "certificateManagement": {"@type": "Manual"},
                        "dnsManagement": {"@type": "Manual"},
                        "dkimManagement": {"@type": "Manual"},
                    }
                }
            },
        ),
        (
            "x:Account/set",
            {
                "create": {
                    user: {
                        "@type": "User",
                        "name": user,
                        "domainId": "#d",
                        "credentials": {"0": {"@type": "Password", "secret": PASSWORD}},
                    }
                    for user in USERS
                }
            },
        ),
    )
    domain_id = created["created"]["d"]["id"]

    log("Stalwart: dev settings (SMTP auth without TLS, relay to Mailpit)")
    jmap(
        (
            "x:SystemSettings/set",
            {
                "update": {
                    "singleton": {
                        "defaultDomainId": domain_id,
                        "defaultHostname": f"stalwart.{DOMAIN}",
                    }
                }
            },
        ),
        # Accept PLAIN/LOGIN on every port except 25, with or without TLS.
        (
            "x:MtaStageAuth/set",
            {
                "update": {
                    "singleton": {
                        "saslMechanisms": {
                            "match": {
                                "0": {
                                    "if": "local_port != 25",
                                    "then": "[plain, login, oauthbearer, xoauth2]",
                                }
                            },
                            "else": "false",
                        }
                    }
                }
            },
        ),
        # Anything not for a local domain goes to Mailpit instead of the
        # Internet.
        (
            "x:MtaRoute/set",
            {
                "create": {
                    "m": {
                        "@type": "Relay",
                        "name": "mailpit",
                        "description": "Dev: deliver all remote mail to Mailpit",
                        "address": "mailpit",
                        "port": 1025,
                        "protocol": "smtp",
                    }
                }
            },
        ),
        (
            "x:MtaOutboundStrategy/set",
            {
                "update": {
                    "singleton": {
                        "route": {
                            "match": {
                                "0": {
                                    "if": "is_local_domain(rcpt_domain)",
                                    "then": "'local'",
                                }
                            },
                            "else": "'mailpit'",
                        }
                    }
                }
            },
        ),
        ("x:Action/set", {"create": {"r": {"@type": "ReloadSettings"}}}),
    )


# --- Mail (IMAP APPEND) ----------------------------------------------------


def imap_login(connect, host, port):
    wait_for(host, port)
    for attempt in range(30):
        try:
            conn = connect()
            conn.login(SEED_USER, PASSWORD)
            return conn
        except (imaplib.IMAP4.error, OSError) as err:
            # Stalwart can take a few seconds to accept new accounts.
            if attempt == 0:
                log(f"waiting for {host} to accept the login")
            if attempt == 29:
                sys.exit(f"seed: IMAP login to {host}:{port} failed: {err}")
            time.sleep(1)


def seed_imap(name, conn):
    appended = 0
    for folder in sorted(p for p in (SEED / "mail").iterdir() if p.is_dir()):
        mailbox = folder.name
        if mailbox != "INBOX":
            conn.create(mailbox)  # Fails harmlessly if it exists.
        typ, data = conn.select(mailbox)
        if typ != "OK":
            sys.exit(f"seed: {name}: cannot select {mailbox}: {data}")
        if int(data[0]) > 0:
            continue
        for path in sorted(folder.glob("*.eml")):
            raw = path.read_bytes().replace(b"\r\n", b"\n").replace(b"\n", b"\r\n")
            date = email.utils.parsedate_to_datetime(
                email.message_from_bytes(raw)["Date"]
            )
            typ, data = conn.append(mailbox, None, imaplib.Time2Internaldate(date), raw)
            if typ != "OK":
                sys.exit(f"seed: {name}: APPEND {path.name} failed: {data}")
            appended += 1
    conn.logout()
    log(f"{name}: appended {appended} messages")
    return appended


def send_test_mail(name, host, port, use_ssl):
    """Sends one message to an outside address; it should land in Mailpit."""
    msg = EmailMessage()
    msg["From"] = SEED_USER
    msg["To"] = "someone@example.org"
    msg["Subject"] = f"Sent through {name}"
    msg["Date"] = email.utils.formatdate()
    msg["Message-ID"] = email.utils.make_msgid(domain=DOMAIN)
    msg.set_content(f"This message was submitted to {name} and relayed to Mailpit.")
    wait_for(host, port)
    if use_ssl:
        smtp = smtplib.SMTP_SSL(host, port, context=INSECURE_TLS, timeout=30)
    else:
        smtp = smtplib.SMTP(host, port, timeout=30)
    with smtp:
        smtp.login(SEED_USER, PASSWORD)
        smtp.send_message(msg)
    log(f"{name}: sent a test message to Mailpit")


# --- Calendars and contacts (CalDAV / CardDAV) -----------------------------

MKCALENDAR = b"""<?xml version="1.0" encoding="utf-8"?>
<C:mkcalendar xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
  <D:set><D:prop>
    <D:displayname>Katna seed</D:displayname>
    <C:supported-calendar-component-set>
      <C:comp name="VEVENT"/><C:comp name="VTODO"/>
    </C:supported-calendar-component-set>
  </D:prop></D:set>
</C:mkcalendar>"""

MKADDRESSBOOK = b"""<?xml version="1.0" encoding="utf-8"?>
<D:mkcol xmlns:D="DAV:" xmlns:A="urn:ietf:params:xml:ns:carddav">
  <D:set><D:prop>
    <D:resourcetype><D:collection/><A:addressbook/></D:resourcetype>
    <D:displayname>Katna seed contacts</D:displayname>
  </D:prop></D:set>
</D:mkcol>"""


def seed_dav(name, cal_home, card_home):
    auth = (SEED_USER, PASSWORD)
    # Radicale creates the user's principal collection on first access.
    http("PROPFIND", cal_home, auth, headers={"Depth": "0"})

    jobs = [
        (cal_home, CALENDAR, "MKCALENDAR", MKCALENDAR, "calendar", "*.ics", "text/calendar"),
        (card_home, ADDRESSBOOK, "MKCOL", MKADDRESSBOOK, "contacts", "*.vcf", "text/vcard"),
    ]
    for home, collection, method, body, folder, pattern, mime in jobs:
        url = f"{home}{collection}/"
        status, raw = http(method, url, auth, body, {"Content-Type": "application/xml"})
        # 405 (Stalwart) or 409 (Radicale): the collection already exists.
        if status not in (201, 405, 409):
            sys.exit(f"seed: {name}: {method} {url} returned {status}: {raw[:300]!r}")
        added = 0
        for path in sorted((SEED / folder).glob(pattern)):
            status, raw = http(
                "PUT",
                url + path.name,
                auth,
                path.read_bytes(),
                {"Content-Type": f"{mime}; charset=utf-8", "If-None-Match": "*"},
            )
            if status in (201, 204):
                added += 1
            elif status != 412:  # 412: already there.
                sys.exit(f"seed: {name}: PUT {path.name} returned {status}: {raw[:300]!r}")
        log(f"{name}: {collection}: added {added} items")


def main():
    setup_stalwart()

    stalwart_imap = imap_login(
        lambda: imaplib.IMAP4_SSL("stalwart", 993, ssl_context=INSECURE_TLS),
        "stalwart",
        993,
    )
    if seed_imap("Stalwart", stalwart_imap):
        send_test_mail("Stalwart", "stalwart", 465, use_ssl=True)

    dovecot_imap = imap_login(lambda: imaplib.IMAP4("dovecot", 31143), "dovecot", 31143)
    if seed_imap("Dovecot", dovecot_imap):
        send_test_mail("Dovecot", "dovecot", 31587, use_ssl=False)

    user = urllib.parse.quote(SEED_USER)
    wait_for("stalwart", 8080)
    seed_dav(
        "Stalwart",
        f"{STALWART_HTTP}/dav/cal/{user}/",
        f"{STALWART_HTTP}/dav/card/{user}/",
    )
    wait_for("radicale", 5232)
    seed_dav("Radicale", f"{RADICALE_HTTP}/{user}/", f"{RADICALE_HTTP}/{user}/")

    log("done")


if __name__ == "__main__":
    main()
