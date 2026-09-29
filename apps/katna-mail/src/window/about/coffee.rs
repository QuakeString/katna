// SPDX-License-Identifier: GPL-3.0-or-later

//! A little play inside "Buy me a coffee": when the pointer comes to rest
//! on it, the label asks "Coffee?", "Tea?", "Pizza?" (each with its own
//! picture hopping in), pleads "Nothing? At all?" with a shake of the head,
//! is sad for a moment, settles for water with a bobbing droplet and blushes
//! with a heart floating up; then it wipes up to "Thank you for using
//! Katna" with sparkles, and returns to the button. It plays once each
//! time, never loops, and not at all with reduced motion. The button keeps
//! its size and stays clickable throughout.

use std::sync::{Arc, LazyLock};

use gpui::{AnyElement, Image, ImageFormat, ImageSource, div, img, prelude::*};
use katna_i18n::tr;
use katna_ui::motion::lerp;
use katna_ui::px;

use crate::widgets::icon;

/// Each step of the play and how long it stays, in ms, once it is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    /// The button as it always is.
    Button,
    /// A line of text, by message id, with a picture before it.
    Say(&'static str, Face),
    /// A face on its own.
    Face(Face),
    /// "Thank you for using Katna" with sparkles.
    Thanks,
}

const PLAY: &[(Step, u64)] = &[
    (Step::Button, 0),
    (Step::Say("about-coffee-coffee", Face::Coffee), 850),
    (Step::Say("about-coffee-tea", Face::Tea), 750),
    (Step::Say("about-coffee-pizza", Face::Pizza), 750),
    (Step::Say("about-coffee-nothing", Face::Pleading), 1300),
    (Step::Face(Face::Crying), 900),
    (Step::Say("about-coffee-water", Face::Droplet), 1500),
    (Step::Face(Face::Smiling), 1000),
    (Step::Thanks, 2600),
    (Step::Button, 0),
];

/// The emoji in the play, as pictures (`about/emoji/README.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Face {
    Coffee,
    Tea,
    Pizza,
    Pleading,
    Crying,
    Droplet,
    Smiling,
    Heart,
    Sparkles,
}

fn picture(face: Face) -> Arc<Image> {
    macro_rules! png {
        ($file:literal) => {{
            static IMAGE: LazyLock<Arc<Image>> = LazyLock::new(|| {
                Arc::new(Image::from_bytes(
                    ImageFormat::Png,
                    include_bytes!(concat!("../../../about/emoji/", $file)).to_vec(),
                ))
            });
            IMAGE.clone()
        }};
    }
    match face {
        Face::Coffee => png!("coffee.png"),
        Face::Tea => png!("tea.png"),
        Face::Pizza => png!("pizza.png"),
        Face::Pleading => png!("pleading-face.png"),
        Face::Crying => png!("crying-face.png"),
        Face::Droplet => png!("droplet.png"),
        Face::Smiling => png!("smiling-face.png"),
        Face::Heart => png!("yellow-heart.png"),
        Face::Sparkles => png!("sparkles.png"),
    }
}

/// `face`, `size` px square.
fn emoji(face: Face, size: f32) -> gpui::Img {
    img(ImageSource::Image(picture(face)))
        .size(px(size))
        .flex_none()
}

/// How long one step takes to scroll up into the next.
const SCROLL_MS: u64 = 320;
/// How long the wipe up to "Thank you" takes.
const WIPE_MS: u64 = 480;

/// Where the play is: moving `from` one step `to` the next, `p` of the
/// way (0 to 1, eased); ms since the play began, and since each of the two
/// steps began coming in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Scene {
    pub from: Step,
    pub to: Step,
    pub p: f32,
    pub ms: u64,
    pub from_ms: u64,
    pub to_ms: u64,
}

fn into_ms(step: Step) -> u64 {
    if step == Step::Thanks {
        WIPE_MS
    } else {
        SCROLL_MS
    }
}

/// How long the whole play lasts.
#[cfg(test)]
fn length() -> u64 {
    PLAY.iter()
        .skip(1)
        .map(|(step, hold)| into_ms(*step) + hold)
        .sum()
}

/// The scene `ms` after the play began, or `None` once it is over.
pub(super) fn scene(ms: u64) -> Option<Scene> {
    let (mut start, mut before) = (0, 0);
    for pair in PLAY.windows(2) {
        let ((from, _), (to, hold)) = (pair[0], pair[1]);
        let moving = into_ms(to);
        if ms < start + moving + hold {
            let t = (ms.saturating_sub(start)) as f32 / moving as f32;
            let t = t.min(1.0);
            // Ease out: quick to leave, gentle to land.
            let p = 1.0 - (1.0 - t).powi(3);
            return Some(Scene {
                from,
                to,
                p,
                ms,
                from_ms: ms - before,
                to_ms: ms - start,
            });
        }
        before = start;
        start += moving + hold;
    }
    None
}

/// 0 to 1 over `span` ms, starting `delay` ms in.
fn progress(ms: u64, delay: u64, span: u64) -> f32 {
    (ms.saturating_sub(delay) as f32 / span as f32).min(1.0)
}

/// Grows from half size past full and back, like a pop.
fn pop(t: f32) -> f32 {
    let (c1, c3) = (1.70158, 2.70158);
    let back = 1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2);
    lerp(0.5, 1.0, back)
}

/// The button's own content: its icon and "Buy me a coffee".
fn button(ink: u32) -> gpui::Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .child(icon("coffee", ink, 22.0))
        .child(tr!("about-coffee"))
}

/// Three sparkles that twinkle out of step, `ms` into the play.
fn sparkles(ms: u64) -> gpui::Div {
    let twinkle = |phase: f32, size: f32, (x, y): (f32, f32)| {
        let s = ((ms as f32 / 520.0 + phase) * std::f32::consts::TAU).sin();
        let k = 0.5 + 0.5 * s;
        let now = lerp(size * 0.65, size, k);
        // Grows and shrinks about its own middle.
        div()
            .absolute()
            .left(px(x + (size - now) / 2.0))
            .top(px(y + (size - now) / 2.0))
            .opacity(lerp(0.7, 1.0, k))
            .child(emoji(Face::Sparkles, now))
    };
    div()
        .relative()
        .flex_none()
        .size(px(32.0))
        .child(twinkle(0.0, 20.0, (0.0, 8.0)))
        .child(twinkle(0.33, 14.0, (16.0, 0.0)))
        .child(twinkle(0.66, 12.0, (19.0, 18.0)))
}

/// The picture before a line, `ms` after the line began coming in: food
/// hops once it lands, the droplet bobs, and the pleading face stays put
/// while the whole line shakes.
fn picture_for_line(face: Face, ms: u64) -> gpui::Div {
    let lift = match face {
        Face::Droplet => 2.5 * (ms as f32 / 190.0).sin(),
        Face::Pleading => 0.0,
        _ => -7.0 * (std::f32::consts::PI * progress(ms, SCROLL_MS, 300)).sin(),
    };
    div().relative().top(px(lift)).child(emoji(face, 22.0))
}

/// A face on its own, `ms` after it began coming in: it pops as it
/// lands, and the smile sends a small heart floating up.
fn face_alone(face: Face, ms: u64) -> gpui::Div {
    const SIZE: f32 = 26.0;
    let now = SIZE * pop(progress(ms, SCROLL_MS - 80, 380));
    let heart = (face == Face::Smiling).then(|| {
        let t = progress(ms, SCROLL_MS + 250, 700);
        let fade = if t < 0.6 {
            1.0
        } else {
            lerp(1.0, 0.0, (t - 0.6) / 0.4)
        };
        div()
            .absolute()
            .left(px(SIZE - 2.0))
            .top(px(lerp(2.0, -12.0, t)))
            .opacity(fade)
            .when(t > 0.0, |d| d.child(emoji(Face::Heart, 14.0)))
    });
    // A box of the full size, so the pop never moves the layout.
    div()
        .relative()
        .size(px(SIZE))
        .flex()
        .items_center()
        .justify_center()
        .child(emoji(face, now))
        .children(heart)
}

/// What one step shows, `ms` into the play and `at` ms after the step
/// began coming in.
fn content(step: Step, ms: u64, at: u64, ink: u32) -> gpui::Div {
    match step {
        Step::Button => button(ink),
        Step::Say(id, face) => {
            // "Nothing? At all?" shakes its head, dying away.
            let shake = if face == Face::Pleading {
                let t = progress(at, SCROLL_MS, 650);
                5.0 * (1.0 - t) * (t * 4.0 * std::f32::consts::TAU).sin()
            } else {
                0.0
            };
            div()
                .relative()
                .left(px(shake))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(picture_for_line(face, at))
                .child(tr!(id))
        }
        Step::Face(face) => face_alone(face, at),
        Step::Thanks => div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .child(tr!("about-coffee-thanks"))
            .child(sparkles(ms)),
    }
}

/// A layer the height of the button with `step` in its middle.
fn layer(step: Step, ms: u64, at: u64, ink: u32) -> gpui::Div {
    div()
        .absolute()
        .left_0()
        .w_full()
        .h(px(HEIGHT))
        .flex()
        .items_center()
        .justify_center()
        .child(content(step, ms, at, ink))
}

/// The button's height, which the play never changes.
pub(super) const HEIGHT: f32 = 42.0;

/// The inside of the button at `scene`: the step going out scrolls up and
/// fades as the next comes up from below; "Thank you" is wiped in from the
/// bottom instead.
pub(super) fn render(scene: Option<Scene>, ink: u32) -> AnyElement {
    let Some(Scene {
        from,
        to,
        p,
        ms,
        from_ms,
        to_ms,
    }) = scene
    else {
        return button(ink).into_any_element();
    };
    let out = layer(from, ms, from_ms, ink)
        .top(px(-HEIGHT * p))
        .opacity(1.0 - p);
    let incoming = if to == Step::Thanks {
        // A window growing up from the bottom edge, showing the step
        // where it will rest.
        div()
            .absolute()
            .left_0()
            .bottom_0()
            .w_full()
            .h(px(HEIGHT * p))
            .overflow_hidden()
            .child(layer(to, ms, to_ms, ink).bottom_0())
    } else {
        layer(to, ms, to_ms, ink)
            .top(px(HEIGHT * (1.0 - p)))
            .opacity(p)
    };
    div()
        .relative()
        .size_full()
        .overflow_hidden()
        .when(p < 1.0, |d| d.child(out))
        .child(incoming)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays_every_step_once_and_ends() {
        let mut seen = Vec::new();
        for ms in (0..length()).step_by(10) {
            let scene = scene(ms).expect("still playing");
            if seen.last() != Some(&scene.to) {
                seen.push(scene.to);
            }
        }
        let steps: Vec<Step> = PLAY.iter().skip(1).map(|(s, _)| *s).collect();
        assert_eq!(seen, steps);
        assert_eq!(scene(length()), None);
        assert_eq!(steps.last(), Some(&Step::Button), "ends on the button");
    }

    #[test]
    fn every_line_is_in_english() {
        for (step, _) in PLAY {
            if let Step::Say(id, _) = step {
                assert_ne!(tr!(id), *id, "{id}");
            }
        }
        assert_ne!(tr!("about-coffee-thanks"), "about-coffee-thanks");
    }

    #[test]
    fn moves_smoothly() {
        let first = scene(0).unwrap();
        assert_eq!((first.from, first.p), (Step::Button, 0.0));
        let settled = scene(SCROLL_MS + 10).unwrap();
        assert_eq!(settled.p, 1.0);
        assert_eq!(settled.to_ms, SCROLL_MS + 10);
        for ms in 0..length() {
            let scene = scene(ms).unwrap();
            assert!((0.0..=1.0).contains(&scene.p));
            assert!(scene.from_ms >= scene.to_ms);
        }
    }

    #[test]
    fn pops_land_at_full_size() {
        assert!((pop(0.0) - 0.5).abs() < 1e-4);
        assert!((pop(1.0) - 1.0).abs() < 1e-4);
        assert!((0..=100).any(|i| pop(i as f32 / 100.0) > 1.0), "overshoots");
    }
}
