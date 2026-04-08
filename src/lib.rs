//! # chislo — числа прописью на русском языке
//!
//! Convert numbers to Russian words (числа прописью) with correct grammatical
//! gender and noun declension.
//!
//! A Rust port of [go-propisyu](https://github.com/rekurt/go-propisyu).
//!
//! ## Features
//!
//! - Numbers up to duodecillions (10^39)
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
mod decimal;
pub(crate) mod decline;
mod dictionary;
mod ordinal;
#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "std")]
use std::fmt;
#[cfg(not(feature = "std"))]
use core::fmt;

#[cfg(not(feature = "std"))]
use alloc::string::String;

pub use currency::{Currency, EUR, RUB, USD};

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

#[cfg(feature = "std")]
impl std::error::Error for Error {}

pub fn int_to_words(n: i64) -> String {
    convert::convert_int_to_words(n, Gender::Masculine)
}

pub fn int_to_words_gender(n: i64, gender: Gender) -> String {
    convert::convert_int_to_words(n, gender)
}

pub fn ordinal(n: i64, gender: Gender) -> String {
    ordinal::ordinal(n, gender)
}

pub fn decimal_to_words(decimal_str: &str) -> Result<String, Error> {
    decimal::decimal_str_to_words(decimal_str)
}

pub fn decimal_to_words_precision(decimal_str: &str, precision: u32) -> Result<String, Error> {
    decimal::decimal_str_to_words_precision(decimal_str, precision)
}

#[cfg(feature = "decimal")]
pub fn decimal_value_to_words(d: rust_decimal::Decimal) -> Result<String, Error> {
    decimal::decimal_value_to_words_impl(d)
}

pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    decline::decline(n, one, two, five)
}

pub fn money(whole: i64, cents: u32, currency: &Currency) -> String {
    currency::money(whole, cents, currency)
}

pub fn money_from_str(amount: &str, currency: &Currency) -> Result<String, Error> {
    currency::money_from_str(amount, currency)
}
