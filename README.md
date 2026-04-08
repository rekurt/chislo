# propisyu

[![Crates.io](https://img.shields.io/crates/v/propisyu.svg)](https://crates.io/crates/propisyu)
[![Documentation](https://docs.rs/propisyu/badge.svg)](https://docs.rs/propisyu)
[![CI](https://github.com/rekurt/propisyu/actions/workflows/ci.yml/badge.svg)](https://github.com/rekurt/propisyu/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

**Числа прописью на русском языке** — Rust-библиотека для преобразования чисел в слова с правильным грамматическим родом и склонением существительных.

Порт библиотеки [go-propisyu](https://github.com/rekurt/go-propisyu) на Rust.

> _Конвертирует числа в текст для чеков, счетов, платёжных поручений, накладных, актов, договоров и голосовых ассистентов._

---

## Возможности

- Числа до дуодециллионов (10^39)
- Три грамматических рода: мужской, женский, средний
- Автоматическое склонение существительных по числу
- Десятичные числа (строки и `rust_decimal::Decimal`)
- Отрицательные числа
- Без внешних зависимостей для целочисленных функций
- Опциональная поддержка `rust_decimal` через feature flag
- Zero-copy словарь — все данные в `const`, без аллокаций

## Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
propisyu = "0.1"
```

Без поддержки `rust_decimal`:

```toml
[dependencies]
propisyu = { version = "0.1", default-features = false }
```

## Быстрый старт

```rust
use propisyu::{int_to_words, int_to_words_gender, decline, decimal_to_words, Gender};

// Целые числа
int_to_words(42);       // "сорок два"
int_to_words(0);        // "ноль"
int_to_words(-5);       // "минус пять"
int_to_words(1_000_000); // "один миллион"

// Грамматический род
int_to_words_gender(1, Gender::Masculine); // "один"
int_to_words_gender(1, Gender::Feminine);  // "одна"
int_to_words_gender(1, Gender::Neuter);    // "одно"
int_to_words_gender(2, Gender::Feminine);  // "две"

// Склонение существительных
decline(1, "рубль", "рубля", "рублей");  // "рубль"
decline(5, "рубль", "рубля", "рублей");  // "рублей"
decline(21, "рубль", "рубля", "рублей"); // "рубль"

// Десятичные числа
decimal_to_words("123.45").unwrap();
// "сто двадцать три целых сорок пять сотых"
```

## Примеры использования

### Кассовый чек (54-ФЗ)

```rust
use propisyu::{int_to_words, decline};

let amount = 1234;
let words = int_to_words(amount);
let currency = decline(amount, "рубль", "рубля", "рублей");
println!("Итого: {words} {currency}");
// Итого: одна тысяча двести тридцать четыре рубля
```

### Счёт-фактура с количеством

```rust
use propisyu::{int_to_words_gender, decline, Gender};

let count = 21;
let words = int_to_words_gender(count, Gender::Feminine);
let unit = decline(count, "штука", "штуки", "штук");
println!("{words} {unit}");
// двадцать одна штука
```

### Платёжное поручение с копейками

```rust
use propisyu::decimal_to_words;

let result = decimal_to_words("42350.50").unwrap();
println!("{result}");
// сорок две тысячи триста пятьдесят целых пятьдесят сотых
```

### С `rust_decimal::Decimal`

```rust
use propisyu::decimal_value_to_words;
use rust_decimal::Decimal;
use std::str::FromStr;

let d = Decimal::from_str("99.99").unwrap();
let result = decimal_value_to_words(d).unwrap();
// девяносто девять целых девяносто девять сотых
```

## API

| Функция | Описание |
|---------|----------|
| `int_to_words(n: i64) -> String` | Число в слова (мужской род по умолчанию) |
| `int_to_words_gender(n: i64, gender: Gender) -> String` | Число в слова с указанием грамматического рода |
| `decimal_to_words(s: &str) -> Result<String, Error>` | Десятичное число из строки в слова |
| `decimal_value_to_words(d: Decimal) -> Result<String, Error>` | `rust_decimal::Decimal` в слова (feature `decimal`) |
| `decline(n: i64, one: &str, two: &str, five: &str) -> String` | Склонение существительного по числу |

### Типы

| Тип | Описание |
|-----|----------|
| `Gender` | Грамматический род: `Masculine`, `Feminine`, `Neuter` |
| `Error` | Ошибки: `InvalidNumber(String)`, `NumberTooLarge` |

## Поддерживаемые масштабы

| Масштаб | Значение | Пример |
|---------|----------|--------|
| Единицы | 10^0 | один |
| Тысячи | 10^3 | одна тысяча |
| Миллионы | 10^6 | один миллион |
| Миллиарды | 10^9 | один миллиард |
| Триллионы | 10^12 | один триллион |
| Квадриллионы | 10^15 | один квадриллион |
| Квинтиллионы | 10^18 | один квинтиллион |
| Секстиллионы | 10^21 | один секстиллион |
| Септиллионы | 10^24 | один септиллион |
| Октиллионы | 10^27 | один октиллион |
| Нониллионы | 10^30 | один нониллион |
| Дециллионы | 10^33 | один дециллион |
| Ундециллионы | 10^36 | один ундециллион |
| Дуодециллионы | 10^39 | один дуодециллион |

## Правила склонения

| Последняя цифра | Форма | Пример |
|-----------------|-------|--------|
| 1 (кроме 11) | единственное число | 1 рубль, 21 рубль |
| 2-4 (кроме 12-14) | родительный ед. | 2 рубля, 23 рубля |
| 0, 5-9, 11-19 | родительный мн. | 5 рублей, 11 рублей |

## Сравнение с аналогами

| Возможность | propisyu (Rust) | go-propisyu (Go) |
|-------------|----------------|-----------------|
| Целые числа | i64 | int |
| Грамматический род | 3 рода | 3 рода |
| Десятичные числа | String + Decimal | String + Decimal |
| Склонение | `decline()` | `Decline()` |
| Масштаб | до 10^39 | до 10^39 |
| Аллокации словаря | 0 (const) | каждый вызов |
| Feature flags | `decimal` | - |

## Связанные проекты

- [go-propisyu](https://github.com/rekurt/go-propisyu) — Go-версия библиотеки

## Лицензия

[MIT](LICENSE)

---

**Ключевые слова:** числа прописью, число в текст, русский язык, склонение, конвертер чисел, Rust, number to words russian, russian number converter, declension, propisyu
