#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

use crate::Gender;
use crate::convert::convert_int_to_words;
use crate::decline::get_declension;

/// Describes a currency for formatting amounts in words.
pub struct Currency<'a> {
    pub whole: (&'a str, &'a str, &'a str),
    pub whole_gender: Gender,
    pub frac: (&'a str, &'a str, &'a str),
    pub frac_gender: Gender,
}

pub const RUB: Currency<'static> = Currency {
    whole: ("рубль", "рубля", "рублей"),
    whole_gender: Gender::Masculine,
    frac: ("копейка", "копейки", "копеек"),
    frac_gender: Gender::Feminine,
};

pub const USD: Currency<'static> = Currency {
    whole: ("доллар", "доллара", "долларов"),
    whole_gender: Gender::Masculine,
    frac: ("цент", "цента", "центов"),
    frac_gender: Gender::Masculine,
};

pub const EUR: Currency<'static> = Currency {
    whole: ("евро", "евро", "евро"),
    whole_gender: Gender::Neuter,
    frac: ("цент", "цента", "центов"),
    frac_gender: Gender::Masculine,
};

pub fn money(whole: i64, cents: u32, currency: &Currency) -> String {
    let whole_words = convert_int_to_words(whole, currency.whole_gender);
    let whole_decl = get_declension(whole, currency.whole.0, currency.whole.1, currency.whole.2);
    let cents_words = convert_int_to_words(cents as i64, currency.frac_gender);
    let cents_decl = get_declension(
        cents as i64,
        currency.frac.0,
        currency.frac.1,
        currency.frac.2,
    );
    format!("{whole_words} {whole_decl} {cents_words} {cents_decl}")
}

pub fn money_from_str(amount: &str, currency: &Currency) -> Result<String, crate::Error> {
    let parts: Vec<&str> = amount.splitn(2, '.').collect();

    let whole: i64 = parts[0]
        .parse()
        .map_err(|_| crate::Error::InvalidNumber(format!("invalid amount: '{}'", parts[0])))?;

    let cents = if parts.len() > 1 {
        parse_cents(parts[1])?
    } else {
        0
    };

    Ok(money(whole, cents, currency))
}

fn parse_cents(frac_str: &str) -> Result<u32, crate::Error> {
    let chars: Vec<char> = frac_str.chars().take(2).collect();
    if chars.len() >= 2 {
        let s: String = chars.into_iter().collect();
        s.parse::<u32>()
            .map_err(|_| crate::Error::InvalidNumber(format!("invalid cents: '{frac_str}'")))
    } else if chars.len() == 1 {
        let s: String = chars.into_iter().collect();
        s.parse::<u32>()
            .map(|d| d * 10)
            .map_err(|_| crate::Error::InvalidNumber(format!("invalid cents: '{frac_str}'")))
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_rub() {
        assert_eq!(
            money(1234, 56, &RUB),
            "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
        );
        assert_eq!(money(1, 1, &RUB), "один рубль одна копейка");
    }

    #[test]
    fn test_money_from_str() {
        assert_eq!(
            money_from_str("1234.56", &RUB).unwrap(),
            "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
        );
        assert!(money_from_str("abc", &RUB).is_err());
    }

    #[test]
    fn test_money_all_currencies() {
        let cases: &[(i64, u32, &Currency, &str)] = &[
            (
                1234,
                56,
                &RUB,
                "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек",
            ),
            (
                1234,
                56,
                &USD,
                "одна тысяча двести тридцать четыре доллара пятьдесят шесть центов",
            ),
            (
                1234,
                56,
                &EUR,
                "одна тысяча двести тридцать четыре евро пятьдесят шесть центов",
            ),
            (1, 0, &EUR, "одно евро ноль центов"),
        ];
        for &(whole, cents, currency, expected) in cases {
            assert_eq!(
                money(whole, cents, currency),
                expected,
                "money({whole}, {cents})"
            );
        }
    }

    #[test]
    fn test_money_declension_boundaries() {
        let cases: &[(i64, u32, &Currency, &str)] = &[
            // RUB
            (0, 0, &RUB, "ноль рублей ноль копеек"),
            (1, 1, &RUB, "один рубль одна копейка"),
            (2, 2, &RUB, "два рубля две копейки"),
            (5, 5, &RUB, "пять рублей пять копеек"),
            (11, 11, &RUB, "одиннадцать рублей одиннадцать копеек"),
            (21, 21, &RUB, "двадцать один рубль двадцать одна копейка"),
            (22, 22, &RUB, "двадцать два рубля двадцать две копейки"),
            // USD
            (1, 1, &USD, "один доллар один цент"),
            (2, 2, &USD, "два доллара два цента"),
            (5, 5, &USD, "пять долларов пять центов"),
            (21, 21, &USD, "двадцать один доллар двадцать один цент"),
        ];
        for &(whole, cents, currency, expected) in cases {
            assert_eq!(
                money(whole, cents, currency),
                expected,
                "declension({whole}, {cents})"
            );
        }
    }

    #[test]
    fn test_money_large_amounts() {
        assert_eq!(money(1_000_000, 0, &RUB), "один миллион рублей ноль копеек");
        assert_eq!(
            money(999_999, 99, &RUB),
            "девятьсот девяносто девять тысяч девятьсот девяносто девять рублей девяносто девять копеек"
        );
    }

    #[test]
    fn test_money_from_str_edge_cases() {
        let ok_cases: &[(&str, &Currency, &str)] = &[
            ("100", &RUB, "сто рублей ноль копеек"),
            ("100.5", &RUB, "сто рублей пятьдесят копеек"),
            ("100.05", &RUB, "сто рублей пять копеек"),
            ("100.999", &RUB, "сто рублей девяносто девять копеек"),
            ("0.00", &RUB, "ноль рублей ноль копеек"),
            ("1.01", &USD, "один доллар один цент"),
            ("1.01", &EUR, "одно евро один цент"),
        ];
        for &(input, currency, expected) in ok_cases {
            assert_eq!(
                money_from_str(input, currency).unwrap(),
                expected,
                "money_from_str(\"{input}\")"
            );
        }
        // Error cases
        assert!(money_from_str("abc", &RUB).is_err());
        assert!(money_from_str("", &RUB).is_err());
    }

    #[test]
    fn test_money_custom_currency() {
        let custom = Currency {
            whole: ("тонна", "тонны", "тонн"),
            whole_gender: Gender::Feminine,
            frac: ("килограмм", "килограмма", "килограммов"),
            frac_gender: Gender::Masculine,
        };
        assert_eq!(money(1, 0, &custom), "одна тонна ноль килограммов");
        assert_eq!(money(2, 1, &custom), "две тонны один килограмм");
        assert_eq!(money(5, 5, &custom), "пять тонн пять килограммов");
    }
}
