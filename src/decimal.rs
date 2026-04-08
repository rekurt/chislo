use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::{Error, Gender};

/// Converts a decimal number string to Russian words.
///
/// The fractional part is truncated (not rounded) to 2 decimal places.
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

/// Converts a rust_decimal::Decimal to Russian words.
#[cfg(feature = "decimal")]
pub(crate) fn decimal_value_to_words_impl(
    d: rust_decimal::Decimal,
) -> Result<String, Error> {
    use rust_decimal::prelude::ToPrimitive;

    let whole = d.trunc().to_i64().ok_or(Error::NumberTooLarge)?;

    let fractional = (d - d.trunc()).abs();
    let hundredths_dec = (fractional * rust_decimal::Decimal::from(100)).trunc();
    let hundredths = hundredths_dec
        .to_i64()
        .ok_or(Error::NumberTooLarge)? as u32;

    Ok(format_decimal_words(whole, hundredths))
}

/// Parse fractional string to hundredths (0-99), truncating to 2 digits.
fn parse_hundredths(frac_str: &str) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    // Take first 2 chars, pad with zero if only 1 char
    let normalized = if frac_str.len() >= 2 {
        &frac_str[..2]
    } else {
        // Single digit: pad with trailing zero (e.g., "5" -> "50")
        return frac_str[..1]
            .parse::<u32>()
            .map(|d| d * 10)
            .map_err(|_| {
                Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'"))
            });
    };

    normalized.parse::<u32>().map_err(|_| {
        Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'"))
    })
}

/// Format the final decimal words string.
fn format_decimal_words(whole: i64, hundredths: u32) -> String {
    let whole_words = convert_int_to_words(whole, Gender::Masculine);
    let hundredths_words = convert_int_to_words(hundredths as i64, Gender::Feminine);
    let hundredths_declension =
        get_declension(hundredths as i64, "сотая", "сотых", "сотых");

    format!("{whole_words} целых {hundredths_words} {hundredths_declension}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_words() {
        let cases = [
            (
                "123.45",
                Ok("сто двадцать три целых сорок пять сотых"),
            ),
            (
                "6453345242432.42",
                Ok("шесть триллионов четыреста пятьдесят три миллиарда триста сорок пять миллионов двести сорок две тысячи четыреста тридцать два целых сорок две сотых"),
            ),
            ("100", Ok("сто целых ноль сотых")),
            ("50.5", Ok("пятьдесят целых пятьдесят сотых")),
            ("0.99", Ok("ноль целых девяносто девять сотых")),
            ("1.999", Ok("один целых девяносто девять сотых")),
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
        // "1" in whole part should be masculine, "1" in hundredths should be feminine
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
            ("50.5", "пятьдесят целых пятьдесят сотых"),
            ("1.999", "один целых девяносто девять сотых"),
        ];

        for (input, expected) in cases {
            let d = Decimal::from_str(input).unwrap();
            let result = decimal_value_to_words_impl(d).unwrap();
            assert_eq!(result, expected, "decimal_value_to_words({input}) failed");
        }
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_negative() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let d = Decimal::from_str("-42.15").unwrap();
        let result = decimal_value_to_words_impl(d).unwrap();
        assert_eq!(result, "минус сорок два целых пятнадцать сотых");
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_precision() {
        use rust_decimal::Decimal;
        use std::str::FromStr;

        let d = Decimal::from_str("3.141592653589793").unwrap();
        let result = decimal_value_to_words_impl(d).unwrap();
        assert_eq!(result, "три целых четырнадцать сотых");
    }

    #[cfg(feature = "decimal")]
    #[test]
    fn test_decimal_value_zero() {
        use rust_decimal::Decimal;

        let result = decimal_value_to_words_impl(Decimal::ZERO).unwrap();
        assert_eq!(result, "ноль целых ноль сотых");
    }
}
