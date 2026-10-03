// SPDX-License-Identifier: GPL-3.0-or-later

//! Motion: springs a view drives itself, and the spring settings Katna uses.
//!
//! GPUI's `with_spring` animates one element. [`Spring`] is for values that
//! shape several elements at once (a sidebar width that also fades labels),
//! kept in the view and advanced on each frame.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use gpui::{SpringConfig, SpringState, Window};

/// How long motion takes compared with normal (1.0): Settings >
/// Appearance > Animation speed, or the desktop's. Every [`Spring`],
/// [`scaled`] spring and [`time`] follows it.
static SPEED: AtomicU32 = AtomicU32::new(0x3f80_0000); // 1.0

/// The slowest and fastest motion can be set to.
pub const SPEED_RANGE: (f32, f32) = (0.25, 4.0);

/// Sets how long motion takes compared with normal, within
/// [`SPEED_RANGE`]. Turning motion off is the reduce-motion setting's job.
pub fn set_speed(factor: f32) {
    let factor = if factor.is_finite() { factor } else { 1.0 };
    let factor = factor.clamp(SPEED_RANGE.0, SPEED_RANGE.1);
    SPEED.store(factor.to_bits(), Ordering::Relaxed);
}

/// How long motion takes compared with normal.
pub fn speed() -> f32 {
    f32::from_bits(SPEED.load(Ordering::Relaxed))
}

/// `config` slowed or sped up by [`speed`]: the same shape over a longer
/// or shorter time. For GPUI's `SpringAnimation`; a [`Spring`] follows the
/// speed by itself.
pub fn scaled(config: SpringConfig) -> SpringConfig {
    scale_config(config, speed())
}

fn scale_config(config: SpringConfig, factor: f32) -> SpringConfig {
    if factor == 1.0 {
        return config;
    }
    // Time stretched by f: stiffness over f², damping over f.
    SpringConfig::new(
        config.stiffness / (factor * factor),
        config.damping / factor,
        config.mass,
    )
}

/// A timed animation's `duration`, slowed or sped up by [`speed`].
pub fn time(duration: Duration) -> Duration {
    duration.mul_f32(speed())
}

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
        let value = self.step(reduce_motion);
        if !self.settled() {
            window.request_animation_frame();
        }
        value
    }

    /// Advances the spring to now and returns its value, for a caller
    /// with no `Window` at hand: while it is not [`settled`](Self::settled),
    /// the caller asks for the next frame itself.
    pub fn step(&mut self, reduce_motion: bool) -> f32 {
        if reduce_motion {
            self.snap(self.target);
            return self.target;
        }
        self.advance(Instant::now());
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
        let dt = now.saturating_duration_since(self.at).as_secs_f32() / speed();
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
    fn scaled_springs_take_longer_or_shorter() {
        let start = SpringState {
            position: 0.0,
            velocity: 0.0,
        };
        let at = |config: SpringConfig, secs: f32| config.step(start, 1.0, secs).position;
        let slow = scale_config(SLIDE, 2.0);
        assert!((at(slow, 0.2) - at(SLIDE, 0.1)).abs() < 1e-3);
        let fast = scale_config(SMOOTH, 0.5);
        assert!((at(fast, 0.05) - at(SMOOTH, 0.1)).abs() < 1e-3);
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
