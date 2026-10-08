// SPDX-License-Identifier: GPL-3.0-or-later

//! Account problems, shown one way everywhere (`docs/DESIGN.md`,
//! Problems): what needs the user wears the amber warning colour, what
//! Katna waits out by itself stays grey, and each comes with the one
//! click that fixes it.
//!
//! - A line at the top of the mail list per problem, in the band the
//!   "select all" line uses: Google or Microsoft signed Katna out (Sign
//!   in), a server refused the saved password (New password), a server
//!   has not answered for half an hour (Try again), or the computer is
//!   offline. Later hides a line for a day; three or more fold to one.
//! - An amber mark on the account's heading in the folder pane and on its
//!   picture in the top bar, where "Work offline" shows its cloud.
//! - "New password": a card where it was clicked that checks the
//!   password with the server before keeping it (`SetPassword`).

use crate::widgets::Tip as _;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Bounds, ClickEvent, Context, Entity, Focusable, FontWeight, MouseDownEvent, Pixels,
    Point, SharedString, Size, Subscription, Task, Window, canvas, deferred, div, point,
    prelude::*, rgba,
};
use katna_core::{AccountId, OAuthProvider};
use katna_dbus::{AccountStatus, state};
use katna_i18n::tr;
use katna_ui::tokens::{space, text};
use katna_ui::{InputEvent, TextInput, WindowDrag, px, unpx};

use super::mail_providers::MailProvider;
use super::notched::{self, RADIUS};
use super::{MailWindow, TOP_BAR_HEIGHT};
use crate::daemon::{self, AddError};
use crate::theme::Theme;
use crate::widgets::{ButtonStyle, button, filled_button, icon, icon_button, line_field};

/// How long every mail account must be unreachable before the list says
/// the computer is offline (a short drop says nothing).
const OFFLINE_AFTER: Duration = Duration::from_secs(10);
/// How long one server may not answer before the list says so; Katna
/// keeps trying meanwhile.
const NO_ANSWER_AFTER: Duration = Duration::from_secs(30 * 60);
/// How long Later hides a line.
const LATER: Duration = Duration::from_secs(24 * 60 * 60);
/// From this many problems the lines fold to one.
const FOLD_FROM: usize = 3;
/// The New password card's width.
const CARD_WIDTH: f32 = 340.0;

/// Who refused or did not answer, at the start of a sentence: the
/// provider by name, else "The mail server".
fn who(address: &str) -> String {
    match MailProvider::for_address(address) {
        MailProvider::Other => tr!("problems-the-server"),
        provider => provider.name(),
    }
}

/// What is wrong with one account, and so the click that fixes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Problem {
    /// The provider stopped letting Katna in: sign in again.
    SignIn {
        id: i64,
        address: String,
        provider: OAuthProvider,
    },
    /// The server refused the saved password: type the new one.
    Password { id: i64, address: String },
    /// The server has not answered for a while; Katna keeps trying.
    NoAnswer { id: i64, address: String },
}

impl Problem {
    fn id(&self) -> i64 {
        match self {
            Self::SignIn { id, .. } | Self::Password { id, .. } | Self::NoAnswer { id, .. } => *id,
        }
    }

    /// Whether it waits for the user (amber), not only for time (grey).
    pub(super) fn needs_you(&self) -> bool {
        !matches!(self, Self::NoAnswer { .. })
    }

    /// The sentence that says what happened.
    pub(super) fn text(&self) -> String {
        match self {
            Self::SignIn {
                address, provider, ..
            } => tr!(
                "problems-signed-out",
                provider = provider.name(),
                address = address.as_str()
            ),
            Self::Password { address, .. } => tr!(
                "problems-password-refused",
                provider = who(address),
                address = address.as_str()
            ),
            Self::NoAnswer { address, .. } => tr!(
                "problems-no-answer",
                provider = who(address),
                address = address.as_str()
            ),
        }
    }

    /// The fix's label.
    pub(super) fn action(&self) -> String {
        match self {
            Self::SignIn { .. } => tr!("sign-in-again-button"),
            Self::Password { .. } => tr!("problems-new-password"),
            Self::NoAnswer { .. } => tr!("problems-try-again"),
        }
    }

    pub(super) fn icon(&self) -> &'static str {
        if self.needs_you() {
            "warning"
        } else {
            "cloud-off"
        }
    }

    /// The colour of its mark: amber when it needs the user.
    pub(super) fn color(&self, th: &Theme) -> u32 {
        if self.needs_you() {
            th.warning
        } else {
            th.text_dim
        }
    }
}

/// The New password card, open where it was asked for.
struct PasswordCard {
    id: i64,
    address: String,
    input: Entity<TextInput>,
    /// Where it was clicked open: the card points there.
    at: Point<Pixels>,
    shown: bool,
    checking: bool,
    /// The server refused this password too.
    refused: bool,
    /// Anything else that went wrong.
    error: Option<String>,
    /// When it began to fade out after closing.
    fading: Option<Instant>,
    _input: Subscription,
    _save: Option<Task<()>>,
}

#[derive(Default)]
pub(super) struct Problems {
    accounts: Vec<AccountStatus>,
    /// When each account was first seen unreachable, this run.
    unreachable: HashMap<i64, Instant>,
    /// Lines hidden by Later, until when.
    later: HashMap<i64, Instant>,
    /// "You're offline" hidden by Later, until when.
    offline_later: Option<Instant>,
    /// Three or more lines shown one by one rather than folded.
    unfolded: bool,
    /// The account signing in in the browser now.
    busy: Option<i64>,
    password: Option<PasswordCard>,
    /// The New password card just opened: its field takes the keys.
    focus: bool,
    _check: Option<Task<()>>,
    _sign_in: Option<Task<()>>,
    _timer: Option<Task<()>>,
    _sync: Option<Task<()>>,
    _refused: Option<Task<()>>,
    _fix: Option<Task<()>>,
    /// Where each account's fix shows on its line, last drawn: a desktop
    /// notification's New password card points there.
    fix_links: Rc<RefCell<HashMap<i64, Bounds<Pixels>>>>,
}

impl Problems {
    /// Every account's problem, the ones that need the user first.
    fn all(&self) -> Vec<Problem> {
        let now = Instant::now();
        let mut out: Vec<Problem> =
            self.accounts
                .iter()
                .filter_map(|a| {
                    let (id, address) = (a.id, a.address.clone());
                    match a.state.as_str() {
                        state::AUTH_FAILED => Some(match a.sign_in.parse::<OAuthProvider>() {
                            Ok(provider) => Problem::SignIn {
                                id,
                                address,
                                provider,
                            },
                            Err(_) => Problem::Password { id, address },
                        }),
                        state::OFFLINE
                            if !self.offline()
                                && self.unreachable.get(&id).is_some_and(|at| {
                                    now.duration_since(*at) >= NO_ANSWER_AFTER
                                }) =>
                        {
                            Some(Problem::NoAnswer { id, address })
                        }
                        _ => None,
                    }
                })
                .collect();
        out.sort_by_key(|p| !p.needs_you());
        out
    }

    /// The problem of account `id`, if it has one now.
    fn of(&self, id: i64) -> Option<Problem> {
        self.all().into_iter().find(|p| p.id() == id)
    }

    /// Whether every mail account that syncs has been unreachable for a
    /// little while: the computer is offline, not one server.
    fn offline(&self) -> bool {
        let mut syncing = self
            .accounts
            .iter()
            // A refused password reaches nothing either way.
            .filter(|a| {
                ![state::NOT_SYNCED, state::PAUSED, state::AUTH_FAILED].contains(&a.state.as_str())
            })
            .peekable();
        syncing.peek().is_some()
            && syncing.all(|a| {
                a.state == state::OFFLINE
                    && self
                        .unreachable
                        .get(&a.id)
                        .is_some_and(|at| at.elapsed() >= OFFLINE_AFTER)
            })
    }

    /// When the lines change by time alone: an unreachable account
    /// passing [`OFFLINE_AFTER`] or [`NO_ANSWER_AFTER`], or a Later ending.
    fn next_change(&self) -> Option<Instant> {
        let mut times: Vec<Instant> = Vec::new();
        for at in self.unreachable.values() {
            times.push(*at + OFFLINE_AFTER);
            times.push(*at + NO_ANSWER_AFTER);
        }
        times.extend(self.later.values().copied());
        times.extend(self.offline_later);
        let now = Instant::now();
        times.into_iter().filter(|t| *t > now).min()
    }

    fn hidden(&self, id: i64) -> bool {
        self.later
            .get(&id)
            .is_some_and(|until| *until > Instant::now())
    }
}

/// What the note says when a mail server refused `count` changes of kind
/// `change` in `address`, which were undone.
fn refused_text(change: &str, count: u32, address: &str) -> String {
    let count = i64::from(count);
    match change {
        "flags" => tr!("problems-refused-flags", count = count, address = address),
        "move" => tr!("problems-refused-move", count = count, address = address),
        "label" => tr!("problems-refused-label", count = count, address = address),
        "delete" => tr!("problems-refused-delete", count = count, address = address),
        _ => tr!("problems-refused-other", count = count, address = address),
    }
}

impl MailWindow {
    /// Says when a mail server refused changes for good, which the
    /// daemon undid, in words of what the user did, with the server's own
    /// under Details.
    pub(super) fn watch_refused(
        &mut self,
        connection: katna_dbus::zbus::Connection,
        cx: &mut Context<Self>,
    ) {
        use futures_lite::StreamExt;
        self.problems._refused = Some(cx.spawn(async move |this, cx| {
            let mut refused = match daemon::changes_refused(&connection).await {
                Ok(refused) => Box::pin(refused),
                Err(err) => {
                    tracing::info!("not following refused changes: {err}");
                    return;
                }
            };
            while let Some(refused) = refused.next().await {
                let shown = this.update(cx, |this, cx| {
                    let address = this
                        .accounts
                        .iter()
                        .find(|a| a.id == AccountId(refused.account))
                        .map(|a| a.address.clone())
                        .unwrap_or_default();
                    let text = refused_text(&refused.change, refused.count, &address);
                    this.show_snackbar_for(text, None, super::FAILURE_TIME, cx);
                    if let Some(snackbar) = &mut this.snackbar {
                        snackbar.undo = Some(daemon::Command::ShowDetails(refused.reason));
                        snackbar.label = Some(tr!("problems-details").into());
                    }
                });
                if shown.is_err() {
                    break;
                }
            }
        }));
    }

    /// Asks the daemon how every account's sync stands; after every
    /// change it reports.
    pub(super) fn check_problems(&mut self, cx: &mut Context<Self>) {
        self.remote.load_provider_pictures();
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.problems._check = Some(cx.spawn(async move |this, cx| {
            let accounts = cx
                .background_executor()
                .spawn(async move { daemon::accounts_status(&connection).await })
                .await;
            this.update(cx, |this, cx| {
                let Ok(accounts) = accounts else {
                    return;
                };
                let problems = &mut this.problems;
                let now = Instant::now();
                for a in &accounts {
                    if a.state == state::OFFLINE {
                        problems.unreachable.entry(a.id).or_insert(now);
                    } else if a.state != state::CONNECTING {
                        // Reconnecting after a drop keeps its first time.
                        problems.unreachable.remove(&a.id);
                    }
                }
                problems
                    .unreachable
                    .retain(|id, _| accounts.iter().any(|a| a.id == *id));
                if problems.accounts != accounts {
                    problems.accounts = accounts;
                    cx.notify();
                }
                this.watch_problem_times(cx);
            })
            .ok();
        }));
    }

    /// Draws the lines again when they change by time alone.
    fn watch_problem_times(&mut self, cx: &mut Context<Self>) {
        let Some(at) = self.problems.next_change() else {
            self.problems._timer = None;
            return;
        };
        self.problems._timer = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(at.saturating_duration_since(Instant::now()))
                .await;
            this.update(cx, |this, cx| {
                cx.notify();
                this.watch_problem_times(cx);
            })
            .ok();
        }));
    }

    /// Account `id`'s problem, for its marks in the folder pane, the top
    /// bar and the account card.
    pub(super) fn account_problem(&self, id: AccountId) -> Option<Problem> {
        self.problems.of(id.0)
    }

    /// The problem the top bar's picture shows: one that needs the user
    /// first.
    pub(super) fn top_problem(&self) -> Option<Problem> {
        self.problems.all().into_iter().next()
    }

    /// Every account's problem, worst first, for the account picture's
    /// tooltip.
    pub(super) fn all_problems(&self) -> Vec<Problem> {
        self.problems.all()
    }

    /// The mark of account `id`'s problem, where an offline account shows
    /// its cloud (the folder pane's heading): its sign in amber, or grey
    /// while Katna waits; a click fixes it.
    pub(super) fn problem_mark(
        &self,
        element: impl Into<gpui::ElementId>,
        problem: Problem,
        size: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        let tooltip = format!("{}\n{}", problem.text(), problem.action());
        div()
            .id(element)
            .flex_none()
            .size(px(size))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(icon(problem.icon(), problem.color(th), size * 0.7))
            .tip(tooltip, th)
            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                cx.stop_propagation();
                this.fix_problem(problem.clone(), event.position(), window, cx);
            }))
    }

    /// `picture` (an account's, `size` across) with the sign of `problem`
    /// on its corner, where "Work offline" puts its cloud.
    pub(super) fn problem_badge(
        &self,
        picture: AnyElement,
        size: f32,
        problem: Option<&Problem>,
        th: &Theme,
    ) -> AnyElement {
        let Some(problem) = problem else {
            return picture;
        };
        let badge = (size * 0.5).round().max(12.0);
        div()
            .flex_none()
            .relative()
            .child(picture)
            .child(
                div()
                    .absolute()
                    .right(px(-space::S1))
                    .bottom(px(-space::S1))
                    .size(px(badge))
                    .rounded_full()
                    .bg(rgba(th.surface))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon(problem.icon(), problem.color(th), badge - 4.0)),
            )
            .into_any_element()
    }

    /// Fixes `problem`: signs in again, opens the New password card at
    /// `at`, or tries the server again.
    pub(super) fn fix_problem(
        &mut self,
        problem: Problem,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match problem {
            Problem::SignIn {
                id,
                address,
                provider,
            } => self.sign_in_account(id, address, provider, cx),
            Problem::Password { id, address } => {
                self.open_password_card(id, address, at, window, cx)
            }
            Problem::NoAnswer { id, .. } => {
                let Some(connection) = self.daemon.clone() else {
                    return;
                };
                // Waiting again from now: the line goes until it is due.
                self.problems.unreachable.insert(id, Instant::now());
                self.problems._sync = Some(cx.spawn(async move |this, cx| {
                    let _ = cx
                        .background_executor()
                        .spawn(async move { daemon::sync_now(&connection, id).await })
                        .await;
                    this.update(cx, |this, cx| this.check_problems(cx)).ok();
                }));
                cx.notify();
            }
        }
    }

    /// Opens the fix of account `id`'s problem once its state is read: a
    /// desktop notification's button, which may have just started the
    /// app. The New password card points at the top of the list, where
    /// the problem's line is.
    pub(super) fn fix_problem_when_known(
        &mut self,
        id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.problems._fix = Some(cx.spawn_in(window, async move |this, cx| {
            let accounts = cx
                .background_executor()
                .spawn(async move { daemon::accounts_status(&connection).await })
                .await;
            this.update_in(cx, |this, window, cx| {
                if let Ok(accounts) = accounts {
                    this.problems.accounts = accounts;
                }
                this.problems.later.remove(&id);
                cx.notify();
                // Once a frame with the line is drawn, at its fix.
                cx.on_next_frame(window, move |_, window, cx| {
                    cx.on_next_frame(window, move |this, window, cx| {
                        let Some(problem) = this.account_problem(AccountId(id)) else {
                            return;
                        };
                        let at = this
                            .problems
                            .fix_links
                            .borrow()
                            .get(&id)
                            .map(|link| link.center())
                            .unwrap_or_else(|| {
                                point(window.viewport_size().width / 2.0, px(TOP_BAR_HEIGHT))
                            });
                        this.fix_problem(problem, at, window, cx);
                    });
                });
            })
            .ok();
        }));
    }

    /// Opens `provider`'s sign-in page for account `id` (`address`), as
    /// a problem line's Sign in does; also from the folder pane's menu.
    pub(super) fn sign_in_account(
        &mut self,
        id: i64,
        address: String,
        provider: OAuthProvider,
        cx: &mut Context<Self>,
    ) {
        if self.problems.busy.is_some() {
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let problems = &mut self.problems;
        problems.busy = Some(id);
        problems._sign_in = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { daemon::sign_in(&connection, provider, Some(id), "").await })
                .await;
            this.update(cx, |this, cx| {
                this.problems.busy = None;
                let text = match result {
                    Ok(_) => {
                        this.problems.accounts.retain(|a| a.id != id);
                        tr!("sign-in-again-done", address = address.as_str())
                    }
                    Err(AddError::Password(detail)) => {
                        tracing::info!(%detail, "sign-in refused");
                        tr!("add-account-sign-in-refused", provider = provider.name())
                    }
                    Err(AddError::Other(err)) => err,
                };
                this.show_snackbar(text, None, cx);
                this.check_problems(cx);
            })
            .ok();
        }));
        cx.notify();
    }

    /// The lines at the top of the mail list: one per problem, folded to
    /// one from [`FOLD_FROM`], and "You're offline" when nothing reaches
    /// any server.
    pub(super) fn render_problems(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let problems = &self.problems;
        let shown: Vec<Problem> = problems
            .all()
            .into_iter()
            .filter(|p| !problems.hidden(p.id()))
            .collect();
        let offline = problems.offline()
            && !problems
                .offline_later
                .is_some_and(|until| until > Instant::now());
        let service = self.render_service_line(th, cx);
        if shown.is_empty() && !offline && service.is_none() {
            return None;
        }
        let mut lines: Vec<AnyElement> = service.into_iter().collect();
        if offline {
            lines.push(
                self.problem_line(
                    "problems-offline".into(),
                    icon("cloud-off", th.text_dim, 18.0),
                    tr!("problems-offline"),
                    Vec::new(),
                    true,
                    None,
                    th,
                    cx,
                )
                .on_click_later(cx.listener(|this, _, _, cx| {
                    this.problems.offline_later = Some(Instant::now() + LATER);
                    this.watch_problem_times(cx);
                    cx.notify();
                })),
            );
        }
        if shown.len() >= FOLD_FROM && !problems.unfolded {
            let count = shown.iter().filter(|p| p.needs_you()).count().max(1) as u64;
            lines.push(
                self.problem_line(
                    "problems-folded".into(),
                    icon("warning", th.warning, 18.0),
                    tr!("problems-accounts-need-you", count = count),
                    vec![(
                        tr!("problems-show"),
                        Box::new(
                            |this: &mut Self,
                             _: &ClickEvent,
                             _: &mut Window,
                             cx: &mut Context<Self>| {
                                this.problems.unfolded = true;
                                cx.notify();
                            },
                        ),
                    )],
                    false,
                    None,
                    th,
                    cx,
                )
                .into_any(),
            );
        } else {
            for problem in shown {
                let id = problem.id();
                let busy = problems.busy == Some(id);
                let fix = problem.clone();
                let action = (!busy).then(|| {
                    (
                        problem.action(),
                        Box::new(
                            move |this: &mut Self,
                                  event: &ClickEvent,
                                  window: &mut Window,
                                  cx: &mut Context<Self>| {
                                this.fix_problem(fix.clone(), event.position(), window, cx)
                            },
                        ) as LineClick,
                    )
                });
                let text = if busy {
                    format!("{} {}", problem.text(), tr!("sign-in-again-waiting"))
                } else {
                    problem.text()
                };
                lines.push(
                    self.problem_line(
                        SharedString::from(format!("problem-{id}")),
                        icon(problem.icon(), problem.color(th), 18.0),
                        text,
                        action.into_iter().collect(),
                        true,
                        Some(id),
                        th,
                        cx,
                    )
                    .on_click_later(cx.listener(move |this, _, _, cx| {
                        this.problems.later.insert(id, Instant::now() + LATER);
                        this.watch_problem_times(cx);
                        cx.notify();
                    })),
                );
            }
        }
        Some(
            div()
                .flex_none()
                .flex()
                .flex_col()
                .children(lines)
                .into_any_element(),
        )
    }

    /// One line: the band "select all" uses, with its sign, what happened,
    /// the fixes in the accent colour and Later. `anchor` marks where the
    /// first fix shows.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn problem_line(
        &self,
        id: SharedString,
        sign: AnyElement,
        text: String,
        actions: Vec<(String, LineClick)>,
        later: bool,
        anchor: Option<i64>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> ProblemLine {
        let link = |id: SharedString, label: String, color: u32| {
            div()
                .id(id)
                .flex_none()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(color))
                .cursor_pointer()
                .keeps_press()
                .hover(|s| s.underline())
                .child(label)
        };
        let links = self.problems.fix_links.clone();
        let actions = actions.into_iter().enumerate().map(|(n, (label, click))| {
            let fix_id: SharedString = if n == 0 {
                format!("{id}-fix").into()
            } else {
                format!("{id}-fix-{n}").into()
            };
            link(fix_id, label, th.accent)
                .relative()
                .on_click(
                    cx.listener(move |this, event, window, cx| click(this, event, window, cx)),
                )
                .when_some(anchor.filter(|_| n == 0), |link, account| {
                    let links = links.clone();
                    link.child(
                        canvas(
                            move |bounds, _, _| {
                                links.borrow_mut().insert(account, bounds);
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    )
                })
                .into_any_element()
        });
        let line = div()
            .flex_none()
            .min_h(px(40.0))
            .py(px(space::S3))
            .px(px(space::S5))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .justify_center()
            .gap_x(px(space::S3))
            .gap_y(px(space::S2))
            .bg(rgba(th.on_pane(th.read_row)))
            .border_b_1()
            .border_color(rgba(th.divider))
            .text_size(px(text::SMALL))
            // The sign stays beside the words when the line wraps.
            .child(
                div()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S3))
                    .child(div().flex_none().child(sign))
                    .child(div().min_w_0().child(text)),
            )
            .children(actions);
        ProblemLine {
            line,
            later: later.then(|| {
                link(
                    format!("{id}-later").into(),
                    tr!("problems-later"),
                    th.text_dim,
                )
            }),
        }
    }

    /// Opens the New password card for account `id` (`address`), pointing
    /// at `at`.
    pub(super) fn open_password_card(
        &mut self,
        id: i64,
        address: String,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new("", cx);
            input.set_accent(accent);
            input.set_masked(true, cx);
            input.set_placeholder(tr!("problems-password-placeholder"));
            input
        });
        let subscription = cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.save_password(cx),
            InputEvent::Cancel => {
                this.close_password_card(cx);
            }
            InputEvent::Changed => {
                if let Some(card) = &mut this.problems.password
                    && (card.refused || card.error.is_some())
                {
                    card.refused = false;
                    card.error = None;
                    cx.notify();
                }
            }
        });
        self.problems.password = Some(PasswordCard {
            id,
            address,
            input,
            at,
            shown: false,
            checking: false,
            refused: false,
            error: None,
            fading: None,
            _input: subscription,
            _save: None,
        });
        self.problems.focus = true;
        cx.notify();
    }

    /// Closes the New password card; whether one was open.
    pub(super) fn close_password_card(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(card) = self
            .problems
            .password
            .as_mut()
            .filter(|c| c.fading.is_none())
        else {
            return false;
        };
        match notched::fade_out(cx) {
            Some(since) => card.fading = Some(since),
            None => self.problems.password = None,
        }
        cx.notify();
        true
    }

    fn save_password(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(card) = self.problems.password.as_mut().filter(|c| !c.checking) else {
            return;
        };
        let password = card.input.read(cx).text().to_owned();
        if password.is_empty() {
            return;
        }
        let (id, address) = (card.id, card.address.clone());
        card.checking = true;
        card.refused = false;
        card.error = None;
        card._save = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { daemon::set_password(&connection, id, &password).await })
                .await;
            this.update(cx, |this, cx| {
                let Some(card) = this.problems.password.as_mut().filter(|c| c.id == id) else {
                    return;
                };
                card.checking = false;
                match result {
                    Ok(()) => {
                        this.close_password_card(cx);
                        this.problems.accounts.retain(|a| a.id != id);
                        this.show_snackbar(
                            tr!("problems-password-saved", address = address.as_str()),
                            None,
                            cx,
                        );
                        this.check_problems(cx);
                    }
                    Err(AddError::Password(detail)) => {
                        tracing::info!(%detail, "new password refused");
                        card.refused = true;
                    }
                    Err(AddError::Other(err)) => card.error = Some(err),
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    /// The New password card, where it was clicked open.
    pub(super) fn render_password_card(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if self
            .problems
            .password
            .as_ref()
            .and_then(|c| c.fading)
            .is_some_and(|since| notched::faded(since, cx))
        {
            self.problems.password = None;
        }
        if std::mem::take(&mut self.problems.focus)
            && let Some(card) = &self.problems.password
        {
            window.focus(&card.input.focus_handle(cx), cx);
        }
        let card = self.problems.password.as_ref()?;
        let provider = MailProvider::for_address(&card.address);
        let help = provider.password_help();
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        // Title, line under it, field, buttons, and the refusal when shown.
        let height = space::S5 * 2.0
            + 24.0
            + space::S2
            + 36.0
            + space::S4
            + crate::widgets::FIELD_HEIGHT
            + space::S4
            + crate::widgets::BUTTON_HEIGHT
            + if card.refused || card.error.is_some() {
                space::S3 + 54.0
            } else {
                0.0
            };
        let chip = Bounds::new(card.at, Size::new(px(1.0), px(1.0)));
        let (x, y, side, along) = notched::place(chip, (CARD_WIDTH, height), (vw, vh), RADIUS);
        let shown = card.shown;
        let masked_label = if shown {
            tr!("problems-password-hide")
        } else {
            tr!("problems-password-show")
        };
        let eye = icon_button(
            "password-card-eye",
            if shown { "eye-off" } else { "eye" },
            18.0,
            th,
        )
        .size(px(32.0))
        .tip(masked_label, th)
        .on_click(cx.listener(|this, _, _, cx| {
            if let Some(card) = &mut this.problems.password {
                card.shown = !card.shown;
                let masked = !card.shown;
                card.input
                    .update(cx, |input, cx| input.set_masked(masked, cx));
            }
            cx.notify();
        }));
        let refusal = (card.refused || card.error.is_some()).then(|| {
            let message = match &card.error {
                Some(err) => err.clone(),
                None if help.is_some() => {
                    tr!(
                        "add-account-app-password-refused",
                        provider = provider.name()
                    )
                }
                None => tr!(
                    "problems-password-refused-again",
                    provider = who(&card.address)
                ),
            };
            div()
                .flex()
                .flex_row()
                .gap(px(space::S3))
                .text_size(px(text::SMALL))
                .text_color(rgba(th.warning))
                .child(div().flex_none().child(icon("warning", th.warning, 16.0)))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(space::S1))
                        .child(message)
                        .children(help.filter(|_| card.error.is_none()).map(|(_, url)| {
                            div()
                                .id("password-card-help")
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.accent))
                                .cursor_pointer()
                                .hover(|s| s.underline())
                                .on_click(move |_, _, cx| cx.open_url(url))
                                .child(tr!("add-account-help-app-password-link"))
                        })),
                )
        });
        let checking = card.checking;
        let save = filled_button(
            "password-card-save",
            if checking {
                tr!("problems-password-checking")
            } else {
                tr!("problems-password-save")
            },
            th,
        )
        .when(checking, |b| b.opacity(0.7))
        .on_click(cx.listener(|this, _, _, cx| this.save_password(cx)));
        let cancel = button("password-card-cancel", ButtonStyle::Text, th)
            .child(tr!("problems-password-cancel"))
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_password_card(cx);
            }));
        let at = card.at;
        let panel = div()
            .id("password-card")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(CARD_WIDTH))
            .p(px(space::S5))
            .flex()
            .flex_col()
            .gap(px(space::S4))
            .map(|d| notched::popover(d, th))
            .text_color(rgba(th.text))
            .on_mouse_down_out(cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                // The click that opened it is not one outside it.
                if event.position != at {
                    this.close_password_card(cx);
                }
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(space::S2))
                    .child(
                        div()
                            .text_size(px(text::SUBTITLE))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(tr!("problems-password-title")),
                    )
                    .child(
                        div()
                            .text_size(px(text::SMALL))
                            .text_color(rgba(th.text_dim))
                            .child(tr!(
                                "problems-password-detail",
                                provider = who(&card.address),
                                address = card.address.as_str()
                            )),
                    ),
            )
            .child(
                line_field("password-card-field", &card.input, th, cx)
                    .pr(px(space::S1))
                    .child(eye),
            )
            .children(refusal)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .gap(px(space::S3))
                    .child(cancel)
                    .child(save),
            )
            .children(notched::notch(side, along, th));
        let panel = match card.fading {
            Some(_) => notched::fading(panel, "password-card-out"),
            None => panel.into_any_element(),
        };
        let layer = div().relative().w(px(vw)).h(px(vh)).child(panel);
        Some(
            deferred(
                katna_ui::anchored()
                    .position(point(px(0.0), px(0.0)))
                    .child(layer),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }
}

/// What a problem line's fix does when clicked.
pub(super) type LineClick =
    Box<dyn Fn(&mut MailWindow, &ClickEvent, &mut Window, &mut Context<MailWindow>)>;

/// A problem line, before its Later link is wired.
pub(super) struct ProblemLine {
    line: gpui::Div,
    later: Option<gpui::Stateful<gpui::Div>>,
}

impl ProblemLine {
    pub(super) fn on_click_later(
        self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut gpui::App) + 'static,
    ) -> AnyElement {
        self.line
            .children(self.later.map(|later| later.on_click(handler)))
            .into_any_element()
    }

    pub(super) fn into_any(self) -> AnyElement {
        self.line.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says_refused_changes_as_what_was_done() {
        let one = refused_text("move", 1, "ada@example.org");
        assert!(one.contains("moving a message") && one.contains("ada@example.org"));
        assert!(refused_text("flags", 3, "a@b").contains("3 messages"));
        assert_ne!(
            refused_text("label", 1, "a@b"),
            refused_text("delete", 1, "a@b")
        );
        assert_eq!(refused_text("other", 2, "a@b"), refused_text("?", 2, "a@b"));
    }
}
