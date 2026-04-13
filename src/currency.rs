#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

use crate::Gender;
use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::parse::{parse_fractional_digits, split_decimal, strip_sign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Currency<'a> {
    pub whole: (&'a str, &'a str, &'a str),
    pub whole_gender: Gender,
    pub frac: (&'a str, &'a str, &'a str),
    pub frac_gender: Gender,
    pub show_frac: bool,
}

impl<'a> Currency<'a> {
    pub const fn new(
        whole: (&'a str, &'a str, &'a str),
        whole_gender: Gender,
        frac: (&'a str, &'a str, &'a str),
        frac_gender: Gender,
    ) -> Self {
        Self {
            whole,
            whole_gender,
            frac,
            frac_gender,
            show_frac: true,
        }
    }

    pub fn from_iso(code: &str) -> Option<&'static Currency<'static>> {
        let bytes = code.as_bytes();
        if bytes.len() != 3 {
            return None;
        }

        let upper = [
            bytes[0].to_ascii_uppercase(),
            bytes[1].to_ascii_uppercase(),
            bytes[2].to_ascii_uppercase(),
        ];

        for &(iso, cur) in ISO_CURRENCIES {
            if iso.as_bytes() == upper {
                return Some(cur);
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    #[default]
    Trunc,
    HalfUp,
    HalfEven,
}

pub const RUB: Currency<'static> = Currency {
    whole: ("рубль", "рубля", "рублей"),
    whole_gender: Gender::Masculine,
    frac: ("копейка", "копейки", "копеек"),
    frac_gender: Gender::Feminine,
    show_frac: true,
};

pub const USD: Currency<'static> = Currency {
    whole: ("доллар", "доллара", "долларов"),
    whole_gender: Gender::Masculine,
    frac: ("цент", "цента", "центов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const EUR: Currency<'static> = Currency {
    whole: ("евро", "евро", "евро"),
    whole_gender: Gender::Neuter,
    frac: ("цент", "цента", "центов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const GBP: Currency<'static> = Currency {
    whole: ("фунт стерлингов", "фунта стерлингов", "фунтов стерлингов"),
    whole_gender: Gender::Masculine,
    frac: ("пенс", "пенса", "пенсов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const CNY: Currency<'static> = Currency {
    whole: ("юань", "юаня", "юаней"),
    whole_gender: Gender::Masculine,
    frac: ("фэнь", "фэня", "фэней"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const JPY: Currency<'static> = Currency {
    whole: ("иена", "иены", "иен"),
    whole_gender: Gender::Feminine,
    frac: ("сен", "сена", "сенов"),
    frac_gender: Gender::Masculine,
    show_frac: false,
};

pub const KZT: Currency<'static> = Currency {
    whole: ("тенге", "тенге", "тенге"),
    whole_gender: Gender::Masculine,
    frac: ("тиын", "тиына", "тиынов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const BYN: Currency<'static> = Currency {
    whole: (
        "белорусский рубль",
        "белорусских рубля",
        "белорусских рублей",
    ),
    whole_gender: Gender::Masculine,
    frac: ("копейка", "копейки", "копеек"),
    frac_gender: Gender::Feminine,
    show_frac: true,
};

pub const UAH: Currency<'static> = Currency {
    whole: ("гривна", "гривны", "гривен"),
    whole_gender: Gender::Feminine,
    frac: ("копейка", "копейки", "копеек"),
    frac_gender: Gender::Feminine,
    show_frac: true,
};

pub const CHF: Currency<'static> = Currency {
    whole: (
        "швейцарский франк",
        "швейцарских франка",
        "швейцарских франков",
    ),
    whole_gender: Gender::Masculine,
    frac: ("сантим", "сантима", "сантимов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

pub const AED: Currency<'static> = Currency {
    whole: ("дирхам", "дирхама", "дирхамов"),
    whole_gender: Gender::Masculine,
    frac: ("филс", "филса", "филсов"),
    frac_gender: Gender::Masculine,
    show_frac: true,
};

const ISO_CURRENCIES: &[(&str, &Currency<'static>)] = &[
    ("RUB", &RUB),
    ("USD", &USD),
    ("EUR", &EUR),
    ("GBP", &GBP),
    ("CNY", &CNY),
    ("JPY", &JPY),
    ("KZT", &KZT),
    ("BYN", &BYN),
    ("UAH", &UAH),
    ("CHF", &CHF),
    ("AED", &AED),
];

pub fn money(whole: i64, cents: u32, currency: &Currency) -> String {
    let whole_words = convert_int_to_words(whole, currency.whole_gender);
    let whole_decl = get_declension(whole, currency.whole.0, currency.whole.1, currency.whole.2);

    if !currency.show_frac {
        return format!("{whole_words} {whole_decl}");
    }

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
    money_from_str_rounded(amount, currency, RoundingMode::Trunc)
}

pub fn money_from_str_rounded(
    amount: &str,
    currency: &Currency,
    mode: RoundingMode,
) -> Result<String, crate::Error> {
    let (negative, rest) = strip_sign(amount);
    let (whole_str, frac_opt) = split_decimal(rest);
    let mut whole: i64 = if negative {
        format!("-{whole_str}").parse()
    } else {
        whole_str.parse()
    }
    .map_err(|_| crate::Error::InvalidNumber(format!("invalid amount: '{whole_str}'")))?;
    let (cents, carry) = round_cents(frac_opt.unwrap_or(""), mode)?;

    if carry {
        if negative {
            whole = whole.checked_sub(1).ok_or(crate::Error::NumberTooLarge)?;
        } else {
            whole = whole.checked_add(1).ok_or(crate::Error::NumberTooLarge)?;
        }
    }

    let result = money(whole, cents, currency);
    if negative && whole == 0 {
        Ok(format!("минус {result}"))
    } else {
        Ok(result)
    }
}

fn round_cents(frac_str: &str, mode: RoundingMode) -> Result<(u32, bool), crate::Error> {
    if frac_str.is_empty() || matches!(mode, RoundingMode::Trunc) {
        return Ok((parse_fractional_digits(frac_str, 2)?, false));
    }

    let to_digit = |c: char| {
        c.to_digit(10).ok_or_else(|| {
            crate::Error::InvalidNumber(format!("invalid fractional part: '{frac_str}'"))
        })
    };

    let mut chars = frac_str.chars();
    let d1 = match chars.next() {
        Some(c) => to_digit(c)?,
        None => return Ok((parse_fractional_digits(frac_str, 2)?, false)),
    };
    let d2 = match chars.next() {
        Some(c) => to_digit(c)?,
        None => return Ok((parse_fractional_digits(frac_str, 2)?, false)),
    };
    let d3 = match chars.next() {
        Some(c) => to_digit(c)?,
        None => return Ok((parse_fractional_digits(frac_str, 2)?, false)),
    };
    let mut tail_nonzero = false;
    for c in chars {
        to_digit(c)?;
        if c != '0' {
            tail_nonzero = true;
        }
    }
    let mut cents = d1 * 10 + d2;

    let round_up = match mode {
        RoundingMode::Trunc => false,
        RoundingMode::HalfUp => d3 >= 5,
        RoundingMode::HalfEven => match d3.cmp(&5) {
            core::cmp::Ordering::Greater => true,
            core::cmp::Ordering::Less => false,
            core::cmp::Ordering::Equal => tail_nonzero || cents % 2 == 1,
        },
    };

    if round_up {
        cents += 1;
    }

    if cents >= 100 {
        Ok((cents - 100, true))
    } else {
        Ok((cents, false))
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
    fn test_money_from_str_comma() {
        assert_eq!(
            money_from_str("1234,56", &RUB).unwrap(),
            "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
        );
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
    fn test_money_new_currencies() {
        assert_eq!(
            money(10, 50, &GBP),
            "десять фунтов стерлингов пятьдесят пенсов"
        );
        assert_eq!(money(3, 25, &CNY), "три юаня двадцать пять фэней");
        assert_eq!(money(100, 0, &JPY), "сто иен");
        assert_eq!(money(1, 0, &JPY), "одна иена");
        assert_eq!(money(500, 0, &KZT), "пятьсот тенге ноль тиынов");
        assert_eq!(money(1, 0, &UAH), "одна гривна ноль копеек");
    }

    #[test]
    fn test_currency_from_iso() {
        assert!(Currency::from_iso("RUB").is_some());
        assert!(Currency::from_iso("rub").is_some());
        assert!(Currency::from_iso("JPY").is_some());
        assert!(Currency::from_iso("XYZ").is_none());
        assert!(Currency::from_iso("RU").is_none());
        assert!(Currency::from_iso("RUBL").is_none());
        let cur = Currency::from_iso("USD").unwrap();
        assert_eq!(money(1, 0, cur), "один доллар ноль центов");
    }

    #[test]
    fn test_money_declension_boundaries() {
        let cases: &[(i64, u32, &Currency, &str)] = &[
            (0, 0, &RUB, "ноль рублей ноль копеек"),
            (1, 1, &RUB, "один рубль одна копейка"),
            (2, 2, &RUB, "два рубля две копейки"),
            (5, 5, &RUB, "пять рублей пять копеек"),
            (11, 11, &RUB, "одиннадцать рублей одиннадцать копеек"),
            (21, 21, &RUB, "двадцать один рубль двадцать одна копейка"),
            (22, 22, &RUB, "двадцать два рубля двадцать две копейки"),
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
        assert!(money_from_str("abc", &RUB).is_err());
        assert!(money_from_str("", &RUB).is_err());
        assert!(
            money_from_str_rounded("1.125x", &RUB, RoundingMode::HalfUp).is_err(),
            "non-digit characters in fractional tail must be rejected"
        );
    }

    #[test]
    fn test_money_rounding_half_up() {
        assert_eq!(
            money_from_str_rounded("100.994", &RUB, RoundingMode::HalfUp).unwrap(),
            "сто рублей девяносто девять копеек"
        );
        assert_eq!(
            money_from_str_rounded("100.995", &RUB, RoundingMode::HalfUp).unwrap(),
            "сто один рубль ноль копеек"
        );
        assert_eq!(
            money_from_str_rounded("100.999", &RUB, RoundingMode::HalfUp).unwrap(),
            "сто один рубль ноль копеек"
        );
    }

    #[test]
    fn test_money_rounding_half_even() {
        assert_eq!(
            money_from_str_rounded("1.125", &RUB, RoundingMode::HalfEven).unwrap(),
            "один рубль двенадцать копеек"
        );
        assert_eq!(
            money_from_str_rounded("1.135", &RUB, RoundingMode::HalfEven).unwrap(),
            "один рубль четырнадцать копеек"
        );
        assert_eq!(
            money_from_str_rounded("1.1251", &RUB, RoundingMode::HalfEven).unwrap(),
            "один рубль тринадцать копеек"
        );
        // non-zero digit beyond 18th position must still break the tie
        assert_eq!(
            money_from_str_rounded(
                "1.1250000000000000001",
                &RUB,
                RoundingMode::HalfEven
            )
            .unwrap(),
            "один рубль тринадцать копеек"
        );
    }

    #[test]
    fn test_money_from_str_negative() {
        assert_eq!(
            money_from_str("-1234.56", &RUB).unwrap(),
            "минус одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"
        );
        assert_eq!(
            money_from_str("-0.50", &RUB).unwrap(),
            "минус ноль рублей пятьдесят копеек"
        );
        assert_eq!(
            money_from_str("-0.01", &RUB).unwrap(),
            "минус ноль рублей одна копейка"
        );
        assert_eq!(
            money_from_str("-5", &USD).unwrap(),
            "минус пять долларов ноль центов"
        );
    }

    #[test]
    fn test_money_rounding_negative_subunit() {
        assert_eq!(
            money_from_str_rounded("-0.995", &RUB, RoundingMode::HalfUp).unwrap(),
            "минус один рубль ноль копеек"
        );
        assert_eq!(
            money_from_str_rounded("-0.995", &RUB, RoundingMode::HalfEven).unwrap(),
            "минус один рубль ноль копеек"
        );
        assert_eq!(
            money_from_str_rounded("-0.125", &RUB, RoundingMode::HalfEven).unwrap(),
            "минус ноль рублей двенадцать копеек"
        );
    }

    #[test]
    fn test_money_from_str_i64_min_boundary() {
        assert!(money_from_str("-9223372036854775808.01", &RUB).is_ok());
        assert_eq!(
            money_from_str("-9223372036854775808.01", &RUB).unwrap(),
            "минус девять квинтиллионов двести двадцать три квадриллиона триста семьдесят два триллиона тридцать шесть миллиардов восемьсот пятьдесят четыре миллиона семьсот семьдесят пять тысяч восемьсот восемь рублей одна копейка"
        );
    }

    #[test]
    fn test_money_custom_currency() {
        let custom = Currency::new(
            ("тонна", "тонны", "тонн"),
            Gender::Feminine,
            ("килограмм", "килограмма", "килограммов"),
            Gender::Masculine,
        );
        assert_eq!(money(1, 0, &custom), "одна тонна ноль килограммов");
        assert_eq!(money(2, 1, &custom), "две тонны один килограмм");
        assert_eq!(money(5, 5, &custom), "пять тонн пять килограммов");
    }
}
