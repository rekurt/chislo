#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::Gender;
use crate::decline::get_declension;
use crate::dictionary::{HUNDREDS, ONES, ORDERS, TEENS, TENS};

/// Converts an integer to Russian words with the specified gender.
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

/// Converts a 3-digit number (triad) to Russian words.
fn triad_to_words(n: u32, order: usize, base_gender: Gender) -> String {
    let h = (n / 100) as usize;
    let t = ((n % 100) / 10) as usize;
    let o = (n % 10) as usize;

    let mut parts: Vec<&str> = Vec::new();

    // Hundreds
    if h > 0 {
        parts.push(HUNDREDS[h - 1]);
    }

    // Tens and ones
    if t == 1 {
        // Teens (10-19): special case
        parts.push(TEENS[o]);
    } else {
        if t > 0 {
            parts.push(TENS[t]);
        }

        if o > 0 {
            // Determine gender form index
            let gender_idx = if order == 0 {
                // Ones place: use specified gender
                clamp_gender(base_gender)
            } else if order == 1 {
                // Thousands: always feminine
                2
            } else {
                // Other orders: always masculine
                1
            };

            parts.push(ONES[o - 1][gender_idx]);
        }
    }

    parts.join(" ")
}

/// Maps Gender enum to array index, defaulting to masculine (1) for invalid values.
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
            (15, "пятнадцать"),
            (40, "сорок"),
            (42, "сорок два"),
            (100, "сто"),
            (305, "триста пять"),
            (512, "пятьсот двенадцать"),
            (1000, "одна тысяча"),
            (2001, "две тысячи один"),
            (
                987654,
                "девятьсот восемьдесят семь тысяч шестьсот пятьдесят четыре",
            ),
        ];

        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "int_to_words({n}) failed"
            );
        }
    }

    #[test]
    fn test_int_to_words_large_numbers() {
        let cases = [
            (1_000_000i64, "один миллион"),
            (
                21_304_015,
                "двадцать один миллион триста четыре тысячи пятнадцать",
            ),
            (1_000_000_000, "один миллиард"),
            (
                2_147_483_647,
                "два миллиарда сто сорок семь миллионов четыреста восемьдесят три тысячи шестьсот сорок семь",
            ),
            (
                6_453_345_242_432,
                "шесть триллионов четыреста пятьдесят три миллиарда триста сорок пять миллионов двести сорок две тысячи четыреста тридцать два",
            ),
        ];

        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "int_to_words({n}) failed"
            );
        }
    }

    #[test]
    fn test_int_to_words_negative_and_spacing() {
        assert_eq!(
            convert_int_to_words(-512, Gender::Masculine),
            "минус пятьсот двенадцать"
        );
        assert_eq!(convert_int_to_words(-1, Gender::Masculine), "минус один");

        // Verify no double spaces
        for n in [100, 1000, 1_000_000, 305, 512, 2001] {
            let words = convert_int_to_words(n, Gender::Masculine);
            assert!(
                !words.contains("  "),
                "Double space found in int_to_words({n}): '{words}'"
            );
        }

        // Verify no trailing spaces
        for n in [0, 1, 100, 1000, 987654] {
            let words = convert_int_to_words(n, Gender::Masculine);
            assert_eq!(words, words.trim(), "Trailing space in int_to_words({n})");
        }
    }

    #[test]
    fn test_int_to_words_gender() {
        assert_eq!(convert_int_to_words(42, Gender::Masculine), "сорок два");
        assert_eq!(convert_int_to_words(42, Gender::Feminine), "сорок две");
        assert_eq!(convert_int_to_words(1, Gender::Neuter), "одно");
        assert_eq!(convert_int_to_words(1, Gender::Feminine), "одна");
        assert_eq!(convert_int_to_words(1, Gender::Masculine), "один");
        assert_eq!(convert_int_to_words(2, Gender::Feminine), "две");
        assert_eq!(convert_int_to_words(2, Gender::Masculine), "два");
        assert_eq!(convert_int_to_words(2, Gender::Neuter), "два");
    }

    #[test]
    fn test_thousands_always_feminine() {
        // Thousands should always use feminine forms
        assert_eq!(convert_int_to_words(1000, Gender::Masculine), "одна тысяча");
        assert_eq!(convert_int_to_words(2000, Gender::Masculine), "две тысячи");
        assert_eq!(
            convert_int_to_words(21000, Gender::Masculine),
            "двадцать одна тысяча"
        );
    }
}
