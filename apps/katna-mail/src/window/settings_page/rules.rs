// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Folders & rules: the mail rules, one list for every account
//! (or one account, picked over the list), in the order they run. Each
//! row has a handle to drag it to another place, an on/off switch, its
//! name and a line saying what it does, a dot in the color of each of its
//! accounts, where it runs, and a pencil that opens the rule editor
//! (`window/rule_editor.rs`). A rule the daemon switched off says why, in
//! red. Under them, the starter rules not used yet, switched off
//! (`starter_rules.rs`). Under the rules, the Folders row: an unread count
//! on every folder, or on the inbox only.

use gpui::{
    AnimationExt, AnyElement, ClickEvent, Context, DragMoveEvent, FontWeight, MouseButton,
    MouseDownEvent, Pixels, Point, Render, SpringAnimation, Task, Window, anchored, deferred, div,
    prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::rules::Rule;
use katna_ui::motion;
use katna_ui::px;
use katna_ui::unpx;

use super::super::row_reorder::Reorder;
use super::super::rule_editor::{self, dot};
use super::super::settings::Change;
use super::MailWindow;
use super::starter_rules::{self, Starter};
use crate::theme::{Theme, fade};
use crate::widgets::{filled_button, icon, icon_button, menu, menu_item, switch, tip};
use crate::{daemon, data};

/// The rules as Settings shows them.
#[derive(Default)]
pub(in crate::window) struct RulesList {
    rules: Vec<Rule>,
    loaded: bool,
    /// The one account whose rules are shown, or every account's.
    account: Option<AccountId>,
    /// The account filter's menu, open at a point.
    menu: Option<Point<Pixels>>,
    load: Option<Task<()>>,
    watch: Option<Task<()>>,
    /// Starter rules being turned on, by key.
    starting: Vec<&'static str>,
    /// Dragging a rule's row to reorder, by rule ID.
    reorder: Reorder<i64>,
}

/// The space between two rule rows.
const ROW_GAP: f32 = 4.0;

/// A change to the rules the daemon makes.
enum RuleOp {
    Enable(i64, bool),
    Reorder(Vec<i64>),
}

/// A rule being dragged to another place: its row in the list shown, and
/// what follows the pointer.
#[derive(Clone)]
struct RuleDrag {
    ix: usize,
}

/// The row itself follows the pointer ([`Reorder`]), so the drag draws
/// nothing of its own.
impl Render for RuleDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl MailWindow {
    /// Reads the rules from the store.
    pub(in crate::window) fn load_rules(&mut self, cx: &mut Context<Self>) {
        let Some(page) = &mut self.settings_page else {
            return;
        };
        let paths = self.paths.clone();
        page.rules.load = Some(cx.spawn(async move |this, cx| {
            let rules = cx
                .background_executor()
                .spawn(async move { data::rules(&paths) })
                .await;
            this.update(cx, |this, cx| {
                if let Some(page) = &mut this.settings_page {
                    match rules {
                        Ok(rules) => page.rules.rules = rules,
                        // A store from before rules: none yet.
                        Err(err) => tracing::warn!("{err}"),
                    }
                    page.rules.loaded = true;
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// Reads the rules again whenever the daemon says they changed (one
    /// it switched off, one saved in another window).
    pub(super) fn watch_rules(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let Some(page) = &mut self.settings_page else {
            return;
        };
        if page.rules.watch.is_some() {
            return;
        }
        page.rules.watch = Some(cx.spawn(async move |this, cx| {
            use futures_lite::StreamExt;
            let Ok(mut changes) = daemon::rules::changes(&connection).await else {
                return;
            };
            while changes.next().await.is_some() {
                if this.update(cx, |this, cx| this.load_rules(cx)).is_err() {
                    return;
                }
            }
        }));
    }

    /// Escape: the account filter's menu.
    pub(in crate::window) fn close_rules_menu(&mut self, cx: &mut Context<Self>) -> bool {
        let closed = self
            .settings_page
            .as_mut()
            .is_some_and(|p| p.rules.menu.take().is_some());
        if closed {
            cx.notify();
        }
        closed
    }

    /// The mail accounts, which rules look at.
    fn rule_accounts(&self) -> Vec<(AccountId, String)> {
        self.accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .map(|a| (a.id, a.address.clone()))
            .collect()
    }

    /// The rules shown: every one, or those of the account picked.
    fn shown_rules(&self) -> Vec<&Rule> {
        let Some(page) = &self.settings_page else {
            return Vec::new();
        };
        page.rules
            .rules
            .iter()
            .filter(|r| page.rules.account.is_none_or(|a| r.covers(a)))
            .collect()
    }

    /// Asks the daemon to switch a rule or change their order, then
    /// reads them again; says so when it fails.
    fn change_rules(&mut self, op: RuleOp, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    match op {
                        RuleOp::Enable(id, on) => {
                            daemon::rules::set_enabled(&connection, id, on).await
                        }
                        RuleOp::Reorder(ids) => daemon::rules::reorder(&connection, &ids).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    this.show_snackbar(tr!("rules-change-failed", error = err), None, cx);
                }
                this.load_rules(cx);
            })
            .ok();
        })
        .detach();
    }

    fn set_rule_on(&mut self, id: i64, on: bool, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page
            && let Some(rule) = page.rules.rules.iter_mut().find(|r| r.id == id)
        {
            rule.enabled = on;
            if on {
                rule.last_error = None;
            }
        }
        cx.notify();
        self.change_rules(RuleOp::Enable(id, on), cx);
    }

    /// The rule shown at `from` was dropped on the one shown at `to`: it
    /// takes that place. With one account picked, the other accounts'
    /// rules keep theirs.
    fn move_rule(&mut self, from: usize, to: usize, cx: &mut Context<Self>) {
        let mut shown: Vec<i64> = self.shown_rules().iter().map(|r| r.id).collect();
        if let Some(page) = &mut self.settings_page {
            page.rules.reorder.moved(&shown, from, to, ROW_GAP);
        }
        if from == to || from >= shown.len() || to >= shown.len() {
            cx.notify();
            return;
        }
        let moved = shown.remove(from);
        shown.insert(to, moved);
        let Some(page) = &mut self.settings_page else {
            return;
        };
        let order = reordered(&page.rules.rules, &shown);
        page.rules
            .rules
            .sort_by_key(|r| order.iter().position(|id| *id == r.id));
        cx.notify();
        self.change_rules(RuleOp::Reorder(order), cx);
    }

    /// The pointer moved while dragging the rule at `from` to `y`.
    fn rule_dragged(&mut self, from: usize, y: f32, cx: &mut Context<Self>) {
        let ids: Vec<i64> = self.shown_rules().iter().map(|r| r.id).collect();
        if let Some(page) = &mut self.settings_page {
            page.rules.reorder.dragged(&ids, from, y, ROW_GAP);
        }
        cx.notify();
    }

    /// The dragged rule was let go: it goes where the rows made room.
    fn drop_rule(&mut self, cx: &mut Context<Self>) {
        let Some((from, to)) = self
            .settings_page
            .as_ref()
            .and_then(|p| p.rules.reorder.drop_move())
        else {
            return;
        };
        self.move_rule(from, to, cx);
    }

    /// Advances the rule rows sliding in Settings > Folders & rules.
    pub(in crate::window) fn tick_rule_reorder(
        &mut self,
        window: &Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) {
        let dropped = self
            .settings_page
            .as_ref()
            .is_some_and(|p| p.rules.reorder.dragging())
            && !cx.has_active_drag();
        if dropped {
            // Let go somewhere with nowhere to drop it.
            self.drop_rule(cx);
        }
        let count = self.shown_rules().len();
        if let Some(page) = &mut self.settings_page {
            page.rules.reorder.tick(count, window, reduce);
        }
    }

    /// New rule: for the account picked over the list, else the one open.
    fn new_rule_here(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mail: Vec<AccountId> = self.rule_accounts().into_iter().map(|(id, _)| id).collect();
        let account = self
            .settings_page
            .as_ref()
            .and_then(|p| p.rules.account)
            .or_else(|| self.account().filter(|a| mail.contains(a)))
            .or_else(|| mail.first().copied());
        self.new_rule(account.map(|a| vec![a.0]).unwrap_or_default(), window, cx);
    }

    /// The ID of the folder called `name` among the user's own folders
    /// of `account`, if there is one.
    fn folder_named(&self, account: AccountId, name: &str) -> Option<i64> {
        self.tree
            .folders_of(account)
            .into_iter()
            .find(|(_, label, role)| {
                *role == crate::sidebar::Role::Other && label.eq_ignore_ascii_case(name)
            })
            .map(|(id, _, _)| id.0)
    }

    /// The editor on starter rule `key`, for every mail account, with
    /// the folders that exist and, where an account lacks one, the folder
    /// saving will make.
    fn edit_starter(&mut self, key: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(starter) = starter_rules::starters().into_iter().find(|s| s.key == key) else {
            return;
        };
        let accounts: Vec<i64> = self.rule_accounts().iter().map(|(id, _)| id.0).collect();
        // Folders to make stand in with IDs below 0.
        let mut new_folders: Vec<(i64, i64, String)> = Vec::new();
        for &account in &accounts {
            for folder in starter.folders() {
                let name = folder.name();
                if self.folder_named(AccountId(account), &name).is_none() {
                    new_folders.push((-(new_folders.len() as i64) - 1, account, name));
                }
            }
        }
        let rule = starter.rule(&accounts, |account, folder| {
            let name = folder.name();
            self.folder_named(AccountId(account), &name).or_else(|| {
                new_folders
                    .iter()
                    .find(|(_, a, n)| *a == account && *n == name)
                    .map(|(id, _, _)| *id)
            })
        });
        self.open_rule_editor(new_folders, rule, window, cx);
    }

    /// Turns on starter rule `key`: makes its folders in every mail
    /// account that lacks them, then saves it for all of them.
    fn turn_on_starter(&mut self, key: &'static str, cx: &mut Context<Self>) {
        let Some(starter) = starter_rules::starters().into_iter().find(|s| s.key == key) else {
            return;
        };
        let accounts: Vec<i64> = self.rule_accounts().iter().map(|(id, _)| id.0).collect();
        if accounts.is_empty() {
            return;
        }
        let Some(page) = &mut self.settings_page else {
            return;
        };
        if page.rules.starting.contains(&key) {
            return;
        }
        page.rules.starting.push(key);
        // The folders there are, and those to make.
        let mut have: Vec<(i64, String, i64)> = Vec::new();
        let mut make: Vec<(i64, String)> = Vec::new();
        for &account in &accounts {
            for folder in starter.folders() {
                let name = folder.name();
                match self.folder_named(AccountId(account), &name) {
                    Some(id) => have.push((account, name, id)),
                    None => make.push((account, name)),
                }
            }
        }
        let name = starter.name();
        if !make.is_empty() {
            self.show_snackbar(
                tr!("settings-rules-starter-turning-on", name = name.as_str()),
                None,
                cx,
            );
        }
        cx.notify();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    for (account, folder) in make {
                        let id = daemon::create_folder(&connection, account, &folder, None).await?;
                        have.push((account, folder, id));
                    }
                    let rule = starter.rule(&accounts, |account, folder| {
                        let name = folder.name();
                        have.iter()
                            .find(|(a, n, _)| *a == account && *n == name)
                            .map(|(_, _, id)| *id)
                    });
                    daemon::rules::save(&connection, &rule).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(page) = &mut this.settings_page {
                    page.rules.starting.retain(|k| *k != key);
                }
                if let Err(err) = result {
                    this.show_snackbar(
                        tr!(
                            "settings-rules-starter-failed",
                            name = name.as_str(),
                            error = err
                        ),
                        None,
                        cx,
                    );
                }
                this.load_rules(cx);
            })
            .ok();
        })
        .detach();
    }

    /// The Rules and Folders rows.
    pub(super) fn rules_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let folders = self.row(
            tr!("settings-folders"),
            None,
            self.switch_row(
                "page-folder-unread-counts",
                tr!("settings-folders-unread-counts"),
                tr!("settings-folders-unread-counts-detail"),
                self.config.mail.folder_unread_counts,
                Change::FolderUnreadCounts(!self.config.mail.folder_unread_counts),
                th,
                cx,
            ),
            th,
        );
        div()
            .flex()
            .flex_col()
            .child(self.row(tr!("settings-rules"), None, self.rules_list(th, cx), th))
            .child(folders)
            .children(self.render_rules_menu(th, cx))
            .into_any_element()
    }

    fn rules_list(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = &self.settings_page else {
            return div().into_any_element();
        };
        let filter = page.rules.account;
        let filter_label = filter
            .and_then(|a| self.account_address(a))
            .unwrap_or_else(|| tr!("settings-rules-all-accounts"));
        let header = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .flex_1()
                    .min_w(px(200.0))
                    .text_size(px(13.0))
                    .line_height(px(18.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("settings-rules-intro")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .id("rules-filter")
                            .map(|d| self.page_control(d, th, cx))
                            .h(px(36.0))
                            .pl(px(12.0))
                            .pr(px(8.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(rgba(th.outline))
                            .text_size(px(14.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(|this, event: &ClickEvent, _, cx| {
                                if let Some(page) = &mut this.settings_page {
                                    page.rules.menu = match page.rules.menu {
                                        Some(_) => None,
                                        None => Some(event.position()),
                                    };
                                }
                                cx.notify();
                            }))
                            .when_some(filter.and_then(|a| self.account_address(a)), |d, a| {
                                d.child(dot(self.account_color(&a, th)))
                            })
                            .child(div().max_w(px(220.0)).truncate().child(filter_label))
                            .child(icon("chevron-down", th.text_dim, 16.0)),
                    )
                    .child(
                        filled_button("rules-new", "", th)
                            .map(|d| self.page_control(d, th, cx))
                            .pl(px(16.0))
                            .pr(px(20.0))
                            .gap(px(6.0))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.new_rule_here(window, cx)),
                            )
                            .child(icon("add", th.on_accent, 18.0))
                            .child(tr!("settings-rules-new")),
                    ),
            );
        let shown = self.shown_rules();
        let empty = page.rules.loaded && shown.is_empty();
        let rows = shown
            .iter()
            .enumerate()
            .map(|(ix, rule)| self.rule_row(ix, rule, th, cx))
            .collect::<Vec<_>>();
        let starters = if page.rules.loaded {
            starter_rules::offered(&page.rules.rules)
        } else {
            Vec::new()
        };
        let starting = page.rules.starting.clone();
        let starter_rows = starters
            .iter()
            .enumerate()
            .map(|(ix, s)| self.starter_row(ix, s, starting.contains(&s.key), th, cx))
            .collect::<Vec<_>>();
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(header)
            .child(div().h(px(8.0)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(ROW_GAP))
                    .on_drag_move(cx.listener(|this, event: &DragMoveEvent<RuleDrag>, _, cx| {
                        let from = event.drag(cx).ix;
                        this.rule_dragged(from, unpx(event.event.position.y), cx)
                    }))
                    .on_drop(cx.listener(|this, _: &RuleDrag, _, cx| this.drop_rule(cx)))
                    .children(rows),
            )
            .when(!starter_rows.is_empty(), |d| {
                d.child(
                    div()
                        .mt(px(if empty { 4.0 } else { 14.0 }))
                        .mb(px(2.0))
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_baseline()
                        .gap_x(px(10.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgba(th.text))
                                .child(tr!("settings-rules-starters")),
                        )
                        .child(
                            div()
                                .text_size(px(13.0))
                                .line_height(px(18.0))
                                .text_color(rgba(th.text_faint))
                                .child(tr!("settings-rules-starters-intro")),
                        ),
                )
                .children(starter_rows)
            })
            .when(empty && starters.is_empty(), |d| {
                d.child(
                    div()
                        .py(px(12.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child(if filter.is_some() {
                            tr!("settings-rules-none-account")
                        } else {
                            tr!("settings-rules-none")
                        }),
                )
            })
            .into_any_element()
    }

    /// One rule's row; where it is narrow, its dots, tag and pencil go
    /// under its name.
    fn rule_row(&self, ix: usize, rule: &Rule, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let id = rule.id;
        let on = rule.enabled;
        let failed = !on && rule.last_error.is_some();
        let summary = rule_editor::summary(rule, |f| self.rule_folder_name(f));
        let handle = div()
            .id(("rule-drag", ix))
            .flex_none()
            .w(px(20.0))
            .h(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(6.0))
            .cursor_grab()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(tr!("settings-rules-drag"), th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, _| {
                    if let Some(page) = this.settings_page.as_mut() {
                        page.rules.reorder.grab(ix, unpx(event.position.y));
                    }
                }),
            )
            .on_drag(RuleDrag { ix }, |drag, _, _, cx| cx.new(|_| drag.clone()))
            .child(icon("drag-handle", th.text_faint, 16.0));
        let toggle = div()
            .id(("rule-switch", ix))
            .map(|d| self.page_control(d, th, cx))
            .flex_none()
            .p(px(4.0))
            .rounded_full()
            .cursor_pointer()
            .tooltip(tip(
                if on {
                    tr!("settings-rules-turn-off")
                } else {
                    tr!("settings-rules-turn-on")
                },
                th,
            ))
            .on_click(cx.listener(move |this, _, _, cx| this.set_rule_on(id, !on, cx)))
            .child(div().with_spring(
                ("rule-switch-spring", ix),
                SpringAnimation::new(motion::SLIDE).to(if on { 1.0 } else { 0.0 }),
                {
                    let th = *th;
                    move |el, s: f32| el.child(switch(s.clamp(0.0, 1.0), &th))
                },
            ));
        let about = div()
            .flex_1()
            .min_w(px(200.0))
            .flex()
            .flex_col()
            .gap(px(1.0))
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(if on { th.text } else { th.text_faint }))
                    .truncate()
                    .child(rule.name.clone()),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(px(18.0))
                    .text_color(rgba(if on { th.text_dim } else { th.text_faint }))
                    .child(summary),
            )
            .when_some(rule.last_error.as_ref().filter(|_| failed), |d, error| {
                d.child(
                    div()
                        .mt(px(2.0))
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(6.0))
                        .text_size(px(13.0))
                        .line_height(px(18.0))
                        .text_color(rgba(th.error))
                        .child(div().mt(px(1.0)).child(icon("warning", th.error, 15.0)))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(rule_editor::error_text(error)),
                        ),
                )
            });
        let dots = rule
            .accounts
            .iter()
            .filter_map(|a| self.account_address(AccountId(*a)))
            .map(|a| dot(self.account_color(&a, th)));
        let tag = if failed {
            tr!("rules-stopped")
        } else {
            rule_editor::runs_on_label(rule.runs_on)
        };
        // On the mail service: the blue tag.
        let remote = rule.runs_on != katna_store::rules::RunsOn::Katna && !failed;
        // Why it stays in Katna, on the tag's tooltip.
        let why = rule
            .runs_note
            .as_ref()
            .filter(|_| !remote && !failed)
            .map(|note| rule_editor::note_text(note, &|id| self.rule_folder_name(id)));
        let side = div()
            .flex_none()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .child(div().flex().flex_row().gap(px(4.0)).children(dots))
            .child(
                div()
                    .id(("rule-runs", ix))
                    .px(px(8.0))
                    .h(px(22.0))
                    .flex()
                    .items_center()
                    .rounded(px(6.0))
                    .bg(rgba(if remote {
                        th.nav_selected
                    } else {
                        fade(th.text_faint, 0.16)
                    }))
                    .text_color(rgba(if remote {
                        th.nav_selected_text
                    } else {
                        th.text_dim
                    }))
                    .text_size(px(12.0))
                    .when_some(why, |d, why| d.tooltip(tip(why, th)))
                    .child(tag),
            )
            .child(
                icon_button(("rule-edit", ix), "pen", 18.0, th)
                    .map(|d| self.page_control(d, th, cx))
                    .size(px(32.0))
                    .tooltip(tip(tr!("settings-rules-edit"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        let rule = this
                            .settings_page
                            .as_ref()
                            .and_then(|p| p.rules.rules.iter().find(|r| r.id == id))
                            .cloned();
                        if let Some(rule) = rule {
                            this.open_rule_editor(Vec::new(), rule, window, cx);
                        }
                    })),
            );
        let reorder = self.settings_page.as_ref().map(|p| &p.rules.reorder);
        let raised = reorder.is_some_and(|r| r.raised(ix, id));
        let row = div()
            .id(("rule-row", ix))
            .mx(px(-8.0))
            .px(px(8.0))
            .py(px(6.0))
            .rounded(px(10.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(8.0))
            .when_some(reorder, |d, r| r.row(d, ix, id, th))
            .child(handle)
            .child(toggle)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_x(px(12.0))
                    .gap_y(px(6.0))
                    .child(about)
                    .child(side),
            );
        // Drawn over the rows after it while lifted or landing.
        if raised {
            deferred(row).with_priority(1).into_any_element()
        } else {
            row.into_any_element()
        }
    }

    /// A starter rule's row: as a rule's, switched off, without a handle
    /// or a tag. `starting` while it is being turned on.
    fn starter_row(
        &self,
        ix: usize,
        starter: &Starter,
        starting: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let key = starter.key;
        let summary = rule_editor::summary(&starter.preview(), starter_rules::preview_folder);
        let toggle = div()
            .id(("starter-switch", ix))
            .map(|d| self.page_control(d, th, cx))
            .flex_none()
            .p(px(4.0))
            .rounded_full()
            .cursor_pointer()
            .tooltip(tip(tr!("settings-rules-turn-on"), th))
            .on_click(cx.listener(move |this, _, _, cx| this.turn_on_starter(key, cx)))
            .child(div().with_spring(
                ("starter-switch-spring", ix),
                SpringAnimation::new(motion::SLIDE).to(if starting { 1.0 } else { 0.0 }),
                {
                    let th = *th;
                    move |el, s: f32| el.child(switch(s.clamp(0.0, 1.0), &th))
                },
            ));
        let about = div()
            .flex_1()
            .min_w(px(200.0))
            .flex()
            .flex_col()
            .gap(px(1.0))
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(th.text_faint))
                    .truncate()
                    .child(starter.name()),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(px(18.0))
                    .text_color(rgba(th.text_faint))
                    .child(summary),
            );
        let edit = icon_button(("starter-edit", ix), "pen", 18.0, th)
            .map(|d| self.page_control(d, th, cx))
            .flex_none()
            .size(px(32.0))
            .tooltip(tip(tr!("settings-rules-edit"), th))
            .on_click(cx.listener(move |this, _, window, cx| this.edit_starter(key, window, cx)));
        div()
            .id(("starter-row", ix))
            .mx(px(-8.0))
            .px(px(8.0))
            .py(px(6.0))
            .rounded(px(10.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(8.0))
            // Where a rule's drag handle is.
            .child(div().flex_none().w(px(20.0)).h(px(32.0)))
            .child(toggle)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_x(px(12.0))
                    .gap_y(px(6.0))
                    .child(about)
                    .child(edit),
            )
            .into_any_element()
    }

    /// The account filter's menu.
    fn render_rules_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let page = self.settings_page.as_ref()?;
        let at = page.rules.menu?;
        let current = page.rules.account;
        let pick = |account: Option<AccountId>| {
            cx.listener(move |this: &mut Self, _: &ClickEvent, _: &mut Window, cx| {
                if let Some(page) = &mut this.settings_page {
                    page.rules.account = account;
                    page.rules.menu = None;
                }
                cx.notify();
            })
        };
        let all = menu_item("rules-filter-all", &tr!("settings-rules-all-accounts"), th)
            .when(current.is_none(), |d| d.text_color(rgba(th.accent)))
            .on_click(pick(None));
        let accounts = self
            .rule_accounts()
            .into_iter()
            .enumerate()
            .map(|(n, (id, address))| {
                menu_item(("rules-filter-account", n), "", th)
                    .gap(px(10.0))
                    .when(current == Some(id), |d| d.text_color(rgba(th.accent)))
                    .child(dot(self.account_color(&address, th)))
                    .child(address)
                    .on_click(pick(Some(id)))
            })
            .collect::<Vec<_>>();
        let close = cx.listener(|this, _, _, cx| {
            cx.stop_propagation();
            this.close_rules_menu(cx);
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("rules-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(
                                menu(th)
                                    .id("rules-menu")
                                    .occlude()
                                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                        cx.stop_propagation()
                                    })
                                    .child(all)
                                    .children(accounts),
                            ),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

/// The order of every rule after the rules `shown` (some of them, in
/// their new order) were reordered: each shown rule takes the place of
/// one shown before, the others keep theirs.
fn reordered(all: &[Rule], shown: &[i64]) -> Vec<i64> {
    let mut next = shown.iter();
    all.iter()
        .map(|r| {
            if shown.contains(&r.id) {
                next.next().copied().unwrap_or(r.id)
            } else {
                r.id
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reordering_some_rules_keeps_the_others_in_place() {
        let all: Vec<Rule> = (1..=5)
            .map(|id| Rule {
                id,
                ..Rule::default()
            })
            .collect();
        // Rules 2, 4 and 5 shown; 5 dragged to the top of them.
        assert_eq!(reordered(&all, &[5, 2, 4]), [1, 5, 3, 2, 4]);
        assert_eq!(reordered(&all, &[2, 1, 3, 4, 5]), [2, 1, 3, 4, 5]);
    }
}
