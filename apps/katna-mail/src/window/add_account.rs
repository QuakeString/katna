// SPDX-License-Identifier: GPL-3.0-or-later

//! Adding a mail account (`docs/ARCHITECTURE.md` §13.6), in a dialog shaped
//! like a web sign-in: the address first, then the password. The daemon
//! finds the servers from the address (`DiscoverAccount`), checks the login
//! and saves the account (`AddImapAccount`). When it cannot find the
//! servers, they are entered by hand. Also the account menu of the app
//! rail, which leads here.

use std::collections::HashMap;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, EntityId, Focusable, FontWeight, Hsla,
    MouseButton, MouseDownEvent, SharedString, Subscription, Task, Window, deferred, div,
    prelude::*, relative, rgba,
};
use katna_dbus::{NewImapAccount, ServerSpec};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;
use katna_ui::{InputEvent, TextInput};

use super::{MailWindow, RailApp};
use crate::daemon::{self, AddError};
use crate::outgoing;
use crate::sidebar::Role;
use crate::theme::{Theme, fade};
use crate::widgets::{avatar, elevation, filled_button, icon, raised};

const WIDTH: f32 = 448.0;
const MENU_WIDTH: f32 = 340.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Address,
    Servers,
    Password,
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
            (Kind::Smtp, Self::Tls) => 465,
            (Kind::Smtp, _) => 587,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Imap,
    Smtp,
}

/// One server's fields.
struct ServerFields {
    host: Entity<TextInput>,
    port: Entity<TextInput>,
    security: Security,
}

/// The open add-account dialog.
pub(super) struct AddAccount {
    step: Step,
    address: Entity<TextInput>,
    password: Entity<TextInput>,
    name: Entity<TextInput>,
    imap: ServerFields,
    smtp: ServerFields,
    username: Entity<TextInput>,
    /// The address the server fields were filled for.
    servers_for: String,
    /// Where the daemon found the servers; `None` once they are edited.
    source: Option<String>,
    accept_invalid_certs: bool,
    show_password: bool,
    busy: bool,
    error: Option<String>,
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
        let imap = ServerFields {
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
        for field in [&imap.host, &imap.port, &smtp.host, &smtp.port, &username] {
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
        window.focus(&address.focus_handle(cx), cx);
        self.add_account = Some(AddAccount {
            step: Step::Address,
            address,
            password,
            name,
            imap,
            smtp,
            username,
            servers_for: String::new(),
            source: None,
            accept_invalid_certs: false,
            show_password: false,
            busy: false,
            error: None,
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
        if let Some(dialog) = &mut self.add_account {
            // Stops a discovery or login that is under way.
            dialog._task = None;
            dialog.closing = true;
        }
        cx.notify();
    }

    /// Fills the server fields from `account`.
    fn fill_servers(
        &mut self,
        account: &NewImapAccount,
        source: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let fields = [
            (
                dialog.imap.host.clone(),
                dialog.imap.port.clone(),
                &account.imap,
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
        dialog.imap.security = Security::parse(&account.imap.security);
        dialog.smtp.security = Security::parse(&account.smtp.security);
        let username = match account.imap.username.trim() {
            "" => account.address.clone(),
            name => name.to_owned(),
        };
        let field = dialog.username.clone();
        fill(&mut dialog.seen, &field, username, cx);
        dialog.accept_invalid_certs = account.imap.accept_invalid_certs;
        dialog.servers_for = account.address.clone();
        dialog.source = source;
    }

    fn add_account_step(&mut self, step: Step, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        dialog.step = step;
        dialog.error = None;
        let focus = match step {
            Step::Address => dialog.address.focus_handle(cx),
            Step::Servers => dialog.imap.host.focus_handle(cx),
            Step::Password => dialog.password.focus_handle(cx),
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

    /// "Server settings": fills in guesses when nothing was found yet.
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
            self.fill_servers(&guess(&address), None, cx);
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

    /// Next (or Enter): goes on from the step that is showing.
    fn add_account_next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = &self.add_account else {
            return;
        };
        if dialog.busy || dialog.closing {
            return;
        }
        match dialog.step {
            Step::Address => self.discover_servers(window, cx),
            Step::Servers => match self.typed_account(cx) {
                Ok(_) => self.add_account_step(Step::Password, window, cx),
                Err(err) => self.add_account_error(err, cx),
            },
            Step::Password => self.sign_in(window, cx),
        }
    }

    /// Asks the daemon for the servers of the typed address.
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
            self.add_account_step(Step::Password, window, cx);
            return;
        }
        dialog.busy = true;
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
                    Ok((account, source)) => {
                        this.fill_servers(&account, Some(source), cx);
                        // Some providers publish no SMTP server.
                        let step = if account.smtp.host.trim().is_empty() {
                            Step::Servers
                        } else {
                            Step::Password
                        };
                        this.add_account_step(step, window, cx);
                    }
                    Err(err) if err == daemon::NOT_RUNNING => this.add_account_error(err, cx),
                    Err(err) => {
                        tracing::info!(%err, "no servers found");
                        this.fill_servers(&guess(&address), None, cx);
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

    /// The account as typed, or what is wrong with it.
    fn typed_account(&self, cx: &Context<Self>) -> Result<NewImapAccount, String> {
        let address = self.add_account_address(cx)?;
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
        Ok(NewImapAccount {
            display_name: dialog.name.read(cx).text().trim().to_owned(),
            address,
            imap: spec(&dialog.imap, "incoming")?,
            smtp: spec(&dialog.smtp, "outgoing")?,
        })
    }

    /// Checks the password with the server and adds the account.
    fn sign_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let account = match self.typed_account(cx) {
            Ok(account) => account,
            Err(err) => {
                self.add_account_error(err, cx);
                return;
            }
        };
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let password = dialog.password.read(cx).text().to_owned();
        if password.is_empty() {
            self.add_account_error(tr!("add-account-password-empty"), cx);
            return;
        }
        // The name is shown and stored in the open, so a password typed or
        // pasted into it by mistake must not go through.
        if is_password(&account.display_name, &password) {
            let name = dialog.name.clone();
            name.update(cx, |n, cx| n.select_all_text(cx));
            window.focus(&name.focus_handle(cx), cx);
            self.add_account_error(tr!("add-account-name-is-password"), cx);
            return;
        }
        dialog.busy = true;
        dialog.error = None;
        let connection = self.daemon.clone();
        let address = account.address.clone();
        let task = cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await.map_err(AddError::Other)?,
                    };
                    daemon::add_account(&connection, &account, &password).await
                })
                .await;
            this.update_in(cx, |this, window, cx| match result {
                Ok(_) => {
                    this.close_add_account(cx);
                    this.show_snackbar(
                        tr!("add-account-added", address = address.as_str()),
                        None,
                        cx,
                    );
                    this.account_added(window, cx);
                }
                Err(AddError::Password(detail)) => {
                    tracing::info!(%detail, "login refused");
                    let hint = match app_password_provider(&address) {
                        Some(provider) => {
                            tr!("add-account-app-password-refused", provider = provider)
                        }
                        None => tr!("add-account-password-refused"),
                    };
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

    pub(super) fn render_add_account(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
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
        let address = dialog.address.read(cx).text().trim().to_owned();

        let (title, subtitle): (String, Option<String>) = match dialog.step {
            Step::Address if dialog.busy => (
                tr!("add-account-title"),
                Some(tr!("add-account-looking", address = address.as_str())),
            ),
            Step::Address => (
                tr!("add-account-title"),
                Some(tr!("add-account-address-intro")),
            ),
            Step::Servers => (
                tr!("add-account-servers-title"),
                Some(tr!("add-account-servers-intro", address = address.as_str())),
            ),
            Step::Password if dialog.busy => (
                tr!("add-account-password-title"),
                Some(tr!("add-account-signing-in")),
            ),
            Step::Password => (tr!("add-account-password-title"), None),
        };
        let error = dialog.error.clone();
        let mut body = div()
            .id("add-account-body")
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .px(px(40.0))
            .pt(px(36.0))
            .pb(px(28.0))
            .child(logo())
            .child(
                div()
                    .mt(px(16.0))
                    .text_size(px(24.0))
                    .line_height(px(32.0))
                    .text_color(rgba(th.text))
                    .child(title),
            )
            .children(subtitle.map(|text| {
                div()
                    .mt(px(8.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(text)
            }));

        match dialog.step {
            Step::Address => {
                body = body.child(div().mt(px(24.0)).child(self.outlined_field(
                    "field-address",
                    tr!("add-account-field-address"),
                    &dialog.address,
                    error.is_some(),
                    th,
                    window,
                    cx,
                )));
            }
            Step::Servers => {
                body = body
                    .child(section_title(
                        tr!("add-account-incoming", protocol = "IMAP"),
                        th,
                    ))
                    .child(self.server_fields(Kind::Imap, error.is_some(), th, window, cx))
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
                    )));
            }
            Step::Password => {
                body = body
                    .child(
                        div().mt(px(12.0)).flex().child(
                            div()
                                .id("account-chip")
                                .h(px(32.0))
                                .pl(px(4.0))
                                .pr(px(8.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(8.0))
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(fade(th.text_faint, 0.6)))
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.add_account_step(Step::Address, window, cx)
                                }))
                                .child(avatar(&address, &address, 24.0))
                                .child(address.clone())
                                .child(icon("chevron-down", th.text_dim, 18.0)),
                        ),
                    )
                    .child(div().mt(px(24.0)).child(self.outlined_field(
                        "field-password",
                        tr!("add-account-field-password"),
                        &dialog.password,
                        error.is_some(),
                        th,
                        window,
                        cx,
                    )))
                    .children(error.clone().map(|error| error_line(error, th)))
                    .child(
                        div()
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
                            .child(icon(
                                if dialog.show_password {
                                    "checkbox-checked"
                                } else {
                                    "checkbox"
                                },
                                if dialog.show_password {
                                    th.accent
                                } else {
                                    th.text_dim
                                },
                                20.0,
                            ))
                            .child(tr!("add-account-show-password")),
                    )
                    .children(app_password_provider(&address).map(|provider| {
                        hint(
                            tr!("add-account-app-password-hint", provider = provider),
                            th,
                        )
                    }))
                    .child(div().mt(px(20.0)).child(self.outlined_field(
                        "field-name",
                        tr!("add-account-field-name"),
                        &dialog.name,
                        false,
                        th,
                        window,
                        cx,
                    )))
                    .child(hint(tr!("add-account-name-hint"), th))
                    .child(hint(self.servers_summary(cx), th));
            }
        }

        // Under the password field on that step, else after the fields.
        if dialog.step != Step::Password {
            body = body.children(error.map(|error| error_line(error, th)));
        }

        let (secondary, secondary_label) = match dialog.step {
            Step::Address | Step::Password => {
                ("add-account-servers", tr!("add-account-servers-button"))
            }
            Step::Servers => ("add-account-back", tr!("add-account-back")),
        };
        let next_label = match dialog.step {
            Step::Password => tr!("add-account-add"),
            _ => tr!("add-account-next"),
        };
        let busy = dialog.busy;
        body = body.child(
            div()
                .mt(px(32.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(
                    text_button(secondary, secondary_label, th).on_click(cx.listener(
                        |this, _, window, cx| {
                            let step = this.add_account.as_ref().map(|d| d.step);
                            match step {
                                Some(Step::Servers) => {
                                    this.add_account_step(Step::Address, window, cx)
                                }
                                Some(_) => this.add_account_servers(window, cx),
                                None => {}
                            }
                        },
                    )),
                )
                .child(div().flex_1())
                .child(
                    text_button("add-account-cancel", tr!("add-account-cancel"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.close_add_account(cx))),
                )
                .child(
                    filled_button("add-account-next", next_label, th)
                        .when(busy, |d| d.opacity(0.6))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.add_account_next(window, cx)),
                        ),
                ),
        );

        let card = div()
            .id("add-account")
            .key_context("AddAccount")
            .occlude()
            .relative()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(200.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(super::PANEL_RADIUS))
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .child(body)
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

    /// "Servers: imap.x.org and smtp.x.org, found in …".
    fn servers_summary(&self, cx: &Context<Self>) -> String {
        let Some(dialog) = &self.add_account else {
            return String::new();
        };
        let imap = dialog.imap.host.read(cx).text().trim().to_owned();
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
            Kind::Imap => (&dialog.imap, "imap"),
            Kind::Smtp => (&dialog.smtp, "smtp"),
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
            .into_any_element()
    }

    /// Picks how a server is secured; a port left at the old default
    /// follows the new one.
    fn set_security(&mut self, kind: Kind, security: Security, cx: &mut Context<Self>) {
        let Some(dialog) = &mut self.add_account else {
            return;
        };
        let fields = match kind {
            Kind::Imap => &mut dialog.imap,
            Kind::Smtp => &mut dialog.smtp,
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
                    d.border_2()
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
        // With one account at a time, the shown one is marked and a click
        // switches to another.
        let shown = self.shown_account();
        let rows = self.accounts.iter().enumerate().map(|(ix, account)| {
            let name = if account.display_name.trim().is_empty() {
                account.address.clone()
            } else {
                account.display_name.clone()
            };
            let id = account.id;
            let current = shown == Some(id);
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
                .when(current, |d| d.bg(rgba(th.nav_selected)))
                .hover(move |s| s.bg(rgba(if current { th.nav_selected } else { th.hover })))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.account_menu = false;
                    this.app = RailApp::Mail;
                    this.settings_page = None;
                    if this.shown_account().is_some() {
                        this.switch_account(id, cx);
                    } else if let Some(inbox) = this.tree.role_folder(id, Role::Inbox) {
                        this.open_folder(inbox, cx);
                    }
                    cx.notify();
                }))
                .child(self.person_avatar(&name, &account.address, 32.0))
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
                                .child(account.address.clone()),
                        ),
                )
                .when(unread > 0, |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(if current {
                                th.nav_selected_text
                            } else {
                                th.text_dim
                            }))
                            .child(crate::format::thousands(unread)),
                    )
                })
                .when(current, |d| {
                    d.child(icon("check", th.nav_selected_text, 20.0))
                })
        });
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
            .hover(|s| s.bg(rgba(th.hover)))
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
        let card = div()
            .id("account-menu")
            .occlude()
            .absolute()
            .right(px(16.0))
            .top(px(4.0))
            .w(px(MENU_WIDTH))
            .p(px(8.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .map(|d| raised(d, th, super::PANEL_RADIUS, 2.0))
            .text_color(rgba(th.text))
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
            .child(add)
            .when(!self.accounts.is_empty(), |d| {
                d.child(
                    div()
                        .id("account-manage")
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
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.account_menu = false;
                            this.open_settings_page(
                                super::settings_page::Section::Accounts,
                                window,
                                cx,
                            );
                        }))
                        .child(
                            div()
                                .size(px(32.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(icon("settings", th.text_dim, 22.0)),
                        )
                        .child(tr!("add-account-menu-manage")),
                )
            })
            .with_animation(
                "account-menu",
                Animation::new(Duration::from_millis(180)).with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-8.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.account_menu = false;
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
pub(super) fn logo() -> AnyElement {
    crate::widgets::katna_mark(40.0)
}

fn error_line(error: String, th: &Theme) -> AnyElement {
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
        .child(div().flex_1().min_w_0().child(error))
        .into_any_element()
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
                    Animation::new(Duration::from_millis(1300)).repeat(),
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
