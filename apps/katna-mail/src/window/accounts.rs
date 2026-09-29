// SPDX-License-Identifier: GPL-3.0-or-later

//! The Accounts part of the Settings page: the accounts Katna syncs, with
//! a way to remove each one, and a way to delete everything Katna keeps.
//! Both ask first in a dialog that says in red what goes, and that mail on
//! the server stays. The daemon does the deleting. Resetting the cache (in
//! General) asks in the same dialog, saying what is downloaded again and
//! what is kept.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    AnyElement, Bounds, Context, Div, DragMoveEvent, ElementId, Entity, Focusable, FontWeight,
    MouseButton, MouseDownEvent, Pixels, SharedString, Stateful, Subscription, Window, canvas,
    deferred, div, prelude::*, rgba,
};
use katna_core::config::AccountsShown;
use katna_core::{Account, AccountId, AccountKind, Config};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::unpx;
use katna_ui::{InputEvent, TextInput};

use super::settings::Change;
use super::{Listing, MailWindow, keymap};
use crate::daemon;
use crate::data::Mail;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, elevation, icon};

const WIDTH: f32 = 500.0;

/// A question before deleting.
pub(super) struct Danger {
    what: What,
    busy: bool,
    error: Option<String>,
    closing: bool,
    /// The dialog has had the keys given to it.
    focused: bool,
    shown: Spring,
}

/// An account's name being changed in Settings > Accounts.
pub(super) struct Renaming {
    account: AccountId,
    input: Entity<TextInput>,
    _subscription: Subscription,
}

/// An account being dragged to a new place in Settings > Accounts. The
/// row itself follows the pointer, so what GPUI draws under it is empty.
#[derive(Clone)]
struct AccountDrag {
    ix: usize,
}

impl Render for AccountDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// The space between two account rows.
const ROW_GAP: f32 = 4.0;

/// Reordering Settings > Accounts: the row being dragged lifts and follows
/// the pointer, the rows it passes slide out of its way, and after a drop
/// or a Move up or down every row glides from where it was to its place.
#[derive(Default)]
pub(super) struct Reorder {
    /// Each row's bounds as last painted, offset included.
    bounds: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
    /// Where the pointer went down on a row's handle.
    grab: Option<(usize, f32)>,
    moving: Option<Moving>,
    /// Rows gliding to their place after the order changed, by account.
    gliding: Vec<(AccountId, Spring)>,
    /// The row whose Move up or down has the keyboard focus.
    focused: Option<usize>,
}

/// A drag under way.
struct Moving {
    from: usize,
    /// Where the row would go if dropped now.
    to: usize,
    grab_y: f32,
    pointer_y: f32,
    /// The other rows moving aside.
    slides: Vec<Spring>,
    /// Each row's offset as drawn this frame.
    drawn: Vec<f32>,
}

impl Reorder {
    /// How far row `ix` is drawn from its place.
    fn offset(&self, ix: usize, id: AccountId) -> f32 {
        match &self.moving {
            Some(moving) => moving.drawn.get(ix).copied().unwrap_or(0.0),
            None => self
                .gliding
                .iter()
                .find(|(g, _)| *g == id)
                .map_or(0.0, |(_, spring)| spring.value()),
        }
    }

    /// Whether row `ix` is drawn above the others: lifted or landing.
    fn raised(&self, ix: usize, id: AccountId) -> bool {
        match &self.moving {
            Some(moving) => moving.from == ix,
            None => self.gliding.first().is_some_and(|(g, _)| *g == id),
        }
    }

    /// Every row's place and height as last painted, or `None` before
    /// they all are.
    fn places(&self, count: usize) -> Option<Vec<(f32, f32)>> {
        let bounds = self.bounds.borrow();
        (0..count)
            .map(|ix| {
                let b = (*bounds.get(ix)?)?;
                Some((unpx(b.top()), unpx(b.size.height)))
            })
            .collect()
    }

    /// The order changed from `old` to `new` (accounts, first to last)
    /// with the rows drawn at `places`: each row glides from there to its
    /// new place, the first of `raised` above the rest.
    fn glide(
        &mut self,
        old: &[AccountId],
        new: &[AccountId],
        places: &[(f32, f32)],
        raised: Option<AccountId>,
    ) {
        let (Some(&first), Some(&(painted, _))) = (old.first(), places.first()) else {
            return;
        };
        // Where the first row goes with no offset.
        let mut top = painted - self.offset(0, first);
        let mut gliding = Vec::new();
        for id in new {
            let Some(old_ix) = old.iter().position(|o| o == id) else {
                continue;
            };
            let (was, height) = places[old_ix];
            if (was - top).abs() > 0.5 {
                let mut spring = Spring::new(motion::SLIDE, was - top);
                spring.set(0.0);
                gliding.push((*id, spring));
            }
            top += height + ROW_GAP;
        }
        if let Some(raised) = raised
            && let Some(ix) = gliding.iter().position(|(g, _)| *g == raised)
        {
            let lifted = gliding.remove(ix);
            gliding.insert(0, lifted);
        }
        self.gliding = gliding;
    }
}

enum What {
    RemoveAccount(Account),
    DeleteAll {
        typed: Entity<TextInput>,
        _subscription: Subscription,
    },
    ResetCache,
}

/// What the daemon did for the dialog.
enum Done {
    Removed(Account),
    DeletedAll,
    /// Messages that lost their body, and bytes deleted.
    CacheReset(u64, u64),
}

impl MailWindow {
    pub(super) fn accounts_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        let list = div()
            .flex()
            .flex_col()
            .gap(px(ROW_GAP))
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<AccountDrag>, _, cx| {
                    let from = event.drag(cx).ix;
                    this.account_dragged(from, unpx(event.event.position.y), cx)
                }),
            )
            .on_drop(cx.listener(|this, _: &AccountDrag, _, cx| this.drop_account(cx)))
            .children({
                let count = accounts.len();
                accounts
                    .iter()
                    .enumerate()
                    .map(|(ix, account)| self.account_row(ix, count, account, th, cx))
                    .collect::<Vec<_>>()
            })
            .when(self.accounts.is_empty(), |d| {
                d.child(
                    div()
                        .py(px(8.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("accounts-none")),
                )
            })
            .child(
                div().pt(px(8.0)).flex().child(
                    crate::widgets::outlined_button("account-add-page", tr!("account-add"), th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.open_add_account(window, cx)),
                        ),
                ),
            );
        let delete_all = div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("accounts-delete-all-about")),
            )
            .child(
                danger_button(
                    "delete-all-open",
                    tr!("accounts-delete-all-open"),
                    false,
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(cx.listener(|this, _, window, cx| this.ask_delete_all(window, cx))),
            );
        let shown = self.config.mail.accounts_shown;
        let mut pane = div().flex().flex_col().gap(px(2.0));
        for (choice, id, label) in [
            (
                AccountsShown::One,
                "page-accounts-one",
                tr!("accounts-shown-one"),
            ),
            (
                AccountsShown::All,
                "page-accounts-all",
                tr!("accounts-shown-all"),
            ),
        ] {
            pane = pane.child(self.radio_row(
                id,
                label,
                shown == choice,
                Change::AccountsShown(choice),
                th,
                cx,
            ));
        }
        let unified = self.config.mail.unified_inbox;
        div()
            .flex()
            .flex_col()
            .child(self.row(
                tr!("accounts-folder-pane"),
                Some(tr!("accounts-folder-pane-detail").as_str()),
                pane,
                th,
            ))
            .child(self.row(
                tr!("accounts-unified"),
                None,
                self.switch_row(
                    "page-accounts-unified",
                    tr!("accounts-unified-switch"),
                    tr!("accounts-unified-switch-detail"),
                    unified,
                    Change::UnifiedInbox(!unified),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                tr!("accounts-row"),
                Some(tr!("accounts-row-detail").as_str()),
                list,
                th,
            ))
            .child(self.row(
                tr!("accounts-delete-all-row"),
                Some(tr!("accounts-delete-all-row-detail").as_str()),
                delete_all,
                th,
            ))
            .into_any_element()
    }

    /// One account in Settings > Accounts: a handle to drag it, its
    /// picture, its name (or the field renaming it), the buttons for its
    /// name and picture, Remove, and Move up and Move down.
    fn account_row(
        &self,
        ix: usize,
        count: usize,
        account: &Account,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = if account.display_name.trim().is_empty() {
            account.address.clone()
        } else {
            account.display_name.clone()
        };
        let id = account.id;
        let renaming = self
            .settings_page
            .as_ref()
            .and_then(|p| p.renaming.as_ref())
            .filter(|r| r.account == id);
        let about = div()
            .flex_grow(1.0)
            .flex_basis(px(180.0))
            .min_w_0()
            .flex()
            .flex_col()
            .map(|d| match renaming {
                Some(renaming) => d.child(
                    div()
                        .id(("account-name-field", ix))
                        .h(px(36.0))
                        .px(px(12.0))
                        .flex()
                        .items_center()
                        .rounded(px(8.0))
                        .border_2()
                        .border_color(rgba(th.accent))
                        .text_size(px(14.0))
                        .child(div().flex_1().min_w_0().child(renaming.input.clone())),
                ),
                None => d.child(
                    div()
                        .truncate()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(name.clone()),
                ),
            })
            .child(
                div()
                    .truncate()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(format!(
                        "{} \u{b7} {}",
                        account.address,
                        kind_name(account.kind)
                    )),
            );
        let own = self.remote.has_own_picture(id);
        let desktop = self.remote.has_desktop_picture();
        let buttons = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .map(|d| match renaming {
                Some(_) => d
                    .child(
                        text_button(("account-name-save", ix), tr!("accounts-name-save"), th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(|this, _, _, cx| this.finish_rename(true, cx))),
                    )
                    .child(
                        text_button(("account-name-cancel", ix), tr!("accounts-name-cancel"), th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(|this, _, _, cx| this.finish_rename(false, cx))),
                    ),
                None => d.child(
                    text_button(("account-rename", ix), tr!("accounts-rename"), th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.start_rename(id, window, cx)
                        })),
                ),
            })
            .child(
                text_button(("account-picture", ix), tr!("accounts-picture-change"), th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_account_picture(id, cx))),
            )
            .when(!own && desktop, |d| {
                d.child(
                    text_button(
                        ("account-picture-desktop", ix),
                        tr!("accounts-picture-reset"),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.use_desktop_picture(id, cx))),
                )
            })
            .when(own, |d| {
                d.child(
                    text_button(
                        ("account-picture-remove", ix),
                        tr!("accounts-picture-remove"),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.reset_account_picture(id, cx)),
                    ),
                )
            })
            .child({
                let account = account.clone();
                danger_button(("account-remove", ix), tr!("accounts-remove"), false, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.ask(What::RemoveAccount(account.clone()), cx)
                    }))
            });
        let arrow = |dir: &'static str, to: Option<usize>, label: String| {
            let button = crate::widgets::icon_button(
                (dir, ix),
                if dir == "account-up" {
                    "chevron-up"
                } else {
                    "chevron-down"
                },
                20.0,
                th,
            )
            .tooltip(crate::widgets::tip(label, th));
            match to {
                Some(to) => button
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.move_account(ix, to, cx))),
                None => button.opacity(0.3).cursor_default(),
            }
        };
        let reorder = self.settings_page.as_ref().map(|p| &p.reorder);
        let offset = reorder.map_or(0.0, |r| r.offset(ix, id));
        let raised = reorder.is_some_and(|r| r.raised(ix, id));
        let dragging = reorder.and_then(|r| r.moving.as_ref());
        let lifted = dragging.is_some_and(|m| m.from == ix);
        // Move up and Move down show on the row the pointer is over, or
        // whose button has the keyboard focus; not on the rows a drag
        // passes over.
        let arrows_shown =
            lifted || (dragging.is_none() && reorder.is_some_and(|r| r.focused == Some(ix)));
        let group = SharedString::from(format!("account-row-{ix}"));
        let handle = div()
            .id(("account-drag", ix))
            .flex_none()
            .size(px(36.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_grab()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(crate::widgets::tip(tr!("accounts-drag"), th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, _| {
                    if let Some(page) = this.settings_page.as_mut() {
                        page.reorder.grab = Some((ix, unpx(event.position.y)));
                    }
                }),
            )
            .on_drag(AccountDrag { ix }, |drag, _, _, cx| {
                cx.new(|_| drag.clone())
            })
            .child(icon("drag-handle", th.text_faint, 20.0));
        let avatar = self.person_avatar(&name, &account.address, 36.0);
        let bounds = reorder.map(|r| r.bounds.clone());
        // The shadow of a lifted row, fading as it lands.
        let lift = if lifted {
            3.0
        } else if raised {
            (offset.abs() / 8.0).min(3.0)
        } else {
            0.0
        };
        // The buttons go below the name, together, where the row is
        // narrow.
        let row = div()
            .id(("account-row", ix))
            .group(group.clone())
            .relative()
            .top(px(offset))
            .mx(px(-8.0))
            .px(px(8.0))
            .py(px(6.0))
            .rounded(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .when(raised, |d| {
                d.bg(rgba(th.surface)).shadow(elevation(th, lift))
            })
            .when_some(bounds, |d, bounds| {
                d.child(
                    canvas(
                        move |b, _, _| {
                            let mut all = bounds.borrow_mut();
                            if all.len() <= ix {
                                all.resize(ix + 1, None);
                            }
                            all[ix] = Some(b);
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
            })
            .child(handle)
            .child(avatar)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .min_h(px(36.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_x(px(12.0))
                    .gap_y(px(4.0))
                    .child(about)
                    .child(buttons),
            )
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .when(!arrows_shown, |d| {
                        d.opacity(0.0).when(dragging.is_none(), |d| {
                            d.group_hover(group, |s| s.opacity(1.0))
                        })
                    })
                    .child(arrow(
                        "account-up",
                        ix.checked_sub(1),
                        tr!("accounts-move-up"),
                    ))
                    .child(arrow(
                        "account-down",
                        (ix + 1 < count).then_some(ix + 1),
                        tr!("accounts-move-down"),
                    )),
            );
        // Drawn over the rows after it while lifted or landing.
        if raised {
            deferred(row).with_priority(1).into_any_element()
        } else {
            row.into_any_element()
        }
    }

    /// The General row's button and what it does.
    pub(super) fn reset_cache_control(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .px(px(8.0))
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("reset-cache-about")),
            )
            .child(
                crate::widgets::outlined_button("reset-cache-open", tr!("reset-cache-button"), th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(|this, _, _, cx| this.ask(What::ResetCache, cx))),
            )
            .into_any_element()
    }

    /// Moves the mail account at `from` to `to` in Settings > Accounts;
    /// the folder pane, the account menu and every other list follow. The
    /// rows glide to their new places, a dragged one from where it was
    /// dropped.
    fn move_account(&mut self, from: usize, to: usize, cx: &mut Context<Self>) {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        let old: Vec<AccountId> = accounts.iter().map(|a| a.id).collect();
        if let Some(page) = self.settings_page.as_mut() {
            let reorder = &mut page.reorder;
            reorder.grab = None;
            if from < old.len() && to < old.len() {
                let mut new = old.clone();
                let moved = new.remove(from);
                new.insert(to, moved);
                if let Some(places) = reorder.places(old.len()) {
                    reorder.glide(&old, &new, &places, Some(moved));
                }
            }
            reorder.moving = None;
        }
        if from != to {
            self.config.mail.move_account(&accounts, from, to);
            self.save_config();
            self.load_tree();
        }
        cx.notify();
    }

    /// The pointer moved while dragging the account at `from` to `y`.
    fn account_dragged(&mut self, from: usize, y: f32, cx: &mut Context<Self>) {
        let count = self.accounts.iter().filter(|a| a.kind.is_mail()).count();
        let ids: Vec<AccountId> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .map(|a| a.id)
            .collect();
        let Some(page) = self.settings_page.as_mut() else {
            return;
        };
        let reorder = &mut page.reorder;
        if from >= count {
            return;
        }
        if reorder.moving.is_none() {
            let grab_y = match reorder.grab.take() {
                Some((ix, grab_y)) if ix == from => grab_y,
                _ => y,
            };
            // Rows still gliding carry on from where they are.
            let drawn: Vec<f32> = ids
                .iter()
                .enumerate()
                .map(|(ix, id)| reorder.offset(ix, *id))
                .collect();
            let mut slides: Vec<Spring> = drawn
                .iter()
                .map(|&at| Spring::new(motion::SLIDE, at))
                .collect();
            for slide in &mut slides {
                slide.set(0.0);
            }
            reorder.gliding.clear();
            reorder.moving = Some(Moving {
                from,
                to: from,
                grab_y,
                pointer_y: y,
                slides,
                drawn,
            });
        }
        let places = reorder.places(count);
        let Some(moving) = reorder.moving.as_mut() else {
            return;
        };
        moving.pointer_y = y;
        if let Some(places) = places {
            let natural: Vec<(f32, f32)> = places
                .iter()
                .zip(&moving.drawn)
                .map(|(&(top, height), drawn)| (top - drawn, height))
                .collect();
            let (top, height) = natural[moving.from];
            let center = top + height / 2.0 + (y - moving.grab_y);
            let to = natural
                .iter()
                .enumerate()
                .filter(|&(ix, &(top, height))| ix != moving.from && top + height / 2.0 < center)
                .count();
            if to != moving.to {
                moving.to = to;
                let step = height + ROW_GAP;
                for (ix, slide) in moving.slides.iter_mut().enumerate() {
                    slide.set(if ix > moving.from && ix <= to {
                        -step
                    } else if ix < moving.from && ix >= to {
                        step
                    } else {
                        0.0
                    });
                }
            }
        }
        cx.notify();
    }

    /// The dragged account was let go: it goes where the rows made room.
    fn drop_account(&mut self, cx: &mut Context<Self>) {
        let Some((from, to)) = self
            .settings_page
            .as_ref()
            .and_then(|p| p.reorder.moving.as_ref())
            .map(|m| (m.from, m.to))
        else {
            return;
        };
        self.move_account(from, to, cx);
    }

    /// Advances the rows sliding in Settings > Accounts for this frame.
    pub(super) fn tick_reorder(&mut self, window: &Window, reduce: bool, cx: &mut Context<Self>) {
        let dropped = self
            .settings_page
            .as_ref()
            .is_some_and(|p| p.reorder.moving.is_some())
            && !cx.has_active_drag();
        if dropped {
            // Let go somewhere with nowhere to drop it.
            self.drop_account(cx);
        }
        let count = self.accounts.iter().filter(|a| a.kind.is_mail()).count();
        let Some(page) = self.settings_page.as_mut() else {
            return;
        };
        let stops = page.tab_stops().clone();
        let reorder = &mut page.reorder;
        reorder.focused = (0..count).find(|&ix| {
            ["account-up", "account-down"]
                .into_iter()
                .any(|dir| stops.focused(&ElementId::from((dir, ix)), window))
        });
        reorder.bounds.borrow_mut().resize(count, None);
        let places = reorder.places(count);
        if let Some(moving) = reorder.moving.as_mut() {
            for (ix, slide) in moving.slides.iter_mut().enumerate() {
                if ix != moving.from {
                    moving.drawn[ix] = slide.tick(window, reduce);
                }
            }
            let mut lifted = moving.pointer_y - moving.grab_y;
            // The lifted row stays between the first row's top and the
            // last row's bottom.
            if let Some(places) = places
                && let Some(last) = places.last()
            {
                let at = |ix: usize| places[ix].0 - moving.drawn[ix];
                let top = at(0);
                let bottom = at(count - 1) + last.1;
                let (own, height) = (at(moving.from), places[moving.from].1);
                lifted = lifted.clamp(top - own, (bottom - height - own).max(top - own));
            }
            moving.drawn[moving.from] = lifted;
        }
        reorder.gliding.retain_mut(|(_, spring)| {
            spring.tick(window, reduce);
            !spring.settled()
        });
    }

    fn start_rename(&mut self, account: AccountId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(current) = self.accounts.iter().find(|a| a.id == account) else {
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let text = current.display_name.clone();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("accounts-name-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input.select_all_text(cx);
            input
        });
        let subscription = cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.finish_rename(true, cx),
            InputEvent::Cancel => this.finish_rename(false, cx),
            InputEvent::Changed => {}
        });
        window.focus(&input.focus_handle(cx), cx);
        if let Some(page) = self.settings_page.as_mut() {
            page.renaming = Some(Renaming {
                account,
                input,
                _subscription: subscription,
            });
        }
        cx.notify();
    }

    /// Saves the typed name (`save`), or leaves the old one.
    fn finish_rename(&mut self, save: bool, cx: &mut Context<Self>) {
        let Some(renaming) = self.settings_page.as_mut().and_then(|p| p.renaming.take()) else {
            return;
        };
        cx.notify();
        if !save {
            return;
        }
        let name = renaming.input.read(cx).text().trim().to_owned();
        let id = renaming.account.0;
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::rename_account(&connection, id, &name).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(()) => this.load_tree(),
                    Err(err) => {
                        this.show_snackbar(tr!("accounts-rename-failed", error = err), None, cx)
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn ask(&mut self, what: What, cx: &mut Context<Self>) {
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.danger = Some(Danger {
            what,
            busy: false,
            error: None,
            closing: false,
            focused: false,
            shown,
        });
        cx.notify();
    }

    fn ask_delete_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let accent = rgba(self.theme(window).error).into();
        let typed = cx.new(|cx| {
            let mut input = TextInput::new(tr!("accounts-confirm-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &typed,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.confirm_danger(cx),
                    InputEvent::Cancel => {
                        this.close_danger(cx);
                    }
                    _ => cx.notify(),
                },
            );
        window.focus(&typed.focus_handle(cx), cx);
        self.ask(
            What::DeleteAll {
                typed,
                _subscription: subscription,
            },
            cx,
        );
        // Its field has the keys already.
        if let Some(danger) = &mut self.danger {
            danger.focused = true;
        }
    }

    /// Closes the dialog unless it is at work. Returns whether it closed.
    pub(super) fn close_danger(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(danger) = self.danger.as_mut().filter(|d| !d.busy && !d.closing) else {
            return false;
        };
        danger.closing = true;
        danger.shown.set(0.0);
        cx.notify();
        true
    }

    /// Whether the dialog's red button may be pressed.
    fn danger_ready(&self, cx: &Context<Self>) -> bool {
        match &self.danger {
            Some(Danger { busy: true, .. }) | None => false,
            Some(Danger {
                what: What::DeleteAll { typed, .. },
                ..
            }) => {
                // The English word also works, for a keyboard without the
                // language's letters or accents.
                let typed = typed.read(cx).text().trim().to_lowercase();
                typed == tr!("accounts-confirm-word").to_lowercase() || typed == "delete"
            }
            Some(_) => true,
        }
    }

    fn confirm_danger(&mut self, cx: &mut Context<Self>) {
        if !self.danger_ready(cx) {
            return;
        }
        let Some(danger) = &mut self.danger else {
            return;
        };
        danger.busy = true;
        danger.error = None;
        let remove = match &danger.what {
            What::RemoveAccount(account) => Some(account.clone()),
            What::DeleteAll { .. } | What::ResetCache => None,
        };
        let reset = matches!(danger.what, What::ResetCache);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    match remove {
                        Some(account) => daemon::remove_account(&connection, account.id.0)
                            .await
                            .map(|()| Done::Removed(account)),
                        None if reset => daemon::reset_cache(&connection)
                            .await
                            .map(|(messages, bytes)| Done::CacheReset(messages, bytes)),
                        None => daemon::delete_all_data(&connection)
                            .await
                            .map(|()| Done::DeletedAll),
                    }
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(done) => {
                    if let Some(danger) = &mut this.danger {
                        danger.busy = false;
                    }
                    this.close_danger(cx);
                    match done {
                        Done::Removed(account) => this.account_removed(&account, cx),
                        Done::DeletedAll => this.all_data_deleted(cx),
                        Done::CacheReset(messages, bytes) => this.cache_reset(messages, bytes, cx),
                    }
                }
                Err(err) => {
                    if let Some(danger) = &mut this.danger {
                        danger.busy = false;
                        danger.error = Some(err);
                    }
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn account_removed(&mut self, account: &Account, cx: &mut Context<Self>) {
        if self
            .config
            .mail
            .account_tabs
            .remove(&account.address.to_lowercase())
            .is_some()
        {
            self.save_config();
        }
        let listed = self
            .folder
            .is_some_and(|f| self.tree.account_of(f) == Some(account.id));
        if listed
            || matches!(
                self.listing,
                Some(Listing::Search { .. } | Listing::Unified { .. })
            )
        {
            self.close_listing(cx);
        }
        self.refresh(true, cx);
        let address = account.address.as_str();
        let text = if account.kind == AccountKind::Local {
            tr!("accounts-removed-local", address = address)
        } else {
            tr!("accounts-removed", address = address)
        };
        self.show_snackbar(text, None, cx);
    }

    /// Back to a first start: the daemon deleted every file and exited.
    fn all_data_deleted(&mut self, cx: &mut Context<Self>) {
        self.config = Config::default();
        keymap::bind(&self.config.shortcuts, cx);
        self.compose = None;
        self.unsent = None;
        self.add_account = None;
        self.settings_page = None;
        self.close_listing(cx);
        self.unread.clear();
        self.mail = Mail::open(&self.paths);
        self.load_tree();
        self.show_snackbar(tr!("accounts-all-deleted"), None, cx);
    }

    /// The daemon deleted what it downloaded and is downloading it again.
    fn cache_reset(&mut self, messages: u64, bytes: u64, cx: &mut Context<Self>) {
        self.refresh(true, cx);
        let text = if messages == 0 {
            tr!("reset-cache-done")
        } else {
            tr!("reset-cache-done-freed", size = crate::format::size(bytes))
        };
        self.show_snackbar(text, None, cx);
    }

    /// Lists nothing, so the next refresh opens the first inbox.
    fn close_listing(&mut self, cx: &mut Context<Self>) {
        self.listing = None;
        self.folder = None;
        self.unified = None;
        self.reader = None;
        self.reading = false;
        self.entries.clear();
        self.tabs.clear();
        self.tab = 0;
        self.selected = None;
        self.checked.clear();
        self.clear_search(cx);
    }

    pub(super) fn render_danger(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let danger = self.danger.as_mut()?;
        let t = danger.shown.tick(window, reduce);
        if danger.closing && danger.shown.settled() {
            self.danger = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        // The keys go to the dialog (to its field, if it has one).
        if !danger.focused {
            danger.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let ready = self.danger_ready(cx);
        let danger = self.danger.as_ref()?;
        let (title, action, busy_text, items): (String, String, String, Vec<String>) =
            match &danger.what {
                What::ResetCache => (
                    tr!("reset-cache-title"),
                    tr!("reset-cache-confirm"),
                    tr!("reset-cache-busy"),
                    vec![
                        tr!("reset-cache-mail"),
                        tr!("reset-cache-index"),
                        tr!("reset-cache-pictures"),
                    ],
                ),
                What::RemoveAccount(account) => {
                    let folders = self.tree.folders_of(account.id).len();
                    (
                        tr!("accounts-remove-title", address = account.address.as_str()),
                        tr!("accounts-remove-confirm"),
                        tr!("accounts-removing"),
                        if account.kind == AccountKind::Local {
                            vec![
                                tr!("accounts-remove-local-mail", folders = folders),
                                tr!("accounts-remove-local-settings"),
                            ]
                        } else {
                            vec![
                                tr!("accounts-remove-mail", folders = folders),
                                tr!("accounts-remove-outbox"),
                                tr!("accounts-remove-settings"),
                            ]
                        },
                    )
                }
                What::DeleteAll { .. } => (
                    tr!("accounts-delete-all-title"),
                    tr!("accounts-delete-all-confirm"),
                    tr!("accounts-deleting"),
                    vec![
                        tr!("accounts-delete-all-accounts"),
                        tr!("accounts-delete-all-contacts"),
                        tr!("accounts-delete-all-settings"),
                        tr!("accounts-delete-all-passwords"),
                    ],
                ),
            };
        let reset = matches!(danger.what, What::ResetCache);
        // Resetting deletes nothing that cannot be downloaded again, so it
        // is not red.
        let tone = if reset { th.accent } else { th.error };
        let warning = div()
            .mt(px(20.0))
            .p(px(16.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(fade(tone, 0.45)))
            .bg(rgba(fade(tone, 0.1)))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(tone))
                    .child(if reset {
                        tr!("reset-cache-deleted")
                    } else {
                        tr!("accounts-deleted-heading")
                    }),
            )
            .children(items.into_iter().map(|item| {
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .child(div().text_color(rgba(tone)).child("\u{2022}"))
                    .child(div().flex_1().min_w_0().child(item))
            }))
            .when(!reset, |d| {
                d.child(
                    div()
                        .pt(px(4.0))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(tone))
                        .child(tr!("accounts-cannot-undo")),
                )
            });
        let server = div()
            .mt(px(12.0))
            .p(px(12.0))
            .rounded(px(12.0))
            .bg(rgba(fade(th.accent, 0.08)))
            .flex()
            .flex_row()
            .gap(px(10.0))
            .child(icon("info", th.accent, 20.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(13.0))
                    .line_height(px(19.0))
                    .text_color(rgba(th.text_dim))
                    .child(match &danger.what {
                        What::ResetCache => tr!("reset-cache-kept"),
                        What::DeleteAll { .. } => tr!("accounts-server-delete-all"),
                        What::RemoveAccount(account) if account.kind == AccountKind::Local => {
                            tr!("accounts-server-local")
                        }
                        What::RemoveAccount(_) => tr!("accounts-server-remove"),
                    }),
            );
        let confirm = match &danger.what {
            What::DeleteAll { typed, .. } => {
                let focus = typed.focus_handle(cx);
                Some(
                    div()
                        .mt(px(20.0))
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .child(tr!("accounts-confirm-prompt")),
                        )
                        .child(
                            div()
                                .id("delete-all-typed")
                                .h(px(44.0))
                                .px(px(14.0))
                                .flex()
                                .items_center()
                                .rounded(px(8.0))
                                .border_2()
                                .border_color(rgba(if ready {
                                    th.error
                                } else {
                                    fade(th.text_faint, 0.8)
                                }))
                                .text_size(px(15.0))
                                .cursor_text()
                                .on_click(move |_, window, cx| window.focus(&focus, cx))
                                .child(div().flex_1().child(typed.clone())),
                        ),
                )
            }
            What::RemoveAccount(_) | What::ResetCache => None,
        };
        let error = danger.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(err)
        });
        let busy = danger.busy;
        let body = div()
            .id("danger-body")
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .px(px(32.0))
            .pt(px(28.0))
            .pb(px(24.0))
            .child(
                div()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(tone, 0.14)))
                    .child(if reset {
                        icon("refresh", tone, 28.0)
                    } else {
                        icon("warning", tone, 28.0)
                    }),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .text_size(px(22.0))
                    .line_height(px(30.0))
                    .child(title),
            )
            .child(warning)
            .child(server)
            .children(confirm)
            .children(error)
            .child(
                div()
                    .mt(px(28.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().flex_1())
                    .child(
                        div()
                            .id("danger-cancel")
                            .focus_ring(th)
                            .h(px(36.0))
                            .px(px(16.0))
                            .flex()
                            .items_center()
                            .rounded_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_danger(cx);
                            }))
                            .child(tr!("accounts-cancel")),
                    )
                    .child(
                        {
                            let label = if busy { busy_text } else { action };
                            if reset {
                                crate::widgets::filled_button("danger-confirm", label, th)
                            } else {
                                danger_button("danger-confirm", label, true, th)
                            }
                        }
                        .focus_ring_filled(th)
                        .when(!ready, |d| d.opacity(0.45).cursor_default())
                        .on_click(cx.listener(|this, _, _, cx| this.confirm_danger(cx))),
                    ),
            );
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let card = div()
            .id("danger")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(200.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(28.0))
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .child(body);
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
                        .id("danger-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_danger(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

/// A red button: outlined in the page, filled in the dialog.
/// A quiet button: accent text, a background under the pointer.
fn text_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(36.0))
        .px(px(12.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label.into())
}

fn danger_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    filled: bool,
    th: &Theme,
) -> Stateful<Div> {
    let label = label.into();
    let button = div()
        .id(id)
        .flex_none()
        .h(px(36.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer();
    if filled {
        button
            .bg(rgba(th.error))
            .text_color(rgba(th.surface))
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .child(label)
    } else {
        button
            .border_1()
            .border_color(rgba(fade(th.error, 0.6)))
            .text_color(rgba(th.error))
            .hover(|s| s.bg(rgba(fade(th.error, 0.08))))
            .child(label)
    }
}

/// The account's type; protocol names stay as they are.
fn kind_name(kind: AccountKind) -> String {
    match kind {
        AccountKind::Imap => "IMAP".into(),
        AccountKind::Jmap => "JMAP".into(),
        AccountKind::Pop3 => "POP3".into(),
        AccountKind::Local => tr!("accounts-kind-imported"),
        AccountKind::CalDav => "CalDAV".into(),
        AccountKind::CardDav => "CardDAV".into(),
    }
}
