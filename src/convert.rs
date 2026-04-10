#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::Gender;
use crate::decline::get_declension;
use crate::dictionary::{HUNDREDS, MINUS, ONES, ORDERS, TEENS, TENS, ZERO};

/// Converts an integer to Russian words with the specified gender.
pub(crate) fn convert_int_to_words(n: i64, gender: Gender) -> String {
    if n == 0 {
        return ZERO.to_string();
    }

    let mut result = String::new();

    if n < 0 {
        result.push_str(MINUS);
        result.push(' ');
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
            // Determine gender form index (0=masculine, 1=feminine, 2=neuter)
            let gender_idx = if order == 0 {
                // Ones place: use specified gender
                base_gender.index()
            } else if order == 1 {
                // Thousands: always feminine
                Gender::Feminine.index()
            } else {
                // Other orders: always masculine
                Gender::Masculine.index()
            };

            parts.push(ONES[o - 1][gender_idx]);
        }
    }

    parts.join(" ")
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

    #[test]
    fn test_single_digits() {
        let cases: [(i64, &str); 10] = [
            (0, "ноль"),
            (1, "один"),
            (2, "два"),
            (3, "три"),
            (4, "четыре"),
            (5, "пять"),
            (6, "шесть"),
            (7, "семь"),
            (8, "восемь"),
            (9, "девять"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "single digit {n}"
            );
        }
    }

    #[test]
    fn test_all_teens() {
        let cases: [(i64, &str); 10] = [
            (10, "десять"),
            (11, "одиннадцать"),
            (12, "двенадцать"),
            (13, "тринадцать"),
            (14, "четырнадцать"),
            (15, "пятнадцать"),
            (16, "шестнадцать"),
            (17, "семнадцать"),
            (18, "восемнадцать"),
            (19, "девятнадцать"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "teen {n}"
            );
        }
    }

    #[test]
    fn test_all_tens() {
        let cases: [(i64, &str); 9] = [
            (10, "десять"),
            (20, "двадцать"),
            (30, "тридцать"),
            (40, "сорок"),
            (50, "пятьдесят"),
            (60, "шестьдесят"),
            (70, "семьдесят"),
            (80, "восемьдесят"),
            (90, "девяносто"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "tens {n}"
            );
        }
    }

    #[test]
    fn test_all_hundreds() {
        let cases: [(i64, &str); 9] = [
            (100, "сто"),
            (200, "двести"),
            (300, "триста"),
            (400, "четыреста"),
            (500, "пятьсот"),
            (600, "шестьсот"),
            (700, "семьсот"),
            (800, "восемьсот"),
            (900, "девятьсот"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "hundreds {n}"
            );
        }
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(
            convert_int_to_words(i64::MAX, Gender::Masculine),
            "девять квинтиллионов двести двадцать три квадриллиона триста семьдесят два триллиона тридцать шесть миллиардов восемьсот пятьдесят четыре миллиона семьсот семьдесят пять тысяч восемьсот семь"
        );
        assert_eq!(
            convert_int_to_words(i64::MIN, Gender::Masculine),
            "минус девять квинтиллионов двести двадцать три квадриллиона триста семьдесят два триллиона тридцать шесть миллиардов восемьсот пятьдесят четыре миллиона семьсот семьдесят пять тысяч восемьсот восемь"
        );
    }

    #[test]
    fn test_gender_all_digits() {
        let cases: &[(i64, Gender, &str)] = &[
            // Digit 1 varies by gender
            (1, Gender::Masculine, "один"),
            (1, Gender::Feminine, "одна"),
            (1, Gender::Neuter, "одно"),
            // Digit 2 varies by gender
            (2, Gender::Masculine, "два"),
            (2, Gender::Feminine, "две"),
            (2, Gender::Neuter, "два"),
            // Digits 3-9 are gender-invariant
            (3, Gender::Masculine, "три"),
            (3, Gender::Feminine, "три"),
            (3, Gender::Neuter, "три"),
            (4, Gender::Masculine, "четыре"),
            (4, Gender::Feminine, "четыре"),
            (5, Gender::Masculine, "пять"),
            (5, Gender::Feminine, "пять"),
            (6, Gender::Masculine, "шесть"),
            (6, Gender::Feminine, "шесть"),
            (7, Gender::Masculine, "семь"),
            (7, Gender::Feminine, "семь"),
            (8, Gender::Masculine, "восемь"),
            (8, Gender::Feminine, "восемь"),
            (9, Gender::Masculine, "девять"),
            (9, Gender::Feminine, "девять"),
            // Gender in compound numbers
            (21, Gender::Feminine, "двадцать одна"),
            (22, Gender::Feminine, "двадцать две"),
            (101, Gender::Feminine, "сто одна"),
            (102, Gender::Feminine, "сто две"),
            (101, Gender::Neuter, "сто одно"),
            (102, Gender::Neuter, "сто два"),
        ];
        for &(n, gender, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, gender),
                expected,
                "gender_all_digits({n}, {gender:?})"
            );
        }
    }

    #[test]
    fn test_zero_triads_and_middle_teens() {
        let cases: [(i64, &str); 10] = [
            (111, "сто одиннадцать"),
            (211, "двести одиннадцать"),
            (311, "триста одиннадцать"),
            (11_000, "одиннадцать тысяч"),
            (12_000, "двенадцать тысяч"),
            (111_000, "сто одиннадцать тысяч"),
            (211_000, "двести одиннадцать тысяч"),
            (1_000_001, "один миллион один"),
            (100_000_100, "сто миллионов сто"),
            (1_001_000, "один миллион одна тысяча"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, Gender::Masculine),
                expected,
                "zero_triads({n})"
            );
        }
    }

    #[test]
    fn test_millions_always_masculine() {
        let cases: &[(i64, Gender, &str)] = &[
            (1_000_000, Gender::Feminine, "один миллион"),
            (2_000_000, Gender::Feminine, "два миллиона"),
            (1_000_000_000, Gender::Feminine, "один миллиард"),
            (2_000_000_000, Gender::Feminine, "два миллиарда"),
            // Thousands still feminine regardless of base gender
            (1_000, Gender::Feminine, "одна тысяча"),
            (2_000, Gender::Feminine, "две тысячи"),
            // Final triad respects base gender
            (1_000_001, Gender::Feminine, "один миллион одна"),
            (1_000_002, Gender::Feminine, "один миллион две"),
        ];
        for &(n, gender, expected) in cases {
            assert_eq!(
                convert_int_to_words(n, gender),
                expected,
                "millions_masculine({n}, {gender:?})"
            );
        }
    }
}
