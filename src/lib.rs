//! # propisyu
//!
//! Convert numbers to Russian words with correct grammatical gender and noun declension.
//!
//! This library supports:
//! - Numbers up to duodecillions (10^39)
//! - Three grammatical genders (masculine, feminine, neuter)
//! - Automatic noun declension
//! - Decimal number support
//!
//! # Examples
//!
//! ```
//! use propisyu::{int_to_words, int_to_words_gender, decline, Gender};
//!
//! assert_eq!(int_to_words(42), "сорок два");
//! assert_eq!(int_to_words_gender(2, Gender::Feminine), "две");
//! assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
//! ```

mod convert;
mod decimal;
pub(crate) mod decline;
mod dictionary;

use std::fmt;

/// Grammatical gender for Russian number words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// Masculine: "один", "два"
    Masculine = 1,
    /// Feminine: "одна", "две"
    Feminine = 2,
    /// Neuter: "одно", "два"
    Neuter = 3,
}

/// Error type for number conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The input string is not a valid number.
    InvalidNumber(String),
    /// The number is too large to convert.
    NumberTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidNumber(msg) => write!(f, "{msg}"),
            Error::NumberTooLarge => write!(f, "number is too large to convert"),
        }
    }
}

impl std::error::Error for Error {}

/// Converts an integer to Russian words using masculine gender.
///
/// # Examples
///
/// ```
/// use propisyu::int_to_words;
///
/// assert_eq!(int_to_words(0), "ноль");
/// assert_eq!(int_to_words(42), "сорок два");
/// assert_eq!(int_to_words(-5), "минус пять");
/// assert_eq!(int_to_words(1000), "одна тысяча");
/// ```
pub fn int_to_words(n: i64) -> String {
    convert::convert_int_to_words(n, Gender::Masculine)
}

/// Converts an integer to Russian words with the specified grammatical gender.
///
/// Gender affects the forms of "один"/"одна"/"одно" and "два"/"две".
///
/// # Examples
///
/// ```
/// use propisyu::{int_to_words_gender, Gender};
///
/// assert_eq!(int_to_words_gender(1, Gender::Masculine), "один");
/// assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
/// assert_eq!(int_to_words_gender(1, Gender::Neuter), "одно");
/// assert_eq!(int_to_words_gender(2, Gender::Feminine), "две");
/// ```
pub fn int_to_words_gender(n: i64, gender: Gender) -> String {
    convert::convert_int_to_words(n, gender)
}

/// Converts a decimal number string to Russian words.
///
/// The fractional part is truncated (not rounded) to 2 decimal places.
/// Format: "{whole} целых {hundredths} {сотая/сотых}"
///
/// # Examples
///
/// ```
/// use propisyu::decimal_to_words;
///
/// assert_eq!(
///     decimal_to_words("123.45").unwrap(),
///     "сто двадцать три целых сорок пять сотых"
/// );
/// ```
///
/// # Errors
///
/// Returns [`Error::InvalidNumber`] if the input string is not a valid number.
pub fn decimal_to_words(decimal_str: &str) -> Result<String, Error> {
    decimal::decimal_str_to_words(decimal_str)
}

/// Converts a `rust_decimal::Decimal` value to Russian words.
///
/// The fractional part is truncated (not rounded) to 2 decimal places.
///
/// # Examples
///
/// ```
/// use propisyu::decimal_value_to_words;
/// use rust_decimal::Decimal;
/// use std::str::FromStr;
///
/// let d = Decimal::from_str("123.45").unwrap();
/// assert_eq!(
///     decimal_value_to_words(d).unwrap(),
///     "сто двадцать три целых сорок пять сотых"
/// );
/// ```
///
/// # Errors
///
/// Returns [`Error::NumberTooLarge`] if the integer part exceeds `i64` range.
#[cfg(feature = "decimal")]
pub fn decimal_value_to_words(d: rust_decimal::Decimal) -> Result<String, Error> {
    decimal::decimal_value_to_words_impl(d)
}

/// Returns the correct Russian noun declension form based on a number.
///
/// # Arguments
///
/// * `n` - The number to determine declension for
/// * `one` - Form for 1, 21, 31... (e.g., "рубль")
/// * `two` - Form for 2-4, 22-24... (e.g., "рубля")
/// * `five` - Form for 0, 5-20, 25-30... (e.g., "рублей")
///
/// # Examples
///
/// ```
/// use propisyu::decline;
///
/// assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
/// assert_eq!(decline(2, "рубль", "рубля", "рублей"), "рубля");
/// assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
/// ```
pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    decline::decline(n, one, two, five)
}
