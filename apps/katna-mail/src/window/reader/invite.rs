// SPDX-License-Identifier: GPL-3.0-or-later

//! The invitation card at the top of a mail that carries a calendar part
//! (`docs/ARCHITECTURE.md` §18): what the event is, Join, "Going? Yes No
//! Maybe" answered through the calendar the event is in, and the user's
//! day around it so a clash shows. An answer to one of the user's own
//! invitations, and a cancellation, get a card too.

use std::sync::{Arc, Mutex};

use gpui::{AnyElement, Context, FontWeight, SharedString, div, prelude::*, rgba};
use jiff::tz::TimeZone;
use katna_core::{AccountId, Paths};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::MessageId;
use katna_store::calendar::{Calendar, CalendarAccess, EditScope, EventData, EventStatus};
use katna_ui::px;

use super::super::MailWindow;
use super::super::calendar::{civil, event_color, read, time_range};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{icon, outlined_button};

/// How far around the event the user's day is shown (seconds).
const AROUND: i64 = 2 * 60 * 60;

/// What the calendar part asks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::window) enum Method {
    /// An invitation, or a changed one.
    Request,
    /// A guest's answer to the user's invitation.
    Reply,
    /// The organizer called it off.
    Cancel,
}

/// The calendar part of a message.
pub(in crate::window) struct Invite {
    pub method: Method,
    pub event: EventData,
    /// The calendar part as sent, for an answer by mail.
    text: String,
    /// The answer mailed to the organizer from here, while no calendar
    /// holds the invitation.
    mailed: Mutex<Option<&'static str>>,
    /// What the user's calendars hold for it, read when first shown.
    look: Mutex<Option<Look>>,
}

/// The event in the user's calendars, and their day around it.
struct Look {
    calendars: Vec<Calendar>,
    /// The event itself, once a calendar has it.
    found: Option<Occurrence>,
    /// The user's other events around it, soonest first.
    around: Vec<Occurrence>,
}

/// The invitation `raw` carries, if any.
pub(in crate::window) fn invite(raw: &[u8]) -> Option<Arc<Invite>> {
    let text = katna_render::calendar_part(raw)?;
    let components = katna_dav::ical::components(&text);
    let calendar = components.iter().find(|c| c.name == "VCALENDAR")?;
    let method = calendar
        .properties
        .iter()
        .find(|p| p.name == "METHOD")
        .map(|p| p.value.to_ascii_uppercase());
    let method = match method.as_deref() {
        Some("REPLY") => Method::Reply,
        Some("CANCEL") => Method::Cancel,
        Some("REQUEST") | None => Method::Request,
        // PUBLISH, COUNTER and the rest: an .ics like any other.
        Some(_) => return None,
    };
    let event = calendar
        .children
        .iter()
        .filter(|c| c.name == "VEVENT")
        .find_map(|c| katna_dav::ical::event(c, &TimeZone::UTC, ""))?;
    Some(Arc::new(Invite {
        method,
        event,
        text,
        mailed: Mutex::new(None),
        look: Mutex::new(None),
    }))
}

impl Invite {
    /// Reads what the user's calendars hold around the event, once.
    fn look(&self, paths: &Paths, tz: &TimeZone) {
        if self.look.lock().unwrap().is_some() {
            return;
        }
        let event = &self.event;
        let (from, to) = (event.start - AROUND, event.end.max(event.start) + AROUND);
        // Birthdays take no time.
        let look = match read(paths, from, to, tz, Some(false), &[]) {
            Ok((calendars, occurrences)) => {
                let (found, others): (Vec<_>, Vec<_>) = occurrences
                    .into_iter()
                    .partition(|o| o.event.data.uid == event.uid);
                let found = found
                    .iter()
                    .find(|o| o.start == event.start)
                    .or(found.first())
                    .cloned();
                let hidden = |o: &Occurrence| {
                    calendars
                        .iter()
                        .any(|c| c.id == o.event.calendar_id && c.hidden)
                };
                let around = others
                    .into_iter()
                    .filter(|o| !o.all_day() && !hidden(o))
                    .filter(|o| o.event.data.status != EventStatus::Cancelled)
                    .collect();
                Look {
                    calendars,
                    found,
                    around,
                }
            }
            Err(err) => {
                tracing::debug!(%err, "no calendar for the invitation");
                Look {
                    calendars: Vec::new(),
                    found: None,
                    around: Vec::new(),
                }
            }
        };
        *self.look.lock().unwrap() = Some(look);
    }

    /// Reads the calendars again when next shown.
    pub(in crate::window) fn forget(&self) {
        self.look.lock().unwrap().take();
    }
}

/// "Going?" before the answers.
fn going_row(th: &Theme) -> gpui::Div {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(8.0))
        .child(
            div()
                .mr(px(8.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("calendar-going")),
        )
}

/// One answer, Yes, No or Maybe; `on` when it is the user's.
fn answer_chip(
    id: (&'static str, usize, &'static str),
    label: String,
    on: bool,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(SharedString::from(format!("{}-{}-{}", id.0, id.2, id.1)))
        .h(px(32.0))
        .px(px(16.0))
        .flex()
        .items_center()
        .rounded_full()
        .border_1()
        .border_color(rgba(if on { th.accent } else { th.outline }))
        .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(if on { th.accent } else { th.text }))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label)
}

/// The person an answer came from: their name, else their address.
fn who(event: &EventData) -> String {
    event
        .attendees
        .iter()
        .find(|a| !a.organizer)
        .or(event.attendees.first())
        .map(|a| {
            if a.name.is_empty() {
                a.email.clone()
            } else {
                a.name.clone()
            }
        })
        .unwrap_or_default()
}

impl MailWindow {
    /// Makes invitation cards read the calendars again.
    pub(in crate::window) fn forget_invite_looks(&self) {
        let Some(reader) = self.reader.as_ref() else {
            return;
        };
        for invite in reader.invites() {
            invite.forget();
        }
    }

    /// The address of the user's that `invite` was sent to, and the
    /// account message `id` came to: the answer goes from there.
    fn invited_as(&self, id: MessageId, invite: &Invite) -> Option<(AccountId, String)> {
        let account = self.mail.as_ref().ok()?.message_account(id)?;
        let address = &self.accounts.iter().find(|a| a.id == account)?.address;
        let attendees = &invite.event.attendees;
        let invited = attendees
            .iter()
            .find(|a| a.email.eq_ignore_ascii_case(address))
            .or_else(|| {
                attendees.iter().find(|a| {
                    self.accounts
                        .iter()
                        .any(|me| me.address.eq_ignore_ascii_case(&a.email))
                })
            })?;
        Some((account, invited.email.clone()))
    }

    /// Answers `invite` by mail to its organizer (iMIP, RFC 6047), for
    /// an invitation no calendar of the user's holds: from the account the
    /// invitation came to, as a reply to it, with the answer in a calendar
    /// part the organizer's calendar reads.
    fn answer_invite_by_mail(
        &mut self,
        id: MessageId,
        invite: Arc<Invite>,
        status: &'static str,
        cx: &mut Context<Self>,
    ) {
        let Some((account, address)) = self.invited_as(id, &invite) else {
            return;
        };
        let event = &invite.event;
        let stamp = jiff::Timestamp::now().as_second();
        let Some(ics) = katna_dav::ical::reply_calendar(&invite.text, &address, status, stamp)
        else {
            return;
        };
        let name = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.display_name.trim().to_owned())
            .filter(|n| !n.is_empty());
        let title = if event.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            event.title.clone()
        };
        let who = name.clone().unwrap_or_else(|| address.clone());
        let (subject, body) = match status {
            "accepted" => (
                tr!("calendar-mail-yes", title = title),
                tr!("calendar-mail-yes-body", name = who),
            ),
            "declined" => (
                tr!("calendar-mail-no", title = title),
                tr!("calendar-mail-no-body", name = who),
            ),
            _ => (
                tr!("calendar-mail-maybe", title = title),
                tr!("calendar-mail-maybe-body", name = who),
            ),
        };
        let header = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.message_id_header(id))
            .map(|h| h.trim_matches(['<', '>']).to_owned());
        let raw = crate::outgoing::build(&crate::outgoing::Outgoing {
            from: Some(crate::outgoing::Mailbox {
                name,
                email: address,
            }),
            to: vec![crate::outgoing::Mailbox {
                name: Some(event.organizer_name.clone()).filter(|n| !n.is_empty()),
                email: event.organizer.clone(),
            }],
            subject,
            body,
            references: header.iter().cloned().collect(),
            in_reply_to: header,
            calendar: Some(("REPLY".to_owned(), ics)),
            ..Default::default()
        });
        let before = invite.mailed.lock().unwrap().replace(status);
        cx.notify();
        let connection = self.daemon.clone();
        let delay = self.config.sending.undo_send_seconds;
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => crate::daemon::connect().await?,
                    };
                    crate::daemon::queue_send(&connection, account.0, &raw, delay).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(outbox) => this.queued(outbox, delay, false, cx),
                Err(err) => {
                    *invite.mailed.lock().unwrap() = before;
                    this.show_snackbar(err, None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// The card of `invite`, in message `ix` (`id`) of the conversation.
    pub(super) fn invite_card(
        &self,
        ix: usize,
        id: MessageId,
        invite: &Arc<Invite>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        invite.look(&self.paths, &self.tz);
        let look = invite.look.lock().unwrap();
        let look = look.as_ref();
        let tz = &self.tz;
        let found = look.and_then(|l| l.found.as_ref());
        // What the calendar has is newer than the mail, once it has it.
        let event = found.map_or(&invite.event, |o| &o.event.data);
        let (start, end) = found.map_or((event.start, event.end), |o| (o.start, o.end));
        let begins = civil(start, tz);
        let when = if event.all_day {
            format::day_month_year(begins)
        } else {
            tr!(
                "calendar-when",
                day = format::day_month_year(begins),
                time = time_range(start, end, tz)
            )
        };
        let heading = match invite.method {
            Method::Request if event.status == EventStatus::Cancelled => {
                tr!("calendar-invite-cancelled")
            }
            Method::Request => tr!("calendar-invite"),
            Method::Cancel => tr!("calendar-invite-cancelled"),
            Method::Reply => {
                let name = who(&invite.event);
                match invite.event.attendees.first().map(|a| a.status.as_str()) {
                    Some("accepted") => tr!("calendar-invite-reply-yes", name = name),
                    Some("declined") => tr!("calendar-invite-reply-no", name = name),
                    Some("tentative") => tr!("calendar-invite-reply-maybe", name = name),
                    _ => tr!("calendar-invite-reply", name = name),
                }
            }
        };
        let title = if event.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            event.title.clone()
        };
        let organizer = if event.organizer_name.is_empty() {
            event.organizer.clone()
        } else {
            event.organizer_name.clone()
        };
        let color = match (look, found) {
            (Some(look), Some(found)) => event_color(&look.calendars, found),
            _ => th.accent,
        };
        let fact = |name: &'static str, text: String| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(12.0))
                .child(
                    div()
                        .flex_none()
                        .pt(px(1.0))
                        .child(icon(name, th.text_dim, 18.0)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_size(px(14.0))
                        .line_height(px(20.0))
                        .text_color(rgba(th.text))
                        .child(text),
                )
        };
        let guests = event.attendees.len();
        let mut facts = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(fact("schedule", when));
        if !event.location.is_empty() {
            facts = facts.child(fact("pin", event.location.clone()));
        }
        if !organizer.is_empty() {
            facts = facts.child(fact(
                "contacts",
                tr!("calendar-invite-organizer", name = organizer),
            ));
        }
        if guests > 0 {
            facts = facts.child(fact("people", tr!("calendar-guests", count = guests)));
        }

        // Going? Yes No Maybe: only once a calendar the user can change
        // holds the invitation.
        let answerable = invite.method == Method::Request
            && event.status != EventStatus::Cancelled
            && look.zip(found).is_some_and(|(look, found)| {
                look.calendars
                    .iter()
                    .any(|c| c.id == found.event.calendar_id && c.access != CalendarAccess::Reader)
                    && !found
                        .event
                        .data
                        .attendees
                        .iter()
                        .any(|a| a.is_self && a.organizer)
            });
        let answers = answerable.then(|| {
            let found = found.cloned().expect("answerable");
            let mine = found.event.data.self_status.clone();
            let answer = |key: &'static str, label: String, status: &'static str| {
                let on = mine == status;
                let occurrence = found.clone();
                answer_chip(("invite", ix, key), label, on, th).on_click(cx.listener(
                    move |this, _, _, cx| {
                        if occurrence.event.data.self_status != status {
                            // An invitation to a series is answered for all
                            // of it, as the organizer asked.
                            this.send_response(
                                occurrence.clone(),
                                status.to_owned(),
                                EditScope::All,
                                cx,
                            );
                        }
                    },
                ))
            };
            going_row(th)
                .child(answer("yes", tr!("calendar-answer-yes"), "accepted"))
                .child(answer("no", tr!("calendar-answer-no"), "declined"))
                .child(answer("maybe", tr!("calendar-answer-maybe"), "tentative"))
        });
        let waiting = invite.method == Method::Request
            && event.status != EventStatus::Cancelled
            && found.is_none();
        // It came to an account without calendars (Google, Outlook and
        // scheduling CalDAV servers put it in theirs): the answer goes to
        // the organizer by mail.
        let by_mail = waiting
            && !event.organizer.is_empty()
            && !self
                .accounts
                .iter()
                .any(|me| me.address.eq_ignore_ascii_case(&event.organizer))
            && self.invited_as(id, invite).is_some_and(|(account, _)| {
                !look.is_some_and(|l| l.calendars.iter().any(|c| c.account == Some(account)))
            });
        let answers = answers.or_else(|| {
            by_mail.then(|| {
                let mailed = *invite.mailed.lock().unwrap();
                let answer = |key: &'static str, label: String, status: &'static str| {
                    let on = mailed == Some(status);
                    let invite = invite.clone();
                    answer_chip(("invite-mail", ix, key), label, on, th).on_click(cx.listener(
                        move |this, _, _, cx| {
                            if *invite.mailed.lock().unwrap() != Some(status) {
                                this.answer_invite_by_mail(id, invite.clone(), status, cx);
                            }
                        },
                    ))
                };
                going_row(th)
                    .child(answer("yes", tr!("calendar-answer-yes"), "accepted"))
                    .child(answer("no", tr!("calendar-answer-no"), "declined"))
                    .child(answer("maybe", tr!("calendar-answer-maybe"), "tentative"))
            })
        });

        let join = (!event.join_url.is_empty() && event.status != EventStatus::Cancelled)
            .then(|| event.join_url.clone());
        // With the Calendar off, the card still answers by mail.
        let day = found
            .filter(|_| self.app_on(super::super::apps::App::Calendar))
            .map(|o| civil(o.start, tz).date());
        let actions = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(8.0))
            .children(join.map(|url| {
                outlined_button(("invite-join", ix), tr!("calendar-join"), th)
                    .on_click(move |_, _, cx| cx.open_url(&url))
            }))
            .children(day.map(|day| {
                outlined_button(("invite-open", ix), tr!("calendar-invite-open"), th).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.open_app(super::super::apps::App::Calendar, cx);
                        this.open_calendar_day(day, None, cx);
                    }),
                )
            }));

        let day_strip = look
            .filter(|l| !l.around.is_empty() && invite.method != Method::Reply)
            .map(|look| self.invite_day(invite, look, start, end, th));

        div()
            .id(("invite", ix))
            .mb(px(16.0))
            .p(px(16.0))
            .map(|d| crate::widgets::tile(d, th))
            .bg(rgba(mix(th.surface, color | 0xff, 0.05)))
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(icon("calendar", color, 18.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text_dim))
                            .child(heading),
                    ),
            )
            .child(
                div()
                    .text_size(px(18.0))
                    .line_height(px(24.0))
                    .text_color(rgba(th.text))
                    .when(
                        invite.method == Method::Cancel || event.status == EventStatus::Cancelled,
                        |d| d.line_through(),
                    )
                    .child(title),
            )
            .child(facts)
            .children(day_strip)
            .children(answers)
            .when(waiting, |d| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(if by_mail {
                            tr!("calendar-invite-by-mail")
                        } else {
                            tr!("calendar-invite-not-yet")
                        }),
                )
            })
            .child(actions)
            .into_any_element()
    }

    /// The user's day around the invitation: their events before, during
    /// and after it, the ones at the same time marked.
    fn invite_day(
        &self,
        invite: &Invite,
        look: &Look,
        start: i64,
        end: i64,
        th: &Theme,
    ) -> AnyElement {
        let tz = &self.tz;
        let clashes = |o: &Occurrence| {
            o.start < end
                && o.end > start
                && o.event.data.busy
                && o.event.data.self_status != "declined"
        };
        let clash_count = look.around.iter().filter(|o| clashes(o)).count();
        let row = |time: String, title: String, color: u32, strong: bool, clash: bool| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .child(
                    div()
                        .flex_none()
                        .w(px(4.0))
                        .h(px(18.0))
                        .rounded(px(2.0))
                        .bg(rgba(color)),
                )
                .child(
                    div()
                        .flex_none()
                        .w(px(140.0))
                        .whitespace_nowrap()
                        .text_color(rgba(if clash { th.error } else { th.text_dim }))
                        .child(time),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_color(rgba(th.text))
                        .when(strong, |d| d.font_weight(FontWeight::MEDIUM))
                        .child(title),
                )
        };
        let mut rows: Vec<(i64, AnyElement)> = look
            .around
            .iter()
            .map(|o| {
                let clash = clashes(o);
                let title = if o.event.data.title.is_empty() {
                    tr!("calendar-no-title")
                } else {
                    o.event.data.title.clone()
                };
                (
                    o.start,
                    row(
                        time_range(o.start, o.end, tz),
                        title,
                        event_color(&look.calendars, o),
                        false,
                        clash,
                    )
                    .into_any_element(),
                )
            })
            .collect();
        let title = if invite.event.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            invite.event.title.clone()
        };
        rows.push((
            start,
            row(time_range(start, end, tz), title, th.accent, true, false)
                .opacity(0.9)
                .into_any_element(),
        ));
        rows.sort_by_key(|(at, _)| *at);
        div()
            .p(px(12.0))
            .rounded(px(8.0))
            .bg(rgba(th.surface))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgba(if clash_count > 0 {
                        th.error
                    } else {
                        th.text_dim
                    }))
                    .child(if clash_count > 0 {
                        tr!("calendar-invite-clashes", count = clash_count)
                    } else {
                        tr!("calendar-invite-your-day")
                    }),
            )
            .children(rows.into_iter().map(|(_, row)| row))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW: &str = "From: priya@acme.example\r\nSubject: Invitation\r\nMIME-Version: 1.0\r\n\
Content-Type: text/calendar; method=REQUEST; charset=utf-8\r\n\r\n\
BEGIN:VCALENDAR\r\nMETHOD:REQUEST\r\nBEGIN:VEVENT\r\nUID:r@test\r\n\
DTSTART:20261001T050000Z\r\nDTEND:20261001T060000Z\r\nSUMMARY:Design review\r\n\
ORGANIZER;CN=Priya:mailto:priya@acme.example\r\n\
ATTENDEE;PARTSTAT=NEEDS-ACTION:mailto:me@example.org\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";

    #[test]
    fn reads_the_invitation_of_a_mail() {
        let invite = invite(RAW.as_bytes()).unwrap();
        assert_eq!(invite.method, Method::Request);
        assert_eq!(invite.event.uid, "r@test");
        assert_eq!(invite.event.title, "Design review");
        assert_eq!(invite.event.end - invite.event.start, 3600);
        let reply = RAW
            .replace("METHOD:REQUEST", "METHOD:REPLY")
            .replace("NEEDS-ACTION", "ACCEPTED");
        let invite = super::invite(reply.as_bytes()).unwrap();
        assert_eq!(invite.method, Method::Reply);
        assert_eq!(who(&invite.event), "me@example.org");
        let publish = RAW.replace("METHOD:REQUEST", "METHOD:PUBLISH");
        assert!(super::invite(publish.as_bytes()).is_none());
    }
}
