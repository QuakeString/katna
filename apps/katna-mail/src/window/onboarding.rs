// SPDX-License-Identifier: GPL-3.0-or-later

//! The first start: a few pages that fill the window until the first
//! account is added. Welcome says what Katna is; Account checks the
//! background service and adds the account; Look picks the layout and
//! colors; Katna account offers a Katna account and what it turns on,
//! or Skip; Share asks whether to send crash reports (`share_ask`); Ready
//! offers the tour of the window (`tour`).

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, SharedString, SpringAnimation, Task,
    Window, div, ease_out_quint, prelude::*, rgba,
};
use katna_core::config::{Density, ReadingPane, Theme as ThemeChoice};
use katna_i18n::tr;
use katna_ui::motion::{self, lerp};
use katna_ui::px;

use super::add_account::text_button;
use super::settings::Change;
use super::{MailWindow, PANEL_RADIUS, share_ask};
use crate::daemon;
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, filled_button, icon};

/// How long a page takes to slide in.
const PAGE_IN: Duration = Duration::from_millis(360);
const CARD_WIDTH: f32 = 600.0;
const CARD_HEIGHT: f32 = 620.0;
/// The scrim over the window behind the pages: light, so the window
/// shows through.
const SCRIM_LIGHT: u32 = 0x0000_0029;
const SCRIM_DARK: u32 = 0x0000_0052;

/// Ease out with a little overshoot, for things that pop in.
fn ease_out_back(t: f32) -> f32 {
    let c = 1.7_f32;
    1.0 + (c + 1.0) * (t - 1.0).powi(3) + c * (t - 1.0).powi(2)
}

/// The pages, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    Welcome,
    Account,
    Look,
    Katna,
    Share,
    Ready,
}

impl Step {
    const ALL: [Step; 6] = [
        Step::Welcome,
        Step::Account,
        Step::Look,
        Step::Katna,
        Step::Share,
        Step::Ready,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }
}

/// Whether `katna-daemon` answers.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Service {
    Checking,
    Running,
    Missing(String),
}

pub(super) struct Onboarding {
    step: Step,
    /// The last move went forward, so the page comes in from the right.
    forward: bool,
    service: Service,
    /// On the Katna account page: `None` shows what an account brings,
    /// `Some(create)` the form to create one or sign in.
    katna_form: Option<bool>,
    _check: Option<Task<()>>,
}

impl Onboarding {
    pub(super) fn new() -> Self {
        Self {
            step: Step::Welcome,
            katna_form: None,
            forward: true,
            service: Service::Checking,
            _check: None,
        }
    }
}

impl MailWindow {
    /// Whether the first-start pages cover the window.
    pub(super) fn onboarding(&self) -> bool {
        self.onboarding.is_some()
    }

    fn onboarding_step(&mut self, step: Step, cx: &mut Context<Self>) {
        let Some(onboarding) = &mut self.onboarding else {
            return;
        };
        onboarding.forward = step.index() >= onboarding.step.index();
        onboarding.step = step;
        onboarding.katna_form = None;
        if step == Step::Account {
            self.check_service(cx);
        }
        cx.notify();
    }

    /// Asks the daemon whether it runs; D-Bus starts it if it can.
    fn check_service(&mut self, cx: &mut Context<Self>) {
        let Some(onboarding) = &mut self.onboarding else {
            return;
        };
        onboarding.service = Service::Checking;
        let connection = self.daemon.clone();
        onboarding._check = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::first_sync_pending(&connection).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(onboarding) = &mut this.onboarding {
                    onboarding.service = match result {
                        Ok(_) => Service::Running,
                        Err(err) => Service::Missing(err),
                    };
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    /// Shows the Katna account form, to create one or to sign in.
    pub(super) fn onboarding_katna_form(&mut self, form: Option<bool>, cx: &mut Context<Self>) {
        if let Some(onboarding) = &mut self.onboarding {
            onboarding.forward = form.is_some();
            onboarding.katna_form = form;
            cx.notify();
        }
    }

    /// Signed in to a Katna account: on to the crash reports.
    pub(super) fn onboarding_katna_done(&mut self, cx: &mut Context<Self>) {
        self.onboarding_step(Step::Share, cx);
    }

    /// The first account is in: on to the look.
    pub(super) fn onboarding_account_added(&mut self, cx: &mut Context<Self>) {
        if self
            .onboarding
            .as_ref()
            .is_some_and(|o| matches!(o.step, Step::Welcome | Step::Account))
        {
            self.onboarding_step(Step::Look, cx);
        }
    }

    /// Leaves the first-start pages, with or without the tour.
    fn finish_onboarding(&mut self, tour: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.onboarding = None;
        if tour {
            self.start_tour(false, window, cx);
        } else {
            self.config.onboarding.done = true;
            self.save_config();
            window.focus(&self.list_focus, cx);
        }
        cx.notify();
    }

    pub(super) fn render_onboarding(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(onboarding) = &self.onboarding else {
            return div().into_any_element();
        };
        let step = onboarding.step;
        let forward = onboarding.forward;
        let katna_form = onboarding.katna_form;
        let phone = self.layout.shape.is_phone();
        let reduce = cx.reduce_motion();
        let (body, actions) = match step {
            Step::Welcome => self.welcome_page(th, cx),
            Step::Account => self.account_page(th, window, cx),
            Step::Look => self.look_page(th, cx),
            Step::Katna => match katna_form {
                // Already signed in on this computer: say so.
                None if !self.katna_signed_in() => self.katna_offer_page(th, cx),
                None => {
                    let (body, main) = self.katna_onboarding_form(false, th, window, cx);
                    (body, actions_row(None, main))
                }
                Some(create) => {
                    let (body, main) = self.katna_onboarding_form(create, th, window, cx);
                    let back = text_button("onboarding-katna-back", tr!("onboarding-back"), th)
                        .on_click(
                            cx.listener(|this, _, _, cx| this.onboarding_katna_form(None, cx)),
                        )
                        .into_any_element();
                    (body, actions_row(Some(back), main))
                }
            },
            Step::Share => self.share_page(th, cx),
            Step::Ready => self.ready_page(th, cx),
        };
        // The page scrolls when it does not fit; its buttons stay put.
        let page = div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                div()
                    .id(("onboarding-body", step.index()))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex_none()
                            .min_h_full()
                            .flex()
                            .flex_col()
                            .justify_center()
                            .child(body),
                    ),
            )
            .child(div().flex_none().child(actions));
        let page = if reduce {
            page.into_any_element()
        } else {
            let from = if forward { 32.0 } else { -32.0 };
            page.with_animation(
                (
                    "onboarding-page",
                    step.index() * 3 + katna_form.map_or(0, |c| 1 + c as usize),
                ),
                Animation::new(PAGE_IN).with_easing(ease_out_quint()),
                move |el, t| el.opacity(t).ml(px(from * (1.0 - t))),
            )
            .into_any_element()
        };
        let card = div()
            .id("onboarding-card")
            .w(px(CARD_WIDTH))
            .max_w_full()
            // The same height on every page, so the card does not jump;
            // all the room there is on a phone.
            .map(|d| {
                if phone {
                    d.h_full()
                } else {
                    d.h(px(CARD_HEIGHT))
                }
            })
            .max_h_full()
            .overflow_hidden()
            .px(px(if phone { 24.0 } else { 48.0 }))
            .pt(px(28.0))
            .pb(px(if phone { 24.0 } else { 32.0 }))
            .flex()
            .flex_col()
            .gap(px(28.0))
            .rounded(px(PANEL_RADIUS))
            .bg(rgba(th.surface))
            .shadow(elevation(th, 3.0))
            .child(step_dots(step, th))
            .child(page);
        // The window as it will be shows through a light, blurred scrim,
        // rather than an empty dark page.
        let scrim = div()
            .id("onboarding-scrim")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .map(|d| {
                let fill = if th.dark { SCRIM_DARK } else { SCRIM_LIGHT };
                if th.frost == 0 {
                    d.bg(rgba(fill))
                } else {
                    d.child(katna_ui::frost::glass(
                        rgba(fill).into(),
                        px(0.0),
                        th.frost as f32,
                    ))
                }
            });
        div()
            .relative()
            .size_full()
            .child(self.render_skeleton(th, cx))
            .child(scrim)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .px(px(16.0))
                    .pb(px(16.0))
                    // Above a phone's apps along the bottom.
                    .when(phone, |d| {
                        d.pt(px(8.0)).pb(px(super::layout::BOTTOM_BAR_HEIGHT + 8.0))
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(card),
            )
            .into_any_element()
    }

    fn welcome_page(&self, th: &Theme, cx: &mut Context<Self>) -> (AnyElement, AnyElement) {
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .pb(px(4.0))
                    .child(crate::widgets::katna_wordmark(96.0)),
            )
            .child(title(tr!("onboarding-welcome-title"), th))
            .child(lead(&tr!("onboarding-welcome-lead"), th))
            .child(
                div()
                    .pt(px(12.0))
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(feature(
                        "bolt",
                        tr!("onboarding-fast-title"),
                        tr!("onboarding-fast-text"),
                        th,
                    ))
                    .child(feature(
                        "inbox",
                        tr!("onboarding-providers-title"),
                        tr!("onboarding-providers-text"),
                        th,
                    ))
                    .child(feature(
                        "lock",
                        tr!("onboarding-private-title"),
                        tr!("onboarding-private-text"),
                        th,
                    )),
            );
        let actions = actions_row(
            None,
            filled_button("onboarding-start", tr!("onboarding-get-started"), th)
                .on_click(cx.listener(|this, _, _, cx| this.onboarding_step(Step::Account, cx))),
        );
        (body.into_any_element(), actions)
    }

    fn account_page(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (AnyElement, AnyElement) {
        let service = self
            .onboarding
            .as_ref()
            .map_or(Service::Checking, |o| o.service.clone());
        let status = match &service {
            Service::Checking => status_line(
                "refresh",
                tr!("onboarding-service-checking"),
                th.text_faint,
                th,
            ),
            Service::Running => status_line(
                "check-circle",
                tr!("onboarding-service-running"),
                th.accent,
                th,
            ),
            Service::Missing(detail) => div()
                .w_full()
                .p(px(16.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .rounded(px(12.0))
                .bg(rgba(fade(th.error, 0.08)))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.error))
                        .child(icon("info", th.error, 20.0))
                        .child(tr!("onboarding-service-missing")),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .line_height(px(20.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("onboarding-service-start")),
                )
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .bg(rgba(th.page))
                        .font_family("monospace")
                        .text_size(px(13.0))
                        .child("systemctl --user enable --now katna-daemon"),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(detail.clone()),
                )
                .child(div().child(
                    text_button("onboarding-recheck", tr!("onboarding-check-again"), th).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.check_service(cx);
                            cx.notify();
                        }),
                    ),
                ))
                .into_any_element(),
        };
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(icon("person-add", th.accent, 48.0))
            .child(title(tr!("onboarding-account-title"), th))
            .child(lead(&tr!("onboarding-account-lead"), th))
            .child(div().pt(px(12.0)).w_full().child(status));
        let ready = service == Service::Running;
        let add = filled_button("onboarding-add", tr!("onboarding-add-account"), th)
            .when(!ready, |d| d.opacity(0.5))
            .on_click(cx.listener(|this, _, window, cx| this.open_add_account(window, cx)));
        let _ = window;
        let actions = actions_row(
            Some(
                text_button("onboarding-back", tr!("onboarding-back"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.onboarding_step(Step::Welcome, cx)))
                    .into_any_element(),
            ),
            add,
        );
        (body.into_any_element(), actions)
    }

    fn look_page(&self, th: &Theme, cx: &mut Context<Self>) -> (AnyElement, AnyElement) {
        let view = &self.config.mail;
        let body = div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .child(title(tr!("onboarding-look-title"), th))
                    .child(lead(&tr!("onboarding-look-lead"), th)),
            )
            .child(label(&tr!("onboarding-reading-pane"), th))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .child(self.pane_choice(
                        ReadingPane::Right,
                        tr!("onboarding-pane-right"),
                        th,
                        cx,
                    ))
                    .child(self.pane_choice(
                        ReadingPane::None,
                        tr!("onboarding-pane-none"),
                        th,
                        cx,
                    )),
            )
            .child(
                div()
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(label(&tr!("onboarding-theme"), th))
                            .children(
                                [
                                    (
                                        ThemeChoice::System,
                                        "theme-system",
                                        tr!("onboarding-theme-system"),
                                    ),
                                    (
                                        ThemeChoice::Light,
                                        "theme-light",
                                        tr!("onboarding-theme-light"),
                                    ),
                                    (
                                        ThemeChoice::Dark,
                                        "theme-dark",
                                        tr!("onboarding-theme-dark"),
                                    ),
                                ]
                                .into_iter()
                                .map(|(choice, id, text)| {
                                    self.radio_row(
                                        id,
                                        text,
                                        view.theme == choice,
                                        Change::Theme(choice),
                                        th,
                                        cx,
                                    )
                                }),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(label(&tr!("onboarding-density"), th))
                            .child(self.radio_row(
                                "density-default",
                                tr!("onboarding-density-default"),
                                view.density == Density::Default,
                                Change::Density(Density::Default),
                                th,
                                cx,
                            ))
                            .child(self.radio_row(
                                "density-compact",
                                tr!("onboarding-density-compact"),
                                view.density == Density::Compact,
                                Change::Density(Density::Compact),
                                th,
                                cx,
                            )),
                    ),
            );
        let actions = actions_row(
            None,
            filled_button("onboarding-look-done", tr!("onboarding-continue"), th)
                .on_click(cx.listener(|this, _, _, cx| this.onboarding_step(Step::Katna, cx))),
        );
        (body.into_any_element(), actions)
    }

    /// What a Katna account turns on, with Create account, I have an
    /// account, and Skip.
    fn katna_offer_page(&self, th: &Theme, cx: &mut Context<Self>) -> (AnyElement, AnyElement) {
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(crate::widgets::katna_mark(48.0))
            .child(title(tr!("onboarding-katna-title"), th))
            .child(lead(&tr!("onboarding-katna-lead"), th))
            .child(
                div()
                    .pt(px(8.0))
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(feature(
                        "read-receipt",
                        tr!("onboarding-katna-receipts-title"),
                        tr!("onboarding-katna-receipts-text"),
                        th,
                    ))
                    .child(feature(
                        "link",
                        tr!("onboarding-katna-links-title"),
                        tr!("onboarding-katna-links-text"),
                        th,
                    ))
                    .child(feature(
                        "activity",
                        tr!("onboarding-katna-activity-title"),
                        tr!("onboarding-katna-activity-text"),
                        th,
                    ))
                    .child(feature(
                        "translate",
                        tr!("onboarding-katna-translate-title"),
                        tr!("onboarding-katna-translate-text"),
                        th,
                    )),
            )
            .child(
                div()
                    .pt(px(6.0))
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_start()
                    .gap(px(6.0))
                    .text_size(px(12.0))
                    .line_height(px(17.0))
                    .text_color(rgba(th.text_faint))
                    .child(
                        div()
                            .flex_none()
                            .pt(px(1.0))
                            .child(icon("lock", th.text_faint, 14.0)),
                    )
                    .child(div().min_w_0().child(tr!("onboarding-katna-private"))),
            );
        let have = text_button("onboarding-katna-have", tr!("katna-have-account"), th)
            .on_click(cx.listener(|this, _, _, cx| this.onboarding_katna_form(Some(false), cx)));
        let create = filled_button("onboarding-katna-create", tr!("katna-create"), th)
            .on_click(cx.listener(|this, _, _, cx| this.onboarding_katna_form(Some(true), cx)));
        let skip = text_button("onboarding-katna-skip", tr!("onboarding-skip"), th)
            .on_click(cx.listener(|this, _, _, cx| this.onboarding_step(Step::Share, cx)));
        // A phone has no room for three buttons in a row: Create account
        // across the card, the other two under it.
        let actions = if self.layout.shape.is_phone() {
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(create.w_full().justify_center())
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .justify_center()
                        .gap(px(8.0))
                        .child(skip)
                        .child(have),
                )
                .into_any_element()
        } else {
            actions_row(
                Some(skip.into_any_element()),
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(have)
                    .child(create),
            )
        };
        (body.into_any_element(), actions)
    }

    fn share_page(&self, th: &Theme, cx: &mut Context<Self>) -> (AnyElement, AnyElement) {
        let body = div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .child(title(share_ask::title_text(), th))
                    .child(lead(&share_ask::lead_text(), th)),
            )
            .child(share_ask::points(th));
        let actions = div()
            .pt(px(8.0))
            .child(share_ask::answers(
                "onboarding-share",
                |this, send, _, cx| {
                    this.apply(Change::SendCrashReports(send), cx);
                    this.onboarding_step(Step::Ready, cx);
                },
                th,
                cx,
            ))
            .into_any_element();
        (body.into_any_element(), actions)
    }

    fn ready_page(&self, th: &Theme, cx: &mut Context<Self>) -> (AnyElement, AnyElement) {
        let address = self
            .accounts
            .first()
            .map(|a| a.address.clone())
            .unwrap_or_default();
        let body = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .size(px(64.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(th.accent, 0.12)))
                    .child(icon("check-circle", th.accent, 36.0))
                    .with_animation(
                        "onboarding-ready",
                        Animation::new(PAGE_IN).with_easing(ease_out_back),
                        |el, t| el.size(px(64.0 * lerp(0.6, 1.0, t))),
                    ),
            )
            .child(title(tr!("onboarding-ready-title"), th))
            .child(lead(
                &if address.is_empty() {
                    tr!("onboarding-ready-lead")
                } else {
                    tr!("onboarding-ready-lead-address", address = address.as_str())
                },
                th,
            ))
            .child(lead(&tr!("onboarding-ready-tour"), th));
        let actions =
            actions_row(
                Some(
                    text_button("onboarding-skip", tr!("onboarding-skip"), th)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.finish_onboarding(false, window, cx)
                        }))
                        .into_any_element(),
                ),
                filled_button("onboarding-tour", tr!("onboarding-take-tour"), th).on_click(
                    cx.listener(|this, _, window, cx| this.finish_onboarding(true, window, cx)),
                ),
            );
        (body.into_any_element(), actions)
    }
}

/// Where the pages are: one dot per page, the current one long.
fn step_dots(step: Step, th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .justify_center()
        .gap(px(6.0))
        .children(Step::ALL.into_iter().map(|s| {
            let on = s == step;
            let done = s.index() < step.index();
            div()
                .h(px(6.0))
                .rounded_full()
                .bg(rgba(if on || done { th.accent } else { th.divider }))
                .with_spring(
                    ("onboarding-dot", s.index()),
                    SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
                    |el, t: f32| el.w(px(6.0 + 18.0 * t.clamp(0.0, 1.0))),
                )
        }))
        .into_any_element()
}

pub(super) fn title(text: impl Into<SharedString>, th: &Theme) -> AnyElement {
    div()
        .text_size(px(24.0))
        .line_height(px(32.0))
        .text_color(rgba(th.text))
        .text_center()
        .child(text.into())
        .into_any_element()
}

pub(super) fn lead(text: &str, th: &Theme) -> AnyElement {
    div()
        .max_w(px(460.0))
        .text_size(px(14.0))
        .line_height(px(21.0))
        .text_color(rgba(th.text_dim))
        .text_center()
        .child(text.to_owned())
        .into_any_element()
}

fn label(text: &str, th: &Theme) -> AnyElement {
    div()
        .px(px(8.0))
        .pb(px(4.0))
        .text_size(px(12.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_faint))
        .child(text.to_uppercase())
        .into_any_element()
}

/// One line of what Katna does: an icon in a round tint, a name and a
/// sentence.
pub(super) fn feature(
    name: &str,
    heading: impl Into<SharedString>,
    text: impl Into<SharedString>,
    th: &Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(16.0))
        .child(
            div()
                .flex_none()
                .size(px(40.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(rgba(fade(th.accent, 0.12)))
                .child(icon(name, th.accent, 22.0)),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text))
                        .child(heading.into()),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .line_height(px(19.0))
                        .text_color(rgba(th.text_dim))
                        .child(text.into()),
                ),
        )
        .into_any_element()
}

fn status_line(name: &str, text: impl Into<SharedString>, color: u32, th: &Theme) -> AnyElement {
    div()
        .w_full()
        .flex()
        .flex_row()
        .justify_center()
        .items_center()
        .gap(px(8.0))
        .text_size(px(13.0))
        .text_color(rgba(th.text_dim))
        .child(icon(name, color, 18.0))
        .child(text.into())
        .into_any_element()
}

/// Back on the left, if any; the main button on the right.
fn actions_row(back: Option<AnyElement>, main: impl IntoElement) -> AnyElement {
    div()
        .pt(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .children(back)
        .child(div().flex_1())
        .child(main)
        .into_any_element()
}
