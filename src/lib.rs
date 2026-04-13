//! # chislo — числа прописью на русском языке
//!
//! Convert numbers to Russian words (числа прописью) with correct grammatical
//! gender and noun declension.
//!
//! A Rust port of [go-propisyu](https://github.com/rekurt/go-propisyu).
//!
//! ## Features
//!
//! - Integers up to `i64::MAX` (≈ 9.22 × 10^18); scale dictionary includes names up to 10^39
//! - Three grammatical genders: masculine, feminine, neuter
//! - Automatic Russian noun declension
//! - Ordinal numbers ("первый", "сорок второй")
//! - Currency formatting ("один рубль двадцать три копейки")
//! - Decimal number support with configurable precision
//! - `no_std` compatible (with `alloc`)
//!
//! ## Quick start
//!
//! ```
//! use chislo::{int_to_words, int_to_words_gender, decline, ordinal, Gender};
//!
//! assert_eq!(int_to_words(42), "сорок два");
//! assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
//! assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
//! assert_eq!(ordinal(42, Gender::Masculine), "сорок второй");
//! ```
//!
//! ## Currency example
//!
//! ```
//! use chislo::{money, RUB};
//!
//! assert_eq!(
//!     money(1234, 56, &RUB),
//!     "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
//! );
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod convert;
mod currency;
mod datetime;
pub(crate) mod decimal;
pub(crate) mod decline;
mod dictionary;
mod display;
mod duration;
mod fraction;
mod ordinal;
mod parse;
mod percent;
#[cfg(feature = "wasm")]
mod wasm;

use core::fmt;

#[cfg(not(feature = "std"))]
use alloc::string::String;

pub use currency::{AED, BYN, CHF, CNY, Currency, EUR, GBP, JPY, KZT, RUB, RoundingMode, UAH, USD};
pub use datetime::{date_to_words, month_name, time_to_words, year_to_words};
pub use display::{GenderedNumber, MoneyAmount, Number, NumberWithNoun, OrdinalNumber};
pub use duration::{
    duration_from_core, duration_from_secs, duration_hms, hours_word, minutes_word, seconds_word,
};
pub use fraction::{fraction, mixed_fraction};
pub use percent::{percent, percent_decimal, percent_decimal_precision, percent_word};

/// Grammatical gender for Russian number words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// Masculine: "один", "два"
    Masculine = 0,
    /// Feminine: "одна", "две"
    Feminine = 1,
    /// Neuter: "одно", "два"
    Neuter = 2,
}

impl Gender {
    /// Returns a 0-based index suitable for `[masculine, feminine, neuter]`
    /// lookup tables.
    #[inline]
    pub(crate) const fn index(self) -> usize {
        self as usize
    }
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

impl core::error::Error for Error {}

/// Converts an integer to Russian words using masculine gender.
///
/// # Examples
///
/// ```
/// use chislo::int_to_words;
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
/// # Examples
///
/// ```
/// use chislo::{int_to_words_gender, Gender};
///
/// assert_eq!(int_to_words_gender(1, Gender::Masculine), "один");
/// assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
/// assert_eq!(int_to_words_gender(1, Gender::Neuter), "одно");
/// ```
pub fn int_to_words_gender(n: i64, gender: Gender) -> String {
    convert::convert_int_to_words(n, gender)
}

/// Converts a number to its ordinal form in Russian.
///
/// # Examples
///
/// ```
/// use chislo::{ordinal, Gender};
///
/// assert_eq!(ordinal(1, Gender::Masculine), "первый");
/// assert_eq!(ordinal(1, Gender::Feminine), "первая");
/// assert_eq!(ordinal(42, Gender::Masculine), "сорок второй");
/// assert_eq!(ordinal(2026, Gender::Masculine), "две тысячи двадцать шестой");
/// ```
pub fn ordinal(n: i64, gender: Gender) -> String {
    ordinal::ordinal(n, gender)
}

/// Converts a decimal number string to Russian words (2 decimal places).
///
/// # Examples
///
/// ```
/// use chislo::decimal_to_words;
///
/// assert_eq!(
///     decimal_to_words("123.45").unwrap(),
///     "сто двадцать три целых сорок пять сотых"
/// );
/// ```
pub fn decimal_to_words(decimal_str: &str) -> Result<String, Error> {
    decimal::decimal_str_to_words(decimal_str)
}

/// Converts a decimal string with specified precision (1-9 decimal places).
///
/// # Examples
///
/// ```
/// use chislo::decimal_to_words_precision;
///
/// assert_eq!(
///     decimal_to_words_precision("3.5", 1).unwrap(),
///     "три целых пять десятых"
/// );
/// assert_eq!(
///     decimal_to_words_precision("3.145", 3).unwrap(),
///     "три целых сто сорок пять тысячных"
/// );
/// ```
pub fn decimal_to_words_precision(decimal_str: &str, precision: u32) -> Result<String, Error> {
    decimal::decimal_str_to_words_precision(decimal_str, precision)
}

/// Converts a `rust_decimal::Decimal` value to Russian words (2 decimal places).
///
/// # Examples
///
/// ```
/// use chislo::decimal_value_to_words;
/// use rust_decimal::Decimal;
/// use std::str::FromStr;
///
/// let d = Decimal::from_str("123.45").unwrap();
/// assert_eq!(
///     decimal_value_to_words(d).unwrap(),
///     "сто двадцать три целых сорок пять сотых"
/// );
/// ```
#[cfg(feature = "decimal")]
pub fn decimal_value_to_words(d: rust_decimal::Decimal) -> Result<String, Error> {
    decimal::decimal_value_to_words_impl(d)
}

/// Converts a `rust_decimal::Decimal` value with the specified precision (1-9).
///
/// # Examples
///
/// ```
/// use chislo::decimal_value_to_words_precision;
/// use rust_decimal::Decimal;
/// use std::str::FromStr;
///
/// let d = Decimal::from_str("3.14159").unwrap();
/// assert_eq!(
///     decimal_value_to_words_precision(d, 5).unwrap(),
///     "три целых четырнадцать тысяч сто пятьдесят девять стотысячных"
/// );
/// ```
#[cfg(feature = "decimal")]
pub fn decimal_value_to_words_precision(
    d: rust_decimal::Decimal,
    precision: u32,
) -> Result<String, Error> {
    decimal::decimal_value_to_words_precision_impl(d, precision)
}

/// Returns the correct Russian noun declension form based on a number.
///
/// # Examples
///
/// ```
/// use chislo::decline;
///
/// assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
/// assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
/// ```
pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    decline::decline(n, one, two, five)
}

/// Formats an amount as words with currency.
///
/// # Examples
///
/// ```
/// use chislo::{money, RUB};
///
/// assert_eq!(
///     money(1234, 56, &RUB),
///     "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
/// );
/// ```
pub fn money(whole: i64, cents: u32, currency: &Currency) -> String {
    currency::money(whole, cents, currency)
}

/// Parses an amount string and formats with currency.
///
/// # Examples
///
/// ```
/// use chislo::{money_from_str, RUB};
///
/// assert_eq!(
///     money_from_str("1234.56", &RUB).unwrap(),
///     "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
/// );
/// ```
pub fn money_from_str(amount: &str, currency: &Currency) -> Result<String, Error> {
    currency::money_from_str(amount, currency)
}

/// Parses an amount string with the given rounding mode and formats it
/// as words with the specified currency. Accepts both `.` and `,` as
/// decimal separators.
///
/// # Examples
///
/// ```
/// use chislo::{money_from_str_rounded, RoundingMode, RUB};
///
/// assert_eq!(
///     money_from_str_rounded("100.995", &RUB, RoundingMode::HalfUp).unwrap(),
///     "сто один рубль ноль копеек"
/// );
/// ```
pub fn money_from_str_rounded(
    amount: &str,
    currency: &Currency,
    mode: RoundingMode,
) -> Result<String, Error> {
    currency::money_from_str_rounded(amount, currency, mode)
}
