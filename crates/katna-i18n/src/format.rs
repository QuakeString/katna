// SPDX-License-Identifier: GPL-3.0-or-later

//! Dates and numbers in the current language's formats, with ICU4X and
//! CLDR's data: month and day names, their order, 12- or 24-hour time,
//! digits, grouping (`12,34,567` in India) and the calendar (Buddhist
//! years in Thai, Solar Hijri in Persian).

use fluent_bundle::FluentValue;
use icu_calendar::Date;
use icu_datetime::fieldsets::{E, MD, T, YMD, YMDE, YMDET};
use icu_datetime::{DateTimeFormatter, NoCalendarFormatter};
use icu_decimal::DecimalFormatter;
use icu_decimal::input::Decimal;
use icu_locale_core::Locale;
use icu_time::{DateTime, Time};
use std::sync::atomic::{AtomicU8, Ordering};

/// How times show: as the language writes them, or always with a 12- or
/// 24-hour clock (Settings > General > Time).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Clock {
    #[default]
    Language,
    Twelve,
    TwentyFour,
}

static CLOCK: AtomicU8 = AtomicU8::new(0);

/// Shows times with `clock` from now on.
pub fn set_clock(clock: Clock) {
    if CLOCK.swap(clock as u8, Ordering::Relaxed) != clock as u8 {
        crate::catalog::rebuild();
    }
}

pub(crate) fn clock() -> Clock {
    match CLOCK.load(Ordering::Relaxed) {
        1 => Clock::Twelve,
        2 => Clock::TwentyFour,
        _ => Clock::Language,
    }
}

/// `tag` with CLDR's hour-cycle keyword for `clock` (`en-US-u-hc-h23`).
fn with_clock(tag: &str, clock: Clock) -> String {
    let cycle = match clock {
        Clock::Language => return tag.to_owned(),
        Clock::Twelve => "h12",
        Clock::TwentyFour => "h23",
    };
    if tag.contains("-u-") {
        format!("{tag}-hc-{cycle}")
    } else {
        format!("{tag}-u-hc-{cycle}")
    }
}

/// The formatters for one locale. Each is `None` if ICU4X has no data for
/// it, and the English (US) one is used instead.
pub(crate) struct Formats {
    time: Option<NoCalendarFormatter<T>>,
    weekday: Option<DateTimeFormatter<E>>,
    day_month: Option<DateTimeFormatter<MD>>,
    date: Option<DateTimeFormatter<YMD>>,
    long: Option<DateTimeFormatter<YMDET>>,
    decimal: Option<DecimalFormatter>,
}

impl Formats {
    pub(crate) fn new(tag: &str, clock: Clock) -> Self {
        let tag = with_clock(tag, clock);
        let tag = tag.as_str();
        let locale = Locale::try_from_str(tag).unwrap_or_else(|_| {
            tracing::warn!(tag, "unknown formats locale; using en-US");
            Locale::try_from_str("en-US").expect("en-US parses")
        });
        let prefs = || (&locale).into();
        Self {
            time: NoCalendarFormatter::try_new(prefs(), T::hm()).ok(),
            weekday: DateTimeFormatter::try_new(prefs(), E::medium()).ok(),
            day_month: DateTimeFormatter::try_new(prefs(), MD::medium()).ok(),
            date: DateTimeFormatter::try_new(prefs(), YMD::short()).ok(),
            long: DateTimeFormatter::try_new(prefs(), YMDE::medium().with_time_hm()).ok(),
            decimal: DecimalFormatter::try_new((&locale).into(), Default::default()).ok(),
        }
    }
}

fn convert(date: jiff::civil::DateTime) -> Option<DateTime<icu_calendar::Iso>> {
    Some(DateTime {
        date: Date::try_new_iso(date.year().into(), date.month() as u8, date.day() as u8).ok()?,
        time: Time::try_new(
            date.hour() as u8,
            date.minute() as u8,
            date.second() as u8,
            0,
        )
        .ok()?,
    })
}

/// The time of day: `09:41`, `9:41 AM`, `৯:৪১ AM`.
pub fn time(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%H:%M").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.time
            .as_ref()
            .map(|t| plain(t.format(&input.time).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// The weekday, short: `Sat`, `शनि`.
pub fn weekday(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%a").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.weekday
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Day and month: `Sep 27`, `27 Sept`, `27 সেপ`.
pub fn day_month(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%b %-d").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.day_month
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// A short full date: `9/27/26`, `27/09/2026`, `2026/09/27`.
pub fn date(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%Y-%m-%d").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.date
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Weekday, date and time: `Sat, 27 Sept 2026, 09:41`.
pub fn long(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%a, %-d %b %Y, %H:%M").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| f.long.as_ref().map(|w| plain(w.format(&input).to_string())))
        .unwrap_or_else(fallback)
}

/// A whole number with the language's digits and grouping: `1,234,567`,
/// `12,34,567`, `১২,৩৪,৫৬৭`.
pub fn number(n: u64) -> String {
    decimal(Decimal::from(n))
}

/// `value` with `fraction` digits after the point: `1.5`, `১.৫`, `1,5`.
pub fn fraction(value: f64, fraction: u8) -> String {
    let scaled = (value * 10f64.powi(fraction.into())).round();
    if !scaled.is_finite() || scaled.abs() >= 9.0e15 {
        return format!("{value:.*}", usize::from(fraction));
    }
    let mut d = Decimal::from(scaled as i64);
    d.multiply_pow10(-i16::from(fraction));
    decimal(d)
}

fn decimal(d: Decimal) -> String {
    crate::catalog::with_formats(|f| f.decimal.as_ref().map(|x| x.format(&d).to_string()))
        .unwrap_or_else(|| d.to_string())
}

/// `text` without the invisible direction marks CLDR puts in Arabic
/// dates (`٢٧‏/٠٩` has a right-to-left mark after the day). GPUI's text
/// layout drops the glyphs before such a mark, so a short date showed only
/// `/٠٩`. The digits keep their order without it. Put the marks back once
/// the vendored text layout handles them (plan L.2).
fn plain(text: String) -> String {
    const MARKS: [char; 3] = ['\u{200e}', '\u{200f}', '\u{061c}'];
    if text.contains(MARKS) {
        text.replace(MARKS, "")
    } else {
        text
    }
}

/// Fluent's number formatter: variables that are numbers take the
/// language's digits and grouping.
pub(crate) fn fluent_number<M>(value: &FluentValue<'_>, _: &M) -> Option<String> {
    let FluentValue::Number(number) = value else {
        return None;
    };
    let digits = number.options.minimum_fraction_digits.unwrap_or(0).min(6);
    if digits == 0 && number.value.fract() == 0.0 && number.value.abs() < 9.0e15 {
        return Some(decimal(Decimal::from(number.value as i64)));
    }
    Some(fraction(number.value, digits.max(1) as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(tag: &str, f: impl FnOnce(&Formats) -> String) -> String {
        f(&Formats::new(tag, Clock::Language))
    }

    fn sample() -> DateTime<icu_calendar::Iso> {
        convert(jiff::civil::date(2026, 9, 27).at(14, 5, 0, 0)).unwrap()
    }

    #[test]
    fn the_three_englishes_differ() {
        let d = sample();
        let date = |tag| {
            with(tag, |f| {
                f.date.as_ref().unwrap().format(&d.date).to_string()
            })
        };
        assert_eq!(date("en-US"), "9/27/26");
        assert_eq!(date("en-GB"), "27/09/2026");
        assert_eq!(date("en-IN"), "27/09/26");
        let num = |tag| {
            with(tag, |f| {
                f.decimal
                    .as_ref()
                    .unwrap()
                    .format(&Decimal::from(1234567u64))
                    .to_string()
            })
        };
        assert_eq!(num("en-US"), "1,234,567");
        assert_eq!(num("en-IN"), "12,34,567");
        let time = |tag| {
            with(tag, |f| {
                f.time.as_ref().unwrap().format(&d.time).to_string()
            })
        };
        assert_eq!(time("en-GB"), "14:05");
        assert!(time("en-US").starts_with("2:05"));
    }

    #[test]
    fn the_clock_setting_wins_over_the_language() {
        let d = sample();
        let time = |tag, clock| {
            plain(
                Formats::new(tag, clock)
                    .time
                    .as_ref()
                    .unwrap()
                    .format(&d.time)
                    .to_string(),
            )
        };
        assert_eq!(time("en-US", Clock::TwentyFour), "14:05");
        assert!(time("en-GB", Clock::Twelve).starts_with("2:05"));
        assert!(time("de-DE", Clock::Twelve).starts_with("2:05"));
        let long = Formats::new("en-US", Clock::TwentyFour)
            .long
            .as_ref()
            .unwrap()
            .format(&d)
            .to_string();
        assert!(long.ends_with("14:05"), "{long}");
    }

    #[test]
    fn every_language_has_formats() {
        let d = sample();
        for language in crate::all() {
            let f = Formats::new(&language.formats, Clock::Language);
            assert!(f.time.is_some(), "{}", language.tag);
            assert!(f.long.is_some(), "{}", language.tag);
            assert!(f.decimal.is_some(), "{}", language.tag);
            let long = f.long.as_ref().unwrap().format(&d).to_string();
            assert!(!long.is_empty(), "{}", language.tag);
        }
    }

    #[test]
    fn arabic_dates_have_no_direction_marks() {
        let marks = |s: &str| s.contains(['\u{200e}', '\u{200f}', '\u{061c}']);
        let raw = with("ar", |f| {
            f.day_month
                .as_ref()
                .unwrap()
                .format(&sample().date)
                .to_string()
        });
        assert!(marks(&raw), "CLDR changed: {raw:?}");
        assert!(!marks(&super::plain(raw)));
    }

    #[test]
    fn bengali_uses_bengali_digits() {
        let n = with("bn-BD", |f| {
            f.decimal
                .as_ref()
                .unwrap()
                .format(&Decimal::from(1234567u64))
                .to_string()
        });
        assert_eq!(n, "১২,৩৪,৫৬৭");
    }
}
#[cfg(test)]
mod show {
    #[test]
    #[ignore]
    fn print_samples() {
        let d = super::convert(jiff::civil::date(2026, 9, 27).at(14, 5, 0, 0)).unwrap();
        for l in crate::all() {
            let f = super::Formats::new(&l.formats, super::Clock::Language);
            println!(
                "{:8} {} | {} | {} | {} | {} | {}",
                l.tag,
                f.long.as_ref().unwrap().format(&d),
                f.date.as_ref().unwrap().format(&d.date),
                f.day_month.as_ref().unwrap().format(&d.date),
                f.weekday.as_ref().unwrap().format(&d.date),
                f.time.as_ref().unwrap().format(&d.time),
                f.decimal
                    .as_ref()
                    .unwrap()
                    .format(&icu_decimal::input::Decimal::from(1234567u64))
            );
        }
    }
}
