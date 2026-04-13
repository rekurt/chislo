#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::dictionary::{FRACTION_UNITS, WHOLE_FORMS};
use crate::parse::{parse_fractional_digits, split_decimal, strip_sign};
use crate::{Error, Gender};

pub(crate) fn decimal_str_to_words(decimal_str: &str) -> Result<String, Error> {
    decimal_str_to_words_precision(decimal_str, 2)
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
    let whole = d.trunc().to_i64().ok_or(Error::NumberTooLarge)?;
    let multiplier = rust_decimal::Decimal::from(10u64.pow(precision));
    let frac = ((abs - abs.trunc()) * multiplier)
        .trunc()
        .to_u64()
        .ok_or(Error::NumberTooLarge)? as u32;

    Ok(decimal_str_to_words_precision_for(
        whole,
        frac,
        precision,
        negative && whole == 0,
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
    let (whole_str, frac_opt) = split_decimal(rest);
    let whole = parse_whole_part(whole_str, negative)?;
    let frac_value = parse_fractional_digits(frac_opt.unwrap_or(""), precision)?;

    Ok(decimal_str_to_words_precision_for(
        whole,
        frac_value,
        precision,
        negative && whole == 0,
    ))
}

fn parse_whole_part(whole_str: &str, negative: bool) -> Result<i64, Error> {
    let raw = if negative {
        format!("-{whole_str}")
    } else {
        String::from(whole_str)
    };

    raw.parse()
        .map_err(|_| Error::InvalidNumber(format!("invalid whole part: '{raw}'")))
}

pub(crate) fn decimal_str_to_words_precision_for(
    whole: i64,
    frac_value: u32,
    precision: u32,
    negative_zero: bool,
) -> String {
    let whole_words = convert_int_to_words(whole, Gender::Feminine);
    let whole_decl = get_declension(whole, WHOLE_FORMS.0, WHOLE_FORMS.1, WHOLE_FORMS.2);
    let frac_words = convert_int_to_words(frac_value as i64, Gender::Feminine);
    let units = &FRACTION_UNITS[precision as usize - 1];
    let frac_decl = get_declension(frac_value as i64, units[0], units[1], units[2]);
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
            ("10,02", Ok("десять целых две сотых")),
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
            ("1000000.5", 1, "один миллион целых пять десятых"),
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
            ("1.50", "одна целая пятьдесят сотых"),
            ("2.25", "две целых двадцать пять сотых"),
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
            let input = "7.123456789";
            assert_eq!(
                decimal_str_to_words_precision(input, precision).unwrap(),
                expected,
                "precision {precision}"
            );
        }
    }

    #[test]
    fn test_decimal_negative() {
        assert_eq!(
            decimal_str_to_words("-123.45").unwrap(),
            "минус сто двадцать три целых сорок пять сотых"
        );
        assert_eq!(
            decimal_str_to_words("-1.01").unwrap(),
            "минус одна целая одна сотая"
        );
    }

    #[test]
    fn test_decimal_i64_min_boundary() {
        assert_eq!(
            decimal_str_to_words("-9223372036854775808.01").unwrap(),
            "минус девять квинтиллионов двести двадцать три квадриллиона триста семьдесят два триллиона тридцать шесть миллиардов восемьсот пятьдесят четыре миллиона семьсот семьдесят пять тысяч восемьсот восемь целых одна сотая"
        );
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_i64_min_boundary() {
        use rust_decimal::Decimal;
        let d = Decimal::from(i64::MIN) + Decimal::new(1, 2);
        assert!(super::decimal_value_to_words_precision_impl(d, 2).is_ok());
    }
}
