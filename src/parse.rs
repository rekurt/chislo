//! Shared parsing helpers for numeric strings.

#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::Error;

/// Parses the fractional part of a decimal number (the digits after the
/// decimal separator) as an integer with a fixed precision.
///
/// - Truncates extra digits when more than `precision` are provided.
/// - Right-pads with `'0'` when fewer are provided (so `"5"` at precision 2
///   becomes `50`, i.e. `.50`).
/// - Returns `0` for an empty string.
pub(crate) fn parse_fractional_digits(frac_str: &str, precision: u32) -> Result<u32, Error> {
    if frac_str.is_empty() {
        return Ok(0);
    }

    let p = precision as usize;
    let mut buf = String::with_capacity(p);
    for c in frac_str.chars().take(p) {
        buf.push(c);
    }
    while buf.chars().count() < p {
        buf.push('0');
    }

    buf.parse::<u32>()
        .map_err(|_| Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'")))
}

/// Splits a decimal-like string on either `'.'` or `','` into whole and
/// fractional parts. Returns `(whole, Option<frac>)`.
pub(crate) fn split_decimal(s: &str) -> (&str, Option<&str>) {
    match s.find(['.', ',']) {
        Some(idx) => (&s[..idx], Some(&s[idx + 1..])),
        None => (s, None),
    }
}

pub(crate) fn strip_sign(s: &str) -> (bool, &str) {
    match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fractional_digits_basic() {
        assert_eq!(parse_fractional_digits("", 2).unwrap(), 0);
        assert_eq!(parse_fractional_digits("5", 2).unwrap(), 50);
        assert_eq!(parse_fractional_digits("50", 2).unwrap(), 50);
        assert_eq!(parse_fractional_digits("56", 2).unwrap(), 56);
        assert_eq!(parse_fractional_digits("999", 2).unwrap(), 99);
    }

    #[test]
    fn test_parse_fractional_digits_precision() {
        assert_eq!(parse_fractional_digits("1", 3).unwrap(), 100);
        assert_eq!(parse_fractional_digits("12", 3).unwrap(), 120);
        assert_eq!(parse_fractional_digits("123", 3).unwrap(), 123);
        assert_eq!(parse_fractional_digits("1234", 3).unwrap(), 123);
        assert_eq!(parse_fractional_digits("123456789", 9).unwrap(), 123456789);
    }

    #[test]
    fn test_parse_fractional_digits_invalid() {
        assert!(parse_fractional_digits("ab", 2).is_err());
        assert!(parse_fractional_digits("1a", 2).is_err());
    }

    #[test]
    fn test_split_decimal() {
        assert_eq!(split_decimal("1234.56"), ("1234", Some("56")));
        assert_eq!(split_decimal("1234,56"), ("1234", Some("56")));
        assert_eq!(split_decimal("1234"), ("1234", None));
        assert_eq!(split_decimal("1234."), ("1234", Some("")));
        assert_eq!(split_decimal("-1.5"), ("-1", Some("5")));
    }

    #[test]
    fn test_strip_sign() {
        assert_eq!(strip_sign("-1.5"), (true, "1.5"));
        assert_eq!(strip_sign("-0.5"), (true, "0.5"));
        assert_eq!(strip_sign("1.5"), (false, "1.5"));
    }
}
