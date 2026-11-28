// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Subscription, the Katna account: create a Katna account, sign in and out, see
//! and sign out the computers signed in, change or reset the password and
//! delete the account. Katna Server's features (read receipts, link
//! tracking, Activity, translation) work only while signed in to an
//! account whose address is confirmed.
//!
//! The daemon talks to the server (`KatnaSignIn` and the rest on D-Bus);
//! this page only asks it. The account's password is never a mail
//! password, and mail logins never go to the server.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, Entity, FontWeight, Subscription, Window,
    div, prelude::*, rgba,
};
use katna_core::config::AiSource;
use katna_dbus::{KatnaAccount, KatnaDevice, KatnaPlan, PimProxy, katna_error};
use katna_i18n::tr;
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::MailWindow;
use super::settings_page::Section;
use crate::daemon;
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{self, filled_button, icon, outlined_button, tag, tile};

/// What the signed-out page, or a signed-in action, is asking for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    SignIn,
    Create,
    /// Forgot the password: the address to mail a code to.
    Forgot,
    /// The mailed code and a new password.
    ForgotCode,
    /// Signed in: changing the password.
    ChangePassword,
    /// Signed in: deleting the account.
    Delete,
    /// Signed in, nothing open.
    Idle,
}

/// The page's state, made when it is first shown.
pub(super) struct KatnaPage {
    /// `None` until the daemon answered.
    account: Option<KatnaAccount>,
    /// The daemon has said whether this computer is signed in, since the
    /// app started.
    answered: bool,
    devices: Vec<KatnaDevice>,
    /// Where the account stands with Katna AI; `None` until asked.
    plan: Option<KatnaPlan>,
    mode: Mode,
    email: Entity<TextInput>,
    password: Entity<TextInput>,
    code: Entity<TextInput>,
    new_password: Entity<TextInput>,
    busy: bool,
    error: Option<String>,
    /// A line saying what just happened ("Code sent").
    notice: Option<String>,
    _subscriptions: Vec<Subscription>,
}

/// What a finished request brings back.
enum Done {
    Account(KatnaAccount, Vec<KatnaDevice>),
    /// The reset code went out; ask for it.
    CodeSent,
    /// A new sign-up code went out.
    Resent,
    PasswordChanged,
}

/// The text for an error from the daemon: a [`katna_error`] name, or
/// anything else as it came.
fn error_text(error: &str) -> String {
    match error {
        katna_error::WRONG_PASSWORD => tr!("katna-error-wrong-password"),
        katna_error::EXISTS => tr!("katna-error-exists"),
        katna_error::BAD_EMAIL => tr!("katna-error-bad-email"),
        katna_error::SHORT_PASSWORD => tr!("katna-error-short-password"),
        katna_error::LONG_PASSWORD => tr!("katna-error-long-password"),
        katna_error::WRONG_CODE => tr!("katna-error-wrong-code"),
        katna_error::CODE_EXPIRED => tr!("katna-error-code-expired"),
        katna_error::TOO_MANY => tr!("katna-error-too-many"),
        katna_error::MAIL_FAILED => tr!("katna-error-mail-failed"),
        katna_error::SIGN_IN => tr!("katna-error-signed-out"),
        katna_error::OFFLINE => tr!("katna-error-offline"),
        katna_error::SERVER => tr!("katna-error-server"),
        other => other.to_owned(),
    }
}

impl MailWindow {
    /// Whether this computer is signed in to a Katna account with a
    /// confirmed address, as last read. Server features check it and show
    /// [`MailWindow::katna_sign_in_needed`] when not.
    pub(super) fn katna_signed_in(&self) -> bool {
        self.katna
            .as_ref()
            .and_then(|page| page.account.as_ref())
            .is_some_and(|account| account.signed_in && account.verified)
    }

    /// Whether the account is still being read after the app started:
    /// until then, server features show [`MailWindow::katna_checking`]
    /// rather than asking to sign in.
    pub(super) fn katna_checking(&self) -> bool {
        self.katna
            .as_ref()
            .is_none_or(|page| !page.answered && (page.busy || page.account.is_none()))
    }

    /// A thin moving bar while [`MailWindow::katna_checking`].
    pub(super) fn katna_checking_bar(&self, id: &'static str, th: &Theme) -> AnyElement {
        div()
            .w(px(160.0))
            .h(px(4.0))
            .rounded_full()
            .overflow_hidden()
            .relative()
            .bg(rgba(fade(th.accent, 0.24)))
            .child(
                div()
                    .absolute()
                    .top_0()
                    .h_full()
                    .w(px(64.0))
                    .rounded_full()
                    .bg(rgba(th.accent))
                    .with_animation(
                        id,
                        Animation::new(katna_ui::motion::time(Duration::from_millis(1300)))
                            .repeat(),
                        |bar, t| bar.left(px(-64.0 + 224.0 * t)),
                    ),
            )
            .into_any_element()
    }

    /// "Sign in to use this", with a button to Settings > Subscription,
    /// for a server feature while signed out.
    pub(super) fn katna_sign_in_needed(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("katna-sign-in-needed")),
            )
            .child(
                outlined_button("katna-sign-in-needed", tr!("katna-sign-in"), th).on_click(
                    cx.listener(|this, _, window, cx| {
                        this.open_settings_page(Section::Subscriptions, window, cx)
                    }),
                ),
            )
            .into_any_element()
    }

    fn katna_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> &mut KatnaPage {
        if self.katna.is_none() {
            let accent: gpui::Hsla = rgba(self.theme(window).accent).into();
            let input = |cx: &mut Context<Self>| {
                cx.new(|cx| {
                    let mut input = TextInput::new("", cx);
                    input.set_accent(accent);
                    input
                })
            };
            let (email, password, code, new_password) =
                (input(cx), input(cx), input(cx), input(cx));
            password.update(cx, |p, cx| p.set_masked(true, cx));
            new_password.update(cx, |p, cx| p.set_masked(true, cx));
            let subscriptions = [&email, &password, &code, &new_password]
                .into_iter()
                .map(|field| {
                    cx.subscribe(field, |this, _, event: &InputEvent, cx| match event {
                        InputEvent::Submit => this.katna_submit(cx),
                        InputEvent::Changed => {
                            if let Some(page) = &mut this.katna
                                && page.error.is_some()
                            {
                                page.error = None;
                                cx.notify();
                            }
                        }
                        InputEvent::Cancel => {}
                    })
                })
                .collect();
            self.katna = Some(KatnaPage {
                account: None,
                answered: false,
                devices: Vec::new(),
                plan: None,
                mode: Mode::SignIn,
                email,
                password,
                code,
                new_password,
                busy: false,
                error: None,
                notice: None,
                _subscriptions: subscriptions,
            });
            self.katna_refresh(cx);
        }
        self.katna.as_mut().expect("just made")
    }

    /// Reads the account from the daemon (again), for a server feature
    /// about to be shown.
    pub(super) fn katna_load(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.katna.is_some() {
            self.katna_refresh(cx);
        } else {
            self.katna_page(window, cx);
        }
    }

    /// Reads the account (and its devices) from the daemon again.
    pub(super) fn katna_refresh(&mut self, cx: &mut Context<Self>) {
        self.katna_run(cx, |pim| async move {
            let account = pim
                .katna_account()
                .await
                .map_err(|e| daemon::describe(&e))?;
            let devices = if account.signed_in && account.verified {
                pim.katna_devices().await.unwrap_or_default()
            } else {
                Vec::new()
            };
            Ok(Done::Account(account, devices))
        });
    }

    /// Runs `request` against the daemon, then shows what it brought.
    fn katna_run<F, Fut>(&mut self, cx: &mut Context<Self>, request: F)
    where
        F: FnOnce(PimProxy<'static>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<Done, String>> + Send,
    {
        if let Some(page) = &mut self.katna {
            page.busy = true;
            page.error = None;
            page.notice = None;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    let pim = PimProxy::new(&connection)
                        .await
                        .map_err(|e| daemon::describe(&e))?;
                    request(pim).await
                })
                .await;
            this.update(cx, |this, cx| {
                this.katna_done(result, cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn katna_done(&mut self, result: Result<Done, String>, cx: &mut Context<Self>) {
        let Some(page) = &mut self.katna else {
            return;
        };
        page.busy = false;
        let done = match result {
            Ok(done) => done,
            Err(error) => {
                page.error = Some(error_text(&error));
                if page.account.is_none() {
                    page.account = Some(KatnaAccount::default());
                }
                return;
            }
        };
        let clear = |field: &Entity<TextInput>, cx: &mut Context<Self>| {
            field.update(cx, |f, cx| f.set_text("", cx));
        };
        match done {
            Done::Account(account, devices) => {
                let was_signed_in = page.account.as_ref().is_some_and(|a| a.signed_in);
                if account.signed_in {
                    page.mode = Mode::Idle;
                } else if was_signed_in || !matches!(page.mode, Mode::SignIn | Mode::Create) {
                    page.mode = Mode::SignIn;
                }
                let ask_plan = account.signed_in && account.verified;
                if !ask_plan {
                    page.plan = None;
                }
                page.account = Some(account);
                page.answered = true;
                page.devices = devices;
                for field in [&page.password, &page.code, &page.new_password] {
                    clear(field, cx);
                }
                if ask_plan {
                    self.katna_load_plan(cx);
                }
            }
            Done::CodeSent => {
                page.mode = Mode::ForgotCode;
                page.notice = Some(tr!("katna-reset-code-sent"));
            }
            Done::Resent => page.notice = Some(tr!("katna-code-resent")),
            Done::PasswordChanged => {
                page.mode = Mode::Idle;
                page.notice = Some(tr!("katna-password-changed"));
                for field in [&page.password, &page.new_password] {
                    clear(field, cx);
                }
            }
        }
    }

    fn katna_mode(&mut self, mode: Mode, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.katna {
            page.mode = mode;
            page.error = None;
            page.notice = None;
            cx.notify();
        }
    }

    /// The main button of what is shown, or Enter in a field.
    fn katna_submit(&mut self, cx: &mut Context<Self>) {
        let Some(page) = &self.katna else {
            return;
        };
        if page.busy {
            return;
        }
        let text = |field: &Entity<TextInput>| field.read(cx).text().to_owned();
        let email = text(&page.email).trim().to_owned();
        let password = text(&page.password);
        let code = text(&page.code);
        let new_password = text(&page.new_password);
        let account = page.account.clone().unwrap_or_default();
        let err = |e: katna_dbus::zbus::Error| daemon::describe(&e);
        match page.mode {
            _ if account.signed_in && !account.verified => {
                self.katna_run(cx, move |pim| async move {
                    let account = pim.katna_verify(&code).await.map_err(err)?;
                    let devices = pim.katna_devices().await.unwrap_or_default();
                    Ok(Done::Account(account, devices))
                });
            }
            Mode::SignIn | Mode::Create => {
                let create = page.mode == Mode::Create;
                self.katna_run(cx, move |pim| async move {
                    let account = if create {
                        pim.katna_sign_up(&email, &password).await
                    } else {
                        pim.katna_sign_in(&email, &password).await
                    }
                    .map_err(err)?;
                    let devices = if account.verified {
                        pim.katna_devices().await.unwrap_or_default()
                    } else {
                        Vec::new()
                    };
                    Ok(Done::Account(account, devices))
                });
            }
            Mode::Forgot => self.katna_run(cx, move |pim| async move {
                pim.katna_reset_password(&email).await.map_err(err)?;
                Ok(Done::CodeSent)
            }),
            Mode::ForgotCode => self.katna_run(cx, move |pim| async move {
                let account = pim
                    .katna_confirm_reset(&email, &code, &new_password)
                    .await
                    .map_err(err)?;
                let devices = pim.katna_devices().await.unwrap_or_default();
                Ok(Done::Account(account, devices))
            }),
            Mode::ChangePassword => self.katna_run(cx, move |pim| async move {
                pim.katna_change_password(&password, &new_password)
                    .await
                    .map_err(err)?;
                Ok(Done::PasswordChanged)
            }),
            Mode::Delete => self.katna_run(cx, move |pim| async move {
                pim.katna_delete_account(&password).await.map_err(err)?;
                Ok(Done::Account(KatnaAccount::default(), Vec::new()))
            }),
            Mode::Idle => {}
        }
    }

    fn katna_sign_out(&mut self, cx: &mut Context<Self>) {
        self.katna_run(cx, |pim| async move {
            pim.katna_sign_out()
                .await
                .map_err(|e| daemon::describe(&e))?;
            Ok(Done::Account(KatnaAccount::default(), Vec::new()))
        });
    }

    fn katna_sign_out_device(&mut self, id: String, cx: &mut Context<Self>) {
        self.katna_run(cx, move |pim| async move {
            let err = |e: katna_dbus::zbus::Error| daemon::describe(&e);
            pim.katna_sign_out_device(&id).await.map_err(err)?;
            let account = pim.katna_account().await.map_err(err)?;
            let devices = pim.katna_devices().await.map_err(err)?;
            Ok(Done::Account(account, devices))
        });
    }

    fn katna_resend(&mut self, cx: &mut Context<Self>) {
        self.katna_run(cx, |pim| async move {
            pim.katna_resend_code()
                .await
                .map_err(|e| daemon::describe(&e))?;
            Ok(Done::Resent)
        });
    }

    /// Settings > Subscription: the Katna account.
    /// The Katna account step of the first start, once "Create account"
    /// or "I have an account" was picked: the form, the code to confirm
    /// the address, or who is signed in. Returns the page and its main
    /// button.
    pub(super) fn katna_onboarding_form(
        &mut self,
        create: bool,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (AnyElement, AnyElement) {
        let page = self.katna_page(window, cx);
        if matches!(page.mode, Mode::SignIn | Mode::Create) {
            page.mode = if create { Mode::Create } else { Mode::SignIn };
        }
        let (mode, busy) = (page.mode, page.busy);
        let account = page.account.clone().unwrap_or_default();
        let status = page
            .error
            .clone()
            .map(|e| (e, th.error))
            .or(page.notice.clone().map(|n| (n, th.text_dim)));
        let (email, password, code) =
            (page.email.clone(), page.password.clone(), page.code.clone());
        let submit = |id: &'static str, label: String, cx: &mut Context<Self>| {
            filled_button(id, label, th)
                .when(busy, |b| b.opacity(0.6))
                .on_click(cx.listener(|this, _, _, cx| this.katna_submit(cx)))
                .into_any_element()
        };
        let form = div().w_full().flex().flex_col().gap(px(16.0));
        let (title, lead, form, main) = if account.signed_in && account.verified {
            (
                tr!("katna-signed-in"),
                account.email.clone(),
                form,
                filled_button("katna-onboarding-done", tr!("onboarding-continue"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.onboarding_katna_done(cx)))
                    .into_any_element(),
            )
        } else if account.signed_in {
            (
                tr!("katna-confirm-title"),
                tr!("katna-confirm-detail", email = account.email.clone()),
                form.child(self.outlined_field(
                    "katna-code",
                    tr!("katna-code"),
                    &code,
                    false,
                    th,
                    window,
                    cx,
                ))
                .child(
                    div().flex().flex_row().justify_center().child(
                        link_button("katna-resend", tr!("katna-resend"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.katna_resend(cx))),
                    ),
                ),
                submit("katna-confirm", tr!("katna-confirm"), cx),
            )
        } else {
            let create = mode == Mode::Create;
            (
                if create {
                    tr!("katna-onboarding-create-title")
                } else {
                    tr!("katna-onboarding-sign-in-title")
                },
                if create {
                    tr!("katna-create-detail")
                } else {
                    tr!("katna-sign-in-detail")
                },
                form.child(self.outlined_field(
                    "katna-email",
                    tr!("katna-email"),
                    &email,
                    false,
                    th,
                    window,
                    cx,
                ))
                .child(self.outlined_field(
                    "katna-password",
                    tr!("katna-password"),
                    &password,
                    false,
                    th,
                    window,
                    cx,
                ))
                .child(div().flex().flex_row().justify_center().child(if create {
                    link_button("katna-to-sign-in", tr!("katna-have-account"), th).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.katna_mode(Mode::SignIn, cx);
                            this.onboarding_katna_form(Some(false), cx)
                        }),
                    )
                } else {
                    link_button("katna-to-create", tr!("katna-create"), th).on_click(cx.listener(
                        |this, _, _, cx| {
                            this.katna_mode(Mode::Create, cx);
                            this.onboarding_katna_form(Some(true), cx)
                        },
                    ))
                })),
                if create {
                    submit("katna-create", tr!("katna-create"), cx)
                } else {
                    submit("katna-sign-in", tr!("katna-sign-in"), cx)
                },
            )
        };
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(crate::widgets::katna_mark(48.0, th))
            .child(super::onboarding::title(title, th))
            .child(super::onboarding::lead(&lead, th))
            .child(
                div()
                    .pt(px(12.0))
                    .w_full()
                    .max_w(px(380.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(form)
                    .when_some(status, |d, (text, color)| {
                        d.child(
                            div()
                                .text_size(px(13.0))
                                .text_center()
                                .text_color(rgba(color))
                                .child(text),
                        )
                    }),
            )
            .into_any_element();
        (body, main)
    }

    pub(super) fn katna_section(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = self.katna_page(window, cx);
        let (mode, busy) = (page.mode, page.busy);
        let account = page.account.clone();
        let error = page.error.clone();
        let notice = page.notice.clone();
        let devices = page.devices.clone();
        let fields = (
            page.email.clone(),
            page.password.clone(),
            page.code.clone(),
            page.new_password.clone(),
        );
        let (email, password, code, new_password) = fields;

        // The Subscription tab holds the Katna account, under its name.
        let intro = div()
            .pt(px(20.0))
            .pb(px(4.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("settings-tab-katna-account")),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(px(19.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("katna-intro")),
            );
        let status = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .when_some(error, |d, error| {
                d.child(
                    self.copyable(error, th)
                        .text_size(px(13.0))
                        .text_color(rgba(th.error)),
                )
            })
            .when_some(notice, |d, notice| {
                d.child(
                    self.copyable(notice, th)
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim)),
                )
            });
        let submit = |id: &'static str, label: String, cx: &mut Context<Self>| {
            filled_button(id, label, th)
                .when(busy, |b| b.opacity(0.6))
                .on_click(cx.listener(|this, _, _, cx| this.katna_submit(cx)))
        };
        let link = |id: gpui::ElementId, label: String| {
            div()
                .id(id)
                .px(px(10.0))
                .py(px(6.0))
                .rounded(px(16.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.accent))
                .cursor_pointer()
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", Some(16.0), th))
                .child(label)
        };
        let buttons = || {
            div()
                .pt(px(4.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
        };
        let form = || div().max_w(px(420.0)).flex().flex_col().gap(px(16.0));

        let Some(account) = account else {
            return div()
                .child(intro)
                .child(
                    div()
                        .pt(px(16.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("katna-checking")),
                )
                .into_any_element();
        };

        let body = if !account.signed_in {
            let (title, detail, form) = match mode {
                Mode::Forgot => (
                    tr!("katna-forgot"),
                    tr!("katna-forgot-detail"),
                    form()
                        .child(self.outlined_field(
                            "katna-email",
                            tr!("katna-email"),
                            &email,
                            false,
                            th,
                            window,
                            cx,
                        ))
                        .child(
                            buttons()
                                .child(submit("katna-send-code", tr!("katna-send-code"), cx))
                                .child(
                                    link("katna-back".into(), tr!("katna-back-to-sign-in"))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.katna_mode(Mode::SignIn, cx)
                                        })),
                                ),
                        ),
                ),
                Mode::ForgotCode => (
                    tr!("katna-forgot"),
                    tr!("katna-forgot-code-detail"),
                    form()
                        .child(self.outlined_field(
                            "katna-code",
                            tr!("katna-code"),
                            &code,
                            false,
                            th,
                            window,
                            cx,
                        ))
                        .child(self.outlined_field(
                            "katna-new-password",
                            tr!("katna-new-password"),
                            &new_password,
                            false,
                            th,
                            window,
                            cx,
                        ))
                        .child(
                            buttons()
                                .child(submit("katna-set-password", tr!("katna-set-password"), cx))
                                .child(
                                    link("katna-back".into(), tr!("katna-back-to-sign-in"))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.katna_mode(Mode::SignIn, cx)
                                        })),
                                ),
                        ),
                ),
                _ => {
                    let create = mode == Mode::Create;
                    (
                        if create {
                            tr!("katna-create")
                        } else {
                            tr!("katna-sign-in")
                        },
                        if create {
                            tr!("katna-create-detail")
                        } else {
                            tr!("katna-sign-in-detail")
                        },
                        form()
                            .child(self.outlined_field(
                                "katna-email",
                                tr!("katna-email"),
                                &email,
                                false,
                                th,
                                window,
                                cx,
                            ))
                            .child(self.outlined_field(
                                "katna-password",
                                tr!("katna-password"),
                                &password,
                                false,
                                th,
                                window,
                                cx,
                            ))
                            .child(
                                buttons()
                                    .child(if create {
                                        submit("katna-create", tr!("katna-create"), cx)
                                    } else {
                                        submit("katna-sign-in", tr!("katna-sign-in"), cx)
                                    })
                                    .child(if create {
                                        link("katna-to-sign-in".into(), tr!("katna-have-account"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.katna_mode(Mode::SignIn, cx)
                                            }))
                                    } else {
                                        link("katna-to-create".into(), tr!("katna-create"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.katna_mode(Mode::Create, cx)
                                            }))
                                    })
                                    .when(!create, |d| {
                                        d.child(
                                            link("katna-to-forgot".into(), tr!("katna-forgot"))
                                                .on_click(cx.listener(|this, _, _, cx| {
                                                    this.katna_mode(Mode::Forgot, cx)
                                                })),
                                        )
                                    }),
                            ),
                    )
                }
            };
            div()
                .child(self.row(title, Some(&detail), form.child(status), th))
                .into_any_element()
        } else if !account.verified {
            // Where the code went matters here, so it is not behind (i).
            let form = form()
                .child(
                    div()
                        .text_size(px(14.0))
                        .line_height(px(20.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("katna-confirm-detail", email = account.email.clone())),
                )
                .child(self.outlined_field(
                    "katna-code",
                    tr!("katna-code"),
                    &code,
                    false,
                    th,
                    window,
                    cx,
                ))
                .child(
                    buttons()
                        .child(submit("katna-confirm", tr!("katna-confirm"), cx))
                        .child(
                            link("katna-resend".into(), tr!("katna-resend"))
                                .on_click(cx.listener(|this, _, _, cx| this.katna_resend(cx))),
                        )
                        .child(
                            link("katna-sign-out".into(), tr!("katna-sign-out"))
                                .on_click(cx.listener(|this, _, _, cx| this.katna_sign_out(cx))),
                        ),
                )
                .child(status);
            div()
                .child(self.row(tr!("katna-confirm-title"), None, form, th))
                .into_any_element()
        } else {
            let password_form = (mode == Mode::ChangePassword).then(|| {
                form()
                    .child(self.outlined_field(
                        "katna-current-password",
                        tr!("katna-current-password"),
                        &password,
                        false,
                        th,
                        window,
                        cx,
                    ))
                    .child(self.outlined_field(
                        "katna-new-password",
                        tr!("katna-new-password"),
                        &new_password,
                        false,
                        th,
                        window,
                        cx,
                    ))
                    .child(
                        buttons()
                            .child(submit(
                                "katna-save-password",
                                tr!("katna-save-password"),
                                cx,
                            ))
                            .child(link("katna-cancel".into(), tr!("katna-cancel")).on_click(
                                cx.listener(|this, _, _, cx| this.katna_mode(Mode::Idle, cx)),
                            )),
                    )
                    .into_any_element()
            });
            let delete_form = (mode == Mode::Delete).then(|| {
                form()
                    .child(self.outlined_field(
                        "katna-delete-password",
                        tr!("katna-password"),
                        &password,
                        false,
                        th,
                        window,
                        cx,
                    ))
                    .child(
                        buttons()
                            .child(
                                filled_button(
                                    "katna-delete-confirm",
                                    tr!("katna-delete-confirm"),
                                    th,
                                )
                                .bg(rgba(th.error))
                                .when(busy, |b| b.opacity(0.6))
                                .on_click(cx.listener(|this, _, _, cx| this.katna_submit(cx))),
                            )
                            .child(link("katna-cancel".into(), tr!("katna-cancel")).on_click(
                                cx.listener(|this, _, _, cx| this.katna_mode(Mode::Idle, cx)),
                            )),
                    )
                    .into_any_element()
            });
            let plan = self.katna.as_ref().and_then(|page| page.plan.clone());
            div()
                .pt(px(space::S5))
                .flex()
                .flex_col()
                .gap(px(space::S5))
                .child(self.katna_account_card(&account.email, th, cx))
                .child(self.katna_plan_card(plan.as_ref(), th, cx))
                .child(self.katna_features_card(th, cx))
                .child(self.katna_devices_card(&devices, th, cx))
                .child(self.katna_security_card(password_form, delete_form, th, cx))
                .child(status)
                .into_any_element()
        };
        div()
            .flex()
            .flex_col()
            .child(intro)
            .child(body)
            .into_any_element()
    }
}

/// What Settings > Subscription says about Katna AI, from the server's
/// plan and the AI settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AiStanding {
    /// Writing help uses the user's own service.
    OwnKey,
    /// Writing help is off.
    Off,
    /// Never used, or the server could not say: the free month starts
    /// with the first use.
    New,
    Trial,
    Paid,
    /// The free month is over and nothing is paid.
    Over,
}

impl AiStanding {
    fn of(source: AiSource, plan: Option<&KatnaPlan>) -> Self {
        match source {
            AiSource::Own => return Self::OwnKey,
            AiSource::Off => return Self::Off,
            AiSource::Katna => {}
        }
        match plan.map(|plan| plan.kind.as_str()) {
            Some(katna_ai::wire::plan::TRIAL) => Self::Trial,
            Some(katna_ai::wire::plan::PAID) => Self::Paid,
            Some(katna_ai::wire::plan::OVER) => Self::Over,
            _ => Self::New,
        }
    }
}

impl MailWindow {
    /// Asks the daemon where the account stands with Katna AI, quietly:
    /// the rest of the page shows meanwhile.
    fn katna_load_plan(&mut self, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let plan = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await.ok()?,
                    };
                    let pim = PimProxy::new(&connection).await.ok()?;
                    pim.katna_ai_plan().await.ok()
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(page) = &mut this.katna {
                    page.plan = Some(plan.unwrap_or_default());
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Signs out every computer but this one.
    fn katna_sign_out_others(&mut self, cx: &mut Context<Self>) {
        let others: Vec<String> = self
            .katna
            .as_ref()
            .map(|page| {
                page.devices
                    .iter()
                    .filter(|device| !device.this)
                    .map(|device| device.id.clone())
                    .collect()
            })
            .unwrap_or_default();
        self.katna_run(cx, move |pim| async move {
            let err = |e: katna_dbus::zbus::Error| daemon::describe(&e);
            for id in &others {
                pim.katna_sign_out_device(id).await.map_err(err)?;
            }
            let account = pim.katna_account().await.map_err(err)?;
            let devices = pim.katna_devices().await.map_err(err)?;
            Ok(Done::Account(account, devices))
        });
    }

    /// The account at the top: its picture, address and Sign out.
    fn katna_account_card(&self, email: &str, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        card_box(th)
            .p(px(space::S5))
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(space::S5))
            .child(widgets::avatar(email, email, 48.0))
            .child(
                div()
                    .flex_1()
                    .min_w(px(180.0))
                    .flex()
                    .flex_col()
                    .gap(px(space::S1))
                    .child(
                        self.copyable(email.to_owned(), th)
                            .text_size(px(text::SUBTITLE))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text)),
                    )
                    .child(
                        div()
                            .text_size(px(text::SMALL))
                            .text_color(rgba(th.text_faint))
                            .child(tr!("katna-signed-in-detail")),
                    ),
            )
            .child(
                widgets::button("katna-sign-out", widgets::ButtonStyle::Outlined, th)
                    .gap(px(space::S3))
                    .child(icon("logout", th.accent, 18.0))
                    .child(tr!("katna-sign-out"))
                    .on_click(cx.listener(|this, _, _, cx| this.katna_sign_out(cx))),
            )
            .into_any_element()
    }

    /// The plan: the free Katna account beside Katna AI, with where the
    /// account stands with it.
    fn katna_plan_card(
        &self,
        plan: Option<&KatnaPlan>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let standing = AiStanding::of(self.config.ai.source, plan);
        let trial_days = plan.map_or(30, |plan| plan.trial_days).max(1);
        let until = plan
            .filter(|plan| plan.until > 0)
            .and_then(|plan| format::local(plan.until, &self.tz))
            .map(katna_i18n::format::day_month_year)
            .unwrap_or_default();
        let checks = |items: [String; 3]| {
            div()
                .flex()
                .flex_col()
                .gap(px(space::S2))
                .children(items.into_iter().map(|item| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S3))
                        .text_size(px(text::SMALL))
                        .text_color(rgba(th.text_dim))
                        .child(icon("check", th.text_faint, 16.0))
                        .child(item)
                }))
        };
        let price = |big: String, small: String| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_baseline()
                .gap(px(space::S3))
                .child(
                    div()
                        .text_size(px(text::DISPLAY))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(th.text))
                        .child(big),
                )
                .child(
                    div()
                        .text_size(px(text::SMALL))
                        .text_color(rgba(th.text_faint))
                        .child(small),
                )
        };
        let heading = |name: &str, title: String, chip: Div| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S3))
                .when(!name.is_empty(), |d| d.child(icon(name, th.text_dim, 20.0)))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_size(px(text::SUBTITLE))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title),
                )
                .child(chip)
        };
        let plan_tile = |chosen: bool| {
            div()
                .flex_1()
                .min_w(px(240.0))
                .p(px(space::S5))
                .flex()
                .flex_col()
                .gap(px(space::S4))
                .rounded(px(radius::SM))
                .border_1()
                .border_color(rgba(if chosen { th.accent } else { inner_edge(th) }))
        };
        let account_tile = plan_tile(false)
            .child(heading(
                "",
                tr!("katna-plan-account"),
                tag(tr!("katna-plan-free"), th),
            ))
            .child(price(
                tr!("katna-plan-account-price"),
                tr!("katna-plan-account-always"),
            ))
            .child(checks([
                tr!("katna-plan-account-tracking"),
                tr!("katna-plan-account-activity"),
                tr!("katna-plan-account-translation"),
            ]));

        let accent_tag = |label: String| {
            tag(label, th)
                .bg(rgba(fade(th.accent, 0.12)))
                .text_color(rgba(th.accent))
        };
        let chip = match standing {
            AiStanding::Trial => accent_tag(tr!("katna-plan-ai-trial")),
            AiStanding::Paid => accent_tag(tr!("katna-plan-ai-active")),
            AiStanding::Over => tag(tr!("katna-plan-ai-over"), th)
                .bg(rgba(fade(th.warning, 0.14)))
                .text_color(rgba(th.warning)),
            AiStanding::OwnKey => tag(tr!("katna-plan-ai-own"), th),
            AiStanding::Off => tag(tr!("katna-plan-ai-off"), th),
            AiStanding::New => tag(tr!("katna-plan-ai-new", days = trial_days), th),
        };
        let price_line = match standing {
            AiStanding::Trial | AiStanding::New => {
                price(tr!("katna-plan-ai-price"), tr!("katna-plan-ai-after-trial"))
            }
            _ => price(tr!("katna-plan-ai-price"), tr!("katna-plan-ai-per-month")),
        };
        let line = |words: String| {
            div()
                .text_size(px(text::SMALL))
                .text_color(rgba(th.text_dim))
                .child(words)
        };
        let progress = |left: u32, color: u32| {
            let done = 1.0 - left.min(trial_days) as f32 / trial_days as f32;
            div()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(rgba(fade(th.text, 0.12)))
                .child(
                    div()
                        .h_full()
                        .rounded_full()
                        .bg(rgba(color))
                        .w(gpui::relative((1.0 - done).max(0.02))),
                )
        };
        let status = match standing {
            AiStanding::Trial => {
                let left = plan.map_or(0, |plan| plan.days_left);
                Some(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(space::S2))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .gap(px(space::S3))
                                .text_size(px(text::SMALL))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("katna-plan-ai-days-left", days = left))
                                .when(!until.is_empty(), |d| {
                                    d.child(tr!("katna-plan-ai-ends", date = until.clone()))
                                }),
                        )
                        .child(progress(left, th.accent))
                        .into_any_element(),
                )
            }
            AiStanding::Paid => Some(
                line(if until.is_empty() {
                    tr!("katna-plan-ai-paid")
                } else {
                    tr!("katna-plan-ai-paid-until", date = until.clone())
                })
                .into_any_element(),
            ),
            AiStanding::Over => Some(progress(0, th.warning).into_any_element()),
            AiStanding::New => {
                Some(line(tr!("katna-plan-ai-new-detail", days = trial_days)).into_any_element())
            }
            AiStanding::OwnKey => Some(line(tr!("katna-plan-ai-own-detail")).into_any_element()),
            AiStanding::Off => Some(line(tr!("katna-plan-ai-off-detail")).into_any_element()),
        };
        let to_ai = |id: &'static str, label: String, filled: bool| {
            widgets::button(
                id,
                if filled {
                    widgets::ButtonStyle::Outlined
                } else {
                    widgets::ButtonStyle::Text
                },
                th,
            )
            .child(label)
            .on_click(
                cx.listener(|this, _, window, cx| this.open_settings_page(Section::Ai, window, cx)),
            )
        };
        let action = match standing {
            AiStanding::Over => to_ai("katna-plan-own-key", tr!("katna-plan-use-own-key"), true),
            AiStanding::Off => to_ai("katna-plan-ai-settings", tr!("katna-plan-turn-on"), true),
            // A text button's words line up with the tile's, past its
            // padding.
            _ => to_ai(
                "katna-plan-ai-settings",
                tr!("katna-plan-ai-settings"),
                false,
            )
            .ml(px(-space::S4)),
        };
        let chosen = matches!(standing, AiStanding::Trial | AiStanding::Paid);
        let ai_tile = plan_tile(chosen)
            .child(heading("sparkle", tr!("katna-plan-ai"), chip))
            .child(price_line)
            .when_some(status, |d, status| d.child(status))
            .child(checks([
                tr!("katna-plan-ai-writing"),
                tr!("katna-plan-ai-summaries"),
                tr!("katna-plan-ai-no-key"),
            ]))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(space::S3))
                    .child(action),
            )
            .when(
                matches!(standing, AiStanding::Trial | AiStanding::New),
                |d| {
                    d.child(
                        div()
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(th.text_faint))
                            .child(tr!("katna-plan-nothing-charged")),
                    )
                },
            );
        let over_note = (standing == AiStanding::Over).then(|| {
            div()
                .mx(px(space::S5))
                .p(px(space::S4))
                .flex()
                .flex_row()
                .gap(px(space::S4))
                .rounded(px(radius::SM))
                .bg(rgba(fade(th.warning, 0.12)))
                .child(icon("warning", th.warning, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(space::S1))
                        .child(
                            div()
                                .text_size(px(text::BODY))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgba(th.text))
                                .child(tr!("katna-plan-over-title")),
                        )
                        .child(
                            div()
                                .text_size(px(text::SMALL))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("katna-plan-over-detail")),
                        ),
                )
        });
        card_box(th)
            .child(card_heading(
                tr!("katna-plan"),
                Some(tr!("katna-plan-detail")),
                th,
            ))
            .when_some(over_note, |d, note| d.child(note))
            .child(
                div()
                    .p(px(space::S5))
                    .pt(px(space::S4))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(space::S4))
                    .child(account_tile)
                    .child(ai_tile),
            )
            .into_any_element()
    }

    /// What the account turns on, each opening its own settings where it
    /// has some.
    fn katna_features_card(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let translate_on = self.config.mail.translation.offer;
        let ai_state = match self.config.ai.source {
            AiSource::Katna => tr!("katna-feature-on"),
            AiSource::Own => tr!("katna-feature-own-key"),
            AiSource::Off => tr!("katna-feature-off"),
        };
        let item = |name: &str, title: String, detail: String| {
            (
                icon(name, th.text_dim, 20.0),
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(text::BODY))
                            .text_color(rgba(th.text))
                            .child(title),
                    )
                    .child(
                        div()
                            .text_size(px(text::SMALL))
                            .text_color(rgba(th.text_faint))
                            .child(detail),
                    ),
            )
        };
        let still = |name: &str, title: String, detail: String| {
            let (icon, words) = item(name, title, detail);
            div()
                .px(px(space::S3))
                .py(px(space::S3))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S4))
                .child(icon)
                .child(words)
        };
        let opens = |id: &'static str,
                     name: &str,
                     title: String,
                     detail: String,
                     state: String,
                     on: bool,
                     section: Section| {
            let (icon, words) = item(name, title, detail);
            widgets::row(id, false, th)
                .child(icon)
                .child(words)
                .child(
                    div()
                        .flex_none()
                        .text_size(px(text::SMALL))
                        .text_color(rgba(if on { th.text_dim } else { th.text_faint }))
                        .child(state),
                )
                .child(widgets::icon("chevron-right", th.text_faint, 20.0))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.open_settings_page(section, window, cx)
                }))
        };
        card_box(th)
            .child(card_heading(
                tr!("katna-features"),
                Some(tr!("katna-features-detail")),
                th,
            ))
            .child(
                div()
                    .px(px(space::S3))
                    .pb(px(space::S3))
                    .flex()
                    .flex_col()
                    .child(still(
                        "read-receipt",
                        tr!("katna-feature-tracking"),
                        tr!("katna-feature-tracking-detail"),
                    ))
                    .child(still(
                        "activity",
                        tr!("katna-feature-activity"),
                        tr!("katna-feature-activity-detail"),
                    ))
                    .child(opens(
                        "katna-feature-translation",
                        "translate",
                        tr!("katna-feature-translation"),
                        tr!("katna-feature-translation-detail"),
                        if translate_on {
                            tr!("katna-feature-on")
                        } else {
                            tr!("katna-feature-off")
                        },
                        translate_on,
                        Section::Reading,
                    ))
                    .child(opens(
                        "katna-feature-ai",
                        "sparkle",
                        tr!("katna-plan-ai"),
                        tr!("katna-feature-ai-detail"),
                        ai_state,
                        self.config.ai.source != AiSource::Off,
                        Section::Ai,
                    )),
            )
            .into_any_element()
    }

    /// The computers signed in, each with Sign out but this one.
    fn katna_devices_card(
        &self,
        devices: &[KatnaDevice],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let others = devices.iter().filter(|device| !device.this).count();
        // This computer first, then the others as the server lists them.
        let mut sorted: Vec<&KatnaDevice> = devices.iter().collect();
        sorted.sort_by_key(|device| !device.this);
        let mut list = div().px(px(space::S3)).pb(px(space::S3)).flex().flex_col();
        for (ix, device) in sorted.into_iter().enumerate() {
            let date = |unix: i64| {
                format::local(unix, &self.tz)
                    .map(katna_i18n::format::day_month_year)
                    .unwrap_or_default()
            };
            let detail = if device.this {
                tr!("katna-device-since", date = date(device.signed_in_at))
            } else {
                tr!(
                    "katna-device-last-seen",
                    date = date(device.last_seen.max(device.signed_in_at))
                )
            };
            let id = device.id.clone();
            list = list.child(
                div()
                    .px(px(space::S3))
                    .py(px(space::S3))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S4))
                    .child(icon("computer", th.text_dim, 20.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .items_center()
                                    .gap(px(space::S3))
                                    .child(
                                        div()
                                            .text_size(px(text::BODY))
                                            .text_color(rgba(th.text))
                                            .child(if device.name.is_empty() {
                                                tr!("katna-device-unnamed")
                                            } else {
                                                device.name.clone()
                                            }),
                                    )
                                    .when(device.this, |d| {
                                        d.child(tag(tr!("katna-device-this-tag"), th))
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(text::SMALL))
                                    .text_color(rgba(th.text_faint))
                                    .child(detail),
                            ),
                    )
                    .when(!device.this, |d| {
                        d.child(
                            outlined_button(
                                ("katna-device-sign-out", ix),
                                tr!("katna-sign-out"),
                                th,
                            )
                            .on_click(cx.listener(
                                move |this, _, _, cx| this.katna_sign_out_device(id.clone(), cx),
                            )),
                        )
                    }),
            );
        }
        let sign_out_others = (others > 1).then(|| {
            widgets::button("katna-sign-out-others", widgets::ButtonStyle::Text, th)
                .child(tr!("katna-sign-out-others"))
                .on_click(cx.listener(|this, _, _, cx| this.katna_sign_out_others(cx)))
        });
        card_box(th)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .child(div().flex_1().min_w_0().child(card_heading(
                        tr!("katna-devices"),
                        Some(tr!("katna-devices-detail")),
                        th,
                    )))
                    .when_some(sign_out_others, |d, button| {
                        d.child(div().pt(px(space::S4)).pr(px(space::S4)).child(button))
                    }),
            )
            .child(list)
            .into_any_element()
    }

    /// The password and deleting the account, each opening its form in
    /// place.
    fn katna_security_card(
        &self,
        password_form: Option<AnyElement>,
        delete_form: Option<AnyElement>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let line = |name: &str,
                    color: u32,
                    title: String,
                    detail: String,
                    button: Option<gpui::Stateful<Div>>,
                    form: Option<AnyElement>| {
            div()
                .px(px(space::S3))
                .py(px(space::S3))
                .flex()
                .flex_col()
                .gap(px(space::S4))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S4))
                        .child(icon(name, color, 20.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(text::BODY))
                                        .text_color(rgba(th.text))
                                        .child(title),
                                )
                                .child(
                                    div()
                                        .text_size(px(text::SMALL))
                                        .text_color(rgba(th.text_faint))
                                        .child(detail),
                                ),
                        )
                        .when_some(button, |d, button| d.child(button)),
                )
                .when_some(form, |d, form| {
                    // Lined up with the words, past the icon.
                    d.child(div().pl(px(20.0 + space::S4)).child(form))
                })
        };
        let open_password = password_form.is_some();
        let open_delete = delete_form.is_some();
        card_box(th)
            .child(card_heading(tr!("katna-security"), None, th))
            .child(
                div()
                    .px(px(space::S3))
                    .pb(px(space::S3))
                    .flex()
                    .flex_col()
                    .child(line(
                        "key",
                        th.text_dim,
                        tr!("katna-password"),
                        tr!("katna-password-detail"),
                        (!open_password).then(|| {
                            outlined_button("katna-change-password", tr!("katna-change"), th)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.katna_mode(Mode::ChangePassword, cx)
                                }))
                        }),
                        password_form,
                    ))
                    .child(line(
                        "trash",
                        th.error,
                        tr!("katna-delete"),
                        tr!("katna-delete-detail"),
                        (!open_delete).then(|| {
                            outlined_button("katna-delete", tr!("katna-delete-short"), th)
                                .text_color(rgba(th.error))
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.katna_mode(Mode::Delete, cx)),
                                )
                        }),
                        delete_form,
                    )),
            )
            .into_any_element()
    }
}

/// A card of Settings > Subscription: a [`tile`] filling the width.
fn card_box(th: &Theme) -> Div {
    tile(div().w_full().flex().flex_col(), th)
        // Dark cards stand off the page by their fill, but these sit on
        // Settings' card of the same fill: the dark rim draws their edge.
        .when(th.dark, |d| d.border_1().border_color(rgba(th.rim)))
}

/// The edge of a tile inside a [`card_box`]: the light card hairline, or
/// the dark rim.
fn inner_edge(th: &Theme) -> u32 {
    if th.dark { th.rim } else { th.card_edge }
}

/// A card's title, and a line under it.
fn card_heading(title: String, detail: Option<String>, th: &Theme) -> Div {
    div()
        .px(px(space::S5))
        .pt(px(space::S5))
        .pb(px(space::S2))
        .flex()
        .flex_col()
        .gap(px(space::S1))
        .child(
            div()
                .text_size(px(text::SUBTITLE))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(th.text))
                .child(title),
        )
        .when_some(detail, |d, detail| {
            d.child(
                div()
                    .text_size(px(text::SMALL))
                    .text_color(rgba(th.text_faint))
                    .child(detail),
            )
        })
}

/// A quiet text button in the accent color.
fn link_button(id: &'static str, label: String, th: &Theme) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .px(px(10.0))
        .py(px(6.0))
        .rounded(px(16.0))
        .text_size(px(13.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .relative()
        .child(crate::widgets::hover_fade("hover-glow", Some(16.0), th))
        .child(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every error the daemon names has its own English line.
    #[test]
    fn errors_have_text() {
        for code in [
            katna_error::WRONG_PASSWORD,
            katna_error::EXISTS,
            katna_error::BAD_EMAIL,
            katna_error::SHORT_PASSWORD,
            katna_error::LONG_PASSWORD,
            katna_error::WRONG_CODE,
            katna_error::CODE_EXPIRED,
            katna_error::TOO_MANY,
            katna_error::MAIL_FAILED,
            katna_error::SIGN_IN,
            katna_error::OFFLINE,
            katna_error::SERVER,
        ] {
            let text = error_text(code);
            assert!(!text.starts_with("katna-"), "{code}: {text}");
            assert_ne!(text, code);
        }
        assert_eq!(
            error_text("The service did not answer"),
            "The service did not answer"
        );
    }
}
