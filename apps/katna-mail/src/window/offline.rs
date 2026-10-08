// SPDX-License-Identifier: GPL-3.0-or-later

//! Accounts taken offline (`[offline]` in the settings,
//! `docs/ARCHITECTURE.md` §6.1): Katna doesn't connect to their servers
//! until each is brought back online or its time ends. Their mail stays
//! readable; what is done meanwhile waits and goes out when it is back.
//!
//! An offline account shows a crossed cloud wherever it appears: before
//! its count in the folder pane (a click brings it back), on the account
//! picture in the top bar and in the account card, beside it in Compose's
//! From. Its right-click menu, the account card and Settings > Accounts
//! take it offline, for an hour, until tomorrow or until brought back.

use std::time::Duration;

use gpui::{
    AnimationExt, AnyElement, Context, Div, ElementId, SpringAnimation, Stateful, div, prelude::*,
    rgba,
};
use jiff::{Timestamp, tz::TimeZone};
use katna_core::AccountId;
use katna_i18n::{format, tr};
use katna_ui::motion;
use katna_ui::{px, tokens};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// How long "For 1 hour" takes an account offline.
const HOUR: i64 = 60 * 60;

/// How long an account goes offline for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OfflineFor {
    /// Until brought back.
    Now,
    Hour,
    /// Until 8 tomorrow morning.
    Tomorrow,
}

impl OfflineFor {
    /// When it ends, in Unix seconds; `None` for never by itself.
    fn until(self) -> Option<i64> {
        match self {
            Self::Now => None,
            Self::Hour => Some(unix_now() + HOUR),
            Self::Tomorrow => super::snooze::tomorrow_morning(&TimeZone::system()),
        }
    }
}

impl MailWindow {
    /// Whether `account` is offline now: `Some(None)` until brought back,
    /// `Some(Some(at))` until `at`.
    pub(super) fn offline_ends(&self, account: AccountId) -> Option<Option<i64>> {
        let address = &self.accounts.iter().find(|a| a.id == account)?.address;
        self.config.offline.ends(address, unix_now())
    }

    pub(super) fn is_account_offline(&self, account: AccountId) -> bool {
        self.offline_ends(account).is_some()
    }

    /// Whether any mail account is offline now.
    pub(super) fn any_account_offline(&self) -> bool {
        self.accounts
            .iter()
            .any(|a| a.kind.is_mail() && self.is_account_offline(a.id))
    }

    /// Whether every mail account is offline now ("Work offline").
    pub(super) fn all_accounts_offline(&self) -> bool {
        let mut mail = self.accounts.iter().filter(|a| a.kind.is_mail()).peekable();
        mail.peek().is_some() && mail.all(|a| self.is_account_offline(a.id))
    }

    /// Takes `account` offline for `time`, or brings it back online.
    pub(super) fn set_account_offline(
        &mut self,
        account: AccountId,
        time: Option<OfflineFor>,
        cx: &mut Context<Self>,
    ) {
        let Some(address) = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.address.clone())
        else {
            return;
        };
        self.config
            .offline
            .set(&address, time.is_some(), time.and_then(OfflineFor::until));
        self.offline_changed(cx);
    }

    /// Takes every mail account offline until brought back ("Work
    /// offline"), or brings them all back.
    pub(super) fn set_all_offline(&mut self, offline: bool, cx: &mut Context<Self>) {
        for account in self.accounts.iter().filter(|a| a.kind.is_mail()) {
            self.config.offline.set(&account.address, offline, None);
        }
        self.offline_changed(cx);
    }

    /// Saves the change, has the daemon follow and redraws.
    fn offline_changed(&mut self, cx: &mut Context<Self>) {
        self.config.offline.prune(unix_now());
        self.save_config();
        self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
        self.retry_offline_downloads(cx);
        self.watch_offline_ends(cx);
        cx.notify();
    }

    /// Redraws when the next account taken offline for a while comes
    /// back (the daemon reconnects it then by itself).
    pub(super) fn watch_offline_ends(&mut self, cx: &mut Context<Self>) {
        let Some(at) = self.config.offline.next_end(unix_now()) else {
            self.offline_end = None;
            return;
        };
        let wait = Duration::from_secs(u64::try_from(at - unix_now()).unwrap_or(0) + 1);
        self.offline_end = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(wait).await;
            this.update(cx, |this, cx| {
                this.retry_offline_downloads(cx);
                this.watch_offline_ends(cx);
                cx.notify();
            })
            .ok();
        }));
    }

    /// An opened message that waited for its account downloads now, if
    /// the account is back.
    fn retry_offline_downloads(&mut self, cx: &mut Context<Self>) {
        self.downloads
            .retain(|_, download| !matches!(download, super::download::Download::Offline));
        self.download_bodies(cx);
    }

    /// What an offline account's line says: "Offline", "Offline until
    /// 9:00 AM", with what waits to go out after it.
    pub(super) fn offline_text(&self, account: AccountId) -> Option<String> {
        let until = self.offline_ends(account)?;
        let state = match until.and_then(|at| Timestamp::from_second(at).ok()) {
            Some(at) => {
                let at = at.to_zoned(TimeZone::system());
                let today = Timestamp::now().to_zoned(TimeZone::system()).date();
                let time = format::time(at.datetime());
                if at.date() == today {
                    tr!("offline-until", time = time)
                } else {
                    tr!(
                        "offline-until-day",
                        day = format::weekday(at.datetime()),
                        time = time
                    )
                }
            }
            None => tr!("offline-state"),
        };
        Some(match self.waiting.get(&account).copied().unwrap_or(0) {
            0 => state,
            waiting => tr!("offline-waiting", state = state, count = waiting),
        })
    }

    /// The crossed cloud that marks an offline account where it is
    /// listed; a click brings it back online.
    pub(super) fn offline_mark(
        &self,
        id: impl Into<ElementId>,
        account: AccountId,
        size: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        let text = self.offline_text(account).unwrap_or_default();
        div()
            .id(id)
            .flex_none()
            .size(px(size))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(icon("cloud-off", th.text_dim, size * 0.7))
            .tooltip(tip(format!("{text}\n{}", tr!("offline-click-online")), th))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.set_account_offline(account, None, cx);
            }))
    }

    /// "Work offline" at the top of the account card: every account at
    /// once. Only with two or more mail accounts; one has its own row.
    pub(super) fn work_offline_row(&self, th: &Theme, cx: &Context<Self>) -> Option<AnyElement> {
        if self.accounts.iter().filter(|a| a.kind.is_mail()).count() < 2 {
            return None;
        }
        let on = self.all_accounts_offline();
        let th_copy = *th;
        Some(
            crate::widgets::row("work-offline", false, th)
                .on_click(cx.listener(move |this, _, _, cx| this.set_all_offline(!on, cx)))
                .child(icon("cloud-off", th.text_dim, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(tokens::text::BODY))
                        .child(tr!("offline-work-offline")),
                )
                .child(div().with_spring(
                    "work-offline-switch",
                    SpringAnimation::new(katna_ui::motion::scaled(motion::SLIDE)).to(if on {
                        1.0
                    } else {
                        0.0
                    }),
                    move |el, s: f32| el.child(crate::widgets::switch(s.clamp(0.0, 1.0), &th_copy)),
                ))
                .into_any_element(),
        )
    }

    /// The line in Compose when its From account is offline: the mail
    /// waits in the Outbox until the account is back.
    pub(super) fn compose_offline_strip(
        &self,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let account = self.compose_from_id()?;
        if !self.is_account_offline(account) {
            return None;
        }
        let name = self.accounts.iter().find(|a| a.id == account).map(|a| {
            if a.display_name.trim().is_empty() {
                a.address.clone()
            } else {
                a.display_name.clone()
            }
        })?;
        Some(
            div()
                .mx(px(tokens::space::S5))
                .mt(px(tokens::space::S2))
                .px(px(tokens::space::S4))
                .py(px(tokens::space::S3))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(tokens::space::S3))
                .rounded(px(tokens::radius::SM))
                .bg(rgba(th.chip))
                .text_size(px(tokens::text::SMALL))
                .text_color(rgba(th.text_dim))
                .child(icon("cloud-off", th.text_dim, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .child(tr!("offline-compose", account = name)),
                )
                .child(
                    div()
                        .id("compose-offline-online")
                        .flex_none()
                        .px(px(tokens::space::S4))
                        .py(px(tokens::space::S2))
                        .rounded_full()
                        .text_color(rgba(th.accent))
                        .cursor_pointer()
                        .relative()
                        .child(crate::widgets::hover_fade("hover-glow", None, th))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_account_offline(account, None, cx);
                        }))
                        .child(tr!("offline-go-online")),
                )
                .into_any_element(),
        )
    }

    /// Settings > Accounts > Connected: a switch for each mail account,
    /// and how long an offline one stays offline.
    pub(super) fn connected_section(
        &self,
        accounts: &[katna_core::Account],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if accounts.is_empty() {
            return None;
        }
        let mut list = div().flex().flex_col();
        for account in accounts {
            let id = account.id;
            let ends = self.offline_ends(id);
            let label = if account.display_name.trim().is_empty() {
                account.address.clone()
            } else {
                account.display_name.clone()
            };
            let detail = self
                .offline_text(id)
                .unwrap_or_else(|| account.address.clone());
            list = list.child(self.switch_row(
                ("account-connected", id.0 as usize),
                label,
                detail,
                ends.is_none(),
                super::settings::Change::AccountOnline(id, ends.is_some()),
                th,
                cx,
            ));
            let Some(until) = ends else {
                continue;
            };
            let picked = match until {
                None => OfflineFor::Now,
                Some(at) if at - unix_now() <= HOUR => OfflineFor::Hour,
                Some(_) => OfflineFor::Tomorrow,
            };
            let chips = [
                (OfflineFor::Hour, tr!("offline-for-hour")),
                (OfflineFor::Tomorrow, tr!("offline-until-tomorrow")),
                (OfflineFor::Now, tr!("offline-until-online")),
            ]
            .into_iter()
            .enumerate()
            .map(|(n, (time, label))| {
                crate::widgets::choice_chip(
                    ("account-offline-for", id.0 as usize * 4 + n),
                    label,
                    picked == time,
                    th,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.set_account_offline(id, Some(time), cx);
                }))
            });
            list = list.child(
                div()
                    .pl(px(tokens::space::S4))
                    .pb(px(tokens::space::S3))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(tokens::space::S3))
                    .children(chips),
            );
        }
        Some(
            self.row(
                tr!("offline-settings-row"),
                Some(tr!("offline-settings-detail").as_str()),
                list,
                th,
            )
            .into_any_element(),
        )
    }

    /// A small crossed cloud on the corner of an account picture of
    /// `size`, when `offline`.
    pub(super) fn offline_badge(
        &self,
        picture: AnyElement,
        size: f32,
        offline: bool,
        th: &Theme,
    ) -> AnyElement {
        if !offline {
            return picture;
        }
        let badge = (size * 0.5).round().max(12.0);
        div()
            .flex_none()
            .relative()
            .child(picture)
            .child(
                div()
                    .absolute()
                    .right(px(-tokens::space::S1))
                    .bottom(px(-tokens::space::S1))
                    .size(px(badge))
                    .rounded_full()
                    .bg(rgba(th.surface))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon("cloud-off", th.text_dim, badge - 4.0)),
            )
            .into_any_element()
    }
}

fn unix_now() -> i64 {
    Timestamp::now().as_second()
}
