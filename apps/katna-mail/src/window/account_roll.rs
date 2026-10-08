// SPDX-License-Identifier: GPL-3.0-or-later

//! The account picture at the top right: a turn of the mouse wheel over
//! it switches to the next or the previous account, in the order of the
//! account card, as picking one there does. The picture rolls from the
//! old account's to the new, as the app's name at the top left rolls:
//! down for the next account, up for the previous.

use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, IntoElement, ParentElement, ScrollDelta, ScrollWheelEvent, Styled, div,
};
use katna_core::AccountId;
use katna_ui::{px, unpx};

use super::MailWindow;
use crate::sidebar::Role;

/// How far the wheel turns for one step: one notch of a mouse wheel, or
/// about as far on a touchpad.
const LINES_PER_STEP: f32 = 1.0;
const PIXELS_PER_STEP: f32 = 48.0;
/// The least time between two steps, so a fast spin moves one step per
/// notch and a touchpad's fling does not race to the end.
const STEP_GAP: Duration = Duration::from_millis(140);
/// A pause in the wheel after which a part-turned notch is forgotten.
const WHEEL_IDLE: Duration = Duration::from_millis(400);

/// What the account picture is rolling from.
pub(super) struct AvatarRoll {
    /// The account whose picture rolls out.
    pub(super) from: Option<AccountId>,
    /// True when the new picture comes in from above (the next account).
    pub(super) down: bool,
    wheel: Notches,
}

impl AvatarRoll {
    pub(super) fn new() -> Self {
        Self {
            from: None,
            down: true,
            wheel: Notches::default(),
        }
    }

    fn turn(&mut self, delta: ScrollDelta, now: Instant) -> i32 {
        self.wheel.turn(delta, now)
    }
}

/// Turns of the mouse wheel counted out in notches, for a control the
/// wheel steps through (the account picture, the Files page's dates).
#[derive(Default)]
pub(super) struct Notches {
    /// The wheel's turn not yet spent on a step.
    wheel: f32,
    /// When the wheel last turned, and when it last stepped.
    wheel_at: Option<Instant>,
    stepped_at: Option<Instant>,
}

impl Notches {
    /// Adds a turn of the wheel; returns +1 for a notch down, -1 for one
    /// up, or 0 while the turn is short of a notch or too soon after the
    /// last step.
    pub(super) fn turn(&mut self, delta: ScrollDelta, now: Instant) -> i32 {
        if self
            .wheel_at
            .is_some_and(|at| now.duration_since(at) > WHEEL_IDLE)
        {
            self.wheel = 0.0;
        }
        self.wheel_at = Some(now);
        let steps = match delta {
            ScrollDelta::Lines(lines) => -lines.y / LINES_PER_STEP,
            ScrollDelta::Pixels(pixels) => -unpx(pixels.y) / PIXELS_PER_STEP,
        };
        // A turn back the other way starts over.
        if steps * self.wheel < 0.0 {
            self.wheel = 0.0;
        }
        self.wheel += steps;
        if self.wheel.abs() < 1.0 {
            return 0;
        }
        let step = self.wheel.signum() as i32;
        self.wheel = 0.0;
        if self
            .stepped_at
            .is_some_and(|at| now.duration_since(at) < STEP_GAP)
        {
            return 0;
        }
        self.stepped_at = Some(now);
        step
    }
}

impl MailWindow {
    /// The account the picture at the top right shows.
    pub(super) fn pictured_account(&self) -> Option<&katna_core::Account> {
        self.account()
            .and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .or_else(|| self.accounts.first())
    }

    /// The mouse wheel over the account picture.
    pub(super) fn wheel_accounts(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        let step = self.avatar_roll.turn(event.delta, Instant::now());
        if step == 0 {
            return;
        }
        // All Accounts is the first stop, before the first account, where
        // the folder pane has it.
        let all = self.shows_unified();
        let ix = if all && self.menu_current() == Some(None) {
            -1
        } else {
            let Some(ix) = self
                .pictured_account()
                .and_then(|shown| self.accounts.iter().position(|a| a.id == shown.id))
            else {
                return;
            };
            ix as isize
        };
        // No wrap past either end: the last account stays until the wheel
        // turns back.
        let next = ix + step as isize;
        if next == -1 && all {
            self.pick_all_accounts(cx);
            return;
        }
        let Some(next) = usize::try_from(next)
            .ok()
            .and_then(|next| self.accounts.get(next))
        else {
            return;
        };
        let id = next.id;
        self.pick_account(id, cx);
    }

    /// Shows `account`, from the account card or the wheel: with one
    /// account at a time it becomes the one on show, else its inbox opens.
    /// The page on show stays: on Calendar, Tasks, Contacts or Notes the
    /// account becomes the one new items go to, and Mail shows it on the
    /// way back. The picture rolls to it.
    pub(super) fn pick_account(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let from = self.pictured_account().map(|a| a.id);
        self.settings_page = None;
        if self.shown_account().is_some() {
            self.switch_account(account, cx);
        } else if let Some(inbox) = self.tree.role_folder(account, Role::Inbox) {
            self.open_folder(inbox, cx);
        }
        let to = self.pictured_account().map(|a| a.id);
        if from != to {
            let order = |id| self.accounts.iter().position(|a| Some(a.id) == id);
            self.avatar_roll.down = order(to) > order(from);
            self.avatar_roll.from = from;
            self.avatar_turn.snap(0.0);
            self.avatar_turn.set(1.0);
        }
        cx.notify();
    }

    /// The account picture, `size` across, rolling from the last account's
    /// while a switch plays.
    pub(super) fn render_rolling_avatar(&self, size: f32) -> AnyElement {
        let face = |id: AccountId| {
            self.accounts.iter().find(|a| a.id == id).map(|account| {
                let name = if account.display_name.trim().is_empty() {
                    &account.address
                } else {
                    &account.display_name
                };
                self.person_avatar(name, &account.address, size)
            })
        };
        let roll = self.avatar_turn.value();
        let shown = self.pictured_account().map(|a| a.id);
        let from = self
            .avatar_roll
            .from
            .filter(|from| roll < 0.999 && Some(*from) != shown);
        // Down: the old picture leaves at the bottom and the new one comes
        // from the top; up is the mirror.
        let sign = if self.avatar_roll.down { 1.0 } else { -1.0 };
        let place = |top: f32, opacity: f32, face: AnyElement| {
            div()
                .absolute()
                .left_0()
                .top(px(top))
                .size(px(size))
                .opacity(opacity.clamp(0.0, 1.0))
                .child(face)
        };
        let mut frame = div()
            .relative()
            .flex_none()
            .size(px(size))
            .rounded_full()
            .overflow_hidden();
        if let Some(old) = from.and_then(face) {
            frame = frame.child(place(sign * size * roll, 1.0 - roll, old));
        }
        if let Some(new) = shown.and_then(face) {
            let (top, opacity) = if from.is_some() {
                (-sign * size * (1.0 - roll), roll)
            } else {
                (0.0, 1.0)
            };
            frame = frame.child(place(top, opacity, new));
        }
        frame.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use gpui::point;

    use super::*;

    fn lines(y: f32) -> ScrollDelta {
        ScrollDelta::Lines(point(0.0, y))
    }

    #[test]
    fn one_notch_moves_one_account() {
        let mut roll = AvatarRoll::new();
        let now = Instant::now();
        // The wheel turned down: the next account.
        assert_eq!(roll.turn(lines(-1.0), now), 1);
        // A second notch right after is too soon.
        assert_eq!(roll.turn(lines(-1.0), now + Duration::from_millis(30)), 0);
        assert_eq!(roll.turn(lines(-1.0), now + Duration::from_millis(200)), 1);
        // Up: the previous one.
        assert_eq!(roll.turn(lines(1.0), now + Duration::from_millis(400)), -1);
    }

    #[test]
    fn a_touchpad_needs_a_notch_worth() {
        let mut roll = AvatarRoll::new();
        let now = Instant::now();
        let pixels = |y: f32| ScrollDelta::Pixels(point(px(0.0), px(y)));
        assert_eq!(roll.turn(pixels(-20.0), now), 0);
        assert_eq!(roll.turn(pixels(-20.0), now + Duration::from_millis(10)), 0);
        assert_eq!(roll.turn(pixels(-20.0), now + Duration::from_millis(20)), 1);
        // A pause forgets a part-turned notch.
        assert_eq!(roll.turn(pixels(30.0), now + Duration::from_millis(900)), 0);
        assert_eq!(
            roll.turn(pixels(10.0), now + Duration::from_millis(1500)),
            0
        );
    }
}
