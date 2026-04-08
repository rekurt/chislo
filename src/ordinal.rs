#[cfg(not(feature = "std"))]
use alloc::{format, string::{String, ToString}, vec::Vec};

use crate::convert::convert_int_to_words;
use crate::dictionary::*;
use crate::Gender;

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
    let gi = gender_index(gender);

    if n == 0 {
        return ["нулевой", "нулевая", "нулевое"][gi].to_string();
    }

    let abs_n = (n as i128).unsigned_abs() as u64;
    let mut result = String::new();

    if n < 0 {
        result.push_str("минус ");
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
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TEENS[o][gi]);
    } else if o > 0 {
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        if t > 0 {
            parts.push(TENS[t]);
        }
        parts.push(ORDINAL_ONES[o - 1][gi]);
    } else if t > 0 {
        if h > 0 {
            parts.push(HUNDREDS[h - 1]);
        }
        parts.push(ORDINAL_TENS[t - 2][gi]);
    } else {
        parts.push(ORDINAL_HUNDREDS[h - 1][gi]);
    }

    parts.join(" ")
}

/// Handles ordinals for exact multiples of 1000+.
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
        ORDINAL_ORDERS[order][gi].to_string()
    } else if remaining <= 9 {
        let prefix = ONES_COMPOUND[remaining as usize - 1];
        let suffix = ORDINAL_ORDERS[order][gi];
        format!("{prefix}{suffix}")
    } else {
        let cardinal = convert_int_to_words(remaining as i64, Gender::Masculine);
        let suffix = ORDINAL_ORDERS[order][gi];
        format!("{cardinal} {suffix}")
    }
}

fn gender_index(g: Gender) -> usize {
    match g {
        Gender::Masculine => 0,
        Gender::Feminine => 1,
        Gender::Neuter => 2,
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
        assert_eq!(ordinal(7, Gender::Masculine), "седьмой");
    }

    #[test]
    fn test_ordinal_compound() {
        assert_eq!(ordinal(21, Gender::Masculine), "двадцать первый");
        assert_eq!(ordinal(42, Gender::Masculine), "сорок второй");
        assert_eq!(ordinal(42, Gender::Feminine), "сорок вторая");
    }

    #[test]
    fn test_ordinal_hundreds() {
        assert_eq!(ordinal(100, Gender::Masculine), "сотый");
        assert_eq!(ordinal(200, Gender::Feminine), "двухсотая");
        assert_eq!(ordinal(101, Gender::Masculine), "сто первый");
    }

    #[test]
    fn test_ordinal_thousands() {
        assert_eq!(ordinal(1000, Gender::Masculine), "тысячный");
        assert_eq!(ordinal(2000, Gender::Masculine), "двухтысячный");
        assert_eq!(ordinal(1000000, Gender::Masculine), "миллионный");
    }

    #[test]
    fn test_ordinal_years() {
        assert_eq!(ordinal(2026, Gender::Masculine), "две тысячи двадцать шестой");
        assert_eq!(ordinal(2000, Gender::Masculine), "двухтысячный");
    }

    #[test]
    fn test_ordinal_negative() {
        assert_eq!(ordinal(-1, Gender::Masculine), "минус первый");
    }
}
