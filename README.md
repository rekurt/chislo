# propisyu

Библиотека для преобразования чисел в слова на русском языке с правильным грамматическим родом и склонением существительных.

Rust-порт библиотеки [go-propisyu](https://github.com/rekurt/go-propisyu).

## Возможности

- Числа до дуодециллионов (10^39)
- Три грамматических рода (мужской, женский, средний)
- Автоматическое склонение существительных
- Поддержка десятичных чисел
- Без внешних зависимостей для целочисленных функций

## Установка

```toml
[dependencies]
propisyu = "0.1.0"
```

## Использование

```rust
use propisyu::{int_to_words, int_to_words_gender, decline, decimal_to_words, Gender};

// Целые числа
assert_eq!(int_to_words(42), "сорок два");
assert_eq!(int_to_words(0), "ноль");
assert_eq!(int_to_words(-5), "минус пять");

// С указанием рода
assert_eq!(int_to_words_gender(1, Gender::Masculine), "один");
assert_eq!(int_to_words_gender(1, Gender::Feminine), "одна");
assert_eq!(int_to_words_gender(1, Gender::Neuter), "одно");

// Склонение
assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");

// Десятичные числа
assert_eq!(
    decimal_to_words("123.45").unwrap(),
    "сто двадцать три целых сорок пять сотых"
);
```

### Пример: кассовый чек

```rust
use propisyu::{int_to_words, decline};

let amount = 1234;
let words = int_to_words(amount);
let currency = decline(amount, "рубль", "рубля", "рублей");
println!("Итого: {words} {currency}");
// Итого: одна тысяча двести тридцать четыре рубля
```

## API

| Функция | Описание |
|---------|----------|
| `int_to_words(n)` | Число в слова (мужской род) |
| `int_to_words_gender(n, gender)` | Число в слова с указанием рода |
| `decimal_to_words(s)` | Десятичное число (строка) в слова |
| `decimal_value_to_words(d)` | `rust_decimal::Decimal` в слова |
| `decline(n, one, two, five)` | Склонение существительного по числу |

## Лицензия

MIT
