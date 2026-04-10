//! Percentage formatting in Russian words.

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::parse::{parse_fractional_digits, split_decimal};
use crate::{Error, Gender};

const PERCENT_ONE: &str = "процент";
const PERCENT_TWO: &str = "процента";
const PERCENT_FIVE: &str = "процентов";

/// Formats an integer percentage with the correct declension.
///
/// # Examples
///
/// ```
/// use chislo::percent;
///
/// assert_eq!(percent(1), "один процент");
/// assert_eq!(percent(2), "два процента");
/// assert_eq!(percent(42), "сорок два процента");
/// assert_eq!(percent(100), "сто процентов");
/// assert_eq!(percent(-5), "минус пять процентов");
/// ```
pub fn percent(n: i64) -> String {
    let words = convert_int_to_words(n, Gender::Masculine);
    let unit = get_declension(n, PERCENT_ONE, PERCENT_TWO, PERCENT_FIVE);
    format!("{words} {unit}")
}

/// Formats a decimal percentage with 2 fractional places in Russian words.
///
/// When a fractional part is present, the unit is always in the genitive
/// singular form "процента" (e.g. `42.25%` → "сорок две целых двадцать пять
/// сотых процента"). Use [`percent_decimal_precision`] for other precisions.
///
/// # Examples
///
/// ```
/// use chislo::percent_decimal;
///
/// assert_eq!(
///     percent_decimal("42.25").unwrap(),
///     "сорок две целых двадцать пять сотых процента"
/// );
/// // No fraction falls back to integer percent:
/// assert_eq!(percent_decimal("100").unwrap(), "сто процентов");
/// ```
pub fn percent_decimal(s: &str) -> Result<String, Error> {
    percent_decimal_precision(s, 2)
}

/// Like [`percent_decimal`] but with a configurable precision (1–9 digits).
pub fn percent_decimal_precision(s: &str, precision: u32) -> Result<String, Error> {
    if precision == 0 || precision > 9 {
        return Err(Error::InvalidNumber(format!(
            "precision must be 1-9, got {precision}"
        )));
    }

    let (whole_str, frac_opt) = split_decimal(s);
    let whole: i64 = whole_str
        .parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{whole_str}'")))?;

    match frac_opt {
        None => Ok(percent(whole)),
        Some(frac_str) => {
            let frac_value = parse_fractional_digits(frac_str, precision)?;
            if frac_value == 0 {
                return Ok(percent(whole));
            }
            // When a fraction is present the unit is always "процента".
            let decimal =
                crate::decimal::decimal_str_to_words_precision_for(whole, frac_value, precision);
            Ok(format!("{decimal} {PERCENT_TWO}"))
        }
    }
}

/// Returns the noun form "процент / процента / процентов" for the given count,
/// without the number itself. Useful when you already have the number
/// formatted and only need the word.
///
/// # Examples
///
/// ```
/// use chislo::percent_word;
///
/// assert_eq!(percent_word(1), "процент");
/// assert_eq!(percent_word(3), "процента");
/// assert_eq!(percent_word(7), "процентов");
/// ```
pub fn percent_word(n: i64) -> String {
    get_declension(n, PERCENT_ONE, PERCENT_TWO, PERCENT_FIVE).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_int() {
        let cases: &[(i64, &str)] = &[
            (0, "ноль процентов"),
            (1, "один процент"),
            (2, "два процента"),
            (3, "три процента"),
            (4, "четыре процента"),
            (5, "пять процентов"),
            (11, "одиннадцать процентов"),
            (21, "двадцать один процент"),
            (22, "двадцать два процента"),
            (100, "сто процентов"),
            (101, "сто один процент"),
            (-5, "минус пять процентов"),
        ];
        for &(n, expected) in cases {
            assert_eq!(percent(n), expected, "percent({n})");
        }
    }

    #[test]
    fn test_percent_decimal_basic() {
        assert_eq!(
            percent_decimal("1.5").unwrap(),
            "одна целая пятьдесят сотых процента"
        );
        assert_eq!(
            percent_decimal("42.25").unwrap(),
            "сорок две целых двадцать пять сотых процента"
        );
        assert_eq!(
            percent_decimal("0.01").unwrap(),
            "ноль целых одна сотая процента"
        );
    }

    #[test]
    fn test_percent_decimal_precision() {
        assert_eq!(
            percent_decimal_precision("3.14159", 5).unwrap(),
            "три целых четырнадцать тысяч сто пятьдесят девять стотысячных процента"
        );
    }

    #[test]
    fn test_percent_decimal_no_fraction() {
        assert_eq!(percent_decimal("100").unwrap(), "сто процентов");
        assert_eq!(percent_decimal("1").unwrap(), "один процент");
        assert_eq!(percent_decimal("100.00").unwrap(), "сто процентов");
    }

    #[test]
    fn test_percent_decimal_errors() {
        assert!(percent_decimal("abc").is_err());
        assert!(percent_decimal_precision("1.5", 0).is_err());
        assert!(percent_decimal_precision("1.5", 10).is_err());
    }

    #[test]
    fn test_percent_word() {
        assert_eq!(percent_word(1), "процент");
        assert_eq!(percent_word(3), "процента");
        assert_eq!(percent_word(5), "процентов");
        assert_eq!(percent_word(21), "процент");
    }
}
