# propisyu

[![Crates.io](https://img.shields.io/crates/v/propisyu.svg)](https://crates.io/crates/propisyu)
[![Documentation](https://docs.rs/propisyu/badge.svg)](https://docs.rs/propisyu)
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
- Decimal numbers (strings and `rust_decimal::Decimal`)
- Negative numbers
- Zero external dependencies for integer functions
- Optional `rust_decimal` support via feature flag
- Zero-copy dictionary — all data in `const`, no allocations

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
propisyu = "0.1"
```

Without `rust_decimal` support:

```toml
[dependencies]
propisyu = { version = "0.1", default-features = false }
```

## Quick Start

```rust
use propisyu::{int_to_words, int_to_words_gender, decline, decimal_to_words, Gender};

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

// Decimal numbers
decimal_to_words("123.45").unwrap();
// "сто двадцать три целых сорок пять сотых"
```

## API

| Function | Description |
|----------|-------------|
| `int_to_words(n)` | Integer to Russian words (masculine) |
| `int_to_words_gender(n, gender)` | Integer to words with grammatical gender |
| `decimal_to_words(s)` | Decimal string to Russian words |
| `decimal_value_to_words(d)` | `rust_decimal::Decimal` to words |
| `decline(n, one, two, five)` | Russian noun declension by number |

### Types

| Type | Description |
|------|-------------|
| `Gender` | Grammatical gender: `Masculine`, `Feminine`, `Neuter` |
| `Error` | Errors: `InvalidNumber(String)`, `NumberTooLarge` |

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

**Keywords:** number to words russian, russian number converter, integer to text, declension, grammatical gender, Rust crate, propisyu, числа прописью
