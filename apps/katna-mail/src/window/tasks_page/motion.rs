// SPDX-License-Identifier: GPL-3.0-or-later

//! How the Tasks page's rows and cards move (`docs/DESIGN.md`, Motion),
//! after `widgets::fold_box`: a row's height glides on the `SLIDE` spring
//! as it comes and goes, with a fade; a ticked row keeps its place a
//! moment, tick filled and title struck, then folds away; rows glide to a
//! new place when their list is sorted; a card rises from level 1 to
//! level 2 under the pointer in `FAST`. Springs go through
//! `motion::scaled` and timed fades through `motion::time`, so Reduce
//! motion and the animation speed apply.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, BoxShadow, Context, Div, ElementId, SharedString,
    SpringAnimation, Stateful, canvas, div, ease_out_quint, prelude::*,
};
use katna_ui::motion::{self, lerp};
use katna_ui::tokens::{duration, elevation as level};
use katna_ui::{px, unpx};

use super::TasksPage;
use crate::theme::Theme;
use crate::widgets::{CARD_REST, card_shadow, elevation};
use crate::window::MailWindow;

/// How long a row ticked off stays where it was, filled and struck,
/// before it folds away.
const HOLD: Duration = duration::SLOW;
/// Longer than a glide on `SLIDE` takes to settle, as
/// `widgets::fold_box` counts it.
const GLIDE: Duration = Duration::from_millis(700);

/// A row ticked off or back on a moment ago.
pub(super) struct Checking {
    /// What it turns to.
    pub(super) done: bool,
    at: Instant,
}

/// What moves on the page, by task.
#[derive(Default)]
pub(super) struct Rows {
    /// Rows ticked or unticked a moment ago: they keep their place until
    /// they have folded away.
    pub(super) checking: HashMap<i64, Checking>,
    /// Rows deleted a moment ago, folding away: when, and the tasks as
    /// they were, kept on the page until then.
    leaving: HashMap<i64, Instant>,
    /// Rows just come, gliding open.
    arriving: HashSet<i64>,
    /// Each row's height, and each task's with its steps, as last drawn.
    heights: Rc<RefCell<HashMap<i64, f32>>>,
    groups: Rc<RefCell<HashMap<i64, f32>>>,
    /// Each list's tasks in the order last drawn.
    orders: RefCell<HashMap<i64, Vec<i64>>>,
    /// Tasks gliding to a new place: from how far above it, and the
    /// glide's number.
    glides: RefCell<HashMap<i64, (f32, usize)>>,
    serial: Cell<usize>,
    /// Cards under the pointer, or left a moment ago: whether it is
    /// over, and the number of the change, which names the rise.
    lift: HashMap<SharedString, (bool, usize)>,
}

impl Rows {
    /// Whether task `id` is ticked off as the page places it: as it was,
    /// while it is still folding away from there.
    pub(super) fn placed_done(&self, id: i64) -> Option<bool> {
        self.checking.get(&id).map(|c| !c.done)
    }

    /// Whether task `id` is ticked off as its tick shows it: as it turns
    /// to, from the click on.
    pub(super) fn ticking(&self, id: i64) -> Option<bool> {
        self.checking.get(&id).map(|c| c.done)
    }

    /// Whether the row of task `id` is folding away now.
    fn folding(&self, id: i64) -> bool {
        self.leaving.contains_key(&id)
            || self
                .checking
                .get(&id)
                .is_some_and(|c| c.at.elapsed() >= motion::time(HOLD))
    }

    /// Whether task `id` is being deleted and still folding away.
    pub(super) fn leaving(&self, id: i64) -> bool {
        self.leaving.contains_key(&id)
    }

    /// The row of task `id`, measured, gliding open when it has just
    /// come and folding away when it goes.
    pub(super) fn row(&self, id: i64, content: AnyElement) -> AnyElement {
        let heights = self.heights.clone();
        let inner = div().flex_none().relative().child(content).child(
            canvas(
                move |bounds, _, _| {
                    heights.borrow_mut().insert(id, unpx(bounds.size.height));
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        );
        // One spring for the row's whole life, aiming at 1 (shown) or 0
        // (folded away), so the tree under it stays the same when a row
        // starts to fold and the tick's spring, the glow and the ripple
        // in it carry on rather than start again.
        let spring = SpringAnimation::new(motion::scaled(motion::SLIDE)).to(if self.folding(id) {
            0.0
        } else {
            1.0
        });
        let spring = if self.arriving.contains(&id) {
            spring.from(0.0)
        } else {
            spring
        };
        let heights = self.heights.clone();
        div()
            .flex_none()
            .child(inner)
            .with_spring(
                ElementId::from((SharedString::from("task-row-motion"), id as usize)),
                spring,
                move |el, s: f32| {
                    if s >= 0.999 {
                        return el;
                    }
                    let s = s.max(0.0);
                    let h = heights.borrow().get(&id).copied().unwrap_or(0.0);
                    el.overflow_hidden().h(px(h * s)).opacity(s)
                },
            )
            .into_any_element()
    }

    /// Task `id` with its steps, measured for [`Rows::reorder`].
    pub(super) fn group(&self, id: i64, group: Stateful<Div>) -> Stateful<Div> {
        let groups = self.groups.clone();
        group.relative().child(
            canvas(
                move |bounds, _, _| {
                    groups.borrow_mut().insert(id, unpx(bounds.size.height));
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
    }

    /// The glide of task `id`'s group, if it is moving to a new place:
    /// wraps it so it starts where it was.
    pub(super) fn glide(&self, id: i64, group: AnyElement) -> AnyElement {
        let Some((from, serial)) = self.glides.borrow().get(&id).copied() else {
            return group;
        };
        div()
            .relative()
            .child(group)
            .with_spring(
                ElementId::from((SharedString::from(format!("task-glide-{id}")), serial)),
                SpringAnimation::new(motion::scaled(motion::SMOOTH))
                    .to(1.0)
                    .from(0.0),
                move |el, s: f32| el.top(px(from * (1.0 - s))),
            )
            .into_any_element()
    }

    /// List `list` now shows its tasks in `order`. Sorted into a new
    /// order of the same tasks, each glides from where it was.
    pub(super) fn reorder(&self, list: i64, order: &[i64]) {
        let mut orders = self.orders.borrow_mut();
        let before = orders.insert(list, order.to_vec());
        let Some(before) = before else {
            return;
        };
        if before == order {
            return;
        }
        let same: HashSet<i64> = before.iter().copied().collect();
        if same.len() != order.len() || !order.iter().all(|id| same.contains(id)) {
            return;
        }
        let groups = self.groups.borrow();
        let tops = |order: &[i64]| {
            let mut top = 0.0;
            let mut at = HashMap::new();
            for id in order {
                at.insert(*id, top);
                top += groups.get(id).copied().unwrap_or(0.0);
            }
            at
        };
        let (was, now) = (tops(&before), tops(order));
        let serial = self.serial.get() + 1;
        self.serial.set(serial);
        let mut glides = self.glides.borrow_mut();
        for id in order {
            let from = was[id] - now[id];
            if from.abs() > 0.5 {
                glides.insert(*id, (from, serial));
            }
        }
    }
}

/// A card's shadow `t` of the way from level 1 at rest to level 2.
pub(super) fn lift_shadow(th: &Theme, t: f32) -> Vec<BoxShadow> {
    let mut shadows = card_shadow(th, lerp(CARD_REST, 1.0, t));
    shadows.extend(elevation(th, level::FLOAT * t));
    shadows
}

impl TasksPage {
    /// `card`, a level 1 card or panel, rising to level 2 while the
    /// pointer is over it, in `FAST`.
    pub(super) fn lifted(
        &self,
        card: Stateful<Div>,
        key: impl Into<SharedString>,
        th: &Theme,
        cx: &mut Context<MailWindow>,
    ) -> AnyElement {
        let key: SharedString = key.into();
        let state = self.motion.lift.get(&key).copied();
        let named = key.clone();
        let card = card.on_hover(cx.listener(move |this, over: &bool, _, cx| {
            let lift = &mut this.tasks.motion.lift;
            if lift.get(&named).map(|(on, _)| *on) == Some(*over) {
                return;
            }
            let n = lift.get(&named).map_or(0, |(_, n)| n + 1);
            lift.insert(named.clone(), (*over, n));
            cx.notify();
        }));
        let Some((over, n)) = state else {
            return card.into_any_element();
        };
        let th = *th;
        card.with_animation(
            ElementId::from((SharedString::from(format!("{key}-lift")), n)),
            Animation::new(motion::time(duration::FAST)).with_easing(ease_out_quint()),
            move |el, t| {
                let t = if over { t } else { 1.0 - t };
                el.shadow(lift_shadow(&th, t))
            },
        )
        .into_any_element()
    }
}

impl MailWindow {
    /// Task `id` was just ticked off (`done`) or back on: its row keeps
    /// its place, tick filled and title struck, then folds away. Not with
    /// Reduce motion.
    pub(super) fn task_check_motion(&mut self, id: i64, done: bool, cx: &mut Context<Self>) {
        if cx.reduce_motion() {
            return;
        }
        // Clicked again before it went: it stays, as it was.
        if self.tasks.motion.checking.remove(&id).is_some() {
            return;
        }
        self.tasks.motion.checking.insert(
            id,
            Checking {
                done,
                at: Instant::now(),
            },
        );
        let (hold, glide) = (motion::time(HOLD), motion::time(GLIDE));
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(hold).await;
            this.update(cx, |_, cx| cx.notify()).ok();
            cx.background_executor().timer(glide).await;
            this.update(cx, |this, cx| {
                if this
                    .tasks
                    .motion
                    .checking
                    .get(&id)
                    .is_some_and(|c| c.at.elapsed() >= hold + glide)
                {
                    this.tasks.motion.checking.remove(&id);
                    // It glides open in its new place.
                    this.task_arrive_motion(vec![id], cx);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Tasks `ids` were just deleted: their rows fold away, and only then
    /// leave the page. `gone` takes them off it.
    pub(super) fn task_leave_motion(
        &mut self,
        ids: &[i64],
        gone: fn(&mut Self, &[i64]),
        cx: &mut Context<Self>,
    ) {
        if cx.reduce_motion() {
            gone(self, ids);
            return;
        }
        let at = Instant::now();
        for id in ids {
            self.tasks.motion.leaving.insert(*id, at);
        }
        let ids = ids.to_vec();
        let glide = motion::time(GLIDE);
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(glide).await;
            this.update(cx, |this, cx| {
                for id in &ids {
                    this.tasks.motion.leaving.remove(id);
                }
                gone(this, &ids);
                // Read again, for a task put back with Undo meanwhile.
                this.load_tasks(cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Tasks `ids` just came (read back new): their rows glide open.
    pub(super) fn task_arrive_motion(&mut self, ids: Vec<i64>, cx: &mut Context<Self>) {
        if ids.is_empty() || cx.reduce_motion() {
            return;
        }
        self.tasks.motion.arriving.extend(ids.iter().copied());
        let glide = motion::time(GLIDE);
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(glide).await;
            this.update(cx, |this, cx| {
                for id in &ids {
                    this.tasks.motion.arriving.remove(id);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}
