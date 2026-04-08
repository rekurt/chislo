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
}
