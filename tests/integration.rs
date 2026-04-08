use chislo::{decimal_to_words, decline, int_to_words, int_to_words_gender, Gender};

// ==================== IntToWords ====================

#[test]
fn test_int_to_words_zero() {
    assert_eq!(int_to_words(0), "ноль");
}

#[test]
fn test_int_to_words_single_digits() {
    assert_eq!(int_to_words(1), "один");
    assert_eq!(int_to_words(7), "семь");
    assert_eq!(int_to_words(9), "девять");
}

#[test]
fn test_int_to_words_teens() {
    assert_eq!(int_to_words(10), "десять");
    assert_eq!(int_to_words(11), "одиннадцать");
    assert_eq!(int_to_words(15), "пятнадцать");
    assert_eq!(int_to_words(19), "девятнадцать");
}

#[test]
fn test_int_to_words_tens() {
    assert_eq!(int_to_words(20), "двадцать");
    assert_eq!(int_to_words(40), "сорок");
    assert_eq!(int_to_words(90), "девяносто");
}

#[test]
fn test_int_to_words_two_digits() {
    assert_eq!(int_to_words(42), "сорок два");
    assert_eq!(int_to_words(99), "девяносто девять");
}

#[test]
fn test_int_to_words_hundreds() {
    assert_eq!(int_to_words(100), "сто");
    assert_eq!(int_to_words(200), "двести");
    assert_eq!(int_to_words(305), "триста пять");
    assert_eq!(int_to_words(512), "пятьсот двенадцать");
    assert_eq!(int_to_words(900), "девятьсот");
}

#[test]
fn test_int_to_words_thousands() {
    assert_eq!(int_to_words(1000), "одна тысяча");
    assert_eq!(int_to_words(2001), "две тысячи один");
    assert_eq!(int_to_words(5000), "пять тысяч");
    assert_eq!(int_to_words(21000), "двадцать одна тысяча");
}

#[test]
fn test_int_to_words_complex() {
    assert_eq!(
        int_to_words(987654),
        "девятьсот восемьдесят семь тысяч шестьсот пятьдесят четыре"
    );
}

#[test]
fn test_int_to_words_million() {
    assert_eq!(int_to_words(1_000_000), "один миллион");
    assert_eq!(
        int_to_words(21_304_015),
        "двадцать один миллион триста четыре тысячи пятнадцать"
    );
}

#[test]
fn test_int_to_words_billion() {
    assert_eq!(int_to_words(1_000_000_000), "один миллиард");
    assert_eq!(
        int_to_words(2_147_483_647),
        "два миллиарда сто сорок семь миллионов четыреста восемьдесят три тысячи шестьсот сорок семь"
    );
}

#[test]
fn test_int_to_words_trillion() {
    assert_eq!(
        int_to_words(6_453_345_242_432),
        "шесть триллионов четыреста пятьдесят три миллиарда триста сорок пять миллионов двести сорок две тысячи четыреста тридцать два"
    );
}

#[test]
fn test_int_to_words_negative() {
    assert_eq!(int_to_words(-1), "минус один");
    assert_eq!(int_to_words(-512), "минус пятьсот двенадцать");
    assert_eq!(
        int_to_words(-1_000_000),
        "минус один миллион"
    );
}

#[test]
fn test_int_to_words_no_double_spaces() {
    let test_numbers = [0, 1, 10, 100, 305, 512, 1000, 2001, 987654, 1_000_000];
    for n in test_numbers {
        let words = int_to_words(n);
        assert!(
            !words.contains("  "),
            "Double space in int_to_words({n}): '{words}'"
        );
    }
}

#[test]
fn test_int_to_words_no_trailing_spaces() {
    let test_numbers = [0, 1, 10, 100, 305, 512, 1000, 2001, 987654];
    for n in test_numbers {
        let words = int_to_words(n);
        assert_eq!(words, words.trim(), "Trailing space in int_to_words({n})");
    }
}

// ==================== IntToWordsGender ====================

#[test]
fn test_gender_masculine() {
    assert_eq!(int_to_words_gender(1, Gender::Masculine), "один");
    assert_eq!(int_to_words_gender(2, Gender::Masculine), "два");
    assert_eq!(int_to_words_gender(42, Gender::Masculine), "сорок два");
}

#[test]
fn test_gender_feminine() {
    assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
    assert_eq!(int_to_words_gender(2, Gender::Feminine), "две");
    assert_eq!(int_to_words_gender(42, Gender::Feminine), "сорок две");
}

#[test]
fn test_gender_neuter() {
    assert_eq!(int_to_words_gender(1, Gender::Neuter), "одно");
    assert_eq!(int_to_words_gender(2, Gender::Neuter), "два");
    assert_eq!(int_to_words_gender(42, Gender::Neuter), "сорок два");
}

#[test]
fn test_gender_does_not_affect_thousands() {
    // Thousands are always feminine regardless of specified gender
    assert_eq!(int_to_words_gender(1000, Gender::Masculine), "одна тысяча");
    assert_eq!(int_to_words_gender(1000, Gender::Feminine), "одна тысяча");
    assert_eq!(int_to_words_gender(1000, Gender::Neuter), "одна тысяча");
    assert_eq!(int_to_words_gender(2000, Gender::Masculine), "две тысячи");
}

#[test]
fn test_gender_does_not_affect_millions() {
    // Millions are always masculine regardless of specified gender
    assert_eq!(
        int_to_words_gender(1_000_000, Gender::Feminine),
        "один миллион"
    );
    assert_eq!(
        int_to_words_gender(2_000_000, Gender::Feminine),
        "два миллиона"
    );
}

#[test]
fn test_gender_only_affects_last_triad() {
    assert_eq!(
        int_to_words_gender(1001, Gender::Feminine),
        "одна тысяча одна"
    );
    assert_eq!(
        int_to_words_gender(1002, Gender::Feminine),
        "одна тысяча две"
    );
    assert_eq!(
        int_to_words_gender(1001, Gender::Neuter),
        "одна тысяча одно"
    );
}

// ==================== Decline ====================

#[test]
fn test_decline_rubles() {
    assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
    assert_eq!(decline(2, "рубль", "рубля", "рублей"), "рубля");
    assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
    assert_eq!(decline(11, "рубль", "рубля", "рублей"), "рублей");
    assert_eq!(decline(21, "рубль", "рубля", "рублей"), "рубль");
    assert_eq!(decline(104, "рубль", "рубля", "рублей"), "рубля");
    assert_eq!(decline(111, "рубль", "рубля", "рублей"), "рублей");
    assert_eq!(decline(1234, "рубль", "рубля", "рублей"), "рубля");
}

#[test]
fn test_decline_dollars() {
    assert_eq!(decline(1, "доллар", "доллара", "долларов"), "доллар");
    assert_eq!(decline(2, "доллар", "доллара", "долларов"), "доллара");
    assert_eq!(decline(5, "доллар", "доллара", "долларов"), "долларов");
}

#[test]
fn test_decline_zero() {
    assert_eq!(decline(0, "рубль", "рубля", "рублей"), "рублей");
}

#[test]
fn test_decline_teens() {
    for n in 11..=19 {
        assert_eq!(
            decline(n, "рубль", "рубля", "рублей"),
            "рублей",
            "decline({n}) should return five-form for teens"
        );
    }
}

// ==================== DecimalToWords ====================

#[test]
fn test_decimal_to_words_simple() {
    assert_eq!(
        decimal_to_words("123.45").unwrap(),
        "сто двадцать три целых сорок пять сотых"
    );
}

#[test]
fn test_decimal_to_words_large() {
    assert_eq!(
        decimal_to_words("6453345242432.42").unwrap(),
        "шесть триллионов четыреста пятьдесят три миллиарда триста сорок пять миллионов двести сорок две тысячи четыреста тридцать два целых сорок две сотых"
    );
}

#[test]
fn test_decimal_to_words_no_fraction() {
    assert_eq!(decimal_to_words("100").unwrap(), "сто целых ноль сотых");
}

#[test]
fn test_decimal_to_words_single_digit_fraction() {
    assert_eq!(
        decimal_to_words("50.5").unwrap(),
        "пятьдесят целых пятьдесят сотых"
    );
}

#[test]
fn test_decimal_to_words_zero_whole() {
    assert_eq!(
        decimal_to_words("0.99").unwrap(),
        "ноль целых девяносто девять сотых"
    );
}

#[test]
fn test_decimal_to_words_truncation() {
    // 1.999 should truncate to .99, not round to 1.00
    assert_eq!(
        decimal_to_words("1.999").unwrap(),
        "один целых девяносто девять сотых"
    );
}

#[test]
fn test_decimal_to_words_one_hundredth() {
    assert_eq!(
        decimal_to_words("5.01").unwrap(),
        "пять целых одна сотая"
    );
}

#[test]
fn test_decimal_to_words_two_hundredths() {
    assert_eq!(
        decimal_to_words("10.02").unwrap(),
        "десять целых две сотых"
    );
}

#[test]
fn test_decimal_to_words_whole_gender_is_masculine() {
    // Whole part should be masculine
    assert_eq!(
        decimal_to_words("1.01").unwrap(),
        "один целых одна сотая"
    );
}

#[test]
fn test_decimal_to_words_errors() {
    assert!(decimal_to_words("abc.45").is_err());
    assert!(decimal_to_words("123.xy").is_err());
}

// ==================== DecimalValueToWords ====================

#[cfg(feature = "decimal")]
mod decimal_value_tests {
    use chislo::decimal_value_to_words;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_decimal_value_simple() {
        let d = Decimal::from_str("123.45").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "сто двадцать три целых сорок пять сотых"
        );
    }

    #[test]
    fn test_decimal_value_zero() {
        assert_eq!(
            decimal_value_to_words(Decimal::ZERO).unwrap(),
            "ноль целых ноль сотых"
        );
    }

    #[test]
    fn test_decimal_value_half() {
        let d = Decimal::from_str("50.5").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "пятьдесят целых пятьдесят сотых"
        );
    }

    #[test]
    fn test_decimal_value_truncation() {
        let d = Decimal::from_str("1.999").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "один целых девяносто девять сотых"
        );
    }

    #[test]
    fn test_decimal_value_negative() {
        let d = Decimal::from_str("-42.15").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "минус сорок два целых пятнадцать сотых"
        );
    }

    #[test]
    fn test_decimal_value_precision() {
        let d = Decimal::from_str("3.141592653589793").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "три целых четырнадцать сотых"
        );
    }

    #[test]
    fn test_decimal_value_one_hundredth() {
        let d = Decimal::from_str("5.01").unwrap();
        assert_eq!(
            decimal_value_to_words(d).unwrap(),
            "пять целых одна сотая"
        );
    }
}

// ==================== Real-world examples ====================

#[test]
fn test_receipt_example() {
    let amount = 1234;
    let words = int_to_words(amount);
    let currency = decline(amount, "рубль", "рубля", "рублей");
    assert_eq!(
        format!("{words} {currency}"),
        "одна тысяча двести тридцать четыре рубля"
    );
}

#[test]
fn test_invoice_feminine() {
    let count = 21;
    let words = int_to_words_gender(count, Gender::Feminine);
    let unit = decline(count, "штука", "штуки", "штук");
    assert_eq!(format!("{words} {unit}"), "двадцать одна штука");
}
