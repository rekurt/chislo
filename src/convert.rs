#[cfg(not(feature = "std"))]
use alloc::{string::{String, ToString}, vec::Vec};

use crate::Gender;
use crate::decline::get_declension;
use crate::dictionary::{HUNDREDS, ONES, ORDERS, TEENS, TENS};

pub(crate) fn convert_int_to_words(n: i64, gender: Gender) -> String {
    if n == 0 {
        return "ноль".to_string();
    }

    let mut result = String::new();

    if n < 0 {
        result.push_str("минус ");
    }

    let mut abs_n = (n as i128).unsigned_abs() as u64;
    let mut parts: Vec<String> = Vec::new();
    let mut order: usize = 0;

    while abs_n > 0 {
        let triad = (abs_n % 1000) as u32;
        abs_n /= 1000;

        if triad != 0 {
            let words = triad_to_words(triad, order, gender);
            let mut part = words;

            if order > 0 && order < ORDERS.len() {
                let order_word = get_declension(
                    triad as i64,
                    ORDERS[order][0],
                    ORDERS[order][1],
                    ORDERS[order][2],
                );
                if !order_word.is_empty() {
                    part.push(' ');
                    part.push_str(order_word);
                }
            }

            parts.push(part);
        }

        order += 1;
    }

    parts.reverse();
    result.push_str(&parts.join(" "));
    result
}

fn triad_to_words(n: u32, order: usize, base_gender: Gender) -> String {
    let h = (n / 100) as usize;
    let t = ((n % 100) / 10) as usize;
    let o = (n % 10) as usize;

    let mut parts: Vec<&str> = Vec::new();

    if h > 0 {
        parts.push(HUNDREDS[h - 1]);
    }

    if t == 1 {
        parts.push(TEENS[o]);
    } else {
        if t > 0 {
            parts.push(TENS[t]);
        }

        if o > 0 {
            let gender_idx = if order == 0 {
                clamp_gender(base_gender)
            } else if order == 1 {
                2
            } else {
                1
            };

            parts.push(ONES[o - 1][gender_idx]);
        }
    }

    parts.join(" ")
}

fn clamp_gender(g: Gender) -> usize {
    match g {
        Gender::Masculine => 1,
        Gender::Feminine => 2,
        Gender::Neuter => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_words_basic() {
        let cases = [
            (0, "ноль"),
            (7, "семь"),
            (42, "сорок два"),
            (1000, "одна тысяча"),
        ];
        for (n, expected) in cases {
            assert_eq!(convert_int_to_words(n, Gender::Masculine), expected);
        }
    }

    #[test]
    fn test_int_to_words_gender() {
        assert_eq!(convert_int_to_words(1, Gender::Neuter), "одно");
        assert_eq!(convert_int_to_words(1, Gender::Feminine), "одна");
        assert_eq!(convert_int_to_words(2, Gender::Feminine), "две");
    }
}
