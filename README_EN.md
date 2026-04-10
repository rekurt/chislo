# chislo

[![Crates.io](https://img.shields.io/crates/v/chislo.svg)](https://crates.io/crates/chislo)
[![Documentation](https://docs.rs/chislo/badge.svg)](https://docs.rs/chislo)
[![CI](https://github.com/rekurt/propisyu/actions/workflows/ci.yml/badge.svg)](https://github.com/rekurt/propisyu/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

**Russian number-to-words converter** — a Rust library for converting integers and decimals into Russian words with correct grammatical gender and noun declension.

Rust port of [go-propisyu](https://github.com/rekurt/go-propisyu).

> _Converts numbers to text for receipts, invoices, payment orders, contracts, voice assistants, and chatbots._

---

## Features

- Numbers up to duodecillions (10^39)
- Three grammatical genders: masculine, feminine, neuter
- Automatic noun declension by number
- Ordinal numbers ("первый", "сорок второй", "двухтысячный")
- **Decimals with correct grammar** ("одна целая пять десятых", not "один целых")
- **Percentages**: `percent`, `percent_decimal` ("forty-two percent" → "сорок два процента")
- **Durations**: `duration_hms`, `duration_from_secs`
- **Dates & times in words**: `date_to_words`, `time_to_words`, `year_to_words`
- **Common fractions**: `fraction`, `mixed_fraction`
- 11 built-in currencies: RUB, USD, EUR, GBP, CNY, JPY, KZT, BYN, UAH, CHF, AED
- ISO 4217 currency lookup (`Currency::from_iso("USD")`)
- Rounding modes (`Trunc`, `HalfUp`, `HalfEven`) for money parsing
- Both `.` and `,` accepted as decimal separators
- Fluent `Display` wrappers (`Number::new(42).feminine()`) — zero-alloc formatting
- Decimals (strings and `rust_decimal::Decimal`)
- Configurable decimal precision (1-9 places)
- Negative numbers
- Optional `rust_decimal` support via feature flag
- Zero-copy dictionary — all data in `const`, no allocations
- `no_std` support (with `alloc`); `Error` implements `core::error::Error`
- WASM bindings via `wasm-bindgen`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
chislo = "0.3"
```

Without `rust_decimal` support:

```toml
[dependencies]
chislo = { version = "0.3", default-features = false }
```

## Breaking change in 0.3.0

The whole part of a decimal number is now rendered in feminine gender (as it
agrees with the implicit word "целая"). This was grammatically wrong in earlier
versions:

```text
before 0.3.0:  decimal_to_words("1.01") → "один целых одна сотая"
since  0.3.0:  decimal_to_words("1.01") → "одна целая одна сотая"
since  0.3.0:  decimal_to_words("2.5")  → "две целых пятьдесят сотых"
```

Only output for numbers whose whole part ends in 1 or 2 (but not 11/12) is
affected.

## Quick Start

```rust
use chislo::{int_to_words, int_to_words_gender, decline, ordinal,
             decimal_to_words, money, RUB, Gender};

// Integers
int_to_words(42);        // "сорок два"
int_to_words(0);         // "ноль"
int_to_words(-5);        // "минус пять"
int_to_words(1_000_000); // "один миллион"

// Grammatical gender
int_to_words_gender(1, Gender::Masculine); // "один"
int_to_words_gender(1, Gender::Feminine);  // "одна"
int_to_words_gender(1, Gender::Neuter);    // "одно"

// Noun declension
decline(1, "рубль", "рубля", "рублей");  // "рубль"
decline(5, "рубль", "рубля", "рублей");  // "рублей"

// Ordinal numbers
ordinal(1, Gender::Masculine);    // "первый"
ordinal(42, Gender::Feminine);    // "сорок вторая"
ordinal(2026, Gender::Masculine); // "две тысячи двадцать шестой"

// Currency
money(1234, 56, &RUB);
// "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"

// Decimal numbers
decimal_to_words("123.45").unwrap();
// "сто двадцать три целых сорок пять сотых"
```

## API

| Function | Description |
|----------|-------------|
| `int_to_words(n)` | Integer to Russian words (masculine) |
| `int_to_words_gender(n, gender)` | Integer to words with grammatical gender |
| `ordinal(n, gender)` | Ordinal number in Russian |
| `decimal_to_words(s)` | Decimal string to Russian words |
| `decimal_to_words_precision(s, precision)` | Decimal with configurable precision (1-9) |
| `decimal_value_to_words(d)` | `rust_decimal::Decimal` to words |
| `decline(n, one, two, five)` | Russian noun declension by number |
| `money(whole, cents, currency)` | Amount in words with currency |
| `money_from_str(amount, currency)` | Parse string amount with currency |

### Types

| Type | Description |
|------|-------------|
| `Gender` | Grammatical gender: `Masculine`, `Feminine`, `Neuter` |
| `Error` | Errors: `InvalidNumber(String)`, `NumberTooLarge` |
| `Currency` | Currency descriptor: `RUB`, `USD`, `EUR` or custom |

## Examples

### Ordinal Numbers

```rust
use chislo::{ordinal, Gender};

ordinal(1, Gender::Masculine);    // "первый"
ordinal(1, Gender::Feminine);     // "первая"
ordinal(42, Gender::Masculine);   // "сорок второй"
ordinal(2000, Gender::Masculine); // "двухтысячный"
ordinal(2026, Gender::Masculine); // "две тысячи двадцать шестой"
```

### Currency Formatting

```rust
use chislo::{money, money_from_str, RUB, USD, EUR};

let rub = money(1234, 56, &RUB);
// "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"

let usd = money(100, 0, &USD);
// "сто долларов ноль центов"

let eur = money_from_str("99.99", &EUR).unwrap();
// "девяносто девять евро девяносто девять центов"
```

### Configurable Precision

```rust
use chislo::decimal_to_words_precision;

decimal_to_words_precision("3.14", 2).unwrap();
// "три целых четырнадцать сотых"

decimal_to_words_precision("3.14159", 5).unwrap();
// "три целых четырнадцать тысяч сто пятьдесят девять стотысячных"
```

## Non-obvious Examples

The library shines beyond accounting — here are some unexpected use cases.

### Text quest — all three genders at once

```rust
use chislo::{int_to_words_gender, decline, Gender};

let loot = [
    (247, Gender::Feminine,  ("монета", "монеты", "монет")),
    (3,   Gender::Neuter,    ("зелье",  "зелья",  "зелий")),
    (1,   Gender::Masculine, ("меч",    "меча",   "мечей")),
];

println!("⚔ Dungeon loot:");
for (count, gender, (one, two, five)) in loot {
    let words = int_to_words_gender(count, gender);
    let unit = decline(count, one, two, five);
    println!("  → {words} {unit}");
}
// ⚔ Dungeon loot:
//   → двести сорок семь монет
//   → три зелья
//   → один меч
```

A single loop handles all three grammatical genders with correct declension.

### Historical dates

```rust
use chislo::{ordinal, Gender};

let events = [
    (1961, "Gagarin in space"),
    (1945, "Victory Day"),
    (2026, "Today"),
];

for (year, event) in events {
    println!("{event}: {} год", ordinal(year, Gender::Masculine));
}
// Gagarin in space: одна тысяча девятьсот шестьдесят первый год
// Victory Day:      одна тысяча девятьсот сорок пятый год
// Today:            две тысячи двадцать шестой год
```

### Receipt with line items

```rust
use chislo::{int_to_words_gender, decline, money, Gender, RUB};

struct Item { name: &'static str, qty: i64, price: i64 }

let items = [
    Item { name: "Milk 3.2%",        qty: 2, price: 89  },
    Item { name: "Borodinsky bread", qty: 1, price: 65  },
    Item { name: "Russian cheese",   qty: 3, price: 245 },
];

for item in &items {
    let qty = int_to_words_gender(item.qty, Gender::Feminine);
    let unit = decline(item.qty, "штука", "штуки", "штук");
    println!("  {} × {} = {} ₽  ({qty} {unit})",
        item.name, item.qty, item.qty * item.price);
}

let total: i64 = items.iter().map(|i| i.qty * i.price).sum();
println!("TOTAL: {}", money(total, 0, &RUB));
// TOTAL: девятьсот семьдесят восемь рублей ноль копеек
```

### Scientific constants in words

```rust
use chislo::{int_to_words, decimal_to_words_precision, decline};

let c = 299_792_458i64; // speed of light, m/s
let c_words = int_to_words(c);
let m = decline(c, "метр", "метра", "метров");
println!("Speed of light: {c_words} {m} в секунду");
// Speed of light: двести девяносто девять миллионов семьсот
// девяносто две тысячи четыреста пятьдесят восемь метров в секунду

let pi = decimal_to_words_precision("3.14159265", 8).unwrap();
println!("π ≈ {pi}");
// π ≈ три целых четырнадцать миллионов сто пятьдесят девять
// тысяч двести шестьдесят пять стомиллионных
```

## `no_std` Support

The library supports `no_std` with an allocator:

```toml
[dependencies]
chislo = { version = "0.2", default-features = false }
```

All core functions work with `alloc`. The `std` feature is enabled by default.

## WASM

The library supports WebAssembly via `wasm-bindgen`:

```toml
[dependencies]
chislo = { version = "0.2", features = ["wasm"] }
```

Available JS functions: `intToWords`, `intToWordsGender`, `ordinal`, `decimalToWords`, `decimalToWordsPrecision`, `decline`.

## Use Cases

- **Fintech / Banking**: payment orders, statements, invoices
- **Accounting**: invoice generation, acts, waybills
- **Fiscal receipts**: POS systems, OFD compliance (Russian 54-FZ)
- **Voice assistants**: TTS number pronunciation
- **Chatbots**: natural language responses with amounts
- **Document generation**: contracts, powers of attorney

## Related Projects

- [go-propisyu](https://github.com/rekurt/go-propisyu) — Go version of this library

## License

[MIT](LICENSE)

---

**Keywords:** number to words russian, russian number converter, integer to text, declension, grammatical gender, Rust crate, chislo, propisyu, числа прописью
