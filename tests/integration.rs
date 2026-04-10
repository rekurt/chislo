use chislo::{
    EUR, Gender, RUB, USD, decimal_to_words, decimal_to_words_precision, decline, int_to_words,
    int_to_words_gender, money, money_from_str, ordinal,
};

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

#[test]
fn test_ordinal_integration() {
    let cases: &[(i64, Gender, &str)] = &[
        (1, Gender::Masculine, "первый"),
        (42, Gender::Feminine, "сорок вторая"),
        (100, Gender::Masculine, "сотый"),
        (2026, Gender::Masculine, "две тысячи двадцать шестой"),
        (1_000_000, Gender::Feminine, "миллионная"),
    ];
    for &(n, gender, expected) in cases {
        assert_eq!(ordinal(n, gender), expected, "ordinal({n})");
    }
}

#[test]
fn test_money_integration() {
    assert_eq!(money(1, 1, &RUB), "один рубль одна копейка");
    assert_eq!(
        money(1000, 50, &USD),
        "одна тысяча долларов пятьдесят центов"
    );
    assert_eq!(
        money_from_str("99.99", &EUR).unwrap(),
        "девяносто девять евро девяносто девять центов"
    );
}

#[test]
fn test_decimal_precision_integration() {
    assert_eq!(
        decimal_to_words_precision("3.14", 2).unwrap(),
        "три целых четырнадцать сотых"
    );
    assert_eq!(
        decimal_to_words_precision("1.5", 1).unwrap(),
        "одна целая пять десятых"
    );
}

#[test]
fn test_real_world_payment_order() {
    let payment = money_from_str("42350.50", &RUB).unwrap();
    assert_eq!(
        payment,
        "сорок две тысячи триста пятьдесят рублей пятьдесят копеек"
    );
    let order_number = ordinal(156, Gender::Masculine);
    assert_eq!(order_number, "сто пятьдесят шестой");
}

#[test]
fn test_boundary_integration() {
    let max_words = int_to_words(i64::MAX);
    assert!(max_words.starts_with("девять квинтиллионов"));
    assert!(!max_words.contains("  "));

    let min_words = int_to_words(i64::MIN);
    assert!(min_words.starts_with("минус девять квинтиллионов"));
    assert!(!min_words.contains("  "));
}
