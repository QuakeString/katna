// SPDX-License-Identifier: GPL-3.0-or-later

//! Adding a mail account (`docs/ARCHITECTURE.md` §13.6), in a dialog of
//! a few steps, after Mailspring's and Thunderbird's:
//!
//! 1. The provider, from a grid of tiles (Google, Microsoft, Yahoo, …, and
//!    Other for any IMAP or POP3 account).
//! 2. Name, address and password, with a line on the app password the
//!    provider asks for. Adding finds the servers from the address
//!    (`DiscoverAccount`), checks the login and saves the account
//!    (`AddImapAccount`, or `AddPop3Account` when the provider only offers
//!    POP3), showing each stage as it goes.
//! 3. When the servers are not found, or on "Edit servers manually": the
//!    incoming server (IMAP or POP3), the outgoing one (SMTP), ports and
//!    security.
//! 4. What got set up.
//!
//! Google and Microsoft accounts can instead sign in in the browser
//! (`SignIn`, OAuth2), the only way for Microsoft's; their tiles do that
//! when this build has the provider's client ID. Also the account menu of
//! the app rail, which leads here.

use crate::widgets::Tip as _;
use std::collections::HashMap;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, EntityId, Focusable, FontWeight, Hsla,
    KeyDownEvent, MouseButton, MouseDownEvent, SharedString, Subscription, Task, Window, deferred,
    div, prelude::*, relative, rgba,
};
use katna_core::{OAuthProvider, Pop3Keep};
use katna_dbus::{NewImapAccount, NewPop3Account, ServerSpec};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::unpx;
use katna_ui::{InputEvent, TextInput};
use katna_ui::{px, tokens};

use super::MailWindow;
use super::MenuKey;
use super::mail_providers::{MailProvider, PasswordHelp};
use crate::daemon::{self, AddError};
use crate::outgoing;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, ScaledEdge, filled_button, icon, icon_button, raised};

/// The dialog's width with the provider tiles, and on the other steps.
const WIDE: f32 = 640.0;
const WIDTH: f32 = 480.0;
const MENU_WIDTH: f32 = 340.0;
/// Below this inner width the tiles go one to a row.
const TWO_COLUMNS: f32 = 440.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Provider,
    /// Name, address and password.
    Form,
    Servers,
    /// Waiting for the provider's sign-in page in the browser.
    Browser(OAuthProvider),
    Done,
}

/// What the dialog is waiting for, for the stages it shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Finding,
    SigningIn,
}

/// How a connection to a server is secured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Security {
    Tls,
    StartTls,
    Plain,
}

impl Security {
    const ALL: [Self; 3] = [Self::Tls, Self::StartTls, Self::Plain];

    fn parse(text: &str) -> Self {
        match text {
            "starttls" => Self::StartTls,
            "plain" => Self::Plain,
            _ => Self::Tls,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Tls => "tls",
            Self::StartTls => "starttls",
            Self::Plain => "plain",
        }
    }

    fn label(self) -> String {
        match self {
            Self::Tls => "SSL/TLS".to_owned(),
            Self::StartTls => "STARTTLS".to_owned(),
            Self::Plain => tr!("add-account-security-none"),
        }
    }

    fn port(self, kind: Kind) -> u16 {
        match (kind, self) {
            (Kind::Imap, Self::Tls) => 993,
            (Kind::Imap, _) => 143,
            (Kind::Pop3, Self::Tls) => 995,
            (Kind::Pop3, _) => 110,
            (Kind::Smtp, Self::Tls) => 465,
            (Kind::Smtp, _) => 587,
        }
    }
}

/// A server's protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Imap,
    Pop3,
    Smtp,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::Imap => "IMAP",
            Self::Pop3 => "POP3",
            Self::Smtp => "SMTP",
        }
    }
}

/// One server's fields.
struct ServerFields {
    host: Entity<TextInput>,
    port: Entity<TextInput>,
    security: Security,
}

/// What was added, for the last step.
struct Added {
    id: i64,
    address: String,
    name: String,
    /// Signed in in the browser with this provider.
    signed_in: Option<OAuthProvider>,
    /// The incoming protocol and server, and the outgoing server.
    incoming: Option<(Kind, String)>,
    smtp: String,
}

/// Linking Zoho's tasks and calendars from the last step.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Linking {
    Busy,
    Done,
    Failed(String),
}

/// The open add-account dialog.
pub(super) struct AddAccount {
    step: Step,
    provider: MailProvider,
    address: Entity<TextInput>,
    password: Entity<TextInput>,
    name: Entity<TextInput>,
    /// The incoming server, IMAP or POP3 (`incoming_kind`).
    incoming: ServerFields,
    incoming_kind: Kind,
    smtp: ServerFields,
    username: Entity<TextInput>,
    /// The servers discovery found, to go back to when IMAP or POP3 is
    /// picked again.
    found_imap: Option<ServerSpec>,
    found_pop3: Option<ServerSpec>,
    /// The address the server fields were filled for.
    servers_for: String,
    /// Where the daemon found the servers; `None` once they are edited.
    source: Option<String>,
    /// The found servers are Google's or Microsoft's.
    sign_in: Option<OAuthProvider>,
    /// A password works for the found servers (not only `sign_in`).
    password_works: bool,
    accept_invalid_certs: bool,
    show_password: bool,
    busy: bool,
    phase: Phase,
    error: Option<String>,
    added: Option<Added>,
    linking: Option<Linking>,
    shown: Spring,
    closing: bool,
    /// The text each field had when last seen, so that text set by the
    /// dialog itself does not count as an edit.
    seen: HashMap<EntityId, String>,
    _task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

/// Sets a field's text without it counting as an edit.
fn fill(
    seen: &mut HashMap<EntityId, String>,
    field: &Entity<TextInput>,
    text: String,
    cx: &mut Context<MailWindow>,
) {
    seen.insert(field.entity_id(), text.clone());
    field.update(cx, |input, cx| input.set_text(text, cx));
}

/// A server spec from what was typed, or what is wrong with it.
fn server_spec(
    host: &str,
    port: &str,
    security: Security,
    username: &str,
    accept_invalid_certs: bool,
    what: &str,
) -> Result<ServerSpec, String> {
    let host = host.trim();
    if host.is_empty() {
        return Err(tr!("add-account-server-missing", kind = what));
    }
    if host.contains(char::is_whitespace) {
        return Err(tr!("add-account-server-space", kind = what));
    }
    let port = match port.trim().parse::<u16>() {
        Ok(port) if port > 0 => port,
        _ => {
            return Err(tr!(
                "add-account-port-invalid",
                kind = what,
                min = "1",
                max = "65535"
            ));
        }
    };
    Ok(ServerSpec {
        host: host.to_owned(),
        port,
        security: security.as_str().to_owned(),
        username: username.trim().to_owned(),
        accept_invalid_certs,
    })
}

/// The usual server names for an address the daemon found nothing for.
fn guess(address: &str) -> NewImapAccount {
    let domain = address.rsplit_once('@').map_or("", |(_, d)| d).trim();
    let spec = |prefix: &str, port: u16| ServerSpec {
        host: format!("{prefix}.{domain}"),
        port,
        security: "tls".to_owned(),
        username: address.to_owned(),
        accept_invalid_certs: false,
    };
    NewImapAccount {
        display_name: String::new(),
        address: address.to_owned(),
        imap: spec("imap", 993),
        smtp: spec("smtp", 465),
    }
}

/// The other protocol's usual name for a server: `imap.x.org` and
/// `pop.x.org` in turn; other names stay.
fn swap_host(host: &str, to: Kind) -> String {
    let host = host.trim();
    let rest = ["imap.", "pop3.", "pop."]
        .iter()
        .find_map(|prefix| host.strip_prefix(prefix));
    match (rest, to) {
        (Some(rest), Kind::Pop3) => format!("pop.{rest}"),
        (Some(rest), _) => format!("imap.{rest}"),
        (None, _) => host.to_owned(),
    }
}

/// Whether `name` is `password` (app passwords are shown in groups, so
/// spaces don't count).
fn is_password(name: &str, password: &str) -> bool {
    let bare = |text: &str| {
        text.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };
    let name = bare(name);
    !name.is_empty() && name == bare(password)
}

/// Providers that refuse the everyday password for IMAP.
fn app_password_provider(address: &str) -> Option<&'static str> {
    let domain = address.rsplit_once('@')?.1.trim().to_lowercase();
    match domain.as_str() {
        "gmail.com" | "googlemail.com" => Some("Gmail"),
        "yahoo.com" | "ymail.com" | "rocketmail.com" => Some("Yahoo Mail"),
        "icloud.com" | "me.com" | "mac.com" => Some("iCloud Mail"),
        "aol.com" => Some("AOL Mail"),
        _ => None,
    }
}

/// Where the daemon found the servers, as the variant of "found in …"
/// (`add-account-servers-found`) to pick.
fn source_key(source: &str) -> &'static str {
    match source {
        "built-in" => "built-in",
        "provider" => "provider",
        "ispdb" => "ispdb",
        "dns-srv" | "mx" => "dns",
        _ => "other",
    }
}

impl MailWindow {
    /// Opens the add-account dialog.
    pub(super) fn open_add_account(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.account_menu = false;
        if self.add_account.as_ref().is_some_and(|a| !a.closing) {
            return;
        }
        let accent: Hsla = rgba(self.theme(window).accent).into();
        let input = |cx: &mut Context<Self>| {
            cx.new(|cx| {
                let mut input = TextInput::new("", cx);
                input.set_accent(accent);
                input
            })
        };
        let (address, password, name, username) = (input(cx), input(cx), input(cx), input(cx));
        password.update(cx, |p, cx| p.set_masked(true, cx));
        let incoming = ServerFields {
            host: input(cx),
            port: input(cx),
            security: Security::Tls,
        };
        let smtp = ServerFields {
            host: input(cx),
            port: input(cx),
            security: Security::Tls,
        };
        let mut subscriptions = Vec::new();
        for field in [&address, &password, &name] {
            subscriptions.push(cx.subscribe_in(
                field,
                window,
                |this, field, event: &InputEvent, window, cx| {
                    this.add_account_event(field, event, window, cx);
                },
            ));
        }
        // Editing a server means it was not found any more.
        for field in [
            &incoming.host,
            &incoming.port,
            &smtp.host,
            &smtp.port,
            &username,
        ] {
            subscriptions.push(cx.subscribe_in(
                field,
                window,
                |this, field, event: &InputEvent, window, cx| {
                    if this.add_account_event(field, event, window, cx)
                        && let Some(dialog) = &mut this.add_account
                    {
                        dialog.source = None;
                    }
                },
            ));
        }
        // The tiles take the keys: the dialog's own focus holds it.
        window.focus(&self.dialog_focus, cx);
        self.add_account = Some(AddAccount {
            step: Step::Provider,
            provider: MailProvider::Other,
            address,
            password,
            name,
            incoming,
            incoming_kind: Kind::Imap,
            smtp,
            username,
            found_imap: None,
            found_pop3: None,
            servers_for: String::new(),
            source: None,
            sign_in: None,
            password_works: true,
            accept_invalid_certs: false,
            show_password: false,
            busy: false,
            phase: Phase::Finding,
            error: None,
            added: None,
            linking: None,
            shown: Spring::new(motion::SLIDE, 0.0),
            closing: false,
            seen: HashMap::new(),
            _task: None,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// Handles a field's event. Returns whether the user edited the field.
    fn add_account_event(
        &mut self,
        field: &Entity<TextInput>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        match event {
            InputEvent::Submit => self.add_account_next(window, cx),
            InputEvent::Cancel => self.close_add_account(cx),
            InputEvent::Changed => {
                let text = field.read(cx).text().to_owned();
                if let Some(dialog) = &mut self.add_account
                    && dialog.seen.get(&field.entity_id()) != Some(&text)
                {
                    dialog.seen.insert(field.entity_id(), text);
                    dialog.error = None;
                    cx.notify();
                    return true;
                }
            }
        }
        false
    }

    fn close_add_account(&mut self, cx: &mut Context<Self>) {
        self.cancel_browser_sign_in(cx);
        if let Some(dialog) = &mut self.add_account {
            // Stops a discovery or login that is under way.
            dialog._task = None;
            dialog.closing = true;
        }
        cx.notify();
    }

    /// Tells the daemon to stop waiting for the browser, if it is.
    fn cancel_browser_sign_in(&mut self, cx: &mut Context<Self>) {
        let waiting = self.add_account.as_ref().is_some_and(|d| {
            (matches!(d.step, Step::Browser(_)) && d.busy) || d.linking == Some(Linking::Busy)
        });
        if let (true, Some(connection)) = (waiting, self.daemon.clone()) {
            cx.background_executor()
                .spawn(async move { daemon::cancel_sign_in(&connection).await })
                .detach();
        }
    }

    /// A provider's tile: signs in in the browser where it can, else asks
    /// for the address and password.
    fn pick_provider(
        &mut self,
        provider: MailProvider,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        dialog.provider = provider;
        match provider.sign_in() {
            Some(oauth) => self.start_sign_in(oauth, window, cx),
            None => self.add_account_step(Step::Form, window, cx),
        }
    }

    /// Signs in to `provider` in the browser; the daemon adds the account
    /// that signed in.
    fn start_sign_in(
        &mut self,
        provider: OAuthProvider,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let hint = self.add_account_address(cx).unwrap_or_default();
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        if dialog.closing {
            return;
        }
        dialog.step = Step::Browser(provider);
        dialog.busy = true;
        dialog.error = None;
        let connection = self.daemon.clone();
        let typed = hint.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await.map_err(AddError::Other)?,
                    };
                    daemon::sign_in(&connection, provider, None, &typed).await
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                if let Some(dialog) = &mut this.add_account {
                    dialog.busy = false;
                }
                match result {
                    Ok(id) => {
                        let address = this
                            .accounts
                            .iter()
                            .find(|a| a.id.0 == id)
                            .map(|a| a.address.clone())
                            .unwrap_or(hint);
                        this.account_done(
                            Added {
                                id,
                                name: String::new(),
                                address,
                                signed_in: Some(provider),
                                incoming: None,
                                smtp: String::new(),
                            },
                            window,
                            cx,
                        );
                    }
                    Err(AddError::Password(detail)) => {
                        tracing::info!(%detail, "sign-in refused");
                        this.add_account_step(Step::Provider, window, cx);
                        this.add_account_error(
                            tr!("add-account-sign-in-refused", provider = provider.name()),
                            cx,
                        );
                    }
                    Err(AddError::Other(err)) => {
                        this.add_account_step(Step::Provider, window, cx);
                        this.add_account_error(err, cx);
                    }
                }
            })
            .ok();
        });
        if let Some(dialog) = &mut self.add_account {
            dialog._task = Some(task);
        }
        cx.notify();
    }

    /// After discovery: signs in in the browser when the provider takes
    /// nothing else, asks for the servers when no SMTP server was found,
    /// else checks the password.
    fn after_discovery(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &self.add_account else {
            return;
        };
        if dialog.password_works {
            // Some providers publish no SMTP server.
            if dialog.smtp.host.read(cx).text().trim().is_empty() {
                self.add_account_step(Step::Servers, window, cx);
                self.add_account_error(tr!("add-account-smtp-not-found"), cx);
            } else {
                self.add_the_account(window, cx);
            }
            return;
        }
        match dialog.sign_in {
            Some(provider) if provider.available() => self.start_sign_in(provider, window, cx),
            provider => {
                let name = provider.map_or("", |p| p.name());
                self.add_account_step(Step::Form, window, cx);
                self.add_account_error(tr!("add-account-sign-in-unavailable", provider = name), cx);
            }
        }
    }

    /// Fills the server fields from what was found or guessed: IMAP when
    /// there is an IMAP server, else POP3.
    fn fill_servers(
        &mut self,
        account: &NewImapAccount,
        pop3: Option<ServerSpec>,
        source: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let imap = (!account.imap.host.trim().is_empty()).then(|| account.imap.clone());
        let pop3 = pop3.filter(|p| !p.host.trim().is_empty());
        let (kind, incoming) = match (&imap, &pop3) {
            (None, Some(pop3)) => (Kind::Pop3, pop3.clone()),
            _ => (Kind::Imap, account.imap.clone()),
        };
        dialog.found_imap = imap;
        dialog.found_pop3 = pop3;
        dialog.incoming_kind = kind;
        let fields = [
            (
                dialog.incoming.host.clone(),
                dialog.incoming.port.clone(),
                &incoming,
            ),
            (
                dialog.smtp.host.clone(),
                dialog.smtp.port.clone(),
                &account.smtp,
            ),
        ];
        for (host, port_field, spec) in fields {
            let port = if spec.port == 0 {
                String::new()
            } else {
                spec.port.to_string()
            };
            fill(&mut dialog.seen, &host, spec.host.clone(), cx);
            fill(&mut dialog.seen, &port_field, port, cx);
        }
        dialog.incoming.security = Security::parse(&incoming.security);
        dialog.smtp.security = Security::parse(&account.smtp.security);
        let username = match incoming.username.trim() {
            "" => account.address.clone(),
            name => name.to_owned(),
        };
        let field = dialog.username.clone();
        fill(&mut dialog.seen, &field, username, cx);
        dialog.accept_invalid_certs = incoming.accept_invalid_certs;
        dialog.servers_for = account.address.clone();
        dialog.source = source;
        dialog.sign_in = None;
        dialog.password_works = true;
    }

    /// Picks IMAP or POP3 for mail coming in: the server found for it, or
    /// the usual name and port.
    fn set_incoming_kind(&mut self, kind: Kind, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        if dialog.incoming_kind == kind {
            return;
        }
        let old = dialog.incoming_kind;
        dialog.incoming_kind = kind;
        let found = match kind {
            Kind::Pop3 => dialog.found_pop3.clone(),
            _ => dialog.found_imap.clone(),
        };
        let (host_field, port_field) = (dialog.incoming.host.clone(), dialog.incoming.port.clone());
        match found {
            Some(spec) => {
                fill(&mut dialog.seen, &host_field, spec.host.clone(), cx);
                fill(&mut dialog.seen, &port_field, spec.port.to_string(), cx);
                dialog.incoming.security = Security::parse(&spec.security);
            }
            None => {
                let host = swap_host(host_field.read(cx).text(), kind);
                fill(&mut dialog.seen, &host_field, host, cx);
                let port = port_field.read(cx).text().trim().to_owned();
                let security = dialog.incoming.security;
                if port.is_empty() || port == security.port(old).to_string() {
                    fill(
                        &mut dialog.seen,
                        &port_field,
                        security.port(kind).to_string(),
                        cx,
                    );
                }
                dialog.source = None;
            }
        }
        dialog.error = None;
        cx.notify();
    }

    fn add_account_step(&mut self, step: Step, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        dialog.step = step;
        dialog.error = None;
        let focus = match step {
            Step::Form => {
                // The first empty field of name, address and password.
                let empty = |f: &Entity<TextInput>| f.read(cx).text().is_empty();
                if empty(&dialog.address) {
                    dialog.address.focus_handle(cx)
                } else if empty(&dialog.password) {
                    dialog.password.focus_handle(cx)
                } else {
                    dialog.address.focus_handle(cx)
                }
            }
            Step::Servers => dialog.incoming.host.focus_handle(cx),
            Step::Provider | Step::Browser(_) | Step::Done => self.dialog_focus.clone(),
        };
        window.focus(&focus, cx);
        cx.notify();
    }

    /// The typed address, or why it is not one.
    fn add_account_address(&self, cx: &Context<Self>) -> Result<String, String> {
        let Some(dialog) = &self.add_account else {
            return Err(String::new());
        };
        let address = dialog.address.read(cx).text().trim().to_owned();
        if address.is_empty() {
            Err(tr!("add-account-address-empty"))
        } else if !outgoing::valid_email(&address) {
            Err(tr!(
                "add-account-address-invalid",
                example = "kay@example.org"
            ))
        } else {
            Ok(address)
        }
    }

    /// "Edit servers manually": fills in guesses when nothing was found yet.
    fn add_account_servers(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let address = match self.add_account_address(cx) {
            Ok(address) => address,
            Err(err) => {
                self.add_account_error(err, cx);
                return;
            }
        };
        if self
            .add_account
            .as_ref()
            .is_some_and(|d| d.servers_for != address)
        {
            self.fill_servers(&guess(&address), None, None, cx);
        }
        self.add_account_step(Step::Servers, window, cx);
    }

    fn add_account_error(&mut self, error: String, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.add_account {
            dialog.error = Some(error);
            dialog.busy = false;
        }
        cx.notify();
    }

    /// The primary button (or Enter): goes on from the step that is
    /// showing.
    fn add_account_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &self.add_account else {
            return;
        };
        if dialog.busy || dialog.closing {
            return;
        }
        match dialog.step {
            Step::Provider | Step::Browser(_) => {}
            Step::Form => {
                if self.form_ready(window, cx) {
                    self.discover_servers(window, cx);
                }
            }
            Step::Servers => match self.typed_servers(cx) {
                Ok(()) => self.add_the_account(window, cx),
                Err(err) => self.add_account_error(err, cx),
            },
            Step::Done => self.close_add_account(cx),
        }
    }

    /// Back: the step before the one showing.
    fn add_account_back(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let back = match dialog.step {
            Step::Form => Step::Provider,
            Step::Servers => Step::Form,
            Step::Browser(_) => {
                self.cancel_browser_sign_in(cx);
                if let Some(dialog) = &mut self.add_account {
                    dialog._task = None;
                    dialog.busy = false;
                }
                Step::Provider
            }
            Step::Provider | Step::Done => return,
        };
        if let Some(dialog) = &mut self.add_account {
            // A lookup or login under way stops.
            dialog._task = None;
            dialog.busy = false;
        }
        self.add_account_step(back, window, cx);
    }

    /// Checks the address, password and name before anything goes out.
    fn form_ready(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Err(err) = self.add_account_address(cx) {
            if let Some(dialog) = &self.add_account {
                window.focus(&dialog.address.focus_handle(cx), cx);
            }
            self.add_account_error(err, cx);
            return false;
        }
        let Some(dialog) = &self.add_account else {
            return false;
        };
        let password = dialog.password.read(cx).text().to_owned();
        if password.is_empty() {
            window.focus(&dialog.password.focus_handle(cx), cx);
            self.add_account_error(tr!("add-account-password-empty"), cx);
            return false;
        }
        // The name is shown and stored in the open, so a password typed or
        // pasted into it by mistake must not go through.
        if is_password(dialog.name.read(cx).text(), &password) {
            let name = dialog.name.clone();
            name.update(cx, |n, cx| n.select_all_text(cx));
            window.focus(&name.focus_handle(cx), cx);
            self.add_account_error(tr!("add-account-name-is-password"), cx);
            return false;
        }
        true
    }

    /// Asks the daemon for the servers of the typed address, then adds the
    /// account with them.
    fn discover_servers(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let address = match self.add_account_address(cx) {
            Ok(address) => address,
            Err(err) => {
                self.add_account_error(err, cx);
                return;
            }
        };
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        if dialog.servers_for == address {
            self.after_discovery(window, cx);
            return;
        }
        dialog.busy = true;
        dialog.phase = Phase::Finding;
        dialog.error = None;
        let connection = self.daemon.clone();
        let lookup = address.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::discover(&connection, &lookup).await
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                if let Some(dialog) = &mut this.add_account {
                    dialog.busy = false;
                }
                match result {
                    Ok(found) => {
                        this.fill_servers(&found.account, Some(found.pop3), Some(found.source), cx);
                        if let Some(dialog) = &mut this.add_account {
                            dialog.sign_in = found.sign_in;
                            dialog.password_works = found.password;
                        }
                        this.after_discovery(window, cx);
                    }
                    Err(err) if err == daemon::NOT_RUNNING => this.add_account_error(err, cx),
                    Err(err) => {
                        tracing::info!(%err, "no servers found");
                        this.fill_servers(&guess(&address), None, None, cx);
                        this.add_account_step(Step::Servers, window, cx);
                        this.add_account_error(
                            tr!("add-account-not-found", address = address.as_str()),
                            cx,
                        );
                    }
                }
            })
            .ok();
        });
        if let Some(dialog) = &mut self.add_account {
            dialog._task = Some(task);
        }
        cx.notify();
    }

    /// The typed servers: incoming and outgoing, or what is wrong with
    /// them.
    fn typed_specs(&self, cx: &Context<Self>) -> Result<(ServerSpec, ServerSpec), String> {
        let Some(dialog) = &self.add_account else {
            return Err(String::new());
        };
        let username = dialog.username.read(cx).text();
        let spec = |fields: &ServerFields, what: &str| {
            server_spec(
                fields.host.read(cx).text(),
                fields.port.read(cx).text(),
                fields.security,
                username,
                dialog.accept_invalid_certs,
                what,
            )
        };
        Ok((
            spec(&dialog.incoming, "incoming")?,
            spec(&dialog.smtp, "outgoing")?,
        ))
    }

    fn typed_servers(&self, cx: &Context<Self>) -> Result<(), String> {
        self.typed_specs(cx).map(|_| ())
    }

    /// Checks the password with the server and adds the account.
    fn add_the_account(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let address = match self.add_account_address(cx) {
            Ok(address) => address,
            Err(err) => {
                self.add_account_step(Step::Form, window, cx);
                self.add_account_error(err, cx);
                return;
            }
        };
        let (incoming, smtp) = match self.typed_specs(cx) {
            Ok(specs) => specs,
            Err(err) => {
                self.add_account_step(Step::Servers, window, cx);
                self.add_account_error(err, cx);
                return;
            }
        };
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let password = dialog.password.read(cx).text().to_owned();
        if password.is_empty() {
            self.add_account_step(Step::Form, window, cx);
            self.add_account_error(tr!("add-account-password-empty"), cx);
            return;
        }
        let display_name = dialog.name.read(cx).text().trim().to_owned();
        let kind = dialog.incoming_kind;
        dialog.busy = true;
        dialog.phase = Phase::SigningIn;
        dialog.error = None;
        let added = Added {
            id: 0,
            address: address.clone(),
            name: display_name.clone(),
            signed_in: None,
            incoming: Some((kind, incoming.host.clone())),
            smtp: smtp.host.clone(),
        };
        let connection = self.daemon.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await.map_err(AddError::Other)?,
                    };
                    if kind == Kind::Pop3 {
                        // Thunderbird's default: mail stays on the server
                        // until it is deleted in Katna.
                        let keep = Pop3Keep::default();
                        let account = NewPop3Account {
                            display_name,
                            address,
                            pop3: incoming,
                            smtp,
                            leave_on_server: keep.leave_on_server,
                            keep_days: keep.days.unwrap_or(0),
                            delete_with_local: keep.delete_with_local,
                        };
                        daemon::add_pop3_account(&connection, &account, &password).await
                    } else {
                        let account = NewImapAccount {
                            display_name,
                            address,
                            imap: incoming,
                            smtp,
                        };
                        daemon::add_account(&connection, &account, &password).await
                    }
                })
                .await;
            this.update_in(cx, |this, window, cx| match result {
                Ok(id) => this.account_done(Added { id, ..added }, window, cx),
                Err(AddError::Password(detail)) => {
                    tracing::info!(%detail, "login refused");
                    let hint = match app_password_provider(&added.address) {
                        Some(provider) => {
                            tr!("add-account-app-password-refused", provider = provider)
                        }
                        None => tr!("add-account-password-refused"),
                    };
                    this.add_account_step(Step::Form, window, cx);
                    this.add_account_error(hint, cx);
                    if let Some(dialog) = &this.add_account {
                        let password = dialog.password.clone();
                        password.update(cx, |p, cx| p.select_all_text(cx));
                        window.focus(&password.focus_handle(cx), cx);
                    }
                }
                Err(AddError::Other(err)) => this.add_account_error(err, cx),
            })
            .ok();
        });
        if let Some(dialog) = &mut self.add_account {
            dialog._task = Some(task);
        }
        cx.notify();
    }

    /// The account is in: shows what got set up, and gets its mail.
    fn account_done(&mut self, added: Added, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.add_account {
            dialog.busy = false;
            dialog.added = Some(added);
            dialog.linking = None;
        }
        self.add_account_step(Step::Done, window, cx);
        self.account_added(window, cx);
    }

    /// Adds another account from the last step.
    fn add_another(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let fields = [
            dialog.address.clone(),
            dialog.password.clone(),
            dialog.name.clone(),
        ];
        for field in fields {
            fill(&mut dialog.seen, &field, String::new(), cx);
        }
        dialog.added = None;
        dialog.linking = None;
        dialog.servers_for.clear();
        dialog.source = None;
        dialog.provider = MailProvider::Other;
        self.add_account_step(Step::Provider, window, cx);
    }

    /// Links Zoho's tasks and calendars to the Zoho Mail account just added.
    fn link_zoho(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let Some(added) = &dialog.added else {
            return;
        };
        let (id, address) = (added.id, added.address.clone());
        dialog.linking = Some(Linking::Busy);
        let connection = self.daemon.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await.map_err(AddError::Other)?,
                    };
                    daemon::sign_in(&connection, OAuthProvider::Zoho, Some(id), &address).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(dialog) = &mut this.add_account {
                    dialog.linking = Some(match result {
                        Ok(_) => Linking::Done,
                        Err(AddError::Password(_)) => Linking::Failed(tr!(
                            "add-account-sign-in-refused",
                            provider = OAuthProvider::Zoho.name()
                        )),
                        Err(AddError::Other(err)) => Linking::Failed(err),
                    });
                }
                cx.notify();
            })
            .ok();
        });
        if let Some(dialog) = &mut self.add_account {
            dialog._task = Some(task);
        }
        cx.notify();
    }

    /// Keys on the dialog itself: Esc goes back or closes, and the arrow
    /// keys move between the tiles.
    fn add_account_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(dialog) = &self.add_account else {
            return;
        };
        let key = event.keystroke.key.as_str();
        let modified = event.keystroke.modifiers.modified();
        match (key, dialog.step) {
            ("escape", _) if !modified => {
                cx.stop_propagation();
                self.close_add_account(cx);
            }
            ("right" | "down" | "left" | "up", Step::Provider) if !modified => {
                cx.stop_propagation();
                // Up and Down go a whole row, over both columns.
                let width = self.room_width().min(WIDE + 32.0) - 32.0;
                let columns = if width - 80.0 >= TWO_COLUMNS { 2 } else { 1 };
                let steps = if matches!(key, "up" | "down") {
                    columns
                } else {
                    1
                };
                for _ in 0..steps {
                    if matches!(key, "right" | "down") {
                        window.focus_next(cx);
                    } else {
                        window.focus_prev(cx);
                    }
                }
            }
            ("enter", Step::Done) if !modified => {
                cx.stop_propagation();
                self.close_add_account(cx);
            }
            _ => {}
        }
    }

    pub(super) fn render_add_account(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let dialog = self.add_account.as_mut()?;
        dialog.shown.set(if dialog.closing { 0.0 } else { 1.0 });
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.add_account = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let dialog = self.add_account.as_ref()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let step = dialog.step;
        let width = match step {
            Step::Provider => WIDE,
            _ => WIDTH,
        }
        .min(vw - 32.0);
        // Narrow windows take less padding, so the fields keep their room.
        let pad = if width < 400.0 { 24.0 } else { 40.0 };
        let busy = dialog.busy;

        let body = match step {
            Step::Provider => self.provider_step(width - 2.0 * pad, th, cx),
            Step::Form => self.form_step(th, window, cx),
            Step::Servers => self.servers_step(th, window, cx),
            Step::Browser(provider) => self.browser_step(provider, th),
            Step::Done => self.done_step(th, cx),
        };
        let footer = self.add_account_footer(th, cx);

        let card = div()
            .id("add-account")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .key_context("AddAccount")
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.add_account_key(event, window, cx)
            }))
            .occlude()
            .relative()
            .w(px(width))
            // Clear of the window's edges and frame, whose height the
            // viewport includes.
            .max_h(px((vh - 112.0).max(240.0)))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.surface))
            .text_color(rgba(th.text))
            .child(
                div()
                    .id("add-account-body")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(pad))
                    .pt(px(32.0))
                    .pb(px(8.0))
                    .child(body),
            )
            .child(
                div()
                    .flex_none()
                    .px(px(pad))
                    .pt(px(16.0))
                    .pb(px(24.0))
                    .child(footer),
            )
            .when(busy, |d| d.child(progress_bar(th)));

        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("add-account-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }

    /// The first step: a tile for each provider.
    fn provider_step(&self, inner: f32, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let error = self.add_account.as_ref().and_then(|d| d.error.clone());
        let columns = if inner >= TWO_COLUMNS { 2 } else { 1 };
        let tile_width = if columns == 2 {
            (inner - 12.0) / 2.0
        } else {
            inner
        };
        let tiles =
            MailProvider::shown().enumerate().map(|(ix, provider)| {
                let wide = provider == MailProvider::Other && columns == 2;
                div()
                    .id(("provider-tile", ix))
                    .w(px(tile_width))
                    .h(px(64.0))
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .relative()
                    .map(|d| crate::widgets::tile(d, th))
                    .cursor_pointer()
                    .child(crate::widgets::tile_hover(th))
                    .focus_ring(th)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.pick_provider(provider, window, cx)
                    }))
                    .child(provider.glyph(32.0, th))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .truncate()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(provider.name()),
                            )
                            .child(
                                div()
                                    .truncate()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_faint))
                                    .child(provider.detail()),
                            ),
                    )
                    .map(|d| if wide { d.mx_auto() } else { d })
            });
        div()
            .flex()
            .flex_col()
            .child(centered_header(
                logo(th),
                tr!("add-account-title"),
                tr!("add-account-providers-intro"),
                th,
            ))
            .child(
                div()
                    .mt(px(24.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(12.0))
                    .children(tiles),
            )
            .children(error.map(|error| self.error_line(error, th)))
            .into_any_element()
    }

    /// The name, address and password, the provider's line on passwords,
    /// and each stage of adding while it goes.
    fn form_step(&self, th: &Theme, window: &Window, cx: &mut Context<Self>) -> AnyElement {
        let Some(dialog) = &self.add_account else {
            return div().into_any_element();
        };
        let provider = dialog.provider;
        let error = dialog.error.clone();
        let (title, intro) = match provider {
            MailProvider::Other => (
                tr!("add-account-form-title-other"),
                tr!("add-account-address-intro"),
            ),
            _ => (
                tr!("add-account-form-title", provider = provider.mail_name()),
                tr!("add-account-form-intro"),
            ),
        };
        let address = dialog.address.read(cx).text().trim().to_owned();
        let password_help = provider.password_help().map(|(help, url)| {
            let text = match help {
                PasswordHelp::AppPassword => {
                    tr!("add-account-app-password-hint", provider = provider.name())
                }
                PasswordHelp::TurnOnImap => {
                    tr!("add-account-help-turn-on-imap", provider = provider.name())
                }
            };
            let link = match help {
                PasswordHelp::AppPassword => tr!("add-account-help-app-password-link"),
                PasswordHelp::TurnOnImap => tr!("add-account-help-turn-on-imap-link"),
            };
            help_box(text, Some((link, url)), th)
        });
        // An address that signs in in the browser, typed under Other.
        let sign_in_instead = (provider == MailProvider::Other && !dialog.busy)
            .then(|| MailProvider::for_address(&address).sign_in())
            .flatten()
            .or(dialog.sign_in.filter(|p| p.available() && !dialog.busy));
        let stages = dialog.busy.then(|| {
            let finding = dialog.phase == Phase::Finding;
            let host = dialog.incoming.host.read(cx).text().trim().to_owned();
            div()
                .mt(px(20.0))
                .p(px(16.0))
                .flex()
                .flex_col()
                .gap(px(10.0))
                .rounded(px(12.0))
                .bg(rgba(fade(th.accent, 0.08)))
                .child(stage(
                    "add-account-stage-find",
                    tr!("add-account-looking", address = address.as_str()),
                    if finding { Stage::Now } else { Stage::Done },
                    th,
                ))
                .child(stage(
                    "add-account-stage-sign-in",
                    if finding || host.is_empty() {
                        tr!("add-account-signing-in")
                    } else {
                        tr!("add-account-stage-signing-in-at", server = host.as_str())
                    },
                    if finding { Stage::Next } else { Stage::Now },
                    th,
                ))
                .into_any_element()
        });
        let show_password = div()
            .id("show-password")
            .mt(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(dialog) = &mut this.add_account {
                    dialog.show_password = !dialog.show_password;
                    let masked = !dialog.show_password;
                    dialog.password.update(cx, |p, cx| p.set_masked(masked, cx));
                }
                cx.notify();
            }))
            .child(crate::widgets::checkbox(
                "add-account-show-password-box",
                crate::widgets::Check::from(dialog.show_password),
                th,
            ))
            .child(tr!("add-account-show-password"));
        div()
            .flex()
            .flex_col()
            .child(header(provider.glyph(40.0, th), title, Some(intro), th))
            .child(div().mt(px(24.0)).child(self.outlined_field(
                "field-name",
                tr!("add-account-field-name"),
                &dialog.name,
                false,
                th,
                window,
                cx,
            )))
            .child(hint(tr!("add-account-name-hint"), th))
            .child(div().mt(px(16.0)).child(self.outlined_field(
                "field-address",
                tr!("add-account-field-address"),
                &dialog.address,
                false,
                th,
                window,
                cx,
            )))
            .child(div().mt(px(16.0)).child(self.outlined_field(
                "field-password",
                tr!("add-account-field-password"),
                &dialog.password,
                false,
                th,
                window,
                cx,
            )))
            .child(show_password)
            .children(error.map(|error| self.error_line(error, th)))
            .children(stages)
            .when(!dialog.busy, |d| d.children(password_help))
            .children(sign_in_instead.map(|provider| {
                div().mt(px(8.0)).flex().child(
                    text_button(
                        "add-account-sign-in-instead",
                        tr!("add-account-sign-in-instead", provider = provider.name()),
                        th,
                    )
                    .ml(px(-12.0))
                    .focus_ring(th)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.start_sign_in(provider, window, cx)
                    })),
                )
            }))
            .child(
                div().mt(px(12.0)).flex().child(
                    text_button(
                        "add-account-edit-servers",
                        tr!("add-account-servers-button"),
                        th,
                    )
                    .ml(px(-12.0))
                    .focus_ring(th)
                    .on_click(
                        cx.listener(|this, _, window, cx| this.add_account_servers(window, cx)),
                    ),
                ),
            )
            .into_any_element()
    }

    /// The servers, typed or corrected by hand.
    fn servers_step(&self, th: &Theme, window: &Window, cx: &mut Context<Self>) -> AnyElement {
        let Some(dialog) = &self.add_account else {
            return div().into_any_element();
        };
        let error = dialog.error.clone();
        let address = dialog.address.read(cx).text().trim().to_owned();
        let kind = dialog.incoming_kind;
        let segment = |choice: Kind| {
            let on = kind == choice;
            div()
                .id(("incoming-kind", choice as usize))
                .flex_1()
                .h(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .gap(px(6.0))
                .rounded_full()
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(on, |d| {
                    d.bg(rgba(th.nav_selected))
                        .text_color(rgba(th.nav_selected_text))
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text_dim))
                        .hover(|s| s.bg(rgba(th.hover)))
                })
                .focus_ring(th)
                .on_click(cx.listener(move |this, _, _, cx| this.set_incoming_kind(choice, cx)))
                .when(on, |d| d.child(icon("check", th.nav_selected_text, 16.0)))
                .child(choice.name())
        };
        let about = match kind {
            Kind::Pop3 => tr!("add-account-pop3-about"),
            _ => tr!("add-account-imap-about"),
        };
        div()
            .flex()
            .flex_col()
            .child(header(
                icon("settings", th.text_dim, 36.0),
                tr!("add-account-servers-title"),
                Some(tr!("add-account-servers-intro", address = address.as_str())),
                th,
            ))
            .child(section_title(tr!("add-account-receive-with"), th))
            .child(
                div()
                    .p(px(4.0))
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .rounded_full()
                    .border_1()
                    .border_color(rgba(fade(th.text_faint, 0.5)))
                    .child(segment(Kind::Imap))
                    .child(segment(Kind::Pop3)),
            )
            .child(hint(about, th))
            .child(section_title(
                tr!("add-account-incoming", protocol = kind.name()),
                th,
            ))
            .child(self.server_fields(kind, error.is_some(), th, window, cx))
            .child(section_title(
                tr!("add-account-outgoing", protocol = "SMTP"),
                th,
            ))
            .child(self.server_fields(Kind::Smtp, false, th, window, cx))
            .child(div().mt(px(16.0)).child(self.outlined_field(
                "field-username",
                tr!("add-account-field-username"),
                &dialog.username,
                false,
                th,
                window,
                cx,
            )))
            .children(error.map(|error| self.error_line(error, th)))
            .into_any_element()
    }

    fn browser_step(&self, provider: OAuthProvider, th: &Theme) -> AnyElement {
        let glyph = match provider {
            OAuthProvider::Microsoft => MailProvider::Microsoft,
            _ => MailProvider::Google,
        }
        .glyph(40.0, th);
        let error = self.add_account.as_ref().and_then(|d| d.error.clone());
        div()
            .flex()
            .flex_col()
            .child(header(
                glyph,
                tr!("add-account-browser-title"),
                Some(tr!("add-account-browser-intro", provider = provider.name())),
                th,
            ))
            .child(
                div()
                    .mt(px(20.0))
                    .p(px(16.0))
                    .rounded(px(12.0))
                    .bg(rgba(fade(th.accent, 0.08)))
                    .child(stage(
                        "add-account-stage-browser",
                        tr!("add-account-stage-browser"),
                        Stage::Now,
                        th,
                    )),
            )
            .child(hint(tr!("add-account-browser-hint"), th))
            .children(error.map(|error| self.error_line(error, th)))
            .into_any_element()
    }

    /// The last step: what got set up.
    fn done_step(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(dialog) = &self.add_account else {
            return div().into_any_element();
        };
        let Some(added) = &dialog.added else {
            return div().into_any_element();
        };
        let name = self
            .accounts
            .iter()
            .find(|a| a.id.0 == added.id)
            .map(|a| a.display_name.clone())
            .filter(|n| !n.trim().is_empty())
            .or_else(|| (!added.name.is_empty()).then(|| added.name.clone()))
            .unwrap_or_else(|| added.address.clone());
        let mut rows: Vec<(String, String)> = Vec::new();
        if let Some(provider) = added.signed_in {
            rows.push((
                tr!("add-account-done-sign-in"),
                tr!(
                    "add-account-done-signed-in-with",
                    provider = provider.name()
                ),
            ));
        }
        if let Some((kind, host)) = &added.incoming {
            rows.push((
                tr!("add-account-done-receiving"),
                format!("{} \u{b7} {host}", kind.name()),
            ));
        }
        if !added.smtp.is_empty() {
            rows.push((
                tr!("add-account-done-sending"),
                format!("SMTP \u{b7} {}", added.smtp),
            ));
        }
        let pop3 = matches!(added.incoming, Some((Kind::Pop3, _)));
        if pop3 {
            rows.push((
                tr!("add-account-done-on-server"),
                tr!("add-account-done-kept"),
            ));
        }
        let zoho = MailProvider::for_address(&added.address) == MailProvider::Zoho
            || dialog.provider == MailProvider::Zoho;
        let linking = dialog.linking.clone();
        let fact = |label: String, value: String| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_x(px(12.0))
                .text_size(px(13.0))
                .line_height(px(20.0))
                .child(
                    div()
                        .w(px(132.0))
                        .text_color(rgba(th.text_faint))
                        .child(label),
                )
                .child(div().flex_1().min_w(px(160.0)).child(value))
        };
        let zoho_row = (zoho && OAuthProvider::Zoho.available()).then(|| {
            let control = match &linking {
                Some(Linking::Done) => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(icon("check", th.accent, 18.0))
                    .child(tr!("add-account-done-linked"))
                    .into_any_element(),
                Some(Linking::Busy) => stage(
                    "add-account-stage-zoho",
                    tr!("add-account-stage-browser"),
                    Stage::Now,
                    th,
                ),
                _ => crate::widgets::outlined_button(
                    "add-account-link-zoho",
                    tr!("add-account-sign-in-with", provider = "Zoho"),
                    th,
                )
                .focus_ring(th)
                .on_click(cx.listener(|this, _, window, cx| this.link_zoho(window, cx)))
                .into_any_element(),
            };
            div()
                .mt(px(16.0))
                .p(px(16.0))
                .flex()
                .flex_col()
                .gap(px(10.0))
                .rounded(px(12.0))
                .border_1()
                .border_color(rgba(fade(th.text_faint, 0.35)))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(tr!("add-account-done-zoho-title")),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .line_height(px(18.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("add-account-done-zoho-about")),
                )
                .child(div().flex().child(control))
                .children(match &linking {
                    Some(Linking::Failed(err)) => Some(self.error_line(err.clone(), th)),
                    _ => None,
                })
        });
        div()
            .flex()
            .flex_col()
            .child(header(
                div()
                    .size(px(40.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(th.accent, 0.14)))
                    .child(icon("check", th.accent, 24.0))
                    .into_any_element(),
                tr!("add-account-done-title"),
                Some(tr!("add-account-done-intro")),
                th,
            ))
            .child(
                div()
                    .mt(px(24.0))
                    .p(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .map(|d| crate::widgets::tile(d, th))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .child(self.person_avatar(&name, &added.address, 40.0))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .truncate()
                                            .text_size(px(15.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(name.clone()),
                                    )
                                    .child(
                                        div()
                                            .truncate()
                                            .text_size(px(13.0))
                                            .text_color(rgba(th.text_faint))
                                            .child(added.address.clone()),
                                    ),
                            ),
                    )
                    .child(div().h(px(1.0)).bg(rgba(th.divider)))
                    .children(rows.into_iter().map(|(label, value)| fact(label, value))),
            )
            .when(added.signed_in.is_none(), |d| {
                d.child(hint(self.servers_summary(cx), th))
            })
            .when(pop3, |d| {
                d.child(hint(tr!("add-account-done-pop3-hint"), th))
            })
            .children(zoho_row)
            .into_any_element()
    }

    /// The buttons along the bottom of each step.
    fn add_account_footer(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(dialog) = &self.add_account else {
            return div().into_any_element();
        };
        let step = dialog.step;
        let busy = dialog.busy;
        let back = matches!(step, Step::Form | Step::Servers | Step::Browser(_));
        let (primary, primary_label) = match step {
            Step::Form | Step::Servers => (true, tr!("add-account-add")),
            Step::Done => (true, tr!("add-account-done")),
            Step::Provider | Step::Browser(_) => (false, String::new()),
        };
        let mut row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0));
        if back {
            row = row.child(
                text_button("add-account-back", tr!("add-account-back"), th)
                    .ml(px(-12.0))
                    .focus_ring(th)
                    .on_click(cx.listener(|this, _, window, cx| this.add_account_back(window, cx))),
            );
        }
        if step == Step::Done {
            row = row.child(
                text_button("add-account-another", tr!("add-account-another"), th)
                    .ml(px(-12.0))
                    .focus_ring(th)
                    .on_click(cx.listener(|this, _, window, cx| this.add_another(window, cx))),
            );
        }
        row = row.child(div().flex_1());
        if step != Step::Done {
            row = row.child(
                text_button("add-account-cancel", tr!("add-account-cancel"), th)
                    .focus_ring(th)
                    .on_click(cx.listener(|this, _, _, cx| this.close_add_account(cx))),
            );
        }
        if primary {
            row = row.child(
                filled_button("add-account-next", primary_label, th)
                    .focus_ring_filled(th)
                    .when(busy, |d| d.opacity(0.6))
                    .on_click(cx.listener(|this, _, window, cx| this.add_account_next(window, cx))),
            );
        }
        row.into_any_element()
    }

    /// "Servers: imap.x.org and smtp.x.org, found in …".
    fn servers_summary(&self, cx: &Context<Self>) -> String {
        let Some(dialog) = &self.add_account else {
            return String::new();
        };
        let imap = dialog.incoming.host.read(cx).text().trim().to_owned();
        let smtp = dialog.smtp.host.read(cx).text().trim().to_owned();
        let servers = if imap == smtp {
            imap
        } else {
            tr!("add-account-servers-pair", imap = imap, smtp = smtp)
        };
        match &dialog.source {
            Some(source) => tr!(
                "add-account-servers-found",
                servers = servers,
                source = source_key(source)
            ),
            None => tr!("add-account-servers-entered", servers = servers),
        }
    }

    /// Host, port and security of one server.
    fn server_fields(
        &self,
        kind: Kind,
        error: bool,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(dialog) = &self.add_account else {
            return div().into_any_element();
        };
        let (fields, prefix) = match kind {
            Kind::Smtp => (&dialog.smtp, "smtp"),
            _ => (&dialog.incoming, "incoming"),
        };
        let chips = Security::ALL.map(|security| {
            let on = fields.security == security;
            div()
                .id((prefix, security as usize))
                .h(px(32.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(if on {
                    th.nav_selected
                } else {
                    fade(th.text_faint, 0.6)
                }))
                .when(on, |d| d.bg(rgba(th.nav_selected)))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(if on {
                    th.nav_selected_text
                } else {
                    th.text_dim
                }))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .focus_ring(th)
                .on_click(cx.listener(move |this, _, _, cx| this.set_security(kind, security, cx)))
                .child(security.label())
        });
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(self.outlined_field(
                (prefix, 100usize),
                tr!("add-account-field-server"),
                &fields.host,
                error,
                th,
                window,
                cx,
            ))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(12.0))
                    .child(div().w(px(96.0)).flex_none().child(self.outlined_field(
                        (prefix, 101usize),
                        tr!("add-account-field-port"),
                        &fields.port,
                        false,
                        th,
                        window,
                        cx,
                    )))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(chips),
                    ),
            )
            // No encryption: say so, without stopping anyone.
            .when(fields.security == Security::Plain, |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(8.0))
                        .text_size(px(12.0))
                        .line_height(px(16.0))
                        .text_color(rgba(th.error))
                        .child(icon("warning", th.error, 16.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(tr!("add-account-security-none-warning")),
                        ),
                )
            })
            .into_any_element()
    }

    /// Picks how a server is secured; a port left at the old default
    /// follows the new one.
    fn set_security(&mut self, kind: Kind, security: Security, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let fields = match kind {
            Kind::Smtp => &mut dialog.smtp,
            _ => &mut dialog.incoming,
        };
        let old = fields.security;
        fields.security = security;
        let port = fields.port.read(cx).text().trim().to_owned();
        if port.is_empty() || port == old.port(kind).to_string() {
            let new = security.port(kind).to_string();
            let field = fields.port.clone();
            fill(&mut dialog.seen, &field, new, cx);
        }
        dialog.source = None;
        cx.notify();
    }

    /// A Material outlined text field: the label sits in the box and moves
    /// onto its frame once there is text or focus.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn outlined_field(
        &self,
        id: impl Into<gpui::ElementId>,
        label: String,
        input: &Entity<TextInput>,
        error: bool,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let focus = input.focus_handle(cx);
        let focused = focus.is_focused(window);
        let floated = focused || !input.read(cx).text().is_empty();
        let frame = if error {
            th.error
        } else if focused {
            th.accent
        } else {
            fade(th.text_faint, 0.8)
        };
        let label_color = if error {
            th.error
        } else if focused {
            th.accent
        } else {
            th.text_faint
        };
        // The frame is its own layer, so the floated label paints over it.
        let outline = div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .rounded(px(4.0))
            .map(|d| {
                if focused || error {
                    d.border_px(2.0)
                } else {
                    d.border_1()
                }
            })
            .border_color(rgba(frame));
        div()
            .id(id.into())
            .relative()
            .h(px(56.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .text_size(px(16.0))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(outline)
            .child(div().flex_1().min_w_0().child(input.clone()))
            .child(
                div()
                    .absolute()
                    .left(px(11.0))
                    .px(px(4.0))
                    .text_color(rgba(label_color))
                    .map(|d| {
                        if floated {
                            d.top(px(-10.0)).text_size(px(12.0)).bg(rgba(th.surface))
                        } else {
                            d.top(px(16.0)).text_size(px(16.0))
                        }
                    })
                    .child(label),
            )
            .into_any_element()
    }

    /// Adds, switches and shows accounts: the card above the rail's
    /// account picture.
    pub(super) fn render_account_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.account_menu {
            return None;
        }
        // What is open is marked: All Accounts or one account. A click
        // on another switches to it.
        let open = self.menu_current();
        let all = self.all_accounts_row(open == Some(None), th, cx);
        let app_menu = self.render_app_menu(th, cx);
        let language = self.render_language_button(th, cx);
        let menu_button = self.app_menu_button(th, cx);
        let rows = self.accounts.iter().enumerate().map(|(ix, account)| {
            let name = if account.display_name.trim().is_empty() {
                account.address.clone()
            } else {
                account.display_name.clone()
            };
            let id = account.id;
            let current = open == Some(Some(id));
            let offline = self.offline_text(id);
            let unread = self
                .tree
                .accounts
                .iter()
                .find(|a| a.id == id)
                .map_or(0, |a| a.unread);
            div()
                .id(("account-row", ix))
                .h(px(56.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .when(current, |d| d.bg(rgba(th.row_selected)))
                .hover(move |s| s.bg(rgba(if current { th.row_selected } else { th.hover })))
                .menu_key(th)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.account_menu = false;
                    this.pick_account(id, cx);
                }))
                .child(self.offline_badge(
                    self.account_ring(
                        &account.address,
                        self.person_avatar(&name, &account.address, 32.0),
                        th,
                    ),
                    32.0,
                    offline.is_some(),
                    th,
                ))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(name),
                        )
                        .child(
                            div()
                                .truncate()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_faint))
                                // An offline account says so, and what waits.
                                .child(offline.clone().unwrap_or_else(|| account.address.clone())),
                        ),
                )
                .when(offline.is_some(), |d| {
                    d.child(self.offline_mark(
                        ("account-row-offline", ix),
                        id,
                        tokens::space::S7,
                        th,
                        cx,
                    ))
                })
                .when(unread > 0, |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text_dim))
                            .child(crate::format::thousands(unread)),
                    )
                })
                .when(current, |d| d.child(icon("check", th.text, 20.0)))
        });
        // The icon row at the top: Settings and the language, with the
        // application menu at its end.
        let icons = div()
            .h(px(48.0))
            .px(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(
                icon_button("account-settings", "settings", 22.0, th)
                    .tip(tr!("settings"), th)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.account_menu = false;
                        this.open_settings_here(window, cx);
                    })),
            )
            .child(language)
            .child(menu_button);
        let add = div()
            .id("account-add")
            .h(px(48.0))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(8.0))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
            .menu_key(th)
            .on_click(cx.listener(|this, _, window, cx| this.open_add_account(window, cx)))
            .child(
                div()
                    .size(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon("person-add", th.text_dim, 22.0)),
            )
            .child(if self.accounts.is_empty() {
                tr!("account-add")
            } else {
                tr!("add-account-menu-another")
            });
        let motion = self.page_motion(cx);
        let card = app_menu.unwrap_or_else(|| {
            let card = div()
                .id("account-menu")
                .key_context(crate::widgets::MENU_CONTEXT)
                .occlude()
                .absolute()
                .right(px(16.0))
                .top(px(4.0))
                // Narrower on a narrow phone, with the same room each side.
                .w(px(MENU_WIDTH.min(self.room_width() - 32.0)))
                .p(px(8.0))
                .flex()
                .flex_col()
                .map(|d| raised(d, th, super::PANEL_RADIUS, 2.0))
                .text_color(rgba(th.text));
            let page = div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(icons)
                .children(self.work_offline_row(th, cx))
                .when(!self.accounts.is_empty(), |d| {
                    d.child(
                        div()
                            .mx(px(16.0))
                            .my(px(4.0))
                            .h(px(1.0))
                            .bg(rgba(th.divider)),
                    )
                })
                .children(all)
                .children(rows)
                .when(!self.accounts.is_empty(), |d| {
                    d.child(
                        div()
                            .mx(px(16.0))
                            .my(px(4.0))
                            .h(px(1.0))
                            .bg(rgba(th.divider)),
                    )
                })
                .child(add);
            self.menu_page_card("account-menu-page", card, page, None, 8.0, motion)
        });
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.account_menu = false;
                this.app_menu = None;
                cx.notify();
            })
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                // Like the other menus: a press anywhere else closes it,
                // the top bar included.
                .child(
                    deferred(
                        div()
                            .id("account-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(1),
                )
                .child(deferred(card).with_priority(2))
                .into_any_element(),
        )
    }
}

/// The Katna Mail mark, as in the top bar.
pub(super) fn logo(th: &Theme) -> AnyElement {
    crate::widgets::katna_mark(40.0, th)
}

/// A step's mark, title and the line under it, centred.
fn centered_header(mark: AnyElement, title: String, intro: String, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .text_center()
        .child(mark)
        .child(
            div()
                .mt(px(16.0))
                .text_size(px(26.0))
                .line_height(px(34.0))
                .text_color(rgba(th.text))
                .child(title),
        )
        .child(
            div()
                .mt(px(8.0))
                .text_size(px(14.0))
                .line_height(px(20.0))
                .text_color(rgba(th.text_dim))
                .child(intro),
        )
        .into_any_element()
}

/// A step's mark, title and the line under it.
fn header(mark: AnyElement, title: String, intro: Option<String>, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .child(mark)
        .child(
            div()
                .mt(px(16.0))
                .text_size(px(24.0))
                .line_height(px(32.0))
                .text_color(rgba(th.text))
                .child(title),
        )
        .children(intro.map(|text| {
            div()
                .mt(px(8.0))
                .text_size(px(14.0))
                .line_height(px(20.0))
                .text_color(rgba(th.text_dim))
                .child(text)
        }))
        .into_any_element()
}

/// A tinted box saying what the provider asks for, with a link to its
/// own page on it.
fn help_box(text: String, link: Option<(String, &'static str)>, th: &Theme) -> AnyElement {
    div()
        .mt(px(20.0))
        .p(px(16.0))
        .flex()
        .flex_row()
        .items_start()
        .gap(px(12.0))
        .rounded(px(12.0))
        .bg(rgba(fade(th.accent, 0.08)))
        .child(icon("info", th.accent, 20.0))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .line_height(px(18.0))
                        .text_color(rgba(th.text_dim))
                        .child(text),
                )
                .children(link.map(|(label, url)| {
                    div().flex().child(
                        div()
                            .id("add-account-help-link")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .rounded(px(4.0))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.underline())
                            .focus_ring(th)
                            .on_click(move |_, _, cx| cx.open_url(url))
                            .child(label)
                            .child(icon("open-external", th.accent, 16.0)),
                    )
                })),
        )
        .into_any_element()
}

/// Where one stage of adding stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Done,
    Now,
    Next,
}

/// One stage of adding: a check once done, a turning arrow while under
/// way, a faint dot before.
fn stage(id: &'static str, text: String, at: Stage, th: &Theme) -> AnyElement {
    let mark = match at {
        Stage::Done => icon("check-circle", th.accent, 20.0),
        Stage::Now => super::nav_menu::turning_arrow(id, th.accent, 20.0).into_any_element(),
        Stage::Next => div()
            .size(px(20.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size(px(8.0))
                    .rounded_full()
                    .bg(rgba(fade(th.text_faint, 0.6))),
            )
            .into_any_element(),
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .text_size(px(14.0))
        .text_color(rgba(match at {
            Stage::Next => th.text_faint,
            _ => th.text,
        }))
        .child(mark)
        .child(div().flex_1().min_w_0().child(text))
        .into_any_element()
}

impl MailWindow {
    /// Why adding the account failed, in words that can be copied.
    fn error_line(&self, error: String, th: &Theme) -> AnyElement {
        div()
            .mt(px(8.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(8.0))
            .text_size(px(12.0))
            .line_height(px(16.0))
            .text_color(rgba(th.error))
            .child(icon("info", th.error, 16.0))
            .child(self.copyable(error, th).flex_1().min_w_0())
            .into_any_element()
    }
}

fn section_title(text: String, th: &Theme) -> AnyElement {
    div()
        .mt(px(24.0))
        .mb(px(12.0))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .child(text)
        .into_any_element()
}

fn hint(text: String, th: &Theme) -> AnyElement {
    div()
        .mt(px(8.0))
        .px(px(16.0))
        .text_size(px(12.0))
        .line_height(px(16.0))
        .text_color(rgba(th.text_faint))
        .child(text)
        .into_any_element()
}

/// A borderless button with an accent label.
pub(super) fn text_button(
    id: &'static str,
    label: impl Into<SharedString>,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    let label: SharedString = label.into();
    div()
        .id(id)
        .h(px(36.0))
        .px(px(12.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
        .child(label)
}

/// The indeterminate bar along the top of the card while the daemon works.
fn progress_bar(th: &Theme) -> AnyElement {
    div()
        // Kept clear of the rounded corners, which do not clip.
        .absolute()
        .top_0()
        .left(px(super::PANEL_RADIUS))
        .right(px(super::PANEL_RADIUS))
        .h(px(4.0))
        .rounded_b(px(2.0))
        .overflow_hidden()
        .bg(rgba(fade(th.accent, 0.24)))
        .child(
            div()
                .absolute()
                .top_0()
                .h_full()
                .w(relative(0.4))
                .bg(rgba(th.accent))
                .with_animation(
                    "add-account-progress",
                    Animation::new(katna_ui::motion::time(Duration::from_millis(1300))).repeat(),
                    |bar, t| bar.left(relative(lerp(-0.4, 1.0, t))),
                ),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_typed_servers() {
        let spec = server_spec(
            " imap.x.org ",
            "993",
            Security::Tls,
            "kay",
            false,
            "incoming",
        );
        assert_eq!(
            spec,
            Ok(ServerSpec {
                host: "imap.x.org".to_owned(),
                port: 993,
                security: "tls".to_owned(),
                username: "kay".to_owned(),
                accept_invalid_certs: false,
            })
        );
        assert!(server_spec("", "993", Security::Tls, "", false, "incoming").is_err());
        assert!(server_spec("a b", "993", Security::Tls, "", false, "incoming").is_err());
        assert!(server_spec("x.org", "0", Security::Tls, "", false, "outgoing").is_err());
        assert!(server_spec("x.org", "99999", Security::Tls, "", false, "outgoing").is_err());
    }

    #[test]
    fn guesses_usual_names() {
        let account = guess("kay@example.org");
        assert_eq!(account.imap.host, "imap.example.org");
        assert_eq!((account.imap.port, account.smtp.port), (993, 465));
        assert_eq!(account.smtp.username, "kay@example.org");
    }

    #[test]
    fn default_ports_follow_security() {
        assert_eq!(Security::StartTls.port(Kind::Imap), 143);
        assert_eq!(Security::StartTls.port(Kind::Smtp), 587);
        assert_eq!(Security::parse("starttls"), Security::StartTls);
        assert_eq!(Security::parse("tls").as_str(), "tls");
    }

    #[test]
    fn pop3_ports_and_names() {
        assert_eq!(Security::Tls.port(Kind::Pop3), 995);
        assert_eq!(Security::StartTls.port(Kind::Pop3), 110);
        assert_eq!(swap_host("imap.x.org", Kind::Pop3), "pop.x.org");
        assert_eq!(swap_host(" pop3.x.org", Kind::Imap), "imap.x.org");
        assert_eq!(swap_host("mail.x.org", Kind::Pop3), "mail.x.org");
    }

    #[test]
    fn a_password_is_not_a_name() {
        assert!(is_password("abcdefghijklmnop", "abcd efgh ijkl mnop"));
        assert!(is_password(" abcd efgh ijkl mnop", "abcdefghijklmnop"));
        assert!(!is_password("", ""));
        assert!(!is_password("Kay Example", "hunter2"));
    }

    #[test]
    fn knows_app_password_providers() {
        assert_eq!(app_password_provider("kay@Gmail.com"), Some("Gmail"));
        assert_eq!(app_password_provider("kay@example.org"), None);
    }
}
