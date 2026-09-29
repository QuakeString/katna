// SPDX-License-Identifier: GPL-3.0-or-later

//! A little play inside "Buy me a coffee": when the pointer comes to rest
//! on it, the label asks "Coffee?", "Tea?", "Pizza?", "Nothing? At all?",
//! is sad for a moment, settles for water and blushes; then it wipes up to
//! "Thank you for using Katna" with sparkles, and returns to the button.
//! It plays once each time, never loops, and not at all with reduced
//! motion. The button keeps its size and stays clickable throughout.

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
    /// A line of text, by message id.
    Say(&'static str),
    /// A face on its own.
    Face(Face),
    /// "Thank you for using Katna" with sparkles.
    Thanks,
}

const PLAY: &[(Step, u64)] = &[
    (Step::Button, 0),
    (Step::Say("about-coffee-coffee"), 850),
    (Step::Say("about-coffee-tea"), 750),
    (Step::Say("about-coffee-pizza"), 750),
    (Step::Say("about-coffee-nothing"), 1300),
    (Step::Face(SAD), 900),
    (Step::Say("about-coffee-water"), 1500),
    (Step::Face(BLUSH), 1000),
    (Step::Thanks, 2600),
    (Step::Button, 0),
];

const SAD: Face = Face::Crying;
const BLUSH: Face = Face::Smiling;

/// The emoji in the play, as pictures (`about/emoji/README.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Face {
    Crying,
    Smiling,
    Sparkles,
}

fn picture(face: Face) -> Arc<Image> {
    static CRYING: LazyLock<Arc<Image>> =
        LazyLock::new(|| png(include_bytes!("../../../about/emoji/crying-face.png")));
    static SMILING: LazyLock<Arc<Image>> =
        LazyLock::new(|| png(include_bytes!("../../../about/emoji/smiling-face.png")));
    static SPARKLES: LazyLock<Arc<Image>> =
        LazyLock::new(|| png(include_bytes!("../../../about/emoji/sparkles.png")));
    match face {
        Face::Crying => CRYING.clone(),
        Face::Smiling => SMILING.clone(),
        Face::Sparkles => SPARKLES.clone(),
    }
}

fn png(bytes: &[u8]) -> Arc<Image> {
    Arc::new(Image::from_bytes(ImageFormat::Png, bytes.to_vec()))
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
/// way (0 to 1, eased), and ms since the play began.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Scene {
    pub from: Step,
    pub to: Step,
    pub p: f32,
    pub ms: u64,
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
    let mut start = 0;
    for pair in PLAY.windows(2) {
        let ((from, _), (to, hold)) = (pair[0], pair[1]);
        let moving = into_ms(to);
        if ms < start + moving + hold {
            let t = (ms.saturating_sub(start)) as f32 / moving as f32;
            let t = t.min(1.0);
            // Ease out: quick to leave, gentle to land.
            let p = 1.0 - (1.0 - t).powi(3);
            return Some(Scene { from, to, p, ms });
        }
        start += moving + hold;
    }
    None
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

/// What one step shows.
fn content(step: Step, ms: u64, ink: u32) -> gpui::Div {
    match step {
        Step::Button => button(ink),
        Step::Say(id) => div().child(tr!(id)),
        Step::Face(face) => div().child(emoji(face, 26.0)),
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
fn layer(step: Step, ms: u64, ink: u32) -> gpui::Div {
    div()
        .absolute()
        .left_0()
        .w_full()
        .h(px(HEIGHT))
        .flex()
        .items_center()
        .justify_center()
        .child(content(step, ms, ink))
}

/// The button's height, which the play never changes.
pub(super) const HEIGHT: f32 = 42.0;

/// The inside of the button at `scene`: the step going out scrolls up and
/// fades as the next comes up from below; "Thank you" is wiped in from the
/// bottom instead.
pub(super) fn render(scene: Option<Scene>, ink: u32) -> AnyElement {
    let Some(Scene { from, to, p, ms }) = scene else {
        return button(ink).into_any_element();
    };
    let out = layer(from, ms, ink).top(px(-HEIGHT * p)).opacity(1.0 - p);
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
            .child(layer(to, ms, ink).bottom_0())
    } else {
        layer(to, ms, ink).top(px(HEIGHT * (1.0 - p))).opacity(p)
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
            if let Step::Say(id) = step {
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
        for ms in 0..length() {
            let p = scene(ms).unwrap().p;
            assert!((0.0..=1.0).contains(&p));
        }
    }
}
