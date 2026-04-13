//! Date and time formatting in Russian words.

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::dictionary::{
    HUNDREDS, MONTHS_GENITIVE, MONTHS_NOMINATIVE, ORDINAL_HUNDREDS_GEN_M, ORDINAL_ONES_GEN_M,
    ORDINAL_ORDERS_GEN_M, ORDINAL_TEENS_GEN_M, ORDINAL_TENS_GEN_M, TENS, compound_prefix,
};
use crate::ordinal::ordinal;
use crate::{Error, Gender};

/// Error returned by datetime functions when given an out-of-range value.
fn invalid_date_field(name: &str, value: u32) -> Error {
    Error::InvalidNumber(format!("invalid {name}: {value}"))
}

/// Formats a full calendar date in Russian words (genitive day + month + year).
///
/// Returns an error if `month` is not in `1..=12` or `day` is not in `1..=31`.
///
/// # Examples
///
/// ```
/// use chislo::date_to_words;
///
/// assert_eq!(
///     date_to_words(2026, 4, 10).unwrap(),
///     "десятое апреля две тысячи двадцать шестого года"
/// );
/// assert_eq!(
///     date_to_words(1945, 5, 9).unwrap(),
///     "девятое мая одна тысяча девятьсот сорок пятого года"
/// );
/// assert_eq!(
///     date_to_words(2000, 1, 1).unwrap(),
///     "первое января двухтысячного года"
/// );
/// ```
pub fn date_to_words(year: i32, month: u32, day: u32) -> Result<String, Error> {
    if !(1..=12).contains(&month) {
        return Err(invalid_date_field("month", month));
    }
    if !(1..=31).contains(&day) {
        return Err(invalid_date_field("day", day));
    }
    if year < 1 {
        return Err(Error::InvalidNumber(format!(
            "year must be positive, got {year}"
        )));
    }

    let day_words = ordinal(day as i64, Gender::Neuter);
    let month_word = MONTHS_GENITIVE[(month - 1) as usize];
    let year_words = year_to_genitive_words(year as u64);
    Ok(format!("{day_words} {month_word} {year_words} года"))
}

/// Formats a year in the genitive masculine case, as used in Russian dates
/// (e.g. "две тысячи двадцать шестого" for 2026).
///
/// # Examples
///
/// ```
/// use chislo::year_to_words;
///
/// assert_eq!(year_to_words(2026), "две тысячи двадцать шестого");
/// assert_eq!(year_to_words(2000), "двухтысячного");
/// assert_eq!(year_to_words(1900), "одна тысяча девятисотого");
/// ```
pub fn year_to_words(year: u64) -> String {
    year_to_genitive_words(year)
}

/// Formats wall-clock time in Russian words.
///
/// # Examples
///
/// ```
/// use chislo::time_to_words;
///
/// assert_eq!(
///     time_to_words(14, 30).unwrap(),
///     "четырнадцать часов тридцать минут"
/// );
/// assert_eq!(time_to_words(1, 0).unwrap(), "один час");
/// assert_eq!(time_to_words(0, 1).unwrap(), "ноль часов одна минута");
/// ```
pub fn time_to_words(hour: u32, minute: u32) -> Result<String, Error> {
    if hour >= 24 {
        return Err(invalid_date_field("hour", hour));
    }
    if minute >= 60 {
        return Err(invalid_date_field("minute", minute));
    }

    let hours = convert_int_to_words(hour as i64, Gender::Masculine);
    let hours_decl = get_declension(hour as i64, "час", "часа", "часов");

    if minute == 0 && hour != 0 {
        return Ok(format!("{hours} {hours_decl}"));
    }

    let minutes = convert_int_to_words(minute as i64, Gender::Feminine);
    let minutes_decl = get_declension(minute as i64, "минута", "минуты", "минут");

    if minute == 0 {
        Ok(format!("{hours} {hours_decl}"))
    } else {
        Ok(format!("{hours} {hours_decl} {minutes} {minutes_decl}"))
    }
}

/// Returns the Russian name of the month in the nominative case (e.g. "апрель").
///
/// # Examples
///
/// ```
/// use chislo::month_name;
///
/// assert_eq!(month_name(1).unwrap(), "январь");
/// assert_eq!(month_name(12).unwrap(), "декабрь");
/// assert!(month_name(0).is_err());
/// ```
pub fn month_name(month: u32) -> Result<&'static str, Error> {
    if !(1..=12).contains(&month) {
        return Err(invalid_date_field("month", month));
    }
    Ok(MONTHS_NOMINATIVE[(month - 1) as usize])
}

// --- internals ---

fn year_to_genitive_words(n: u64) -> String {
    if n == 0 {
        return "нулевого".to_string();
    }

    let last_triad = (n % 1000) as u32;
    let mut result = String::new();

    if last_triad > 0 {
        let prefix_n = n - last_triad as u64;
        if prefix_n > 0 {
            result.push_str(&convert_int_to_words(prefix_n as i64, Gender::Masculine));
            result.push(' ');
        }
        result.push_str(&ordinal_triad_genitive_masculine(last_triad));
    } else {
        result.push_str(&ordinal_round_order_genitive_masculine(n));
    }

    result
}

fn ordinal_triad_genitive_masculine(n: u32) -> String {
    let h = (n / 100) as usize;
    let t = ((n % 100) / 10) as usize;
    let o = (n % 10) as usize;

    let mut parts: Vec<&str> = Vec::new();

    if t == 1 {
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TEENS_GEN_M[o]);
    } else if o > 0 {
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        if t > 0 {
            parts.push(TENS[t]);
        }
        parts.push(ORDINAL_ONES_GEN_M[o - 1]);
    } else if t > 0 {
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TENS_GEN_M[t - 2]);
    } else {
        parts.push(ORDINAL_HUNDREDS_GEN_M[h - 1]);
    }

    parts.join(" ")
}

fn ordinal_round_order_genitive_masculine(n: u64) -> String {
    let mut remaining = n;
    let mut order: usize = 0;

    while remaining > 0 && remaining % 1000 == 0 {
        remaining /= 1000;
        order += 1;
    }

    if order == 0 || order >= ORDINAL_ORDERS_GEN_M.len() {
        return convert_int_to_words(n as i64, Gender::Masculine);
    }

    if remaining == 1 {
        ORDINAL_ORDERS_GEN_M[order].to_string()
    } else if let Some(prefix) = compound_prefix(remaining) {
        let suffix = ORDINAL_ORDERS_GEN_M[order];
        format!("{prefix}{suffix}")
    } else {
        let cardinal = convert_int_to_words(remaining as i64, Gender::Masculine);
        let suffix = ORDINAL_ORDERS_GEN_M[order];
        format!("{cardinal} {suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_to_genitive_various() {
        let cases: &[(u64, &str)] = &[
            (1, "первого"),
            (10, "десятого"),
            (21, "двадцать первого"),
            (100, "сотого"),
            (200, "двухсотого"),
            (1000, "тысячного"),
            (2000, "двухтысячного"),
            (2026, "две тысячи двадцать шестого"),
            (1945, "одна тысяча девятьсот сорок пятого"),
            (1900, "одна тысяча девятисотого"),
            (1941, "одна тысяча девятьсот сорок первого"),
            (2001, "две тысячи первого"),
            (10000, "десятитысячного"),
            (12000, "двенадцатитысячного"),
            (15000, "пятнадцатитысячного"),
            (20000, "двадцатитысячного"),
            (25000, "двадцатипятитысячного"),
            (100000, "стотысячного"),
            (200000, "двухсоттысячного"),
            (500000, "пятисоттысячного"),
        ];
        for &(y, expected) in cases {
            assert_eq!(year_to_words(y), expected, "year_to_words({y})");
        }
    }

    #[test]
    fn test_date_to_words_basic() {
        assert_eq!(
            date_to_words(2026, 4, 10).unwrap(),
            "десятое апреля две тысячи двадцать шестого года"
        );
        assert_eq!(
            date_to_words(1945, 5, 9).unwrap(),
            "девятое мая одна тысяча девятьсот сорок пятого года"
        );
        assert_eq!(
            date_to_words(2000, 1, 1).unwrap(),
            "первое января двухтысячного года"
        );
        assert_eq!(
            date_to_words(2024, 2, 29).unwrap(),
            "двадцать девятое февраля две тысячи двадцать четвёртого года"
        );
    }

    #[test]
    fn test_date_to_words_errors() {
        assert!(date_to_words(2026, 0, 10).is_err());
        assert!(date_to_words(2026, 13, 10).is_err());
        assert!(date_to_words(2026, 4, 0).is_err());
        assert!(date_to_words(2026, 4, 32).is_err());
        assert!(date_to_words(0, 4, 10).is_err());
        assert!(date_to_words(-1, 4, 10).is_err());
    }

    #[test]
    fn test_time_to_words_basic() {
        assert_eq!(
            time_to_words(14, 30).unwrap(),
            "четырнадцать часов тридцать минут"
        );
        assert_eq!(time_to_words(1, 0).unwrap(), "один час");
        assert_eq!(time_to_words(2, 0).unwrap(), "два часа");
        assert_eq!(time_to_words(5, 0).unwrap(), "пять часов");
        assert_eq!(time_to_words(0, 1).unwrap(), "ноль часов одна минута");
        assert_eq!(
            time_to_words(23, 59).unwrap(),
            "двадцать три часа пятьдесят девять минут"
        );
    }

    #[test]
    fn test_time_to_words_errors() {
        assert!(time_to_words(24, 0).is_err());
        assert!(time_to_words(0, 60).is_err());
    }

    #[test]
    fn test_month_name() {
        assert_eq!(month_name(1).unwrap(), "январь");
        assert_eq!(month_name(4).unwrap(), "апрель");
        assert_eq!(month_name(12).unwrap(), "декабрь");
        assert!(month_name(0).is_err());
        assert!(month_name(13).is_err());
    }
}
