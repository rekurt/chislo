#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::Gender;
use crate::convert::convert_int_to_words;
use crate::dictionary::*;

/// Converts a number to its ordinal form in Russian.
///
/// Only the last component becomes ordinal; preceding words stay cardinal.
///
/// # Examples
///
/// ```
/// use chislo::{ordinal, Gender};
///
/// assert_eq!(ordinal(1, Gender::Masculine), "первый");
/// assert_eq!(ordinal(1, Gender::Feminine), "первая");
/// assert_eq!(ordinal(42, Gender::Masculine), "сорок второй");
/// assert_eq!(ordinal(100, Gender::Masculine), "сотый");
/// assert_eq!(ordinal(2026, Gender::Masculine), "две тысячи двадцать шестой");
/// ```
pub fn ordinal(n: i64, gender: Gender) -> String {
    let gi = gender.index();

    if n == 0 {
        return ZERO_ORDINAL[gi].to_string();
    }

    let abs_n = (n as i128).unsigned_abs() as u64;
    let mut result = String::new();

    if n < 0 {
        result.push_str(MINUS);
        result.push(' ');
    }

    let last_triad = (abs_n % 1000) as u32;

    if last_triad > 0 {
        let prefix_n = abs_n - last_triad as u64;
        if prefix_n > 0 {
            result.push_str(&convert_int_to_words(prefix_n as i64, Gender::Masculine));
            result.push(' ');
        }
        result.push_str(&ordinal_triad(last_triad, gi));
    } else {
        result.push_str(&ordinal_round_order(abs_n, gi));
    }

    result
}

/// Converts last triad (1-999) to ordinal form.
fn ordinal_triad(n: u32, gi: usize) -> String {
    let h = (n / 100) as usize;
    let t = ((n % 100) / 10) as usize;
    let o = (n % 10) as usize;

    let mut parts: Vec<&str> = Vec::new();

    if t == 1 {
        // Teen: hundreds stay cardinal, teen becomes ordinal
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TEENS[o][gi]);
    } else if o > 0 {
        // Has ones: hundreds + tens stay cardinal, ones becomes ordinal
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        if t > 0 {
            parts.push(TENS[t]);
        }
        parts.push(ORDINAL_ONES[o - 1][gi]);
    } else if t > 0 {
        // Ends with tens: hundreds stay cardinal, tens becomes ordinal
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TENS[t - 2][gi]); // index 0=20, 1=30, ...
    } else {
        // Only hundreds
        parts.push(ORDINAL_HUNDREDS[h - 1][gi]);
    }

    parts.join(" ")
}

/// Handles ordinals for exact multiples of 1000+ (e.g., 2000 → двухтысячный).
fn ordinal_round_order(n: u64, gi: usize) -> String {
    let mut remaining = n;
    let mut order: usize = 0;

    while remaining > 0 && remaining % 1000 == 0 {
        remaining /= 1000;
        order += 1;
    }

    if order == 0 || order >= ORDINAL_ORDERS.len() {
        return convert_int_to_words(n as i64, Gender::Masculine);
    }

    if remaining == 1 {
        // 1000 → тысячный, 1000000 → миллионный
        ORDINAL_ORDERS[order][gi].to_string()
    } else if let Some(prefix) = compound_prefix(remaining) {
        let suffix = ORDINAL_ORDERS[order][gi];
        format!("{prefix}{suffix}")
    } else {
        let cardinal = convert_int_to_words(remaining as i64, Gender::Masculine);
        let suffix = ORDINAL_ORDERS[order][gi];
        format!("{cardinal} {suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal_basic() {
        assert_eq!(ordinal(0, Gender::Masculine), "нулевой");
        assert_eq!(ordinal(0, Gender::Feminine), "нулевая");
        assert_eq!(ordinal(0, Gender::Neuter), "нулевое");
        assert_eq!(ordinal(1, Gender::Masculine), "первый");
        assert_eq!(ordinal(1, Gender::Feminine), "первая");
        assert_eq!(ordinal(1, Gender::Neuter), "первое");
        assert_eq!(ordinal(2, Gender::Masculine), "второй");
        assert_eq!(ordinal(3, Gender::Feminine), "третья");
        assert_eq!(ordinal(4, Gender::Masculine), "четвёртый");
        assert_eq!(ordinal(5, Gender::Masculine), "пятый");
        assert_eq!(ordinal(6, Gender::Masculine), "шестой");
        assert_eq!(ordinal(7, Gender::Masculine), "седьмой");
        assert_eq!(ordinal(8, Gender::Masculine), "восьмой");
        assert_eq!(ordinal(9, Gender::Masculine), "девятый");
    }

    #[test]
    fn test_ordinal_teens() {
        assert_eq!(ordinal(10, Gender::Masculine), "десятый");
        assert_eq!(ordinal(11, Gender::Masculine), "одиннадцатый");
        assert_eq!(ordinal(12, Gender::Feminine), "двенадцатая");
        assert_eq!(ordinal(15, Gender::Masculine), "пятнадцатый");
        assert_eq!(ordinal(19, Gender::Neuter), "девятнадцатое");
    }

    #[test]
    fn test_ordinal_tens() {
        assert_eq!(ordinal(20, Gender::Masculine), "двадцатый");
        assert_eq!(ordinal(30, Gender::Feminine), "тридцатая");
        assert_eq!(ordinal(40, Gender::Masculine), "сороковой");
        assert_eq!(ordinal(50, Gender::Masculine), "пятидесятый");
        assert_eq!(ordinal(90, Gender::Neuter), "девяностое");
    }

    #[test]
    fn test_ordinal_compound() {
        assert_eq!(ordinal(21, Gender::Masculine), "двадцать первый");
        assert_eq!(ordinal(42, Gender::Masculine), "сорок второй");
        assert_eq!(ordinal(42, Gender::Feminine), "сорок вторая");
        assert_eq!(ordinal(99, Gender::Masculine), "девяносто девятый");
    }

    #[test]
    fn test_ordinal_hundreds() {
        assert_eq!(ordinal(100, Gender::Masculine), "сотый");
        assert_eq!(ordinal(200, Gender::Feminine), "двухсотая");
        assert_eq!(ordinal(300, Gender::Masculine), "трёхсотый");
        assert_eq!(ordinal(500, Gender::Feminine), "пятисотая");
        assert_eq!(ordinal(101, Gender::Masculine), "сто первый");
        assert_eq!(ordinal(110, Gender::Masculine), "сто десятый");
        assert_eq!(ordinal(150, Gender::Masculine), "сто пятидесятый");
        assert_eq!(
            ordinal(999, Gender::Masculine),
            "девятьсот девяносто девятый"
        );
    }

    #[test]
    fn test_ordinal_thousands() {
        assert_eq!(ordinal(1000, Gender::Masculine), "тысячный");
        assert_eq!(ordinal(2000, Gender::Masculine), "двухтысячный");
        assert_eq!(ordinal(3000, Gender::Feminine), "трёхтысячная");
        assert_eq!(ordinal(5000, Gender::Masculine), "пятитысячный");
        assert_eq!(ordinal(1000000, Gender::Masculine), "миллионный");
        assert_eq!(ordinal(2000000, Gender::Masculine), "двухмиллионный");
        assert_eq!(ordinal(10000, Gender::Masculine), "десятитысячный");
        assert_eq!(ordinal(12000, Gender::Masculine), "двенадцатитысячный");
        assert_eq!(ordinal(15000, Gender::Feminine), "пятнадцатитысячная");
        assert_eq!(ordinal(20000, Gender::Masculine), "двадцатитысячный");
        assert_eq!(ordinal(25000, Gender::Neuter), "двадцатипятитысячное");
        assert_eq!(ordinal(100000, Gender::Masculine), "стотысячный");
        assert_eq!(ordinal(200000, Gender::Feminine), "двухсоттысячная");
        assert_eq!(ordinal(500000, Gender::Masculine), "пятисоттысячный");
    }

    #[test]
    fn test_ordinal_years() {
        assert_eq!(
            ordinal(2026, Gender::Masculine),
            "две тысячи двадцать шестой"
        );
        assert_eq!(
            ordinal(1945, Gender::Masculine),
            "одна тысяча девятьсот сорок пятый"
        );
        assert_eq!(ordinal(2000, Gender::Masculine), "двухтысячный");
    }

    #[test]
    fn test_ordinal_negative() {
        assert_eq!(ordinal(-1, Gender::Masculine), "минус первый");
        assert_eq!(ordinal(-42, Gender::Feminine), "минус сорок вторая");
    }

    #[test]
    fn test_ordinal_all_digits_all_genders() {
        let cases: &[(i64, Gender, &str)] = &[
            (1, Gender::Masculine, "первый"),
            (1, Gender::Feminine, "первая"),
            (1, Gender::Neuter, "первое"),
            (2, Gender::Masculine, "второй"),
            (2, Gender::Feminine, "вторая"),
            (2, Gender::Neuter, "второе"),
            (3, Gender::Masculine, "третий"),
            (3, Gender::Feminine, "третья"),
            (3, Gender::Neuter, "третье"),
            (4, Gender::Masculine, "четвёртый"),
            (4, Gender::Feminine, "четвёртая"),
            (4, Gender::Neuter, "четвёртое"),
            (5, Gender::Masculine, "пятый"),
            (5, Gender::Feminine, "пятая"),
            (5, Gender::Neuter, "пятое"),
            (6, Gender::Masculine, "шестой"),
            (6, Gender::Feminine, "шестая"),
            (6, Gender::Neuter, "шестое"),
            (7, Gender::Masculine, "седьмой"),
            (7, Gender::Feminine, "седьмая"),
            (7, Gender::Neuter, "седьмое"),
            (8, Gender::Masculine, "восьмой"),
            (8, Gender::Feminine, "восьмая"),
            (8, Gender::Neuter, "восьмое"),
            (9, Gender::Masculine, "девятый"),
            (9, Gender::Feminine, "девятая"),
            (9, Gender::Neuter, "девятое"),
        ];
        for &(n, gender, expected) in cases {
            assert_eq!(ordinal(n, gender), expected, "ordinal({n}, {gender:?})");
        }
    }

    #[test]
    fn test_ordinal_compound_hundreds() {
        let cases: &[(i64, Gender, &str)] = &[
            (115, Gender::Masculine, "сто пятнадцатый"),
            (115, Gender::Feminine, "сто пятнадцатая"),
            (342, Gender::Masculine, "триста сорок второй"),
            (342, Gender::Feminine, "триста сорок вторая"),
            (899, Gender::Masculine, "восемьсот девяносто девятый"),
            (250, Gender::Masculine, "двести пятидесятый"),
            (400, Gender::Masculine, "четырёхсотый"),
            (600, Gender::Feminine, "шестисотая"),
            (711, Gender::Neuter, "семьсот одиннадцатое"),
        ];
        for &(n, gender, expected) in cases {
            assert_eq!(ordinal(n, gender), expected, "ordinal({n}, {gender:?})");
        }
    }

    #[test]
    fn test_ordinal_large_round() {
        let cases: &[(i64, Gender, &str)] = &[
            (1_000_000_000, Gender::Masculine, "миллиардный"),
            (1_000_000_000, Gender::Feminine, "миллиардная"),
            (9_000_000_000, Gender::Masculine, "девятимиллиардный"),
            (1_000_000_000_000, Gender::Masculine, "триллионный"),
        ];
        for &(n, gender, expected) in cases {
            assert_eq!(ordinal(n, gender), expected, "ordinal({n}, {gender:?})");
        }
    }
}
