// Katna: kinetic (momentum) scrolling for touchpads on Wayland.
//
// The compositor sends two-finger scrolling as `wl_pointer.axis` events with
// the `finger` source and ends it with `axis_stop` when the fingers lift. On
// macOS the system keeps a flick gliding; on Wayland each client does it
// itself (GTK and Qt do), so without this a flick stops dead like a mouse
// wheel. `Kinetic` remembers the last moments of finger scrolling and, when
// the fingers lift while still moving, gives a `Glide` that slows down
// exponentially.

use std::collections::VecDeque;

/// Only finger scrolling this recent (ms before the lift) sets the speed.
const WINDOW_MS: u32 = 100;
/// Fingers that rested this long (ms) before lifting do not glide.
const MAX_PAUSE_MS: u32 = 50;
/// Slower flicks than this (px per ms) do not glide.
const START_SPEED: f32 = 0.25;
/// A glide ends once it is this slow (px per ms).
const STOP_SPEED: f32 = 0.02;
/// Fastest glide (px per ms), so a hard flick cannot fly off.
const MAX_SPEED: f32 = 10.0;
/// Time constant of the slowdown (ms): speed falls to 37% after this long.
const TAU_MS: f32 = 400.0;

/// Recent finger scrolling of the current gesture.
#[derive(Default)]
pub(crate) struct Kinetic {
    samples: VecDeque<(u32, [f32; 2])>,
}

impl Kinetic {
    /// Forget the gesture, as when another kind of scrolling starts.
    pub(crate) fn clear(&mut self) {
        self.samples.clear();
    }

    /// Finger scrolling of `delta` px at `time` (ms, the compositor's clock).
    pub(crate) fn push(&mut self, time: u32, delta: [f32; 2]) {
        while let Some(&(t, _)) = self.samples.front() {
            if time.wrapping_sub(t) > WINDOW_MS {
                self.samples.pop_front();
            } else {
                break;
            }
        }
        self.samples.push_back((time, delta));
    }

    /// The fingers lifted at `time`: the glide to run, if they were moving.
    pub(crate) fn release(&mut self, time: u32) -> Option<Glide> {
        let samples = std::mem::take(&mut self.samples);
        let recent: Vec<_> = samples
            .into_iter()
            .filter(|&(t, _)| time.wrapping_sub(t) <= WINDOW_MS)
            .collect();
        let (last, _) = *recent.last()?;
        if time.wrapping_sub(last) > MAX_PAUSE_MS || recent.len() < 2 {
            return None;
        }
        // The first sample only marks when the window starts; the distance
        // after it was covered in the time up to the last sample.
        let span = last.wrapping_sub(recent[0].0).max(1) as f32;
        let mut distance = [0.0f32; 2];
        for &(_, d) in &recent[1..] {
            distance[0] += d[0];
            distance[1] += d[1];
        }
        let mut velocity = [distance[0] / span, distance[1] / span];
        let speed = velocity[0].hypot(velocity[1]);
        if speed < START_SPEED {
            return None;
        }
        if speed > MAX_SPEED {
            velocity = velocity.map(|v| v * MAX_SPEED / speed);
        }
        Some(Glide {
            velocity,
            elapsed: 0.0,
        })
    }
}

/// A flick gliding on after the fingers lifted.
#[derive(Clone, Debug)]
pub(crate) struct Glide {
    /// Speed at the lift, px per ms.
    velocity: [f32; 2],
    /// Time since the lift, ms.
    elapsed: f32,
}

impl Glide {
    /// Moves on by `dt` ms: the distance covered (px), or `None` once the
    /// glide has come to rest.
    pub(crate) fn advance(&mut self, dt: f32) -> Option<[f32; 2]> {
        let before = (-self.elapsed / TAU_MS).exp();
        let speed = self.velocity[0].hypot(self.velocity[1]) * before;
        if speed < STOP_SPEED {
            return None;
        }
        self.elapsed += dt.max(0.0);
        let after = (-self.elapsed / TAU_MS).exp();
        let part = TAU_MS * (before - after);
        Some([self.velocity[0] * part, self.velocity[1] * part])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A flick of `steps` finger events `every` ms apart, each `delta` px.
    fn flick(kinetic: &mut Kinetic, start: u32, steps: u32, every: u32, delta: f32) -> u32 {
        let mut t = start;
        for _ in 0..steps {
            kinetic.push(t, [0.0, delta]);
            t = t.wrapping_add(every);
        }
        t.wrapping_sub(every)
    }

    fn run(mut glide: Glide) -> (f32, u32) {
        let mut total = 0.0;
        let mut ticks = 0;
        while let Some(d) = glide.advance(8.0) {
            total += d[1];
            ticks += 1;
            assert!(ticks < 10_000, "the glide never stops");
        }
        (total, ticks)
    }

    #[test]
    fn a_quick_flick_glides_and_slows_down() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 1000, 10, 8, 16.0); // 2 px per ms
        let mut glide = k.release(last + 4).expect("a flick glides");
        let first = glide.advance(8.0).unwrap()[1];
        let second = glide.advance(8.0).unwrap()[1];
        assert!(first > 0.0 && second > 0.0 && second < first);
        // About the speed of the fingers at first: 2 px/ms over 8 ms.
        assert!((first - 16.0).abs() < 1.0, "{first}");
        let (rest, ticks) = run(glide);
        // Distance ≈ v·τ = 800 px, over about τ·ln(v/stop) ≈ 1.8 s.
        assert!((700.0..800.0).contains(&(first + second + rest)));
        assert!((150..300).contains(&ticks), "{ticks}");
    }

    #[test]
    fn direction_follows_the_fingers() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 0, 10, 8, -16.0);
        let (total, _) = run(k.release(last).unwrap());
        assert!(total < -500.0);
    }

    #[test]
    fn slow_scrolling_does_not_glide() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 0, 20, 8, 1.0); // 0.125 px per ms
        assert!(k.release(last + 4).is_none());
    }

    #[test]
    fn resting_before_lifting_does_not_glide() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 0, 10, 8, 16.0);
        assert!(k.release(last + 80).is_none());
    }

    #[test]
    fn only_the_end_of_the_gesture_counts() {
        let mut k = Kinetic::default();
        // Fast at first, then slow for the last 150 ms.
        let t = flick(&mut k, 0, 10, 8, 40.0);
        let last = flick(&mut k, t + 8, 20, 8, 0.5);
        assert!(k.release(last + 2).is_none());
    }

    #[test]
    fn a_lone_event_does_not_glide() {
        let mut k = Kinetic::default();
        k.push(5, [0.0, 50.0]);
        assert!(k.release(6).is_none());
    }

    #[test]
    fn release_forgets_the_gesture() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 0, 10, 8, 16.0);
        assert!(k.release(last).is_some());
        assert!(k.release(last).is_none());
    }

    #[test]
    fn a_hard_flick_is_capped() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, 0, 10, 8, 400.0); // 50 px per ms
        let mut glide = k.release(last).unwrap();
        let first = glide.advance(1.0).unwrap()[1];
        assert!(first <= MAX_SPEED + 0.01, "{first}");
    }

    #[test]
    fn the_compositor_clock_may_wrap() {
        let mut k = Kinetic::default();
        let last = flick(&mut k, u32::MAX - 40, 10, 8, 16.0);
        assert!(k.release(last).is_some());
    }
}
