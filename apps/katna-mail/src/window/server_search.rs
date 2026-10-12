// SPDX-License-Identifier: GPL-3.0-or-later

//! "More results on server" (`docs/ARCHITECTURE.md` §7.3): under the
//! results found here, a section of mail found by asking the mail server,
//! for mail that is not downloaded and so not searchable here by its
//! words. It starts on Enter or a pause in typing, only for searches with
//! words in them and only while some mail is not downloaded. The server
//! answers with messages whose headers are here already, so their lines
//! show at once; opening one downloads it.

use std::collections::HashSet;
use std::time::Duration;

use gpui::{AnyElement, Context, FontWeight, SharedString, Task, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_search::{Query, TextField};
use katna_store::MessageId;
use katna_ui::tokens::{space, text};
use katna_ui::{WindowDrag, px};

use super::{Listing, MailWindow};
use crate::daemon;
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::spinner;

/// How long typing pauses before the server is asked.
const PAUSE: Duration = Duration::from_millis(900);

/// The search on the server for the query shown.
pub(super) struct ServerSearch {
    query: String,
    state: State,
    /// What the server found, newest first.
    found: Vec<MessageId>,
    /// Where the server's lines start in the list, when it found any the
    /// list does not show already.
    pub from: Option<usize>,
    task: Option<Task<()>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    /// Waiting for typing to pause (or Enter).
    Waiting,
    Searching,
    Done,
    Failed(String),
}

impl ServerSearch {
    /// The line under the results, if one shows: what the search on the
    /// server is doing, when it adds nothing to the list yet.
    fn status(&self) -> Option<Status> {
        match &self.state {
            State::Waiting => None,
            State::Searching => Some(Status::Searching),
            State::Done if self.from.is_none() => Some(Status::NothingMore),
            State::Done => None,
            State::Failed(_) => Some(Status::Failed),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Searching,
    NothingMore,
    Failed,
}

/// Whether `query` has words to look for in the messages' text, which is
/// what only the server can search in mail not downloaded.
fn has_words(query: &Query) -> bool {
    match query {
        Query::Text { field, .. } | Query::Prefix { field, .. } => *field == TextField::Any,
        Query::And(items) | Query::Or(items) => items.iter().any(has_words),
        Query::All | Query::Not(_) | Query::Filter(_) => false,
    }
}

impl MailWindow {
    /// After the results found here for `query` are listed: keeps the
    /// server's lines of the same query under them, or starts a new search
    /// on the server once typing pauses.
    pub(super) fn after_local_results(&mut self, query: &str, cx: &mut Context<Self>) {
        if self
            .server_search
            .as_ref()
            .is_some_and(|server| server.query == query)
        {
            self.add_server_lines();
            return;
        }
        self.server_search = None;
        let now = jiff::Timestamp::now().as_second();
        let wanted = Query::parse_at(query, now).is_ok_and(|q| has_words(&q));
        let only = self.shown_account();
        let more = self
            .mail
            .as_ref()
            .is_ok_and(|mail| mail.has_mail_not_downloaded(only));
        if !wanted || !more {
            return;
        }
        let query = query.to_owned();
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(PAUSE).await;
            this.update(cx, |this, cx| this.search_server_now(cx)).ok();
        });
        self.server_search = Some(ServerSearch {
            query,
            state: State::Waiting,
            found: Vec::new(),
            from: None,
            task: Some(task),
        });
    }

    /// Asks the server now, if a search there is waiting (Enter, or
    /// typing paused).
    pub(super) fn search_server_now(&mut self, cx: &mut Context<Self>) {
        let account = self.shown_account().map_or(0, |a| a.0);
        let connection = self.daemon.clone();
        let Some(server) = &mut self.server_search else {
            return;
        };
        if !matches!(server.state, State::Waiting | State::Failed(_)) {
            return;
        }
        server.state = State::Searching;
        let query = server.query.clone();
        server.task = Some(cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::search_server(&connection, &query, account).await
                })
                .await;
            this.update(cx, |this, cx| this.server_answered(found, cx))
                .ok();
        }));
        self.list_state.remeasure();
        cx.notify();
    }

    fn server_answered(&mut self, found: Result<Vec<i64>, String>, cx: &mut Context<Self>) {
        let Some(server) = &mut self.server_search else {
            return;
        };
        server.task = None;
        match found {
            Ok(found) => {
                server.state = State::Done;
                server.found = found.into_iter().map(MessageId).collect();
            }
            Err(err) => {
                tracing::info!(%err, "searching the server");
                server.state = State::Failed(err);
            }
        }
        self.add_server_lines();
        self.list_state.remeasure();
        cx.notify();
    }

    /// Puts the server's lines under the ones found here, leaving out
    /// conversations the list shows already.
    fn add_server_lines(&mut self) {
        let only = self.shown_account();
        let Some(server) = &mut self.server_search else {
            return;
        };
        if !matches!(self.listing, Some(Listing::Search { .. })) {
            return;
        }
        server.from = None;
        let Ok(mail) = &self.mail else {
            return;
        };
        let shown: HashSet<EntryKey> = self.entries.iter().map(|e| e.key).collect();
        let lines: Vec<_> = mail
            .hit_entries(&server.found, self.config.mail.conversations, only)
            .into_iter()
            .filter(|entry| !shown.contains(&entry.key))
            .collect();
        if lines.is_empty() {
            self.reset_list(true);
            return;
        }
        server.from = Some(self.entries.len());
        self.entries.extend(lines);
        self.reset_list(true);
    }

    /// Ends the search on the server, with the search it belonged to.
    pub(super) fn drop_server_search(&mut self) {
        self.server_search = None;
    }

    /// The heading over line `ix` when the server's lines start there.
    pub(super) fn server_heading_at(&self, ix: usize, th: &Theme) -> Option<AnyElement> {
        let server = self.server_search.as_ref()?;
        if !matches!(self.listing, Some(Listing::Search { .. })) {
            return None;
        }
        (server.from == Some(ix)).then(|| heading(tr!("search-server-more"), ix == 0, th))
    }

    /// The line under line `ix` when it is the last and the server has
    /// something to say there.
    pub(super) fn server_status_after(
        &self,
        ix: usize,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if ix + 1 != self.entries.len() {
            return None;
        }
        self.server_status(th, cx)
    }

    /// What the search on the server is doing, for under the results or
    /// in place of an empty list.
    pub(super) fn server_status(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !matches!(self.listing, Some(Listing::Search { .. })) {
            return None;
        }
        let status = self.server_search.as_ref()?.status()?;
        let line = div()
            .px(px(space::S5))
            .py(px(space::S4))
            .flex()
            .items_center()
            .gap(px(space::S2))
            .text_size(px(text::CAPTION))
            .text_color(rgba(th.text_dim));
        Some(match status {
            Status::Searching => line
                .child(spinner("server-search-spinner", th.text_dim, 14.0))
                .child(tr!("search-server-searching"))
                .into_any_element(),
            Status::NothingMore => line.child(tr!("search-server-nothing")).into_any_element(),
            Status::Failed => line
                .child(tr!("search-server-failed"))
                .child(
                    div()
                        .id("server-search-again")
                        .cursor_pointer()
                        .keeps_press()
                        .text_color(rgba(th.accent))
                        .font_weight(FontWeight::MEDIUM)
                        .child(tr!("search-server-again"))
                        .on_click(cx.listener(|this, _, _, cx| this.search_server_now(cx))),
                )
                .into_any_element(),
        })
    }

    /// The text for an empty list of results while the server may still
    /// find some.
    pub(super) fn server_empty_text(&self) -> Option<SharedString> {
        match self.server_search.as_ref()?.state {
            State::Waiting | State::Searching => Some(tr!("search-server-empty-searching").into()),
            _ => None,
        }
    }
}

/// A section heading in the list, as the Snoozed folder's day groups.
fn heading(label: String, first: bool, th: &Theme) -> AnyElement {
    div()
        .px(px(space::S5))
        .pt(px(if first { space::S3 } else { space::S5 }))
        .pb(px(space::S2))
        .text_size(px(text::CAPTION))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .border_b_1()
        .border_color(rgba(super::list::row_line(th)))
        .child(label)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_words_ask_the_server() {
        let words = |q: &str| has_words(&Query::parse_at(q, 0).unwrap());
        assert!(words("budget"));
        assert!(words("from:ada budget"));
        assert!(words("budget OR plan"));
        assert!(!words("from:ada"));
        assert!(!words("subject:plan is:unread"));
        assert!(!words("-budget"));
    }
}
