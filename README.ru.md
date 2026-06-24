<div align="center">

# chislo · числа прописью

**Преобразует любое число в правильные русские слова — род, склонение, валюты, даты.**

[![Crates.io](https://img.shields.io/crates/v/chislo.svg?logo=rust)](https://crates.io/crates/chislo)
[![Documentation](https://docs.rs/chislo/badge.svg)](https://docs.rs/chislo)
[![CI](https://github.com/rekurt/chislo/actions/workflows/ci.yml/badge.svg)](https://github.com/rekurt/chislo/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/rustc-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[**🌐 Демо и документация**](https://rekurt.github.io/chislo/) · [**📖 API**](https://docs.rs/chislo) · [**🇬🇧 English**](README.md)

</div>

Rust-библиотека для преобразования чисел в слова с правильным грамматическим родом и
склонением существительных. Порт библиотеки [go-propisyu](https://github.com/rekurt/go-propisyu) на Rust.

> _Конвертирует числа в текст для чеков, счетов, платёжных поручений, накладных, актов, договоров и голосовых ассистентов._

---

## Возможности

- Целые числа в диапазоне `i64` (≈ 9.22 × 10^18, квинтиллионы); словарь масштабов поддерживает до 10^39
- Три грамматических рода: мужской, женский, средний
- Автоматическое склонение существительных по числу
- Порядковые числительные ("первый", "сорок второй", "двухтысячный")
- **Десятичные дроби с корректной грамматикой** ("одна целая пять десятых", не "один целых")
- **Проценты**: `percent`, `percent_decimal` ("сорок два процента", "одна целая пять десятых процента")
- **Длительности**: `duration_hms`, `duration_from_secs` ("один час тридцать минут")
- **Даты и время прописью**: `date_to_words`, `time_to_words`, `year_to_words` ("десятое апреля две тысячи двадцать шестого года")
- **Обыкновенные дроби**: `fraction`, `mixed_fraction` ("три пятых", "одна целая две пятых")
- Форматирование валют: 11 встроенных (RUB, USD, EUR, GBP, CNY, JPY, KZT, BYN, UAH, CHF, AED) + пользовательские
- Поиск валюты по ISO 4217 коду (`Currency::from_iso("RUB")`)
- Режимы округления (`Trunc`, `HalfUp`, `HalfEven`) при парсинге денежных сумм
- Поддержка и точки, и запятой как десятичного разделителя
- `Display`-обёртки (`Number::new(42).feminine()`) — форматирование без лишних аллокаций
- Настраиваемая точность десятичных дробей (1-9 знаков)
- Отрицательные числа
- Опциональная поддержка `rust_decimal` через feature flag
- Zero-copy словарь — все данные в `const`, без аллокаций
- Поддержка `no_std` (с `alloc`)
- WASM-привязки через `wasm-bindgen`

## Установка

```toml
[dependencies]
chislo = "0.3"
```

Без поддержки `rust_decimal`:

```toml
[dependencies]
chislo = { version = "0.3", default-features = false }
```

## Быстрый старт

```rust
use chislo::{int_to_words, int_to_words_gender, decline, ordinal,
             decimal_to_words, money, RUB, Gender};

// Целые числа
int_to_words(42);        // "сорок два"
int_to_words(0);         // "ноль"
int_to_words(-5);        // "минус пять"
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

// Порядковые числительные
ordinal(1, Gender::Masculine);    // "первый"
ordinal(42, Gender::Feminine);    // "сорок вторая"
ordinal(2026, Gender::Masculine); // "две тысячи двадцать шестой"

// Валюта
money(1234, 56, &RUB);
// "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"

// Десятичные числа
decimal_to_words("123.45").unwrap();
// "сто двадцать три целых сорок пять сотых"
```

## Примеры использования

### Кассовый чек (54-ФЗ)

```rust
use chislo::{int_to_words, decline};

let amount = 1234;
let words = int_to_words(amount);
let currency = decline(amount, "рубль", "рубля", "рублей");
println!("Итого: {words} {currency}");
// Итого: одна тысяча двести тридцать четыре рубля
```

### Проценты, длительности, даты

```rust
use chislo::{percent_decimal, duration_hms, date_to_words};

percent_decimal("1.5").unwrap();      // "одна целая пятьдесят сотых процента"
duration_hms(1, 30, 0);               // "один час тридцать минут"
date_to_words(2026, 4, 10).unwrap();  // "десятое апреля две тысячи двадцать шестого года"
```

### Обыкновенные дроби

```rust
use chislo::{fraction, mixed_fraction};

fraction(1, 2).unwrap();          // "одна вторая"
fraction(22, 7).unwrap();         // "двадцать две седьмых"
mixed_fraction(3, 2, 5).unwrap(); // "три целых две пятых"
```

### Fluent API (`Display`-обёртки)

```rust
use chislo::{Gender, Number, RUB};

println!("{}", Number::new(2026).ordinal(Gender::Masculine)); // две тысячи двадцать шестой
println!("{}", Number::new(5).with_noun("рубль", "рубля", "рублей")); // пять рублей
println!("{}", Number::new(1234).money(56, &RUB));
// одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек
```

### Форматирование валют

```rust
use chislo::{money, money_from_str, money_from_str_rounded, Currency, RoundingMode,
             RUB, USD, EUR, GBP, JPY};

money(1234, 56, &RUB);                  // "…рубля пятьдесят шесть копеек"
money(10, 50, &GBP);                    // "десять фунтов стерлингов пятьдесят пенсов"
money(100, 0, &JPY);                    // "сто иен" (без дробной части)
money_from_str("99,99", &EUR).unwrap(); // запятая как разделитель
money_from_str_rounded("100.995", &RUB, RoundingMode::HalfUp).unwrap();
// "сто один рубль ноль копеек"

let cur = Currency::from_iso("USD").unwrap(); // поиск по ISO 4217
money(1, 0, cur);                              // "один доллар ноль центов"
```

> Больше примеров — в каталоге [`examples/`](examples/) (`cargo run --example receipt`)
> и в полной документации на [docs.rs/chislo](https://docs.rs/chislo).

## Справочник API

### Числа

| Функция | Описание |
|---------|----------|
| `int_to_words(n: i64) -> String` | Число в слова (мужской род по умолчанию) |
| `int_to_words_gender(n: i64, gender: Gender) -> String` | Число в слова с указанием грамматического рода |
| `ordinal(n: i64, gender: Gender) -> String` | Порядковое числительное ("первый", "сорок второй") |
| `decline(n: i64, one: &str, two: &str, five: &str) -> String` | Склонение существительного по числу |

### Десятичные дроби

| Функция | Описание |
|---------|----------|
| `decimal_to_words(s: &str) -> Result<String, Error>` | Десятичное число из строки в слова (2 знака) |
| `decimal_to_words_precision(s: &str, precision: u32) -> Result<String, Error>` | С указанной точностью (1–9 знаков) |
| `decimal_value_to_words(d: Decimal) -> Result<String, Error>` | `rust_decimal::Decimal` в слова (feature `decimal`) |
| `fraction(numer, denom) -> Result<String, Error>` | Обыкновенная дробь ("три пятых") |
| `mixed_fraction(whole, numer, denom) -> Result<String, Error>` | Смешанная дробь ("три целых две пятых") |

### Проценты, длительности, даты, время

| Функция | Описание |
|---------|----------|
| `percent(n: i64) -> String` | Целый процент ("сорок два процента") |
| `percent_decimal(s: &str) -> Result<String, Error>` | Дробный процент |
| `duration_hms(h, m, s) -> String` | Длительность из часов/минут/секунд |
| `duration_from_secs(secs: u64) -> String` | Длительность из секунд (с днями) |
| `date_to_words(year, month, day) -> Result<String, Error>` | Дата прописью |
| `year_to_words(year) -> String` | Год в родительном падеже |
| `time_to_words(hour, minute) -> Result<String, Error>` | Время суток прописью |

### Валюта

| Функция / тип | Описание |
|---------------|----------|
| `money(whole, cents, &Currency) -> String` | Сумма прописью |
| `money_from_str(amount, &Currency) -> Result<String, Error>` | Разбор строки (точка или запятая) |
| `money_from_str_rounded(amount, &Currency, RoundingMode)` | С явным режимом округления |
| `Currency::from_iso(code)` | Поиск встроенной валюты по ISO 4217 |
| Константы | `RUB`, `USD`, `EUR`, `GBP`, `CNY`, `JPY`, `KZT`, `BYN`, `UAH`, `CHF`, `AED` |

### Типы

| Тип | Описание |
|-----|----------|
| `Gender` | Грамматический род: `Masculine`, `Feminine`, `Neuter` |
| `Error` | `InvalidNumber(String)`, `NumberTooLarge` (реализует `core::error::Error`) |
| `Currency` | Описание валюты (поле `show_frac` управляет выводом дробной части) |
| `RoundingMode` | `Trunc`, `HalfUp`, `HalfEven` |
| `Number` и др. | `Display`-обёртки для fluent API |

## Правила склонения

| Последняя цифра | Форма | Пример |
|-----------------|-------|--------|
| 1 (кроме 11) | единственное число | 1 рубль, 21 рубль |
| 2-4 (кроме 12-14) | родительный ед. | 2 рубля, 23 рубля |
| 0, 5-9, 11-19 | родительный мн. | 5 рублей, 11 рублей |

## `no_std` и WASM

```toml
# no_std (с alloc)
chislo = { version = "0.3", default-features = false }

# WebAssembly через wasm-bindgen
chislo = { version = "0.3", features = ["wasm"] }
```

Все основные функции работают с `alloc`; `Error` реализует `core::error::Error`,
поэтому `?` работает и без `std`. JS-функции: `intToWords`, `intToWordsGender`,
`ordinal`, `decimalToWords`, `decimalToWordsPrecision`, `decline`.

## Связанные проекты

- [go-propisyu](https://github.com/rekurt/go-propisyu) — Go-версия библиотеки

## Лицензия

[MIT](LICENSE) © rekurt

---

**Ключевые слова:** числа прописью, число в текст, русский язык, склонение, конвертер чисел, Rust, number to words russian, russian number converter, declension, chislo
