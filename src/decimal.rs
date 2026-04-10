#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::dictionary::FRACTION_UNITS;
use crate::{Error, Gender};

pub(crate) fn decimal_str_to_words(decimal_str: &str) -> Result<String, Error> {
    let (negative, rest) = strip_sign(decimal_str);
    let parts: Vec<&str> = rest.splitn(2, '.').collect();

    let whole_str = parts[0];
    let whole_abs: i64 = whole_str
        .parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{whole_str}'")))?;
    let whole = if negative { -whole_abs } else { whole_abs };
    let negative_zero = negative && whole_abs == 0;

    let frac_str = if parts.len() > 1 { parts[1] } else { "00" };
    let hundredths = parse_hundredths(frac_str)?;
    let units = &FRACTION_UNITS[1];
    Ok(format_decimal_words(
        whole,
        hundredths,
        units,
        negative_zero,
    ))
}

#[cfg(feature = "decimal")]
pub(crate) fn decimal_value_to_words_impl(d: rust_decimal::Decimal) -> Result<String, Error> {
    decimal_value_to_words_precision_impl(d, 2)
}

#[cfg(feature = "decimal")]
pub(crate) fn decimal_value_to_words_precision_impl(
    d: rust_decimal::Decimal,
    precision: u32,
) -> Result<String, Error> {
    if precision == 0 || precision > 9 {
        return Err(Error::InvalidNumber(format!(
            "precision must be 1-9, got {precision}"
        )));
    }
    use rust_decimal::prelude::ToPrimitive;

    let negative = d.is_sign_negative();
    let abs = d.abs();
    let whole_abs = abs.trunc().to_i64().ok_or(Error::NumberTooLarge)?;
    let whole = if negative { -whole_abs } else { whole_abs };

    let multiplier = rust_decimal::Decimal::from(10u64.pow(precision));
    let frac_dec = ((abs - abs.trunc()) * multiplier).trunc();
    let frac = frac_dec.to_u64().ok_or(Error::NumberTooLarge)? as u32;

    let units = &FRACTION_UNITS[precision as usize - 1];
    Ok(format_decimal_words(
        whole,
        frac,
        units,
        negative && whole_abs == 0,
    ))
}

pub(crate) fn decimal_str_to_words_precision(
    decimal_str: &str,
    precision: u32,
) -> Result<String, Error> {
    if precision == 0 || precision > 9 {
        return Err(Error::InvalidNumber(format!(
            "precision must be 1-9, got {precision}"
        )));
    }

    let (negative, rest) = strip_sign(decimal_str);
    let parts: Vec<&str> = rest.splitn(2, '.').collect();

    let whole_str = parts[0];
    let whole_abs: i64 = whole_str
        .parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{whole_str}'")))?;
    let whole = if negative { -whole_abs } else { whole_abs };
    let negative_zero = negative && whole_abs == 0;

    let frac_str = if parts.len() > 1 { parts[1] } else { "" };
    let frac_value = parse_fraction(frac_str, precision)?;

    let units = &FRACTION_UNITS[precision as usize - 1];
    Ok(format_decimal_words(
        whole,
        frac_value,
        units,
        negative_zero,
    ))
}

fn strip_sign(s: &str) -> (bool, &str) {
    match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s),
    }
}

fn parse_fraction(frac_str: &str, precision: u32) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    let p = precision as usize;
    let chars: String = frac_str.chars().take(p).collect();
    let normalized = if chars.len() >= p {
        chars
    } else {
        let mut s = chars;
        while s.len() < p {
            s.push('0');
        }
        s
    };

    normalized
        .parse::<u32>()
        .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
}

fn parse_hundredths(frac_str: &str) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    let chars: Vec<char> = frac_str.chars().take(2).collect();
    if chars.len() >= 2 {
        let normalized: String = chars.into_iter().collect();
        normalized
            .parse::<u32>()
            .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
    } else {
        let c: String = chars.into_iter().collect();
        c.parse::<u32>()
            .map(|d| d * 10)
            .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
    }
}

fn format_decimal_words(whole: i64, frac: u32, units: &[&str; 3], negative_zero: bool) -> String {
    let whole_words = convert_int_to_words(whole, Gender::Feminine);
    let whole_decl = get_declension(whole, "целая", "целых", "целых");
    let frac_words = convert_int_to_words(frac as i64, Gender::Feminine);
    let frac_decl = get_declension(frac as i64, units[0], units[1], units[2]);
    let sign = if negative_zero { "минус " } else { "" };
    format!("{sign}{whole_words} {whole_decl} {frac_words} {frac_decl}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::ToString;

    #[test]
    fn test_decimal_to_words() {
        let cases = [
            ("123.45", Ok("сто двадцать три целых сорок пять сотых")),
            ("100", Ok("сто целых ноль сотых")),
            ("50.5", Ok("пятьдесят целых пятьдесят сотых")),
            ("5.01", Ok("пять целых одна сотая")),
            ("10.02", Ok("десять целых две сотых")),
        ];

        for (input, expected) in cases {
            let result = decimal_str_to_words(input);
            assert_eq!(
                result.as_deref().map_err(|e| format!("{e}")),
                expected.map_err(|e: &str| e.to_string()),
                "decimal_to_words(\"{input}\") failed"
            );
        }
    }

    #[test]
    fn test_decimal_to_words_errors() {
        assert!(decimal_str_to_words("abc.45").is_err());
        assert!(decimal_str_to_words("123.xy").is_err());
    }

    #[test]
    fn test_decimal_to_words_whole_feminine() {
        let result = decimal_str_to_words("1.01").unwrap();
        assert_eq!(result, "одна целая одна сотая");
    }

    #[test]
    fn test_decimal_feminine_whole() {
        // "целая" (nominative singular) only for numbers ending in 1 (but not 11-19).
        // Everything else uses "целых" (genitive plural, mathematical convention).
        let cases: &[(&str, u32, &str)] = &[
            ("1.5", 1, "одна целая пять десятых"),
            ("2.5", 1, "две целых пять десятых"),
            ("3.5", 1, "три целых пять десятых"),
            ("5.5", 1, "пять целых пять десятых"),
            ("11.5", 1, "одиннадцать целых пять десятых"),
            ("21.5", 1, "двадцать одна целая пять десятых"),
            ("22.5", 1, "двадцать две целых пять десятых"),
            ("101.5", 1, "сто одна целая пять десятых"),
            ("1001.5", 1, "одна тысяча одна целая пять десятых"),
            // 1_000_000 ends in 0, so "целых" (not "целая"):
            ("1000000.5", 1, "один миллион целых пять десятых"),
            // 1_000_001 ends in 1, so "целая":
            ("1000001.5", 1, "один миллион одна целая пять десятых"),
        ];
        for &(input, p, expected) in cases {
            assert_eq!(
                decimal_str_to_words_precision(input, p).unwrap(),
                expected,
                "feminine_whole(\"{input}\", {p})"
            );
        }
    }

    #[test]
    fn test_decimal_negative_zero() {
        assert_eq!(
            decimal_str_to_words("-0.5").unwrap(),
            "минус ноль целых пятьдесят сотых"
        );
        assert_eq!(
            decimal_str_to_words("-0.01").unwrap(),
            "минус ноль целых одна сотая"
        );
        assert_eq!(
            decimal_str_to_words_precision("-0.25", 2).unwrap(),
            "минус ноль целых двадцать пять сотых"
        );
        assert_eq!(
            decimal_str_to_words_precision("-0.001", 3).unwrap(),
            "минус ноль целых одна тысячная"
        );
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_to_words() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let cases = [
            ("123.45", "сто двадцать три целых сорок пять сотых"),
            ("0.00", "ноль целых ноль сотых"),
        ];

        for (input, expected) in cases {
            let d = Decimal::from_str(input).unwrap();
            let result = decimal_value_to_words_impl(d).unwrap();
            assert_eq!(result, expected, "decimal_value_to_words({input}) failed");
        }
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_zero() {
        use rust_decimal::Decimal;
        let result = decimal_value_to_words_impl(Decimal::ZERO).unwrap();
        assert_eq!(result, "ноль целых ноль сотых");
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_to_words_precision() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let cases: &[(&str, u32, &str)] = &[
            ("1.5", 1, "одна целая пять десятых"),
            ("2.5", 1, "две целых пять десятых"),
            ("3.14", 2, "три целых четырнадцать сотых"),
            ("0.001", 3, "ноль целых одна тысячная"),
            (
                "3.14159",
                5,
                "три целых четырнадцать тысяч сто пятьдесят девять стотысячных",
            ),
            ("-1.5", 1, "минус одна целая пять десятых"),
            ("-0.5", 1, "минус ноль целых пять десятых"),
        ];
        for &(input, precision, expected) in cases {
            let d = Decimal::from_str(input).unwrap();
            assert_eq!(
                decimal_value_to_words_precision_impl(d, precision).unwrap(),
                expected,
                "decimal_value_to_words_precision({input}, {precision}) failed"
            );
        }
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_to_words_precision_errors() {
        use rust_decimal::Decimal;
        use std::str::FromStr;
        let d = Decimal::from_str("1.5").unwrap();
        assert!(decimal_value_to_words_precision_impl(d, 0).is_err());
        assert!(decimal_value_to_words_precision_impl(d, 10).is_err());
    }

    #[test]
    fn test_decimal_precision() {
        assert_eq!(
            decimal_str_to_words_precision("3.5", 1).unwrap(),
            "три целых пять десятых"
        );
        assert_eq!(
            decimal_str_to_words_precision("3.145", 3).unwrap(),
            "три целых сто сорок пять тысячных"
        );
        assert_eq!(
            decimal_str_to_words_precision("0.001", 3).unwrap(),
            "ноль целых одна тысячная"
        );
    }

    #[test]
    fn test_decimal_precision_errors() {
        assert!(decimal_str_to_words_precision("3.5", 0).is_err());
        assert!(decimal_str_to_words_precision("3.5", 10).is_err());
    }

    #[test]
    fn test_decimal_all_precisions() {
        let cases: &[(u32, &str)] = &[
            (1, "семь целых одна десятая"),
            (2, "семь целых двенадцать сотых"),
            (3, "семь целых сто двадцать три тысячных"),
            (
                4,
                "семь целых одна тысяча двести тридцать четыре десятитысячных",
            ),
            (
                5,
                "семь целых двенадцать тысяч триста сорок пять стотысячных",
            ),
            (
                6,
                "семь целых сто двадцать три тысячи четыреста пятьдесят шесть миллионных",
            ),
            (
                7,
                "семь целых один миллион двести тридцать четыре тысячи пятьсот шестьдесят семь десятимиллионных",
            ),
            (
                8,
                "семь целых двенадцать миллионов триста сорок пять тысяч шестьсот семьдесят восемь стомиллионных",
            ),
            (
                9,
                "семь целых сто двадцать три миллиона четыреста пятьдесят шесть тысяч семьсот восемьдесят девять миллиардных",
            ),
        ];
        for &(precision, expected) in cases {
            assert_eq!(
                decimal_str_to_words_precision("7.123456789", precision).unwrap(),
                expected,
                "precision {precision}"
            );
        }
    }

    #[test]
    fn test_decimal_padding_and_truncation() {
        let cases: &[(&str, u32, &str)] = &[
            // Padding: "3.1" p3 → frac "1" padded to "100" → 100
            ("3.1", 3, "три целых сто тысячных"),
            // Padding: "3.5" p2 → frac "5" padded to "50" → 50
            ("3.5", 2, "три целых пятьдесят сотых"),
            // No fraction: "5" p2 → empty → 0
            ("5", 2, "пять целых ноль сотых"),
            // Truncation: "3.456" p1 → takes "4" → 4
            ("3.456", 1, "три целых четыре десятых"),
            // Truncation: "3.789" p2 → takes "78" → 78
            ("3.789", 2, "три целых семьдесят восемь сотых"),
        ];
        for &(input, precision, expected) in cases {
            assert_eq!(
                decimal_str_to_words_precision(input, precision).unwrap(),
                expected,
                "padding/trunc(\"{input}\", {precision})"
            );
        }
    }

    #[test]
    fn test_decimal_edge_cases() {
        // Zero whole with fraction
        assert_eq!(
            decimal_str_to_words_precision("0.5", 1).unwrap(),
            "ноль целых пять десятых"
        );
        assert_eq!(
            decimal_str_to_words_precision("0.99", 2).unwrap(),
            "ноль целых девяносто девять сотых"
        );
        // Negative decimal
        assert_eq!(
            decimal_str_to_words("-5.25").unwrap(),
            "минус пять целых двадцать пять сотых"
        );
        assert_eq!(
            decimal_str_to_words("-1.01").unwrap(),
            "минус одна целая одна сотая"
        );
        // Large whole part
        assert_eq!(
            decimal_str_to_words("999999.01").unwrap(),
            "девятьсот девяносто девять тысяч девятьсот девяносто девять целых одна сотая"
        );
        // Error cases
        assert!(decimal_str_to_words("").is_err());
        assert!(decimal_str_to_words("abc").is_err());
        assert!(decimal_str_to_words("1.2.3").is_err());
        assert!(decimal_str_to_words("12.ab").is_err());
    }

    #[test]
    fn test_decimal_default_hundredths_edge() {
        let cases: &[(&str, &str)] = &[
            ("0.00", "ноль целых ноль сотых"),
            ("0.0", "ноль целых ноль сотых"),
            ("0.1", "ноль целых десять сотых"),
            ("99.99", "девяносто девять целых девяносто девять сотых"),
            ("-1.01", "минус одна целая одна сотая"),
        ];
        for &(input, expected) in cases {
            assert_eq!(
                decimal_str_to_words(input).unwrap(),
                expected,
                "hundredths_edge(\"{input}\")"
            );
        }
    }
}
