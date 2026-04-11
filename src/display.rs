//! Builder-style wrappers that implement [`core::fmt::Display`].
//!
//! These let you plug number formatting directly into `write!`, `format!`,
//! `println!`, etc. without the caller having to materialise an intermediate
//! `String`.
//!
//! ```
//! use chislo::{Gender, Number, RUB};
//!
//! let n = Number::new(42);
//! assert_eq!(n.masculine().to_string(), "сорок два");
//! assert_eq!(n.feminine().to_string(), "сорок две");
//! assert_eq!(n.ordinal(Gender::Masculine).to_string(), "сорок второй");
//! assert_eq!(
//!     Number::new(1234).money(56, &RUB).to_string(),
//!     "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
//! );
//! ```

use core::fmt;

use crate::{Currency, Gender, int_to_words_gender, money, ordinal};

/// A number with no fixed representation yet. Call one of the `.masculine()`,
/// `.feminine()`, `.ordinal(...)`, `.money(...)` methods to get a `Display`-able
/// wrapper.
#[derive(Debug, Clone, Copy)]
pub struct Number(i64);

impl Number {
    /// Wraps an integer for fluent formatting.
    pub const fn new(n: i64) -> Self {
        Self(n)
    }

    /// Formats as a masculine cardinal (`один`, `два`, ...).
    pub const fn masculine(self) -> GenderedNumber {
        GenderedNumber {
            n: self.0,
            gender: Gender::Masculine,
        }
    }

    /// Formats as a feminine cardinal (`одна`, `две`, ...).
    pub const fn feminine(self) -> GenderedNumber {
        GenderedNumber {
            n: self.0,
            gender: Gender::Feminine,
        }
    }

    /// Formats as a neuter cardinal (`одно`, `два`, ...).
    pub const fn neuter(self) -> GenderedNumber {
        GenderedNumber {
            n: self.0,
            gender: Gender::Neuter,
        }
    }

    /// Formats with an explicit grammatical gender.
    pub const fn gender(self, gender: Gender) -> GenderedNumber {
        GenderedNumber { n: self.0, gender }
    }

    /// Formats as an ordinal (`первый`, `вторая`, ...).
    pub const fn ordinal(self, gender: Gender) -> OrdinalNumber {
        OrdinalNumber { n: self.0, gender }
    }

    /// Binds this number to a set of noun declension forms so it formats as
    /// e.g. `5 рублей`.
    pub const fn with_noun<'a>(
        self,
        one: &'a str,
        two: &'a str,
        five: &'a str,
    ) -> NumberWithNoun<'a> {
        NumberWithNoun {
            n: self.0,
            forms: (one, two, five),
        }
    }

    /// Formats as a currency amount.
    pub const fn money<'a>(self, cents: u32, currency: &'a Currency<'a>) -> MoneyAmount<'a> {
        MoneyAmount {
            whole: self.0,
            cents,
            currency,
        }
    }
}

/// A number with a chosen grammatical gender.
#[derive(Debug, Clone, Copy)]
pub struct GenderedNumber {
    n: i64,
    gender: Gender,
}

impl fmt::Display for GenderedNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&int_to_words_gender(self.n, self.gender))
    }
}

/// A number rendered as an ordinal ("сорок второй").
#[derive(Debug, Clone, Copy)]
pub struct OrdinalNumber {
    n: i64,
    gender: Gender,
}

impl fmt::Display for OrdinalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&ordinal(self.n, self.gender))
    }
}

/// A number paired with noun declension forms ("5 рублей").
#[derive(Debug, Clone, Copy)]
pub struct NumberWithNoun<'a> {
    n: i64,
    forms: (&'a str, &'a str, &'a str),
}

impl fmt::Display for NumberWithNoun<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let words = crate::int_to_words(self.n);
        let noun = crate::decline(self.n, self.forms.0, self.forms.1, self.forms.2);
        write!(f, "{words} {noun}")
    }
}

/// A currency amount ready for formatting.
#[derive(Debug, Clone, Copy)]
pub struct MoneyAmount<'a> {
    whole: i64,
    cents: u32,
    currency: &'a Currency<'a>,
}

impl fmt::Display for MoneyAmount<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&money(self.whole, self.cents, self.currency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RUB;
    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    #[test]
    fn test_number_builder_cardinals() {
        assert_eq!(Number::new(42).masculine().to_string(), "сорок два");
        assert_eq!(Number::new(42).feminine().to_string(), "сорок две");
        assert_eq!(Number::new(1).neuter().to_string(), "одно");
    }

    #[test]
    fn test_number_builder_ordinal() {
        assert_eq!(
            Number::new(42).ordinal(Gender::Masculine).to_string(),
            "сорок второй"
        );
        assert_eq!(
            Number::new(2026).ordinal(Gender::Masculine).to_string(),
            "две тысячи двадцать шестой"
        );
    }

    #[test]
    fn test_number_builder_noun() {
        assert_eq!(
            Number::new(5)
                .with_noun("рубль", "рубля", "рублей")
                .to_string(),
            "пять рублей"
        );
        assert_eq!(
            Number::new(21)
                .with_noun("рубль", "рубля", "рублей")
                .to_string(),
            "двадцать один рубль"
        );
    }

    #[test]
    fn test_number_builder_money() {
        assert_eq!(
            Number::new(1234).money(56, &RUB).to_string(),
            "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
        );
    }

    #[test]
    fn test_display_no_alloc_in_caller() {
        // Regression test: Display wrappers must be usable via write!.
        use core::fmt::Write;
        let mut s = String::new();
        write!(&mut s, "{}", Number::new(42).masculine()).unwrap();
        assert_eq!(s, "сорок два");
    }
}
