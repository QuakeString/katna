// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna's own new-mail sound: three soft bell notes rising (G5, C6, E6),
//! the last one ringing out, about a second and a half long and as loud
//! as a sound can be without clipping. The desktop's own new-mail sounds
//! can be too faint to hear; this one is made here, as a WAV file, rather
//! than shipped.

/// Samples per second.
const RATE: u32 = 48_000;
/// The notes: when each starts (seconds), its pitch (Hz), how long it
/// rings (the time its loudness takes to fall to a third) and how loud
/// it is.
const NOTES: [(f32, f32, f32, f32); 3] = [
    (0.0, 783.99, 0.16, 0.8),
    (0.09, 1046.5, 0.18, 0.85),
    (0.18, 1318.51, 0.5, 1.0),
];
/// A bell's partials: multiple of the pitch, loudness, and how much
/// sooner than the note it fades.
const PARTIALS: [(f32, f32, f32); 4] = [
    (1.0, 1.0, 1.0),
    (2.0, 0.32, 0.55),
    (3.0, 0.1, 0.35),
    (4.2, 0.04, 0.2),
];
/// The whole sound, in seconds.
const LENGTH: f32 = 1.5;
/// The loudest sample, as a share of full scale (about -1 dB).
const PEAK: f32 = 0.89;

/// The sound's samples, mono.
pub(super) fn samples() -> Vec<f32> {
    let count = (LENGTH * RATE as f32) as usize;
    let mut out = vec![0.0f32; count];
    for (start, pitch, ring, gain) in NOTES {
        let first = (start * RATE as f32) as usize;
        for (n, sample) in out.iter_mut().enumerate().skip(first) {
            let t = (n - first) as f32 / RATE as f32;
            // A 4 ms rise keeps the start from clicking.
            let attack = (t / 0.004).min(1.0);
            let mut value = 0.0;
            for (multiple, loudness, fade) in PARTIALS {
                let decay = (-t / (ring * fade)).exp();
                value += loudness * decay * (std::f32::consts::TAU * pitch * multiple * t).sin();
            }
            *sample += gain * attack * value;
        }
    }
    // A soft echo, as from a small room.
    let delay = (0.07 * RATE as f32) as usize;
    for n in (delay..count).rev() {
        out[n] += 0.18 * out[n - delay];
    }
    // The last 60 ms fade to nothing.
    let tail = (0.06 * RATE as f32) as usize;
    for (k, sample) in out[count - tail..].iter_mut().enumerate() {
        *sample *= 1.0 - (k + 1) as f32 / tail as f32;
    }
    let loudest = out.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    if loudest > 0.0 {
        for sample in &mut out {
            *sample *= PEAK / loudest;
        }
    }
    out
}

/// The sound as a 16-bit mono WAV file.
pub(super) fn wav() -> Vec<u8> {
    let samples = samples();
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
    fn the_chime_is_loud_and_does_not_clip() {
        let samples = samples();
        assert_eq!(samples.len(), (LENGTH * RATE as f32) as usize);
        let loudest = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        assert!((loudest - PEAK).abs() < 1e-3, "{loudest}");
        // Loud through its first half second, not only at its peak.
        let first = &samples[..RATE as usize / 2];
        let rms = (first.iter().map(|s| s * s).sum::<f32>() / first.len() as f32).sqrt();
        assert!(rms > 0.15, "{rms}");
        assert_eq!(samples.last().copied(), Some(0.0));
        let wav = wav();
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(wav.len(), 44 + samples.len() * 2);
    }
}
