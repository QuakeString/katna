// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Katna account: create a Katna account, sign in and out, see
//! and sign out the computers signed in, change or reset the password and
//! delete the account. Katna Server's features (read receipts, link
//! tracking, Activity, translation) work only while signed in to an
//! account whose address is confirmed.
//!
//! The daemon talks to the server (`KatnaSignIn` and the rest on D-Bus);
//! this page only asks it. The account's password is never a mail
//! password, and mail logins never go to the server.

use gpui::{AnyElement, Context, Entity, FontWeight, Subscription, Window, div, prelude::*, rgba};
use katna_dbus::{KatnaAccount, KatnaDevice, PimProxy, katna_error};
use katna_i18n::tr;
use katna_ui::{InputEvent, TextInput, px};

use super::MailWindow;
use super::settings_page::Section;
use crate::daemon;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{filled_button, outlined_button};

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
    devices: Vec<KatnaDevice>,
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

    /// "Sign in to use this", with a button to Settings > Katna account,
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
                        this.open_settings_page(Section::KatnaAccount, window, cx)
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
                devices: Vec::new(),
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
                page.account = Some(account);
                page.devices = devices;
                for field in [&page.password, &page.code, &page.new_password] {
                    clear(field, cx);
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

    /// Settings > Katna account.
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

        let intro = div()
            .pt(px(20.0))
            .pb(px(4.0))
            .text_size(px(13.0))
            .line_height(px(19.0))
            .text_color(rgba(th.text_dim))
            .child(tr!("katna-intro"));
        let status = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .when_some(error, |d, error| {
                d.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.error))
                        .child(error),
                )
            })
            .when_some(notice, |d, notice| {
                d.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(notice),
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
                .hover(|s| s.bg(rgba(th.hover)))
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
            let signed_in = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(div().text_size(px(14.0)).child(account.email.clone()))
                .child(
                    outlined_button("katna-sign-out", tr!("katna-sign-out"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.katna_sign_out(cx))),
                );
            let mut list = div().flex().flex_col();
            for (ix, device) in devices.iter().enumerate() {
                let when = format::local(device.signed_in_at, &self.tz)
                    .map(format::long_date)
                    .unwrap_or_default();
                let id = device.id.clone();
                list = list.child(
                    div()
                        .px(px(8.0))
                        .py(px(6.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .rounded(px(8.0))
                        .hover(|s| s.bg(rgba(th.hover)))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .child(div().text_size(px(14.0)).child(if device.name.is_empty() {
                                    tr!("katna-device-unnamed")
                                } else {
                                    device.name.clone()
                                }))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgba(th.text_faint))
                                        .child(if device.this {
                                            tr!("katna-device-this", date = when)
                                        } else {
                                            tr!("katna-device-since", date = when)
                                        }),
                                ),
                        )
                        .when(!device.this, |d| {
                            d.child(
                                link(("katna-device-sign-out", ix).into(), tr!("katna-sign-out"))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.katna_sign_out_device(id.clone(), cx)
                                    })),
                            )
                        }),
                );
            }
            let password_row = if mode == Mode::ChangePassword {
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
            } else {
                div()
                    .flex()
                    .child(
                        outlined_button("katna-change-password", tr!("katna-change-password"), th)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.katna_mode(Mode::ChangePassword, cx)
                            })),
                    )
                    .into_any_element()
            };
            let delete_row = if mode == Mode::Delete {
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
            } else {
                div()
                    .flex()
                    .child(
                        outlined_button("katna-delete", tr!("katna-delete"), th)
                            .text_color(rgba(th.error))
                            .on_click(
                                cx.listener(|this, _, _, cx| this.katna_mode(Mode::Delete, cx)),
                            ),
                    )
                    .into_any_element()
            };
            div()
                .child(self.row(
                    tr!("katna-signed-in"),
                    Some(&tr!("katna-signed-in-detail")),
                    signed_in,
                    th,
                ))
                .child(self.row(
                    tr!("katna-devices"),
                    Some(&tr!("katna-devices-detail")),
                    list,
                    th,
                ))
                .child(self.row(
                    tr!("katna-password"),
                    Some(&tr!("katna-password-detail")),
                    password_row,
                    th,
                ))
                .child(self.row(
                    tr!("katna-delete"),
                    Some(&tr!("katna-delete-detail")),
                    delete_row,
                    th,
                ))
                .child(div().pt(px(8.0)).child(status))
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
