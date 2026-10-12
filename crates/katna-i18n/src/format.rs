// SPDX-License-Identifier: GPL-3.0-or-later

//! Dates and numbers in the current language's formats, with ICU4X and
//! CLDR's data: month and day names, their order, 12- or 24-hour time,
//! digits, grouping (`12,34,567` in India) and the calendar (Buddhist
//! years in Thai, Solar Hijri in Persian).

use fluent_bundle::FluentValue;
use icu_calendar::cal::Gregorian;
use icu_calendar::types::Weekday;
use icu_calendar::week::WeekInformation;
use icu_calendar::{AnyCalendar, Date, Iso, Ref};
use icu_datetime::fieldsets::{E, M, MD, MDE, MDT, T, YM, YMD, YMDE, YMDET};
use icu_datetime::pattern::{DateTimePattern, FixedCalendarDateTimeNames};
use icu_datetime::{DateTimeFormatter, NoCalendarFormatter};
use icu_decimal::DecimalFormatter;
use icu_decimal::input::Decimal;
use icu_decimal::options::GroupingStrategy;
use icu_locale_core::Locale;
use icu_time::{DateTime, Time};
use std::sync::atomic::{AtomicU8, Ordering};
use writeable::TryWriteable;

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
    /// Written out, for headings: `Sunday`, `4 October`, `Sunday, 4 October`.
    weekday_long: Option<DateTimeFormatter<E>>,
    day_month_long: Option<DateTimeFormatter<MD>>,
    weekday_day_month_long: Option<DateTimeFormatter<MDE>>,
    day_month_time: Option<DateTimeFormatter<MDT>>,
    date: Option<DateTimeFormatter<YMD>>,
    long: Option<DateTimeFormatter<YMDET>>,
    decimal: Option<DecimalFormatter>,
    /// For a calendar or a date picker: month names and years in the
    /// language's calendar (a Gregorian month is Shahrivar–Mehr in
    /// Persian), the shortest weekday names, the first day of the week
    /// and years without grouping.
    medium_date: Option<DateTimeFormatter<YMD>>,
    month: Option<DateTimeFormatter<M>>,
    month_year: Option<DateTimeFormatter<YM>>,
    /// CLDR's short weekday names (`Su`), which no field set gives.
    weekday_short: Option<(FixedCalendarDateTimeNames<Gregorian>, DateTimePattern)>,
    first_weekday: Option<Weekday>,
    year: Option<DecimalFormatter>,
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
            weekday_long: DateTimeFormatter::try_new(prefs(), E::long()).ok(),
            day_month_long: DateTimeFormatter::try_new(prefs(), MD::long()).ok(),
            weekday_day_month_long: DateTimeFormatter::try_new(prefs(), MDE::long()).ok(),
            day_month_time: DateTimeFormatter::try_new(prefs(), MD::medium().with_time_hm()).ok(),
            date: DateTimeFormatter::try_new(prefs(), YMD::short()).ok(),
            long: DateTimeFormatter::try_new(prefs(), YMDE::medium().with_time_hm()).ok(),
            decimal: DecimalFormatter::try_new((&locale).into(), Default::default()).ok(),
            medium_date: DateTimeFormatter::try_new(prefs(), YMD::medium()).ok(),
            month: DateTimeFormatter::try_new(prefs(), M::long()).ok(),
            month_year: DateTimeFormatter::try_new(prefs(), YM::long()).ok(),
            weekday_short: "cccccc".parse().ok().and_then(|pattern: DateTimePattern| {
                let mut names = FixedCalendarDateTimeNames::try_new(prefs()).ok()?;
                names.include_for_pattern(&pattern).ok()?;
                Some((names, pattern))
            }),
            first_weekday: WeekInformation::try_new((&locale).into())
                .ok()
                .map(|w| w.first_weekday),
            year: DecimalFormatter::try_new((&locale).into(), GroupingStrategy::Never.into()).ok(),
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

/// The weekday written out: `Sunday`, `रविवार`.
pub fn weekday_long(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%A").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.weekday_long
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Day and month written out: `October 4`, `4 October`.
pub fn day_month_long(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%B %-d").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.day_month_long
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Weekday, day and month written out: `Sunday, October 4`,
/// `Sunday 4 October`.
pub fn weekday_day_month_long(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%A, %B %-d").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.weekday_day_month_long
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Day, month and time: `Sep 27, 8:00 AM`, `27 Sept, 08:00`.
pub fn day_month_time(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%b %-d, %H:%M").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.day_month_time
            .as_ref()
            .map(|w| plain(w.format(&input).to_string()))
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

/// Day, month and year: `May 14, 2002`, `14 May 2002`.
pub fn day_month_year(date: jiff::civil::DateTime) -> String {
    let fallback = || date.strftime("%b %-d, %Y").to_string();
    let Some(input) = convert(date) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        f.medium_date
            .as_ref()
            .map(|w| plain(w.format(&input.date).to_string()))
    })
    .unwrap_or_else(fallback)
}

/// Between the two ends of a span of months or years (CLDR's
/// interval fallback pattern, `{0} – {1}`).
const SPAN: &str = " – ";

/// `date` in the language's calendar (Solar Hijri in Persian).
fn local(f: &Formats, date: Date<Iso>) -> Option<Date<Ref<'_, AnyCalendar>>> {
    Some(date.to_calendar(f.month.as_ref()?.calendar()))
}

fn iso(date: jiff::civil::Date) -> Option<Date<Iso>> {
    Date::try_new_iso(date.year().into(), date.month() as u8, date.day() as u8).ok()
}

/// The months the days from `first` to `last` fall in, in the language's
/// calendar, for a calendar's title: `(None, "September 2026")`, or the
/// start and the end of a span, `(Some("September"), "October 2026")`,
/// `(Some("شهریور ۱۴۰۴"), "مهر ۱۴۰۵")`. The caller joins the two.
pub fn months(first: jiff::civil::Date, last: jiff::civil::Date) -> (Option<String>, String) {
    let fallback = || {
        if (first.year(), first.month()) == (last.year(), last.month()) {
            (None, first.strftime("%B %Y").to_string())
        } else if first.year() == last.year() {
            (
                Some(first.strftime("%B").to_string()),
                last.strftime("%B %Y").to_string(),
            )
        } else {
            (
                Some(first.strftime("%B %Y").to_string()),
                last.strftime("%B %Y").to_string(),
            )
        }
    };
    let (Some(start), Some(end)) = (iso(first), iso(last)) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        let (a, b) = (local(f, start)?, local(f, end)?);
        let month = f.month.as_ref()?;
        let month_year = f.month_year.as_ref()?;
        let written = |m: &DateTimeFormatter<YM>, d: &Date<Iso>| plain(m.format(d).to_string());
        Some(if a.year().extended_year() != b.year().extended_year() {
            (Some(written(month_year, &start)), written(month_year, &end))
        } else if a.month().ordinal != b.month().ordinal {
            (
                Some(plain(month.format(&start).to_string())),
                written(month_year, &end),
            )
        } else {
            (None, written(month_year, &start))
        })
    })
    .unwrap_or_else(fallback)
}

/// The days of Gregorian `month` (1 to 12) as month names in the
/// language's calendar, for a date picker: `September`, `সেপ্টেম্বর`,
/// `شهریور – مهر`.
pub fn month_name(month: i8) -> String {
    let month = month.clamp(1, 12);
    let fallback = || {
        jiff::civil::Date::new(2026, month, 1)
            .map(|d| d.strftime("%B").to_string())
            .unwrap_or_default()
    };
    let Ok(first) = jiff::civil::Date::new(2026, month, 1) else {
        return fallback();
    };
    let (Some(start), Some(end)) = (iso(first), iso(first.last_of_month())) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| {
        let m = f.month.as_ref()?;
        let (a, b) = (m.format(&start).to_string(), m.format(&end).to_string());
        Some(if a == b {
            plain(a)
        } else {
            plain(a + SPAN + &b)
        })
    })
    .unwrap_or_else(fallback)
}

/// The Gregorian month `date` is in, with its year, in the language's
/// calendar, for a calendar's title: `September 2026`, `2026年9月`,
/// `شهریور – مهر ۱۴۰۵`.
pub fn month_year(date: jiff::civil::Date) -> String {
    match months(date.first_of_month(), date.last_of_month()) {
        (None, one) => one,
        (Some(first), last) => first + SPAN + &last,
    }
}

/// The day of the month `date` is, in the language's calendar and
/// digits, for a calendar's day cells: `27`, `২৭`, `۵` (5 Mehr).
pub fn day_number(date: jiff::civil::Date) -> String {
    let Some(day) = iso(date) else {
        return date.day().to_string();
    };
    let day = crate::catalog::with_formats(|f| Some(local(f, day)?.day_of_month().0))
        .unwrap_or(date.day() as u8);
    number(day.into())
}

/// The first day of the week: Sunday in the US, Monday in most of
/// Europe, Saturday in Egypt.
pub fn first_weekday() -> jiff::civil::Weekday {
    crate::catalog::with_formats(|f| f.first_weekday)
        .and_then(|w| jiff::civil::Weekday::from_monday_one_offset(w as i8).ok())
        .unwrap_or(jiff::civil::Weekday::Monday)
}

/// The seven weekdays' shortest names, from the first day of the week
/// (see [`first_weekday`]), for a calendar's columns: `Su`, `Mo`, ….
pub fn weekdays_short() -> [(jiff::civil::Weekday, String); 7] {
    let first = first_weekday();
    std::array::from_fn(|ix| {
        let day = first.wrapping_add(ix as i64);
        // 2024-01-01 was a Monday.
        let date = jiff::civil::date(2024, 1, 1 + day.to_monday_zero_offset());
        let fallback = || date.strftime("%a").to_string().chars().take(2).collect();
        let name = Date::try_new_gregorian(2024, 1, date.day() as u8)
            .ok()
            .and_then(|input| crate::catalog::with_formats(|f| short_weekday(f, input)))
            .unwrap_or_else(fallback);
        (day, name)
    })
}

fn short_weekday(f: &Formats, date: Date<Gregorian>) -> Option<String> {
    let (names, pattern) = f.weekday_short.as_ref()?;
    let input = DateTime {
        date,
        time: Time::start_of_day(),
    };
    let text = names.with_pattern_unchecked(pattern).format(&input);
    Some(plain(text.try_write_to_string().ok()?.into_owned()))
}

/// Gregorian `year` in the language's calendar and digits, with no
/// grouping: `2026`, `২০২৬`, `2569` (Thai), `۱۴۰۴ – ۱۴۰۵` (Persian).
pub fn year(year: i16) -> String {
    let fallback = || year.to_string();
    let (Ok(start), Ok(end)) = (
        Date::try_new_iso(year.into(), 1, 1),
        Date::try_new_iso(year.into(), 12, 31),
    ) else {
        return fallback();
    };
    crate::catalog::with_formats(|f| years(f, start, end)).unwrap_or_else(fallback)
}

fn years(f: &Formats, start: Date<Iso>, end: Date<Iso>) -> Option<String> {
    let written = |y: i32| {
        f.year
            .as_ref()
            .map(|x| x.format(&Decimal::from(y)).to_string())
    };
    let (a, b) = (
        local(f, start)?.year().extended_year(),
        local(f, end)?.year().extended_year(),
    );
    Some(if a == b {
        written(a)?
    } else {
        written(a)? + SPAN + &written(b)?
    })
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
/// `/٠٩`. The digits keep their order without it. The glyphs are lost in
/// `cosmic-text` 0.19 (`ShapeLine::layout_to_buffer` drops a leading run
/// of numbers when the first strong character, the mark, makes the line
/// right to left), not in `gpui-pre`; put the marks back once that is
/// fixed.
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
    fn months_with_years_and_days_with_times() {
        let f = Formats::new("en-US", Clock::Language);
        let september = Date::try_new_gregorian(2026, 9, 1).unwrap();
        let month = f.month_year.as_ref().unwrap().format(&september);
        assert_eq!(plain(month.to_string()), "September 2026");
        let at = plain(
            f.day_month_time
                .as_ref()
                .unwrap()
                .format(&sample())
                .to_string(),
        );
        assert!(at.starts_with("Sep 27, 2:05"), "{at}");
        let gb = Formats::new("en-GB", Clock::Language);
        let d = sample();
        let long = |f: &Option<DateTimeFormatter<MDE>>| {
            plain(f.as_ref().unwrap().format(&d.date).to_string())
        };
        assert_eq!(long(&gb.weekday_day_month_long), "Sunday 27 September");
        assert_eq!(long(&f.weekday_day_month_long), "Sunday, September 27");
        let day = plain(
            gb.day_month_long
                .as_ref()
                .unwrap()
                .format(&d.date)
                .to_string(),
        );
        assert_eq!(day, "27 September");
        let weekday = plain(f.weekday_long.as_ref().unwrap().format(&d.date).to_string());
        assert_eq!(weekday, "Sunday");
    }

    #[test]
    fn every_language_has_formats() {
        let d = sample();
        for language in crate::all() {
            let f = Formats::new(&language.formats, Clock::Language);
            assert!(f.time.is_some(), "{}", language.tag);
            assert!(f.long.is_some(), "{}", language.tag);
            assert!(f.decimal.is_some(), "{}", language.tag);
            assert!(f.medium_date.is_some(), "{}", language.tag);
            assert!(f.month.is_some(), "{}", language.tag);
            assert!(f.month_year.is_some(), "{}", language.tag);
            assert!(f.day_month_time.is_some(), "{}", language.tag);
            assert!(f.weekday_day_month_long.is_some(), "{}", language.tag);
            assert!(f.weekday_short.is_some(), "{}", language.tag);
            assert!(f.first_weekday.is_some(), "{}", language.tag);
            assert!(f.year.is_some(), "{}", language.tag);
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
    fn date_picker_names() {
        let month = |tag, m: u8| {
            with(tag, |f| {
                let d = Date::try_new_gregorian(2026, m, 1).unwrap();
                f.month.as_ref().unwrap().format(&d).to_string()
            })
        };
        assert_eq!(month("en-US", 9), "September");
        assert_eq!(month("de", 3), "März");
        // Gregorian names even where another calendar is the default.
        assert_eq!(month("th", 1), "มกราคม");
        let first = |tag| Formats::new(tag, Clock::Language).first_weekday.unwrap();
        assert_eq!(first("en-US"), Weekday::Sunday);
        assert_eq!(first("de"), Weekday::Monday);
        let short = |tag| {
            let monday = Date::try_new_gregorian(2024, 1, 1).unwrap();
            with(tag, |f| short_weekday(f, monday).unwrap())
        };
        assert_eq!(short("en-US"), "Mo");
        assert_eq!(short("de"), "Mo.");
        let year = |tag| {
            with(tag, |f| {
                let d = Decimal::from(2026u64);
                f.year.as_ref().unwrap().format(&d).to_string()
            })
        };
        assert_eq!(year("en-US"), "2026");
        assert_eq!(year("bn-BD"), "২০২৬");
        assert_eq!(year("fa"), "۲۰۲۶");
        // What the functions give before a language is applied.
        assert_eq!(super::first_weekday(), jiff::civil::Weekday::Sunday);
        let days = super::weekdays_short();
        assert_eq!(days[0], (jiff::civil::Weekday::Sunday, "Su".to_owned()));
        assert_eq!(days[6], (jiff::civil::Weekday::Saturday, "Sa".to_owned()));
        assert_eq!(super::month_name(5), "May");
        let may14 = jiff::civil::date(2002, 5, 14).at(0, 0, 0, 0);
        assert_eq!(super::day_month_year(may14), "May 14, 2002");
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

    #[test]
    fn persian_titles_count_in_solar_hijri() {
        let f = Formats::new("fa", Clock::Language);
        // 2026-09-27 is 5 Mehr 1405; 2026-01-01 is 11 Dey 1404.
        let d = |y, m, day| local(&f, Date::try_new_iso(y, m, day).unwrap()).unwrap();
        assert_eq!(d(2026, 9, 27).year().extended_year(), 1405);
        assert_eq!(d(2026, 9, 27).day_of_month().0, 5);
        assert_eq!(d(2026, 1, 1).year().extended_year(), 1404);
        let month = f.month.as_ref().unwrap();
        let mehr = month
            .format(&Date::try_new_iso(2026, 9, 27).unwrap())
            .to_string();
        assert_eq!(super::plain(mehr), "مهر");
        let year = |tag, y| {
            let start = Date::try_new_iso(y, 1, 1).unwrap();
            let end = Date::try_new_iso(y, 12, 31).unwrap();
            years(&Formats::new(tag, Clock::Language), start, end).unwrap()
        };
        assert_eq!(year("fa", 2026), "۱۴۰۴ – ۱۴۰۵");
        assert_eq!(year("th", 2026), "2569");
        assert_eq!(year("en-US", 2026), "2026");
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
