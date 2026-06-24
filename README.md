<div align="center">

# chislo · числа прописью

**Turn any number into correct Russian words — gender, declension, currency, dates and all.**

[![Crates.io](https://img.shields.io/crates/v/chislo.svg?logo=rust)](https://crates.io/crates/chislo)
[![Documentation](https://docs.rs/chislo/badge.svg)](https://docs.rs/chislo)
[![CI](https://github.com/rekurt/chislo/actions/workflows/ci.yml/badge.svg)](https://github.com/rekurt/chislo/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/crates/d/chislo.svg)](https://crates.io/crates/chislo)
[![MSRV](https://img.shields.io/badge/rustc-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[**🌐 Live demo & docs**](https://rekurt.github.io/chislo/) · [**📖 API reference**](https://docs.rs/chislo) · [**🇷🇺 Русская версия**](README.ru.md)

</div>

---

```rust
use chislo::{money, ordinal, date_to_words, Gender, RUB};

money(1234, 56, &RUB);              // одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек
ordinal(2026, Gender::Masculine);  // две тысячи двадцать шестой
date_to_words(2026, 4, 10);        // десятое апреля две тысячи двадцать шестого года
```

Russian numerals are hard: numbers change form by **gender** (`один` / `одна` / `одно`),
nouns **decline** by the digit (`1 рубль`, `2 рубля`, `5 рублей`), and decimals must agree
with an implicit feminine word (`одна целая`, not `один целых`). **chislo** gets all of it
right — so your receipts, invoices, payment orders and voice prompts read like a human wrote them.

## Why chislo

- ⚖️ **Grammatically correct** — three genders, full noun declension, proper decimal agreement.
- 🧾 **Built for documents** — money, percentages, durations, dates and times, fractions.
- 💱 **11 currencies** out of the box (RUB, USD, EUR, GBP, CNY, JPY, KZT, BYN, UAH, CHF, AED) + ISO 4217 lookup.
- ⚡ **Zero-alloc dictionary** — all data is `const`; `Display` wrappers format with no extra `String`.
- 🪶 **Tiny & portable** — `no_std` (with `alloc`), optional `rust_decimal`, and WASM bindings.
- 🦀 **i64 range** — up to quintillions; scale dictionary reaches 10³⁹.

## Install

```toml
[dependencies]
chislo = "0.3"
```

| Feature   | Default | What it adds                            |
|-----------|:-------:|-----------------------------------------|
| `std`     | ✅       | Standard library integration            |
| `decimal` | ✅       | `rust_decimal::Decimal` support         |
| `wasm`    | ➖       | `wasm-bindgen` bindings for the browser |

`no_std`: `chislo = { version = "0.3", default-features = false }`

## Quick start

```rust
use chislo::{int_to_words, int_to_words_gender, decline, ordinal,
             decimal_to_words, money, Gender, RUB};

// Integers
int_to_words(42);                          // "сорок два"
int_to_words(-1_000_000);                  // "минус один миллион"

// Gender agreement
int_to_words_gender(1, Gender::Feminine);  // "одна"
int_to_words_gender(2, Gender::Feminine);  // "две"

// Noun declension by number
decline(1,  "рубль", "рубля", "рублей");   // "рубль"
decline(5,  "рубль", "рубля", "рублей");   // "рублей"
decline(21, "рубль", "рубля", "рублей");   // "рубль"

// Ordinals, money, decimals
ordinal(42, Gender::Feminine);             // "сорок вторая"
money(100, 0, &RUB);                       // "сто рублей ноль копеек"
decimal_to_words("123.45").unwrap();       // "сто двадцать три целых сорок пять сотых"
```

### Fluent `Display` API — write numbers straight into `format!`

```rust
use chislo::{Number, Gender, RUB};

format!("{}", Number::new(2026).ordinal(Gender::Masculine)); // "две тысячи двадцать шестой"
format!("{}", Number::new(1234).money(56, &RUB));            // "…рубля пятьдесят шесть копеек"
```

## What's in the box

| Area | Functions |
|------|-----------|
| **Integers** | `int_to_words`, `int_to_words_gender`, `ordinal` |
| **Declension** | `decline(n, one, two, five)` |
| **Decimals** | `decimal_to_words`, `decimal_to_words_precision` (1–9 places), `decimal_value_to_words` |
| **Fractions** | `fraction`, `mixed_fraction` |
| **Money** | `money`, `money_from_str`, `money_from_str_rounded`, `Currency::from_iso` |
| **Percent** | `percent`, `percent_decimal` |
| **Duration** | `duration_hms`, `duration_from_secs` |
| **Date & time** | `date_to_words`, `year_to_words`, `time_to_words` |

Full reference and runnable examples: **[docs.rs/chislo](https://docs.rs/chislo)** ·
[`examples/`](examples/) (`cargo run --example receipt`).

## Use cases

Fiscal receipts (54-ФЗ) · payment orders & bank statements · invoices, acts, waybills ·
contracts & powers of attorney · voice assistants / TTS · chatbots that quote amounts.

## Links

- 🌐 **Landing & live demo:** https://rekurt.github.io/chislo/
- 📖 **API docs:** https://docs.rs/chislo
- 📦 **crates.io:** https://crates.io/crates/chislo
- 📝 **Changelog:** [CHANGELOG.md](CHANGELOG.md) · **Contributing:** [CONTRIBUTING.md](CONTRIBUTING.md)
- 🇷🇺 **Документация на русском:** [README.ru.md](README.ru.md)
- 🐹 **Go version:** [go-propisyu](https://github.com/rekurt/go-propisyu)

## License

[MIT](LICENSE) © rekurt

<sub><b>Keywords:</b> number to words russian · russian number converter · числа прописью ·
склонение · грамматический род · currency to words · Rust crate · chislo</sub>
