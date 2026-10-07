// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna's own sounds, made here rather than shipped: the sets of
//! Settings > Notifications > Sounds (Katna, Nature, Birds, Animals,
//! Insects, Electronic, Morning). Each is a few notes, chirps or bursts of
//! filtered noise, at most two seconds long, as loud as it can be without
//! clipping, and the same every time.

use std::f32::consts::{PI, TAU};

use super::chime;

/// Samples per second.
pub(super) const RATE: u32 = 48_000;

/// The samples of Katna's sound `id` (a set's sound), mono, or `None` for
/// a sound not made here.
pub(super) fn samples(id: &str) -> Option<Vec<f32>> {
    if id == super::CHIME {
        return Some(chime::samples());
    }
    let mut s = Sound::new(1.5);
    match id {
        // Katna: soft bells.
        "katna.twin-bells" => {
            for at in [0.0, 0.5] {
                s.bell(at, 1046.5, 0.18, 0.8);
                s.bell(at + 0.12, 1568.0, 0.25, 0.9);
            }
        }
        "katna.soft-bell" => s.bell(0.0, 1174.66, 0.45, 1.0),
        "katna.rising" => {
            s.bell(0.0, 783.99, 0.14, 0.8);
            s.bell(0.08, 1174.66, 0.25, 1.0);
        }
        "katna.falling" => {
            s.bell(0.0, 1046.5, 0.14, 0.9);
            s.bell(0.14, 659.25, 0.35, 1.0);
        }

        // Nature: water, chimes, wind.
        "nature.water-drop" => {
            s.drop(0.0, 900.0, 2200.0, 0.12, 1.0);
            s.drop(0.28, 1100.0, 2600.0, 0.1, 0.55);
        }
        "nature.wind-chimes" => {
            let notes = [
                (0.0, 2093.0),
                (0.11, 2637.0),
                (0.19, 3136.0),
                (0.32, 2349.0),
                (0.41, 2794.0),
            ];
            for (at, f) in notes {
                s.bell(at, f, 0.4, 0.5);
            }
        }
        "nature.ripple" => {
            for (i, at) in [0.0, 0.12, 0.24].into_iter().enumerate() {
                let i = i as f32;
                s.drop(
                    at,
                    700.0 + i * 220.0,
                    1600.0 + i * 300.0,
                    0.1,
                    1.0 - i * 0.25,
                );
            }
        }
        "nature.breeze" => s.noise(0.0, 1.2, 1.0, Band::Sweep(900.0, 380.0, 0.7), 0.0),
        "nature.pebble" => {
            s.drop(0.0, 1500.0, 500.0, 0.12, 1.0);
            s.drop(0.2, 1100.0, 420.0, 0.14, 0.7);
        }

        // Birds: syllables that sweep and warble, as songbirds sing.
        "birds.robin" => {
            s.syllable(0.0, 0.07, 2600.0, 3400.0, 1.0);
            s.syllable(0.09, 0.06, 3400.0, 2800.0, 0.85);
            s.syllable(0.24, 0.08, 3000.0, 3900.0, 1.0);
            s.syllable(0.34, 0.06, 3900.0, 3300.0, 0.8);
            s.syllable(0.5, 0.14, 3700.0, 2900.0, 0.9);
        }
        "birds.wren-trill" => {
            for n in 0..12 {
                s.syllable(n as f32 * 0.055, 0.04, 4200.0, 5100.0, 0.7);
            }
            s.syllable(0.72, 0.16, 3000.0, 4600.0, 1.0);
            s.syllable(0.92, 0.14, 4600.0, 3400.0, 0.85);
        }
        "birds.finch" => {
            s.syllable(0.0, 0.07, 3200.0, 4400.0, 1.0);
            s.syllable(0.1, 0.09, 4400.0, 3000.0, 0.9);
            s.syllable(0.3, 0.07, 3200.0, 4400.0, 0.7);
        }
        "birds.swallow" => {
            s.syllable(0.0, 0.14, 2400.0, 4200.0, 1.0);
            s.syllable(0.16, 0.14, 4200.0, 2600.0, 0.9);
        }
        // Down a minor third, as a cuckoo calls: something went wrong.
        "birds.cuckoo" => {
            s.hoot(0.0, 0.2, 698.46, 690.0, 1.0);
            s.hoot(0.26, 0.38, 587.33, 570.0, 0.95);
        }

        // Animals.
        "animals.tree-frog" => {
            for n in 0..3 {
                s.buzz(n as f32 * 0.1, 0.08, 1400.0, 1400.0, 5600.0, 0.8);
            }
        }
        "animals.owl" => {
            s.hoot(0.0, 0.5, 420.0, 390.0, 1.0);
            s.hoot(0.6, 0.7, 400.0, 360.0, 0.9);
        }
        "animals.cat" => s.meow(0.0, 0.75),
        "animals.dolphin" => {
            for n in 0..6 {
                s.click(n as f32 * 0.03, 0.6);
            }
            s.syllable(0.25, 0.3, 6000.0, 9000.0, 0.7);
        }
        "animals.bullfrog" => {
            s.buzz(0.0, 0.26, 110.0, 105.0, 440.0, 1.0);
            s.buzz(0.32, 0.3, 110.0, 100.0, 440.0, 1.0);
        }

        // Insects.
        "insects.cricket" => {
            for chirp in [0.0, 0.35] {
                for n in 0..4 {
                    s.pip(chirp + n as f32 * 0.035, 0.022, 4600.0, 1.0);
                }
            }
        }
        "insects.cicada" => s.noise(0.0, 1.1, 0.8, Band::Pass(5200.0, 6.0), 70.0),
        "insects.katydid" => {
            for at in [0.0, 0.22, 0.44] {
                s.noise(at, 0.1, 1.0, Band::Pass(7000.0, 12.0), 45.0);
            }
        }
        "insects.bee" => s.buzz(0.0, 0.9, 220.0, 240.0, 900.0, 0.8),
        "insects.fly" => s.buzz(0.0, 0.7, 180.0, 150.0, 1100.0, 0.8),

        // Electronic.
        "electronic.blip" => {
            s.square(0.0, 0.12, 1318.5, 1.0);
            s.square(0.1, 0.2, 1760.0, 1.0);
        }
        "electronic.beacon" => {
            for at in [0.0, 0.3, 0.6] {
                s.pip(at, 0.18, 1975.5, 1.0);
            }
        }
        "electronic.arcade" => {
            for (n, f) in [523.25, 659.25, 783.99, 1046.5].into_iter().enumerate() {
                s.square(n as f32 * 0.07, 0.12, f, 1.0);
            }
        }
        "electronic.whoosh" => s.noise(0.0, 0.45, 1.0, Band::Sweep(400.0, 4000.0, 1.5), 0.0),
        "electronic.buzz" => {
            s.square(0.0, 0.3, 220.0, 1.0);
            s.square(0.2, 0.4, 180.0, 1.0);
        }

        // Morning: kalimba and marimba.
        "morning.kalimba" => {
            for (n, f) in [523.25, 659.25, 783.99, 1046.5].into_iter().enumerate() {
                s.mallet(n as f32 * 0.11, f, 0.25, 1.0);
            }
        }
        "morning.sunrise" => {
            let notes = [392.0, 523.25, 659.25, 783.99, 1046.5, 1318.5];
            for (n, f) in notes.into_iter().enumerate() {
                s.mallet(n as f32 * 0.13, f, 0.3, 0.9);
            }
        }
        "morning.marimba" => {
            s.mallet(0.0, 659.25, 0.2, 1.0);
            s.mallet(0.16, 783.99, 0.25, 1.0);
        }
        "morning.glockenspiel" => {
            s.bell(0.0, 1568.0, 0.22, 0.9);
            s.bell(0.12, 2093.0, 0.3, 1.0);
        }
        "morning.low-marimba" => {
            s.mallet(0.0, 392.0, 0.22, 1.0);
            s.mallet(0.18, 293.66, 0.3, 1.0);
        }
        _ => return None,
    }
    // Only what rings (bells, chimes, bars) echoes, as in a small room;
    // calls and buzzes stay clean.
    let echo = ["katna", "nature.", "morning."]
        .iter()
        .any(|p| id.starts_with(p))
        && id != "nature.breeze";
    // Noise and buzzes sound louder than bells at the same peak.
    let peak = if id.starts_with("insects.")
        || id.starts_with("electronic.")
        || id == "nature.breeze"
        || id.starts_with("animals.")
    {
        0.6
    } else {
        0.89
    };
    Some(s.finish(peak, echo))
}

/// The filter of a burst of noise.
#[derive(Clone, Copy)]
enum Band {
    /// Around a pitch (Hz), with its sharpness (Q).
    Pass(f32, f32),
    /// Gliding from one pitch to another, with its sharpness.
    Sweep(f32, f32, f32),
}

/// A sound being made.
struct Sound {
    out: Vec<f32>,
    /// Noise that is the same every time.
    seed: u32,
}

impl Sound {
    fn new(seconds: f32) -> Self {
        Self {
            out: vec![0.0; (seconds * RATE as f32) as usize],
            seed: 0x9E37_79B9,
        }
    }

    /// Adds `sample(t, n)` from `start` (seconds) for `length` seconds,
    /// `t` the time since `start`.
    fn add(&mut self, start: f32, length: f32, mut sample: impl FnMut(f32) -> f32) {
        let first = (start * RATE as f32) as usize;
        let count = (length * RATE as f32) as usize;
        for (n, out) in self.out.iter_mut().skip(first).take(count).enumerate() {
            *out += sample(n as f32 / RATE as f32);
        }
    }

    /// A bell: pitch `f`, ringing `ring` seconds (to a third), with its
    /// partials fading sooner.
    fn bell(&mut self, start: f32, f: f32, ring: f32, gain: f32) {
        const PARTIALS: [(f32, f32, f32); 4] = [
            (1.0, 1.0, 1.0),
            (2.0, 0.32, 0.55),
            (3.0, 0.1, 0.35),
            (4.2, 0.04, 0.2),
        ];
        let length = (ring * 7.0).min(self.rest(start));
        self.add(start, length, |t| {
            let attack = (t / 0.004).min(1.0);
            let mut v = 0.0;
            for (m, a, fade) in PARTIALS {
                v += a * (-t / (ring * fade)).exp() * (TAU * f * m * t).sin();
            }
            gain * attack * v
        });
    }

    /// A struck wooden bar or tine: kalimba, marimba.
    fn mallet(&mut self, start: f32, f: f32, ring: f32, gain: f32) {
        const PARTIALS: [(f32, f32, f32); 3] =
            [(1.0, 1.0, 1.0), (4.0, 0.18, 0.25), (9.2, 0.04, 0.1)];
        let length = (ring * 7.0).min(self.rest(start));
        self.add(start, length, |t| {
            let attack = (t / 0.002).min(1.0);
            let mut v = 0.0;
            for (m, a, fade) in PARTIALS {
                v += a * (-t / (ring * fade)).exp() * (TAU * f * m * t).sin();
            }
            gain * attack * v
        });
    }

    /// A pure tone's pitch gliding from `from` to `to` over `length`,
    /// shaped by `envelope` (0 to 1 over the length), with `harmonics`
    /// (multiple, loudness) and a warble of `vibrato` (share of the pitch).
    #[allow(clippy::too_many_arguments)]
    fn glide(
        &mut self,
        start: f32,
        length: f32,
        from: f32,
        to: f32,
        gain: f32,
        harmonics: &[(f32, f32)],
        vibrato: f32,
        envelope: impl Fn(f32) -> f32,
    ) {
        let mut phase = 0.0f32;
        let length = length.min(self.rest(start));
        self.add(start, length, |t| {
            let x = t / length;
            let f = from * (to / from).powf(x) * (1.0 + vibrato * (TAU * 38.0 * t).sin());
            phase += TAU * f / RATE as f32;
            let mut v = (phase).sin();
            for (m, a) in harmonics {
                v += a * (phase * m).sin();
            }
            gain * envelope(x) * v
        });
    }

    /// A bird's syllable: a quick sweep, swelling and fading.
    fn syllable(&mut self, start: f32, length: f32, from: f32, to: f32, gain: f32) {
        self.glide(start, length, from, to, gain, &[(2.0, 0.12)], 0.015, |x| {
            (PI * x).sin().powf(1.5)
        });
    }

    /// A hoot: a soft round tone with a slow swell (owl, cuckoo).
    fn hoot(&mut self, start: f32, length: f32, from: f32, to: f32, gain: f32) {
        self.glide(
            start,
            length,
            from,
            to,
            gain,
            &[(2.0, 0.08), (3.0, 0.03)],
            0.004,
            |x| {
                let rise = (x / 0.18).min(1.0);
                let fall = ((1.0 - x) / 0.35).min(1.0);
                rise * rise * fall
            },
        );
    }

    /// A drop of water: a quick upward sweep that dies away.
    fn drop(&mut self, start: f32, from: f32, to: f32, length: f32, gain: f32) {
        self.glide(start, length, from, to, gain, &[], 0.0, |x| {
            (x / 0.05).min(1.0) * (1.0 - x).powi(2)
        });
    }

    /// A short pure beep with soft edges.
    fn pip(&mut self, start: f32, length: f32, f: f32, gain: f32) {
        self.glide(start, length, f, f, gain, &[], 0.0, |x| {
            (PI * x).sin().powf(0.5)
        });
    }

    /// A click (dolphin).
    fn click(&mut self, start: f32, gain: f32) {
        self.glide(start, 0.004, 3000.0, 3000.0, gain, &[], 0.0, |x| {
            (PI * x).sin()
        });
    }

    /// A soft square-wave beep (electronic).
    fn square(&mut self, start: f32, length: f32, f: f32, gain: f32) {
        // Odd harmonics, fewer than a true square: less harsh.
        self.glide(
            start,
            length,
            f,
            f,
            gain * 0.6,
            &[(3.0, 0.33), (5.0, 0.2), (7.0, 0.14)],
            0.0,
            |x| (x / 0.03).min(1.0) * (1.0 - x).powf(0.7),
        );
    }

    /// A buzzing reed through a low-pass filter at `cutoff` (frog, bee).
    fn buzz(&mut self, start: f32, length: f32, from: f32, to: f32, cutoff: f32, gain: f32) {
        let mut filter = Biquad::low_pass(cutoff, 2.0);
        let mut phase = 0.0f32;
        let length = length.min(self.rest(start));
        self.add(start, length, |t| {
            let x = t / length;
            let f = from * (to / from).powf(x);
            phase = (phase + f / RATE as f32).fract();
            let saw = 2.0 * phase - 1.0;
            let envelope = (x / 0.08).min(1.0) * ((1.0 - x) / 0.25).min(1.0);
            gain * envelope * filter.run(saw)
        });
    }

    /// A cat's meow: a voice falling in pitch whose bright part opens
    /// then closes ("mee-ow"), its harmonics shaped by where that is.
    fn meow(&mut self, start: f32, length: f32) {
        let mut phase = 0.0f32;
        let length = length.min(self.rest(start));
        self.add(start, length, |t| {
            let x = t / length;
            let f = 560.0 * (420.0f32 / 560.0).powf(x) * (1.0 + 0.01 * (TAU * 6.0 * t).sin());
            phase += TAU * f / RATE as f32;
            let bright = 900.0 + 1900.0 * (PI * x.powf(0.7)).sin();
            let mut v = 0.0;
            for k in 1..=12 {
                let k = k as f32;
                let distance = (k * f - bright) / 650.0;
                v += ((-distance * distance).exp() + 0.12 / k) * (phase * k).sin();
            }
            let envelope = (x / 0.12).min(1.0) * ((1.0 - x) / 0.3).min(1.0);
            envelope * v
        });
    }

    /// A burst of filtered noise, pulsing `pulse` times a second if not 0.
    fn noise(&mut self, start: f32, length: f32, gain: f32, band: Band, pulse: f32) {
        let (from, to, q) = match band {
            Band::Pass(f, q) => (f, f, q),
            Band::Sweep(from, to, q) => (from, to, q),
        };
        let mut filter = Biquad::band_pass(from, q);
        let mut seed = self.seed;
        let length = length.min(self.rest(start));
        self.add(start, length, |t| {
            let x = t / length;
            if from != to {
                filter.retune_band(from * (to / from).powf(x), q);
            }
            // xorshift: white noise, the same every time.
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let white = seed as f32 / u32::MAX as f32 * 2.0 - 1.0;
            let envelope = (x / 0.05).min(1.0) * ((1.0 - x) / 0.4).min(1.0);
            let tremolo = if pulse > 0.0 {
                0.5 - 0.5 * (TAU * pulse * t).cos()
            } else {
                1.0
            };
            gain * envelope * tremolo * filter.run(white)
        });
        self.seed = seed;
    }

    /// Seconds left after `start`.
    fn rest(&self, start: f32) -> f32 {
        (self.out.len() as f32 / RATE as f32 - start).max(0.0)
    }

    /// Trims the silence at the end, adds a soft echo if `echo`, fades
    /// the last 40 ms and scales the loudest sample to `peak`.
    fn finish(mut self, peak: f32, echo: bool) -> Vec<f32> {
        let loudest = self.out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        if loudest == 0.0 {
            return self.out;
        }
        let last = self
            .out
            .iter()
            .rposition(|s| s.abs() > loudest * 0.002)
            .unwrap_or(0);
        let tail = (0.04 * RATE as f32) as usize;
        let delay = if echo {
            (0.07 * RATE as f32) as usize
        } else {
            0
        };
        self.out.truncate((last + delay + tail).min(self.out.len()));
        if echo {
            for n in (delay..self.out.len()).rev() {
                self.out[n] += 0.18 * self.out[n - delay];
            }
        }
        let count = self.out.len();
        let tail = tail.min(count);
        for (k, sample) in self.out[count - tail..].iter_mut().enumerate() {
            *sample *= 1.0 - (k + 1) as f32 / tail as f32;
        }
        let loudest = self.out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        for sample in &mut self.out {
            *sample *= peak / loudest;
        }
        self.out
    }
}

/// A two-pole filter (RBJ's audio EQ cookbook).
struct Biquad {
    b: [f32; 3],
    a: [f32; 2],
    x: [f32; 2],
    y: [f32; 2],
}

impl Biquad {
    fn low_pass(f: f32, q: f32) -> Self {
        let (cos, alpha) = Self::angle(f, q);
        let a0 = 1.0 + alpha;
        Self {
            b: [
                (1.0 - cos) / 2.0 / a0,
                (1.0 - cos) / a0,
                (1.0 - cos) / 2.0 / a0,
            ],
            a: [-2.0 * cos / a0, (1.0 - alpha) / a0],
            x: [0.0; 2],
            y: [0.0; 2],
        }
    }

    fn band_pass(f: f32, q: f32) -> Self {
        let mut filter = Self {
            b: [0.0; 3],
            a: [0.0; 2],
            x: [0.0; 2],
            y: [0.0; 2],
        };
        filter.retune_band(f, q);
        filter
    }

    /// A band-pass filter's new pitch, keeping what it holds.
    fn retune_band(&mut self, f: f32, q: f32) {
        let (cos, alpha) = Self::angle(f, q);
        let a0 = 1.0 + alpha;
        self.b = [alpha / a0, 0.0, -alpha / a0];
        self.a = [-2.0 * cos / a0, (1.0 - alpha) / a0];
    }

    fn angle(f: f32, q: f32) -> (f32, f32) {
        let w = TAU * f.min(RATE as f32 * 0.45) / RATE as f32;
        (w.cos(), w.sin() / (2.0 * q))
    }

    fn run(&mut self, x: f32) -> f32 {
        let y = self.b[0] * x + self.b[1] * self.x[0] + self.b[2] * self.x[1]
            - self.a[0] * self.y[0]
            - self.a[1] * self.y[1];
        self.x = [x, self.x[0]];
        self.y = [y, self.y[0]];
        y
    }
}

/// `samples` as a 16-bit mono WAV file.
pub(super) fn wav(samples: &[f32]) -> Vec<u8> {
    let data = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    // PCM, one channel.
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&RATE.to_le_bytes());
    out.extend_from_slice(&(RATE * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data.to_le_bytes());
    for sample in samples {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16;
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_set_sound_is_made_short_and_loud_without_clipping() {
        for set in super::super::SETS
            .iter()
            .filter(|s| s.id != super::super::SYSTEM)
        {
            for id in set.sounds {
                let samples = samples(id).unwrap_or_else(|| panic!("{id} is not made"));
                let seconds = samples.len() as f32 / RATE as f32;
                assert!((0.15..=1.6).contains(&seconds), "{id}: {seconds} s");
                let loudest = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
                assert!((0.55..=0.9).contains(&loudest), "{id}: peak {loudest}");
                assert!(samples.iter().all(|s| s.is_finite()), "{id}");
                assert_eq!(samples.last().copied(), Some(0.0), "{id}");
                // The same every time.
                assert_eq!(samples, super::samples(id).unwrap(), "{id}");
            }
        }
        assert!(samples("no-such-sound").is_none());
    }

    #[test]
    fn a_wav_holds_its_samples() {
        let wav = wav(&[0.0, 0.5, -0.5]);
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(wav.len(), 44 + 6);
    }
}
