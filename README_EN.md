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
- Currency formatting: RUB, USD, EUR and custom currencies
- Decimal numbers (strings and `rust_decimal::Decimal`)
- Configurable decimal precision (1-9 places)
- Negative numbers
- Zero external dependencies for integer functions
- Optional `rust_decimal` support via feature flag
- Zero-copy dictionary — all data in `const`, no allocations
- `no_std` support (with `alloc`)
- WASM bindings via `wasm-bindgen`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
chislo = "0.2"
```

Without `rust_decimal` support:

```toml
[dependencies]
chislo = { version = "0.2", default-features = false }
```

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
