// SPDX-License-Identifier: GPL-3.0-or-later

//! Activity: who opened mail sent with open and click tracking and who
//! followed its links (`docs/ARCHITECTURE.md` §16.1). A button beside the
//! search box counts what is new and opens a list of it, newest first; its
//! Details button opens a report over a period (the last 7 or 30 days, all
//! time or chosen dates): open and click rates, opens and clicks by day,
//! and the subject lines by open rate.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, MouseButton,
    MouseDownEvent, Task, Window, anchored, canvas, deferred, div, point, prelude::*, rgba,
};
use jiff::civil::Date;
use katna_core::{Account, AccountId};
use katna_i18n::tr;
use katna_store::{ActivityItem, Insights, MessageActivity};
use katna_ui::{TextInput, px, unpx};

use super::MailWindow;
use crate::data::{Entry, Mail};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button, icon_button_colored, raised, tip};

/// At most this many tracked messages are read.
const LIMIT: u32 = 500;
/// At most this many opens and clicks are listed under the button.
const FEED: u32 = 100;
/// At most this many opens and clicks are counted for the report.
const EVENTS: u32 = 20_000;
/// The list under the button.
const MENU_WIDTH: f32 = 400.0;
/// Days drawn one bar each; longer periods get a bar per week.
const DAY_BARS: i64 = 62;

/// Opens and clicks over many messages: of every recipient of every
/// message, how many opened it and followed a link.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Rates {
    pub messages: usize,
    pub recipients: usize,
    pub opened: usize,
    pub clicked: usize,
}

impl Rates {
    pub fn of(list: &[MessageActivity]) -> Self {
        list.iter().fold(Self::default(), |sum, m| Self {
            messages: sum.messages + 1,
            recipients: sum.recipients + m.recipients.len(),
            opened: sum.opened + m.opened(),
            clicked: sum.clicked + m.clicked(),
        })
    }
}

/// `part` of `whole` in whole percent.
fn percent(part: usize, whole: usize) -> u64 {
    if whole == 0 {
        0
    } else {
        ((part as f64 / whole as f64) * 100.0).round() as u64
    }
}

/// Messages by how many of their recipients opened them, then clicked,
/// newest first among equals.
pub(super) fn by_open_rate(mut list: Vec<MessageActivity>) -> Vec<MessageActivity> {
    let rate = |m: &MessageActivity| {
        (
            percent(m.opened(), m.recipients.len()),
            percent(m.clicked(), m.recipients.len()),
        )
    };
    // Stable: the store lists newest first.
    list.sort_by_key(|m| std::cmp::Reverse(rate(m)));
    list
}

/// The period a report covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Period {
    Week,
    Month,
    All,
    /// From the first day to the last, both included.
    Custom(Date, Date),
}

impl Period {
    fn label(self) -> String {
        match self {
            Period::Week => tr!("activity-range-week"),
            Period::Month => tr!("activity-range-month"),
            Period::All => tr!("activity-range-all"),
            Period::Custom(..) => tr!("activity-range-custom"),
        }
    }

    /// The first and last day, in local dates; `first` is the oldest day
    /// to count from for All.
    fn days(self, today: Date, first: Date) -> (Date, Date) {
        match self {
            Period::Week => (today.saturating_sub(jiff::Span::new().days(6)), today),
            Period::Month => (today.saturating_sub(jiff::Span::new().days(29)), today),
            Period::All => (first.min(today), today),
            Period::Custom(a, b) => (a.min(b), a.max(b)),
        }
    }
}

/// `2026-09-01` or `2026/9/1`.
fn parse_day(text: &str) -> Option<Date> {
    let mut parts = text.trim().split(['-', '/']);
    let year = parts.next()?.trim().parse().ok()?;
    let month = parts.next()?.trim().parse().ok()?;
    let day = parts.next()?.trim().parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Date::new(year, month, day).ok()
}

/// Opens and clicks counted per bar: one bar a day, or a week for long
/// periods. Returns the bars and the days each covers.
fn bars(items: &[(Date, bool)], first: Date, last: Date) -> (Vec<(u32, u32)>, i64) {
    let days = last.since(first).map_or(0, |s| i64::from(s.get_days())) + 1;
    let step = if days > DAY_BARS { 7 } else { 1 };
    let count = usize::try_from((days + step - 1) / step)
        .unwrap_or(1)
        .max(1);
    let mut bars = vec![(0u32, 0u32); count];
    for &(day, click) in items {
        let Ok(since) = day.since(first) else {
            continue;
        };
        let at = i64::from(since.get_days());
        if at < 0 || at >= days {
            continue;
        }
        if let Some(bar) = usize::try_from(at / step)
            .ok()
            .and_then(|ix| bars.get_mut(ix))
        {
            if click {
                bar.1 += 1;
            } else {
                bar.0 += 1;
            }
        }
    }
    (bars, step)
}

/// A day as the report names it: "Sep 28", with the year when it is not
/// this year's.
fn day_label(day: Date) -> String {
    let at = day.to_datetime(jiff::civil::Time::midnight());
    if day.year() == jiff::Zoned::now().year() {
        katna_i18n::format::day_month(at)
    } else {
        katna_i18n::format::day_month_year(at)
    }
}

/// A time to answer: "5 minutes", "3 hours", "2 days".
fn duration(secs: i64) -> String {
    let minutes = (secs / 60).max(1);
    if minutes < 60 {
        tr!("insights-minutes", count = minutes)
    } else if minutes < 48 * 60 {
        tr!("insights-hours", count = (minutes + 30) / 60)
    } else {
        tr!("insights-days", count = (minutes + 720) / 1440)
    }
}

/// Mail received by weekday and hour, darker where more arrives; the
/// language's first day of the week on top.
fn hours_grid(hours: &[[u32; 24]; 7], th: &Theme) -> AnyElement {
    let top = hours.iter().flatten().copied().max().unwrap_or(0).max(1) as f32;
    let rows = katna_i18n::format::weekdays_short().map(|(day, name)| {
        let row = hours
            .get(usize::try_from(day.to_monday_zero_offset()).unwrap_or(0))
            .copied()
            .unwrap_or([0; 24]);
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .child(
                div()
                    .flex_none()
                    .w(px(36.0))
                    .text_size(px(11.0))
                    .text_color(rgba(th.text_dim))
                    .child(name),
            )
            .children(row.into_iter().map(|n| {
                let strength = n as f32 / top;
                div()
                    .flex_1()
                    .h(px(16.0))
                    .rounded(px(3.0))
                    .bg(rgba(if n == 0 {
                        fade(th.text_faint, 0.12)
                    } else {
                        fade(th.accent, 0.15 + 0.85 * strength)
                    }))
            }))
    });
    let labels = [0u64, 6, 12, 18].map(|hour| {
        div()
            .flex_1()
            .text_size(px(11.0))
            .text_color(rgba(th.text_dim))
            .child(katna_i18n::format::number(hour))
    });
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .children(rows)
        .child(
            div()
                .flex()
                .flex_row()
                .child(div().flex_none().w(px(38.0)))
                .children(labels),
        )
        .into_any_element()
}

/// The list under the Activity button.
pub(super) struct Menu {
    feed: Vec<ActivityItem>,
    /// The newest event seen before it opened: newer ones are marked.
    seen: i64,
}

/// The Details report.
pub(super) struct Report {
    period: Period,
    /// The tracked messages sent in the period, by open rate.
    list: Vec<MessageActivity>,
    /// Opens and clicks in the period, by day.
    bars: Vec<(u32, u32)>,
    /// Days per bar.
    step: i64,
    first: Date,
    last: Date,
    /// Custom dates as typed.
    dates: [Entity<TextInput>; 2],
    /// The custom date fields are showing.
    editing: bool,
    error: bool,
    /// What the mailbox did in the period, once counted.
    insights: Option<Result<Insights, String>>,
    /// The counting.
    counting: Option<Task<()>>,
    /// The account menu is open.
    accounts_open: bool,
    accounts_arrow: crate::widgets::Fold,
}

impl MailWindow {
    /// Whether the Activity button shows: once mail was tracked, or while
    /// signed in to a Katna account.
    pub(super) fn activity_shown(&self) -> bool {
        self.has_activity() || self.katna_signed_in()
    }

    /// Whether there is tracked mail.
    pub(super) fn has_activity(&self) -> bool {
        self.mail.as_ref().is_ok_and(|mail| mail.has_tracking())
    }

    /// Counts the opens and clicks not seen yet, for the button.
    pub(super) fn count_activity(&mut self) {
        let seen = self.config.mail.activity_seen;
        self.activity_unseen = self
            .mail
            .as_ref()
            .map_or(0, |mail| mail.activity_after(seen));
    }

    fn toggle_activity(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.activity.take().is_some() {
            cx.notify();
            return;
        }
        self.katna_load(window, cx);
        let Ok(mail) = &self.mail else { return };
        let feed = self.activity_list(mail);
        let newest = mail.last_activity();
        let seen = self.config.mail.activity_seen;
        self.activity = Some(Menu { feed, seen });
        if newest != seen {
            self.config.mail.activity_seen = newest;
            self.save_config();
        }
        self.activity_unseen = 0;
        cx.notify();
    }

    /// Reads the activity again while it is open.
    pub(super) fn refresh_activity(&mut self) {
        self.count_activity();
        if let Ok(mail) = &self.mail
            && self.activity.is_some()
        {
            let feed = self.activity_list(mail);
            if let Some(menu) = &mut self.activity {
                menu.feed = feed;
            }
        }
        if let Some(period) = self.activity_report.as_ref().map(|r| r.period) {
            self.fill_report(period);
        }
    }

    /// The newest opens and clicks, less the ones cleared or removed.
    fn activity_list(&self, mail: &Mail) -> Vec<ActivityItem> {
        let settings = &self.config.mail;
        let removed = &settings.activity_removed;
        let limit = FEED + u32::try_from(removed.len()).unwrap_or(u32::MAX);
        let mut feed = mail.activity_feed(0, settings.activity_cleared, limit);
        feed.retain(|item| !removed.contains(&item.seq));
        feed.truncate(FEED as usize);
        feed
    }

    /// Takes one open or click off the list.
    fn remove_activity_item(&mut self, seq: i64, cx: &mut Context<Self>) {
        let settings = &mut self.config.mail;
        if seq > settings.activity_cleared && !settings.activity_removed.contains(&seq) {
            settings.activity_removed.push(seq);
            self.save_config();
        }
        if let Some(menu) = &mut self.activity {
            menu.feed.retain(|item| item.seq != seq);
        }
        cx.notify();
    }

    /// Takes every open and click so far off the list; new ones still
    /// come.
    fn clear_activity(&mut self, cx: &mut Context<Self>) {
        let newest = self
            .activity
            .iter()
            .flat_map(|menu| &menu.feed)
            .map(|item| item.seq)
            .max();
        let settings = &mut self.config.mail;
        if let Some(newest) = newest.filter(|&n| n > settings.activity_cleared) {
            settings.activity_cleared = newest;
            settings.activity_removed.retain(|&seq| seq > newest);
            self.save_config();
        }
        if let Some(menu) = &mut self.activity {
            menu.feed.clear();
        }
        cx.notify();
    }

    /// Opens the sent message an open or click was about.
    fn open_activity_item(&mut self, item: &ActivityItem, cx: &mut Context<Self>) {
        let found = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.sent_copy(item.account, &item.message_id));
        match found {
            Some(message) => {
                self.activity = None;
                self.open_entry_in_window(Entry::message(message), cx);
            }
            None => self.show_snackbar(tr!("activity-message-gone"), None, cx),
        }
        cx.notify();
    }

    fn open_report(&mut self, cx: &mut Context<Self>) {
        self.activity = None;
        let date = |cx: &mut Context<Self>| {
            cx.new(|cx| TextInput::new(tr!("search-dates-placeholder"), cx))
        };
        let today = jiff::Zoned::now().date();
        self.activity_report = Some(Report {
            period: Period::Month,
            list: Vec::new(),
            bars: Vec::new(),
            step: 1,
            first: today,
            last: today,
            dates: [date(cx), date(cx)],
            editing: false,
            error: false,
            insights: None,
            counting: None,
            accounts_open: false,
            accounts_arrow: crate::widgets::Fold::default(),
        });
        self.fill_report(Period::Month);
        self.count_insights(cx);
        cx.notify();
    }

    /// The account the report counts, or `None` for all of them. A
    /// remembered account that is gone counts all.
    fn report_account(&self) -> Option<&Account> {
        let chosen = &self.config.mail.activity_account;
        if chosen.is_empty() {
            return None;
        }
        self.accounts
            .iter()
            .find(|a| a.address.trim().eq_ignore_ascii_case(chosen))
    }

    /// Counts the report's figures for `period`.
    fn fill_report(&mut self, period: Period) {
        let Ok(mail) = &self.mail else { return };
        let tz = &self.tz;
        let account = self.report_account().map(|a| a.id);
        let ours = |id: AccountId| account.is_none_or(|a| a == id);
        let mut all = mail.tracked(LIMIT);
        all.retain(|m| ours(m.account));
        let day_of = |unix: i64| {
            jiff::Timestamp::from_second(unix)
                .ok()
                .map(|at| at.to_zoned(tz.clone()).date())
        };
        let today = jiff::Zoned::now().with_time_zone(tz.clone()).date();
        let oldest = all
            .iter()
            .filter_map(|m| m.sent_at.and_then(day_of))
            .min()
            .unwrap_or(today);
        let (first, last) = period.days(today, oldest);
        let within = |day: Option<Date>| day.is_some_and(|d| d >= first && d <= last);
        let list: Vec<MessageActivity> = all
            .into_iter()
            .filter(|m| within(m.sent_at.and_then(day_of)))
            .collect();
        let since = first
            .to_zoned(tz.clone())
            .map_or(0, |z| z.timestamp().as_millisecond());
        let events: Vec<(Date, bool)> = mail
            .activity_feed(since, 0, EVENTS)
            .into_iter()
            .filter(|item| !item.maybe && ours(item.account))
            .filter_map(|item| Some((day_of(item.at.div_euclid(1000))?, item.click)))
            .collect();
        let (bars, step) = bars(&events, first, last);
        if let Some(report) = &mut self.activity_report {
            report.period = period;
            report.list = by_open_rate(list);
            report.bars = bars;
            report.step = step;
            report.first = first;
            report.last = last;
        }
    }

    fn pick_period(&mut self, period: Period, cx: &mut Context<Self>) {
        if let Some(report) = &mut self.activity_report {
            report.editing = false;
            report.error = false;
        }
        self.fill_report(period);
        self.count_insights(cx);
        cx.notify();
    }

    /// Counts the mailbox insights for the report's period on a
    /// background thread.
    fn count_insights(&mut self, cx: &mut Context<Self>) {
        let account = self.report_account().map(|a| a.id);
        let Some(report) = &mut self.activity_report else {
            return;
        };
        let tz = self.tz.clone();
        let start = |day: Date| {
            day.to_zoned(tz.clone())
                .map_or(0, |z| z.timestamp().as_second())
        };
        let since = if report.period == Period::All {
            0
        } else {
            start(report.first)
        };
        let until = report.last.tomorrow().map_or(i64::MAX, start);
        let me: Vec<String> = self
            .accounts
            .iter()
            .map(|a| a.address.trim().to_lowercase())
            .collect();
        let paths = self.paths.clone();
        report.insights = None;
        report.counting = Some(cx.spawn(async move |this, cx| {
            let insights = cx
                .background_executor()
                .spawn(
                    async move { crate::data::insights(&paths, account, &me, since, until, &tz) },
                )
                .await;
            this.update(cx, |this, cx| {
                if let Some(report) = &mut this.activity_report {
                    report.insights = Some(insights);
                    report.counting = None;
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn edit_custom(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(report) = &mut self.activity_report else {
            return;
        };
        report.editing = true;
        report.error = false;
        let (first, last) = (report.first, report.last);
        for (input, day) in report.dates.clone().iter().zip([first, last]) {
            if input.read(cx).text().trim().is_empty() {
                input.update(cx, |input, cx| input.set_text(day.to_string(), cx));
            }
        }
        let focus = report.dates[0].focus_handle(cx);
        window.focus(&focus, cx);
        cx.notify();
    }

    fn apply_custom(&mut self, cx: &mut Context<Self>) {
        let Some(report) = &mut self.activity_report else {
            return;
        };
        let [a, b] = [0, 1].map(|ix| parse_day(report.dates[ix].read(cx).text()));
        report.error = a.is_none() || b.is_none();
        if let (Some(a), Some(b)) = (a, b) {
            self.fill_report(Period::Custom(a, b));
            self.count_insights(cx);
        }
        cx.notify();
    }

    fn toggle_report_accounts(&mut self, cx: &mut Context<Self>) {
        if let Some(report) = &mut self.activity_report {
            report.accounts_open = !report.accounts_open;
        }
        cx.notify();
    }

    /// Counts the report for the account with `address` (lower case), or
    /// for all accounts when it is empty, and remembers the choice.
    fn pick_report_account(&mut self, address: String, cx: &mut Context<Self>) {
        let Some(report) = &mut self.activity_report else {
            return;
        };
        report.accounts_open = false;
        let period = report.period;
        if self.config.mail.activity_account != address {
            self.config.mail.activity_account = address;
            self.save_config();
            self.fill_report(period);
            self.count_insights(cx);
        }
        cx.notify();
    }

    fn close_report(&mut self, cx: &mut Context<Self>) {
        self.activity_report = None;
        cx.notify();
    }

    /// Closes the list or the report; `true` if one was open.
    pub(super) fn dismiss_activity(&mut self, cx: &mut Context<Self>) -> bool {
        if let Some(report) = &mut self.activity_report
            && report.accounts_open
        {
            report.accounts_open = false;
            cx.notify();
            return true;
        }
        let closed = self.activity.take().is_some() || self.activity_report.take().is_some();
        if closed {
            cx.notify();
        }
        closed
    }

    /// The Activity button beside the search box, with the number of new
    /// opens and clicks.
    pub(super) fn render_activity_button(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.activity_shown() {
            return None;
        }
        let open = self.activity.is_some();
        let unseen = self.activity_unseen;
        let anchor = self.activity_button.clone();
        Some(
            div()
                .flex_none()
                .relative()
                .child(
                    icon_button_colored(
                        "activity-button",
                        "pulse",
                        24.0,
                        if open { th.accent } else { th.text_dim },
                        th,
                    )
                    .when(open, |d| d.bg(rgba(th.hover)))
                    .tooltip(tip(tr!("folder-activity"), th))
                    .on_click(cx.listener(|this, _, window, cx| this.toggle_activity(window, cx))),
                )
                .when(unseen > 0, |d| {
                    let text = if unseen > 99 {
                        "99+".to_owned()
                    } else {
                        format::thousands(unseen as u64)
                    };
                    d.child(
                        div()
                            .absolute()
                            .top(px(2.0))
                            .right(px(0.0))
                            .min_w(px(18.0))
                            .h(px(18.0))
                            .px(px(5.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .bg(rgba(th.accent))
                            .text_color(rgba(th.on_accent))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .child(text),
                    )
                })
                .child(
                    canvas(
                        move |bounds, _, _| anchor.set(Some(bounds)),
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element(),
        )
    }

    /// One open or click, as a line of the list.
    fn render_item(
        &self,
        ix: usize,
        item: &ActivityItem,
        new: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let who = item
            .name
            .clone()
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| item.email.clone());
        let subject = if item.subject.trim().is_empty() {
            tr!("activity-no-subject")
        } else {
            item.subject.clone()
        };
        let text = if item.click {
            tr!("activity-feed-clicked", who = who, subject = subject)
        } else if item.maybe {
            tr!("activity-feed-maybe", who = who, subject = subject)
        } else {
            tr!("activity-feed-opened", who = who, subject = subject)
        };
        let now = jiff::Timestamp::now().as_second();
        let when = format::local(item.at.div_euclid(1000), &self.tz)
            .zip(format::local(now, &self.tz))
            .map(|(at, now)| format::list_date(at, now))
            .unwrap_or_default();
        let detail = match &item.link {
            Some(link) => format!("{when} · {link}"),
            None => when,
        };
        let open = item.clone();
        let seq = item.seq;
        div()
            .id(("activity-item", ix))
            .group("activity-item")
            .pl(px(16.0))
            .pr(px(4.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .rounded(px(8.0))
            .cursor_pointer()
            .when(new, |d| d.bg(rgba(fade(th.accent, 0.08))))
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| this.open_activity_item(&open, cx)))
            .child(
                div()
                    .flex_none()
                    .size(px(32.0))
                    .rounded_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgba(fade(th.accent, if item.maybe { 0.06 } else { 0.14 })))
                    .child(icon(
                        if item.click { "link" } else { "eye" },
                        if item.maybe { th.text_faint } else { th.accent },
                        18.0,
                    )),
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
                            .text_size(px(14.0))
                            .line_height(px(20.0))
                            .when(new, |d| d.font_weight(FontWeight::MEDIUM))
                            .child(text),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(detail),
                    ),
            )
            .child(
                // Shows on the row under the pointer.
                div()
                    .flex_none()
                    .invisible()
                    .group_hover("activity-item", |s| s.visible())
                    .child(
                        icon_button(("activity-remove", ix), "close", 18.0, th)
                            .size(px(32.0))
                            .tooltip(tip(tr!("activity-remove"), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.remove_activity_item(seq, cx);
                            })),
                    ),
            )
            .into_any_element()
    }

    /// The list under the Activity button.
    pub(super) fn render_activity_menu(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.activity.as_ref()?;
        let button = self.activity_button.get()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let width = MENU_WIDTH.min(vw - 16.0);
        // Under the button, its right edge on the button's, inside the
        // window.
        let right = unpx(button.origin.x + button.size.width);
        let left = (right - width).clamp(8.0, (vw - width - 8.0).max(8.0));
        let top = unpx(button.origin.y + button.size.height) + 6.0;
        let height = (vh - top - 16.0).clamp(160.0, 560.0);
        let items: Vec<AnyElement> = menu
            .feed
            .iter()
            .enumerate()
            .map(|(ix, item)| self.render_item(ix, item, item.seq > menu.seen, th, cx))
            .collect();
        let checking = self.katna_checking();
        let empty = items.is_empty().then(|| {
            if checking {
                return div()
                    .px(px(24.0))
                    .py(px(48.0))
                    .flex()
                    .justify_center()
                    .child(self.katna_checking_bar("activity-checking", th));
            }
            div()
                .px(px(24.0))
                .py(px(32.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(12.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(icon("activity", th.text_faint, 40.0))
                .child(div().text_center().child(tr!("activity-feed-empty")))
        });
        let card = div()
            .id("activity-menu")
            .occlude()
            .absolute()
            .left(px(left))
            .top(px(top))
            .w(px(width))
            .max_h(px(height))
            .flex()
            .flex_col()
            .overflow_hidden()
            .map(|d| raised(d, th, super::PANEL_RADIUS, 2.0))
            .text_color(rgba(th.text))
            .child(
                div()
                    .flex_none()
                    .h(px(56.0))
                    .pl(px(20.0))
                    .pr(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .border_b_1()
                    .border_color(rgba(th.divider))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(tr!("folder-activity")),
                    )
                    .when(!menu.feed.is_empty(), |d| {
                        d.child(
                            div()
                                .id("activity-clear")
                                .h(px(32.0))
                                .px(px(14.0))
                                .flex()
                                .items_center()
                                .rounded_full()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.accent))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(|this, _, _, cx| this.clear_activity(cx)))
                                .child(tr!("activity-clear-all")),
                        )
                    })
                    .child(
                        div()
                            .id("activity-details")
                            .h(px(32.0))
                            .px(px(14.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .rounded_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(|this, _, _, cx| this.open_report(cx)))
                            .child(icon("activity", th.accent, 18.0))
                            .child(tr!("activity-details")),
                    ),
            )
            // New opens and clicks arrive only while signed in.
            .when(!checking && !self.katna_signed_in(), |d| {
                d.child(
                    div()
                        .flex_none()
                        .px(px(16.0))
                        .pt(px(12.0))
                        .child(self.katna_sign_in_needed(th, cx)),
                )
            })
            .child(
                div()
                    .id("activity-feed")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p(px(8.0))
                    .child(
                        div()
                            .flex_none()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .children(items)
                            .children(empty),
                    ),
            )
            .with_animation(
                "activity-menu",
                Animation::new(katna_ui::motion::time(Duration::from_millis(180)))
                    .with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-8.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.activity = None;
                cx.notify();
            })
        };
        let layer = div()
            .relative()
            .w(px(vw))
            .h(px(vh))
            .child(
                div()
                    .id("activity-menu-scrim")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .occlude()
                    .on_mouse_down(MouseButton::Left, close())
                    .on_mouse_down(MouseButton::Right, close()),
            )
            .child(card);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(2)
                .into_any_element(),
        )
    }

    /// The Details report.
    pub(super) fn render_activity_report(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let report = self.activity_report.as_ref()?;
        let list = &report.list;
        let viewport = window.viewport_size();
        let height = (unpx(viewport.height) - 96.0).clamp(320.0, 760.0);
        let rates = Rates::of(list);
        let stat = |value: String, label: String| {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(div().text_size(px(28.0)).line_height(px(36.0)).child(value))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
        };
        let rate = |part: usize, whole: usize| {
            tr!(
                "activity-percent",
                percent = format::thousands(percent(part, whole))
            )
        };
        let chip = |id: usize, label: String, on: bool| {
            crate::widgets::choice_chip(("activity-period", id), label.clone(), on, th)
        };
        let custom = matches!(report.period, Period::Custom(..)) || report.editing;
        let periods = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(6.0))
            .children(
                [Period::Week, Period::Month, Period::All]
                    .into_iter()
                    .enumerate()
                    .map(|(ix, period)| {
                        chip(ix, period.label(), !custom && report.period == period).on_click(
                            cx.listener(move |this, _, _, cx| this.pick_period(period, cx)),
                        )
                    }),
            )
            .child(
                chip(3, tr!("activity-range-custom"), custom)
                    .on_click(cx.listener(|this, _, window, cx| this.edit_custom(window, cx))),
            );
        let field = |ix: usize, label: String| {
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    div()
                        .h(px(36.0))
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(rgba(if report.error { th.error } else { th.outline }))
                        .text_size(px(14.0))
                        .child(report.dates[ix].clone()),
                )
        };
        let dates = report.editing.then(|| {
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_end()
                        .gap(px(12.0))
                        .child(field(0, tr!("activity-range-from")))
                        .child(field(1, tr!("activity-range-to")))
                        .child(
                            crate::widgets::filled_button(
                                "activity-apply",
                                tr!("activity-range-apply"),
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| this.apply_custom(cx))),
                        ),
                )
                .when(report.error, |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.error))
                            .child(self.copyable(tr!("search-dates-unreadable"), th)),
                    )
                })
        });
        let periods = div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .child(div().flex_1().min_w_0().child(periods))
            .child(self.render_report_accounts(report, th, cx));
        let days = if report.first == report.last {
            day_label(report.first)
        } else {
            format!("{} – {}", day_label(report.first), day_label(report.last))
        };
        let span_label = tr!(
            "activity-range-of",
            days = days,
            account = self.report_account().map_or_else(
                || tr!("activity-accounts-all"),
                |a| a.address.trim().to_owned()
            )
        );
        let summary = div()
            .flex()
            .flex_row()
            .gap(px(16.0))
            .child(stat(
                format::thousands(rates.messages as u64),
                tr!("activity-messages"),
            ))
            .child(stat(
                rate(rates.opened, rates.recipients),
                tr!("activity-open-rate"),
            ))
            .child(stat(
                rate(rates.clicked, rates.recipients),
                tr!("activity-click-rate"),
            ));
        let chart = self.render_chart(report, th);
        let bar = |part: usize, whole: usize, color: u32| {
            let share = percent(part, whole) as f32 / 100.0;
            div()
                .h(px(6.0))
                .w_full()
                .rounded(px(3.0))
                .bg(rgba(fade(color, 0.16)))
                .child(
                    div()
                        .h_full()
                        .w(gpui::relative(share))
                        .rounded(px(3.0))
                        .bg(rgba(color)),
                )
        };
        let now = jiff::Timestamp::now().as_second();
        let rows = list.iter().enumerate().map(|(ix, message)| {
            let whole = message.recipients.len();
            let subject = if message.subject.trim().is_empty() {
                tr!("activity-no-subject")
            } else {
                message.subject.clone()
            };
            let when = message
                .sent_at
                .and_then(|at| format::local(at, &self.tz))
                .zip(format::local(now, &self.tz))
                .map(|(at, now)| format::list_date(at, now))
                .unwrap_or_default();
            div()
                .id(("activity", ix))
                .flex_none()
                .py(px(12.0))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(subject),
                        )
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(when),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(16.0))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(tr!(
                                    "activity-opened",
                                    opened = message.opened(),
                                    recipients = whole
                                ))
                                .child(bar(message.opened(), whole, th.accent)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(tr!(
                                    "activity-clicked",
                                    clicked = message.clicked(),
                                    recipients = whole
                                ))
                                .child(bar(message.clicked(), whole, th.accent)),
                        ),
                )
        });
        let heading = |text: String| {
            div()
                .pt(px(20.0))
                .pb(px(4.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text)
        };
        let empty = list.is_empty().then(|| {
            div()
                .py(px(32.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(12.0))
                .text_color(rgba(th.text_dim))
                .text_size(px(14.0))
                .child(icon("activity", th.text_faint, 48.0))
                .child(tr!("activity-nothing-period"))
        });
        let close = cx.listener(|this, _, _, cx| this.close_report(cx));
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                // Inside the room below the top bar with even margins; the
                // report scrolls when the window is too short for it. No
                // veil: the window stays as it is around the dialog.
                .p(px(24.0))
                .child(
                    div()
                        .id("activity-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(close),
                )
                .child(
                    div()
                        .id("activity-dialog")
                        .occlude()
                        .w(px(680.0_f32.min(unpx(viewport.width) - 32.0)))
                        .max_w_full()
                        .h(px(height))
                        .max_h_full()
                        .flex()
                        .flex_col()
                        .map(|d| crate::widgets::dialog(d, th, th.surface))
                        .child(
                            div()
                                .flex_none()
                                .h(px(64.0))
                                .pl(px(24.0))
                                .pr(px(12.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .border_b_1()
                                .border_color(rgba(th.divider))
                                .child(
                                    div()
                                        .flex_1()
                                        .text_size(px(20.0))
                                        .child(tr!("activity-report")),
                                )
                                .child(
                                    icon_button("activity-close", "close", 20.0, th)
                                        .tooltip(tip(tr!("activity-close"), th))
                                        .on_click(
                                            cx.listener(|this, _, _, cx| this.close_report(cx)),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .id("activity-report")
                                .flex_1()
                                .min_h_0()
                                .overflow_y_scroll()
                                .child(
                                    div()
                                        .flex_none()
                                        .px(px(24.0))
                                        .pt(px(16.0))
                                        .pb(px(24.0))
                                        .flex()
                                        .flex_col()
                                        .gap(px(12.0))
                                        .child(periods)
                                        .children(dates)
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(rgba(th.text_dim))
                                                .child(span_label),
                                        )
                                        .map(|d| {
                                            if self.katna_checking() {
                                                d.child(self.katna_checking_bar(
                                                    "activity-report-checking",
                                                    th,
                                                ))
                                            } else if !self.katna_signed_in() {
                                                d.child(self.katna_sign_in_needed(th, cx))
                                            } else {
                                                d
                                            }
                                        })
                                        .child(summary)
                                        .child(heading(tr!("activity-by-day")))
                                        .child(chart)
                                        .when(!list.is_empty(), |d| {
                                            d.child(heading(tr!("activity-by-open-rate")))
                                        })
                                        .child(div().flex().flex_col().children(rows))
                                        .children(empty)
                                        .child(self.render_insights(report, th)),
                                ),
                        ),
                )
                .into_any_element(),
        )
    }

    /// The account picker beside the periods: All accounts, or one.
    fn render_report_accounts(
        &self,
        report: &Report,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let mail: Vec<&Account> = self.accounts.iter().filter(|a| a.kind.is_mail()).collect();
        let chosen = self.report_account();
        let name = |a: &Account| {
            if a.display_name.trim().is_empty() {
                a.address.trim().to_owned()
            } else {
                a.display_name.trim().to_owned()
            }
        };
        let label = chosen.map_or_else(|| tr!("activity-accounts-all"), name);
        let button = div()
            .id("activity-accounts")
            .h(px(28.0))
            .max_w(px(240.0))
            .pl(px(if chosen.is_some() { 4.0 } else { 10.0 }))
            .pr(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.outline))
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .text_size(px(13.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .when(!report.accounts_open, |d| {
                d.tooltip(tip(tr!("activity-accounts-tip"), th))
            })
            .when_some(chosen, |d, a| {
                d.child(self.person_avatar(&name(a), a.address.trim(), 20.0))
            })
            .child(div().min_w_0().truncate().child(label))
            .child(crate::widgets::fold_arrow(
                "activity-accounts-arrow",
                &report.accounts_arrow,
                report.accounts_open,
                th.text_dim,
                16.0,
            ))
            .on_click(cx.listener(|this, _, _, cx| this.toggle_report_accounts(cx)));
        let check = |on: bool| {
            div()
                .flex_none()
                .size(px(18.0))
                .when(on, |d| d.child(icon("check", th.accent, 18.0)))
        };
        let item = |id: usize| {
            div()
                .id(("activity-account", id))
                .min_h(px(40.0))
                .py(px(4.0))
                .pl(px(12.0))
                .pr(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
        };
        let all = item(0)
            .child(check(chosen.is_none()))
            .child(
                div()
                    .size(px(28.0))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon("people", th.text_dim, 20.0)),
            )
            .child(div().flex_1().child(tr!("activity-accounts-all")))
            .on_click(cx.listener(|this, _, _, cx| {
                this.pick_report_account(String::new(), cx);
            }));
        let accounts = mail.iter().enumerate().map(|(ix, a)| {
            let address = a.address.trim().to_lowercase();
            let on = chosen.is_some_and(|c| c.id == a.id);
            let shown = name(a);
            let second = (shown != a.address.trim()).then(|| {
                div()
                    .truncate()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(a.address.trim().to_owned())
            });
            item(ix + 1)
                .child(check(on))
                .child(self.person_avatar(&shown, a.address.trim(), 28.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(div().truncate().child(shown))
                        .children(second),
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.pick_report_account(address.clone(), cx);
                }))
        });
        let menu = report.accounts_open.then(|| {
            let list = crate::widgets::menu(th)
                .id("activity-accounts-menu")
                .w(px(300.0))
                .max_h(px(360.0))
                .overflow_y_scroll()
                .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                    if let Some(report) = &mut this.activity_report {
                        report.accounts_open = false;
                    }
                    cx.notify();
                }))
                .child(all)
                .child(div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider)))
                .children(accounts);
            deferred(
                div().absolute().bottom_0().right_0().child(
                    anchored()
                        .anchor(gpui::Anchor::TopRight)
                        .offset(point(px(0.0), px(6.0)))
                        .snap_to_window_with_margin(px(8.0))
                        .child(div().occlude().child(list)),
                ),
            )
            .with_priority(3)
        });
        div()
            .relative()
            .flex_none()
            .child(button)
            .children(menu)
            .into_any_element()
    }

    /// What the mailbox did in the period: mail sent and received, replies,
    /// the people most written with, and when mail arrives.
    fn render_insights(&self, report: &Report, th: &Theme) -> AnyElement {
        let heading = |text: String| {
            div()
                .pt(px(20.0))
                .pb(px(4.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text)
        };
        let section = div()
            .mt(px(12.0))
            .pt(px(8.0))
            .border_t_1()
            .border_color(rgba(th.divider))
            .flex()
            .flex_col()
            .child(
                div()
                    .pt(px(12.0))
                    .text_size(px(18.0))
                    .child(tr!("insights-heading")),
            );
        let insights = match &report.insights {
            None => {
                return section
                    .child(
                        div()
                            .py(px(16.0))
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("insights-counting")),
                    )
                    .into_any_element();
            }
            Some(Err(err)) => {
                tracing::warn!("{err}");
                return section
                    .child(
                        div()
                            .py(px(16.0))
                            .text_size(px(14.0))
                            .text_color(rgba(th.error))
                            .child(self.copyable(tr!("insights-failed"), th)),
                    )
                    .into_any_element();
            }
            Some(Ok(insights)) => insights,
        };
        let stat = |value: String, label: String| {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(div().text_size(px(28.0)).line_height(px(36.0)).child(value))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
        };
        let counts = div()
            .pt(px(12.0))
            .flex()
            .flex_row()
            .gap(px(16.0))
            .child(stat(
                format::thousands(insights.sent as u64),
                tr!("insights-sent"),
            ))
            .child(stat(
                format::thousands(insights.received as u64),
                tr!("insights-received"),
            ))
            .child(div().flex_1());
        let reply_line = |share: String, replies: &katna_store::Replies| {
            let median = replies
                .median
                .map(|secs| tr!("insights-median", time = duration(secs)));
            div()
                .py(px(6.0))
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(div().text_size(px(14.0)).child(share))
                .children(median.map(|m| {
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(m)
                }))
                .child(
                    div()
                        .h(px(6.0))
                        .w_full()
                        .rounded(px(3.0))
                        .bg(rgba(fade(th.accent, 0.16)))
                        .child(
                            div()
                                .h_full()
                                .w(gpui::relative(
                                    percent(replies.replied, replies.messages) as f32 / 100.0,
                                ))
                                .rounded(px(3.0))
                                .bg(rgba(th.accent)),
                        ),
                )
        };
        let most = insights
            .people
            .iter()
            .map(|p| p.sent + p.received)
            .max()
            .unwrap_or(1)
            .max(1);
        let people = insights.people.iter().map(|person| {
            let name = person.name.clone().unwrap_or_else(|| person.email.clone());
            let share = (person.sent + person.received) as f32 / most as f32;
            div()
                .py(px(6.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .child(self.person_avatar(&name, &person.email, 28.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(6.0))
                                        .text_size(px(14.0))
                                        .child(div().min_w_0().truncate().child(name))
                                        .children(self.muted_mark(&person.email, 16.0, th)),
                                )
                                .child(
                                    div()
                                        .flex_none()
                                        .text_size(px(12.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(tr!(
                                            "insights-person-counts",
                                            sent = person.sent,
                                            received = person.received
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .h(px(4.0))
                                .w(gpui::relative(share))
                                .rounded(px(2.0))
                                .bg(rgba(fade(th.accent, 0.7))),
                        ),
                )
        });
        section
            .child(counts)
            .child(heading(tr!("insights-replies")))
            .child(reply_line(
                tr!(
                    "insights-you-replied",
                    percent =
                        format::thousands(percent(insights.mine.replied, insights.mine.messages)),
                    replied = insights.mine.replied,
                    messages = insights.mine.messages
                ),
                &insights.mine,
            ))
            .child(reply_line(
                tr!(
                    "insights-they-replied",
                    percent = format::thousands(percent(
                        insights.theirs.replied,
                        insights.theirs.messages
                    )),
                    replied = insights.theirs.replied,
                    messages = insights.theirs.messages
                ),
                &insights.theirs,
            ))
            .when(!insights.people.is_empty(), |d| {
                d.child(heading(tr!("insights-people")))
                    .child(div().flex().flex_col().children(people))
            })
            .child(heading(tr!("insights-hours-heading")))
            .child(hours_grid(&insights.hours, th))
            .into_any_element()
    }

    /// Opens and clicks as bars, one a day (or a week), with a key.
    fn render_chart(&self, report: &Report, th: &Theme) -> AnyElement {
        const HEIGHT: f32 = 120.0;
        let top = report
            .bars
            .iter()
            .map(|&(opens, clicks)| opens.max(clicks))
            .max()
            .unwrap_or(0)
            .max(1) as f32;
        let click_color = th.text;
        let bars = report.bars.iter().map(|&(opens, clicks)| {
            let tall = |n: u32| (n as f32 / top * HEIGHT).max(if n > 0 { 2.0 } else { 0.0 });
            div()
                .flex_1()
                .min_w_0()
                .h_full()
                .flex()
                .flex_row()
                .items_end()
                .justify_center()
                .gap(px(1.0))
                .child(
                    div()
                        .flex_1()
                        .max_w(px(10.0))
                        .h(px(tall(opens)))
                        .rounded_t(px(2.0))
                        .bg(rgba(th.accent)),
                )
                .child(
                    div()
                        .flex_1()
                        .max_w(px(10.0))
                        .h(px(tall(clicks)))
                        .rounded_t(px(2.0))
                        .bg(rgba(fade(click_color, 0.55))),
                )
        });
        let (opens, clicks) = report.bars.iter().fold((0u64, 0u64), |(o, c), &(bo, bc)| {
            (o + u64::from(bo), c + u64::from(bc))
        });
        let key = |color: u32, text: String| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(div().size(px(10.0)).rounded(px(2.0)).bg(rgba(color)))
                .child(text)
        };
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .h(px(HEIGHT))
                    .flex()
                    .flex_row()
                    .items_end()
                    .gap(px(2.0))
                    .border_b_1()
                    .border_color(rgba(th.divider))
                    .children(bars),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .text_size(px(11.0))
                    .text_color(rgba(th.text_dim))
                    .child(div().flex_1().child(day_label(report.first)))
                    .child(day_label(report.last)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(key(
                        th.accent,
                        tr!("activity-opens", count = format::thousands(opens)),
                    ))
                    .child(key(
                        fade(click_color, 0.55),
                        tr!("activity-clicks", count = format::thousands(clicks)),
                    ))
                    .when(report.step > 1, |d| {
                        d.child(div().child(tr!("activity-by-week")))
                    }),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use katna_store::RecipientActivity;

    use super::*;

    fn message(subject: &str, opened: usize, clicked: usize, of: usize) -> MessageActivity {
        MessageActivity {
            account: katna_core::AccountId(1),
            subject: subject.into(),
            sent_at: Some(0),
            links: 1,
            recipients: (0..of)
                .map(|i| RecipientActivity {
                    email: format!("r{i}@example.org"),
                    name: None,
                    opens: u32::from(i < opened),
                    maybe_opens: 0,
                    clicks: u32::from(i < clicked),
                    last: None,
                })
                .collect(),
        }
    }

    #[test]
    fn rates_count_recipients() {
        let list = [message("a", 1, 0, 2), message("b", 2, 1, 2)];
        assert_eq!(
            Rates::of(&list),
            Rates {
                messages: 2,
                recipients: 4,
                opened: 3,
                clicked: 1
            }
        );
        assert_eq!(percent(3, 4), 75);
        assert_eq!(percent(0, 0), 0);
    }

    #[test]
    fn best_subject_lines_first() {
        let list = vec![
            message("newest, unread", 0, 0, 3),
            message("half", 1, 0, 2),
            message("all, clicked", 2, 1, 2),
            message("all", 3, 0, 3),
        ];
        let subjects: Vec<String> = by_open_rate(list).into_iter().map(|m| m.subject).collect();
        assert_eq!(subjects, ["all, clicked", "all", "half", "newest, unread"]);
    }

    #[test]
    fn periods_end_today() {
        let today = Date::new(2026, 9, 28).unwrap();
        let first = Date::new(2026, 1, 5).unwrap();
        assert_eq!(
            Period::Week.days(today, first),
            (Date::new(2026, 9, 22).unwrap(), today)
        );
        assert_eq!(
            Period::Month.days(today, first),
            (Date::new(2026, 8, 30).unwrap(), today)
        );
        assert_eq!(Period::All.days(today, first), (first, today));
        // Dates typed the wrong way round still make a period.
        let (a, b) = (
            Date::new(2026, 9, 1).unwrap(),
            Date::new(2026, 8, 1).unwrap(),
        );
        assert_eq!(Period::Custom(a, b).days(today, first), (b, a));
        assert_eq!(parse_day(" 2026/9/1 "), Some(a));
        assert_eq!(parse_day("2026-13-01"), None);
    }

    #[test]
    fn counts_by_day_or_week() {
        let first = Date::new(2026, 9, 1).unwrap();
        let day = |d: i8| Date::new(2026, 9, d).unwrap();
        let items = [
            (day(1), false),
            (day(1), true),
            (day(3), false),
            (Date::new(2026, 8, 31).unwrap(), false),
        ];
        let (bars, step) = bars(&items, first, day(7));
        assert_eq!(step, 1);
        assert_eq!(bars.len(), 7);
        assert_eq!(bars[0], (1, 1));
        assert_eq!(bars[2], (1, 0));
        let last = Date::new(2026, 12, 31).unwrap();
        let (weeks, step) = super::bars(&[(day(8), false)], first, last);
        assert_eq!(step, 7);
        assert_eq!(weeks.len(), 18);
        assert_eq!(weeks[1], (1, 0));
    }
}
