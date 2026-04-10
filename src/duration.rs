//! Duration formatting in Russian words.

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::{Gender, dictionary::ZERO};

const HOUR_FORMS: (&str, &str, &str) = ("час", "часа", "часов");
const MINUTE_FORMS: (&str, &str, &str) = ("минута", "минуты", "минут");
const SECOND_FORMS: (&str, &str, &str) = ("секунда", "секунды", "секунд");
const DAY_FORMS: (&str, &str, &str) = ("день", "дня", "дней");

/// Formats a duration given as hours, minutes, and seconds.
///
/// Zero components are omitted. If all three are zero, `"ноль секунд"` is
/// returned.
///
/// # Examples
///
/// ```
/// use chislo::duration_hms;
///
/// assert_eq!(duration_hms(1, 30, 0), "один час тридцать минут");
/// assert_eq!(duration_hms(0, 5, 10), "пять минут десять секунд");
/// assert_eq!(duration_hms(2, 0, 0), "два часа");
/// assert_eq!(duration_hms(0, 0, 0), "ноль секунд");
/// ```
pub fn duration_hms(hours: u64, minutes: u64, seconds: u64) -> String {
    let mut parts: Vec<String> = Vec::new();

    if hours > 0 {
        parts.push(format_component(
            hours as i64,
            Gender::Masculine,
            HOUR_FORMS,
        ));
    }
    if minutes > 0 {
        parts.push(format_component(
            minutes as i64,
            Gender::Feminine,
            MINUTE_FORMS,
        ));
    }
    if seconds > 0 {
        parts.push(format_component(
            seconds as i64,
            Gender::Feminine,
            SECOND_FORMS,
        ));
    }

    if parts.is_empty() {
        return format!("{} {}", ZERO, SECOND_FORMS.2);
    }

    parts.join(" ")
}

/// Formats a duration given as a total number of seconds by splitting into
/// days / hours / minutes / seconds.
///
/// # Examples
///
/// ```
/// use chislo::duration_from_secs;
///
/// assert_eq!(duration_from_secs(3_661), "один час одна минута одна секунда");
/// assert_eq!(duration_from_secs(90), "одна минута тридцать секунд");
/// assert_eq!(
///     duration_from_secs(90_061),
///     "один день один час одна минута одна секунда"
/// );
/// ```
pub fn duration_from_secs(total_secs: u64) -> String {
    let days = total_secs / 86_400;
    let remainder = total_secs % 86_400;
    let hours = remainder / 3_600;
    let minutes = (remainder % 3_600) / 60;
    let seconds = remainder % 60;

    if days == 0 {
        return duration_hms(hours, minutes, seconds);
    }

    let mut parts: Vec<String> = Vec::new();
    parts.push(format_component(days as i64, Gender::Masculine, DAY_FORMS));
    if hours > 0 {
        parts.push(format_component(
            hours as i64,
            Gender::Masculine,
            HOUR_FORMS,
        ));
    }
    if minutes > 0 {
        parts.push(format_component(
            minutes as i64,
            Gender::Feminine,
            MINUTE_FORMS,
        ));
    }
    if seconds > 0 {
        parts.push(format_component(
            seconds as i64,
            Gender::Feminine,
            SECOND_FORMS,
        ));
    }
    parts.join(" ")
}

/// Formats a [`core::time::Duration`] in Russian words (seconds precision).
///
/// # Examples
///
/// ```
/// use chislo::duration_from_core;
/// use std::time::Duration;
///
/// assert_eq!(
///     duration_from_core(Duration::from_secs(3_600)),
///     "один час"
/// );
/// ```
pub fn duration_from_core(d: core::time::Duration) -> String {
    duration_from_secs(d.as_secs())
}

fn format_component(n: i64, gender: Gender, forms: (&str, &str, &str)) -> String {
    let words = convert_int_to_words(n, gender);
    let unit = get_declension(n, forms.0, forms.1, forms.2);
    let mut s = words;
    s.push(' ');
    s.push_str(unit);
    s
}

/// Returns the Russian word form of "час/часа/часов" for the given count.
pub fn hours_word(n: i64) -> String {
    get_declension(n, HOUR_FORMS.0, HOUR_FORMS.1, HOUR_FORMS.2).to_string()
}

/// Returns the Russian word form of "минута/минуты/минут" for the given count.
pub fn minutes_word(n: i64) -> String {
    get_declension(n, MINUTE_FORMS.0, MINUTE_FORMS.1, MINUTE_FORMS.2).to_string()
}

/// Returns the Russian word form of "секунда/секунды/секунд" for the given count.
pub fn seconds_word(n: i64) -> String {
    get_declension(n, SECOND_FORMS.0, SECOND_FORMS.1, SECOND_FORMS.2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_hms_basic() {
        let cases: &[(u64, u64, u64, &str)] = &[
            (0, 0, 0, "ноль секунд"),
            (1, 0, 0, "один час"),
            (2, 0, 0, "два часа"),
            (5, 0, 0, "пять часов"),
            (1, 30, 0, "один час тридцать минут"),
            (1, 1, 1, "один час одна минута одна секунда"),
            (0, 2, 0, "две минуты"),
            (0, 0, 21, "двадцать одна секунда"),
            (
                24,
                60,
                60,
                "двадцать четыре часа шестьдесят минут шестьдесят секунд",
            ),
        ];
        for &(h, m, s, expected) in cases {
            assert_eq!(duration_hms(h, m, s), expected, "duration_hms({h},{m},{s})");
        }
    }

    #[test]
    fn test_duration_from_secs_basic() {
        assert_eq!(duration_from_secs(0), "ноль секунд");
        assert_eq!(duration_from_secs(60), "одна минута");
        assert_eq!(duration_from_secs(3_600), "один час");
        assert_eq!(
            duration_from_secs(3_661),
            "один час одна минута одна секунда"
        );
        assert_eq!(duration_from_secs(90), "одна минута тридцать секунд");
    }

    #[test]
    fn test_duration_from_secs_days() {
        assert_eq!(duration_from_secs(86_400), "один день");
        assert_eq!(duration_from_secs(172_800), "два дня");
        assert_eq!(
            duration_from_secs(90_061),
            "один день один час одна минута одна секунда"
        );
    }

    #[test]
    fn test_duration_from_core() {
        assert_eq!(
            duration_from_core(core::time::Duration::from_secs(3_600)),
            "один час"
        );
    }

    #[test]
    fn test_duration_word_helpers() {
        assert_eq!(hours_word(1), "час");
        assert_eq!(hours_word(3), "часа");
        assert_eq!(hours_word(5), "часов");
        assert_eq!(minutes_word(1), "минута");
        assert_eq!(minutes_word(3), "минуты");
        assert_eq!(minutes_word(5), "минут");
        assert_eq!(seconds_word(1), "секунда");
        assert_eq!(seconds_word(3), "секунды");
        assert_eq!(seconds_word(5), "секунд");
    }
}
