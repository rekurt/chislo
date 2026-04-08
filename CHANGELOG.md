# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-04-08

### Added

- `ordinal(n, gender)` - ordinal numbers ("первый", "сорок второй", "двухтысячный")
- `money(whole, cents, currency)` - currency formatting with built-in RUB, USD, EUR
- `money_from_str(amount, currency)` - parse string amount with currency
- `Currency` struct for custom currencies
- `decimal_to_words_precision(s, precision)` - configurable decimal precision (1-9 places)
- `no_std` support (with `alloc`) via `std` feature flag
- WASM bindings via `wasm` feature flag with wasm-bindgen

## [0.1.0] - 2026-04-08

### Added

- `int_to_words` - convert integers to Russian words (masculine gender)
- `int_to_words_gender` - convert integers with grammatical gender (masculine, feminine, neuter)
- `decimal_to_words` - convert decimal strings to Russian words
- `decimal_value_to_words` - convert `rust_decimal::Decimal` to Russian words (feature `decimal`)
- `decline` - Russian noun declension by number
- Support for numbers up to duodecillions (10^39)
- Negative number support
- Zero-allocation dictionary using `const` arrays
- Comprehensive test suite (67 tests)
- CI/CD with GitHub Actions
- MIT license
