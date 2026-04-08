#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::dictionary::FRACTION_UNITS;
use crate::{Error, Gender};

pub(crate) fn decimal_str_to_words(decimal_str: &str) -> Result<String, Error> {
    let parts: Vec<&str> = decimal_str.splitn(2, '.').collect();

    let whole_str = parts[0];
    let whole: i64 = whole_str
        .parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{whole_str}'")))?;

    let frac_str = if parts.len() > 1 { parts[1] } else { "00" };
    let hundredths = parse_hundredths(frac_str)?;
    Ok(format_decimal_words(whole, hundredths))
}

#[cfg(feature = "decimal")]
pub(crate) fn decimal_value_to_words_impl(d: rust_decimal::Decimal) -> Result<String, Error> {
    use rust_decimal::prelude::ToPrimitive;

    let whole = d.trunc().to_i64().ok_or(Error::NumberTooLarge)?;
    let fractional = (d - d.trunc()).abs();
    let hundredths_dec = (fractional * rust_decimal::Decimal::from(100)).trunc();
    let hundredths = hundredths_dec.to_i64().ok_or(Error::NumberTooLarge)? as u32;
    Ok(format_decimal_words(whole, hundredths))
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

    let parts: Vec<&str> = decimal_str.splitn(2, '.').collect();

    let whole_str = parts[0];
    let whole: i64 = whole_str
        .parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{whole_str}'")))?;

    let frac_str = if parts.len() > 1 { parts[1] } else { "" };
    let frac_value = parse_fraction(frac_str, precision)?;

    let units = &FRACTION_UNITS[precision as usize - 1];

    let whole_words = convert_int_to_words(whole, Gender::Masculine);
    let frac_words = convert_int_to_words(frac_value as i64, Gender::Feminine);
    let frac_decl = get_declension(frac_value as i64, units[0], units[1], units[2]);

    Ok(format!("{whole_words} целых {frac_words} {frac_decl}"))
}

fn parse_fraction(frac_str: &str, precision: u32) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    let p = precision as usize;
    let normalized = if frac_str.len() >= p {
        &frac_str[..p]
    } else {
        let mut s = frac_str.to_string();
        while s.len() < p {
            s.push('0');
        }
        return s
            .parse::<u32>()
            .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")));
    };

    normalized
        .parse::<u32>()
        .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
}

fn parse_hundredths(frac_str: &str) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    let normalized = if frac_str.len() >= 2 {
        &frac_str[..2]
    } else {
        return frac_str[..1]
            .parse::<u32>()
            .map(|d| d * 10)
            .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")));
    };

    normalized
        .parse::<u32>()
        .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
}

fn format_decimal_words(whole: i64, hundredths: u32) -> String {
    let whole_words = convert_int_to_words(whole, Gender::Masculine);
    let hundredths_words = convert_int_to_words(hundredths as i64, Gender::Feminine);
    let hundredths_declension = get_declension(hundredths as i64, "сотая", "сотых", "сотых");

    format!("{whole_words} целых {hundredths_words} {hundredths_declension}")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result, "один целых одна сотая");
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
}
