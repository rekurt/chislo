use chislo::{Gender, decimal_to_words, decline, int_to_words, int_to_words_gender};

#[test]
fn test_int_to_words_zero() {
    assert_eq!(int_to_words(0), "ноль");
}

#[test]
fn test_int_to_words_basic() {
    assert_eq!(int_to_words(7), "семь");
    assert_eq!(int_to_words(42), "сорок два");
    assert_eq!(int_to_words(1000), "одна тысяча");
}

#[test]
fn test_int_to_words_negative() {
    assert_eq!(int_to_words(-1), "минус один");
    assert_eq!(int_to_words(-512), "минус пятьсот двенадцать");
    assert_eq!(int_to_words(-1_000_000), "минус один миллион");
}

#[test]
fn test_gender_masculine() {
    assert_eq!(int_to_words_gender(1, Gender::Masculine), "один");
    assert_eq!(int_to_words_gender(2, Gender::Masculine), "два");
}

#[test]
fn test_gender_feminine() {
    assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
    assert_eq!(int_to_words_gender(2, Gender::Feminine), "две");
}

#[test]
fn test_gender_neuter() {
    assert_eq!(int_to_words_gender(1, Gender::Neuter), "одно");
}

#[test]
fn test_decline_rubles() {
    assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
    assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
    assert_eq!(decline(21, "рубль", "рубля", "рублей"), "рубль");
}

#[test]
fn test_decimal_to_words_simple() {
    assert_eq!(
        decimal_to_words("123.45").unwrap(),
        "сто двадцать три целых сорок пять сотых"
    );
}

#[test]
fn test_decimal_to_words_one_hundredth() {
    assert_eq!(decimal_to_words("5.01").unwrap(), "пять целых одна сотая");
}

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
