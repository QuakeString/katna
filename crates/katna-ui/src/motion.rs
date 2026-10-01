// SPDX-License-Identifier: GPL-3.0-or-later

//! Motion: springs a view drives itself, and the spring settings Katna uses.
//!
//! GPUI's `with_spring` animates one element. [`Spring`] is for values that
//! shape several elements at once (a sidebar width that also fades labels),
//! kept in the view and advanced on each frame.

use std::time::Instant;

use gpui::{SpringConfig, SpringState, Window};

/// Critically damped, settles in about 250 ms: fades and color changes.
pub const SMOOTH: SpringConfig = SpringConfig::new(700.0, 52.9, 1.0);
/// Critically damped, settles in about 450 ms: slow, quiet fades.
pub const GENTLE: SpringConfig = SpringConfig::new(216.0, 29.4, 1.0);
/// Slightly bouncy, settles in about 350 ms: panels that slide or grow.
pub const SLIDE: SpringConfig = SpringConfig::new(420.0, 34.0, 1.0);
/// Fast and critically damped: hover feedback.
pub const QUICK: SpringConfig = SpringConfig::new(1600.0, 80.0, 1.0);

/// Values closer than this to the target count as settled.
const EPSILON: f32 = 0.002;

/// A spring toward a target that can change at any time without a jump.
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    config: SpringConfig,
    state: SpringState,
    target: f32,
    at: Instant,
}

impl Spring {
    /// A spring at rest at `value`.
    pub fn new(config: SpringConfig, value: f32) -> Self {
        Self {
            config,
            state: SpringState {
                position: value,
                velocity: 0.0,
            },
            target: value,
            at: Instant::now(),
        }
    }

    /// Moves the target; the current position and velocity carry on.
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.advance(Instant::now());
            self.target = target;
        }
    }

    pub fn target(&self) -> f32 {
        self.target
    }

    /// Jumps to `value` with no motion.
    pub fn snap(&mut self, value: f32) {
        self.target = value;
        self.state = SpringState {
            position: value,
            velocity: 0.0,
        };
        self.at = Instant::now();
    }

    /// Advances the spring to now and returns its value. While it moves,
    /// asks `window` for another frame. Honors the reduce-motion setting.
    pub fn tick(&mut self, window: &Window, reduce_motion: bool) -> f32 {
        if reduce_motion {
            self.snap(self.target);
            return self.target;
        }
        self.advance(Instant::now());
        if !self.settled() {
            window.request_animation_frame();
        }
        self.state.position
    }

    /// The value at the last tick.
    pub fn value(&self) -> f32 {
        self.state.position
    }

    pub fn settled(&self) -> bool {
        self.state
            == SpringState {
                position: self.target,
                velocity: 0.0,
            }
    }

    fn advance(&mut self, now: Instant) {
        let dt = now.saturating_duration_since(self.at).as_secs_f32();
        self.at = now;
        if self.settled() || dt == 0.0 {
            return;
        }
        self.state = self.config.step(self.state, self.target, dt);
        if self.config.is_settled(self.state, self.target, EPSILON) {
            self.state = SpringState {
                position: self.target,
                velocity: 0.0,
            };
        }
    }
}

/// Linear interpolation from `a` to `b`.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn run(spring: &mut Spring, time: Duration) {
        spring.at -= time;
        spring.advance(Instant::now());
    }

    #[test]
    fn settles_on_the_target() {
        for config in [SMOOTH, SLIDE, QUICK] {
            let mut spring = Spring::new(config, 0.0);
            spring.set(1.0);
            assert!(!spring.settled());
            run(&mut spring, Duration::from_millis(40));
            let early = spring.value();
            assert!(early > 0.0 && early < 1.0, "{early}");
            run(&mut spring, Duration::from_secs(2));
            assert!(spring.settled());
            assert_eq!(spring.value(), 1.0);
        }
    }

    #[test]
    fn retargeting_keeps_the_position() {
        let mut spring = Spring::new(SMOOTH, 0.0);
        spring.set(1.0);
        run(&mut spring, Duration::from_millis(60));
        let at = spring.value();
        spring.set(0.0);
        assert!((spring.value() - at).abs() < 0.05);
    }

    #[test]
    fn smooth_does_not_overshoot() {
        let mut spring = Spring::new(SMOOTH, 0.0);
        spring.set(1.0);
        for _ in 0..100 {
            run(&mut spring, Duration::from_millis(8));
            assert!(spring.value() <= 1.0 + 1e-4, "{}", spring.value());
        }
    }
}
