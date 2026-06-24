# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-06-24

Documentation and metadata release — no API or behaviour changes.

### Changed

- Fixed `repository` and `homepage` metadata to point at the correct
  `rekurt/chislo` repository (previously `rekurt/propisyu`).
- Reworked the README: an English-first, concise overview in `README.md`
  with the full Russian reference moved to `README.ru.md`.

### Added

- Documentation site published to GitHub Pages — a landing page with an
  interactive in-browser WASM demo, alongside the rustdoc API reference.

## [0.3.0] - 2026-04-10

### Fixed (breaking)

- **Грамматика десятичных дробей.** Целая часть дроби теперь выводится в
  женском роде (согласование с подразумеваемым словом "целая"), а само слово
  "целая/целых" склоняется по числу. Примеры:
  - `decimal_to_words("1.01")` → `"одна целая одна сотая"` (было
    `"один целых одна сотая"`)
  - `decimal_to_words("2.5")` → `"две целых пятьдесят сотых"`
  - `decimal_to_words("-1.01")` → `"минус одна целая одна сотая"`
  Затрагивает `decimal_to_words`, `decimal_to_words_precision` и
  `decimal_value_to_words`.
- **Сохранение знака.** Минус теперь корректно сохраняется для значений в
  диапазоне `-1 < x < 0` в `decimal_to_words`, `decimal_to_words_precision`,
  `decimal_value_to_words`, `money_from_str`, `money_from_str_rounded` и
  `percent_decimal_precision`.
- `FRACTION_UNITS[1]` используется для "сотая/сотых" вместо жёстко зашитой
  строки.

### Added

- **Проценты**: `percent(n)`, `percent_decimal(s)`, `percent_decimal_precision(s, p)`,
  `percent_word(n)` — форматирование целых и дробных процентов со склонением.
- **Длительности**: `duration_hms(h, m, s)`, `duration_from_secs(secs)`,
  `duration_from_core(Duration)`, `hours_word`, `minutes_word`, `seconds_word`.
- **Даты и время**: `date_to_words(year, month, day)`,
  `year_to_words(year)` (родительный падеж, для "… года"),
  `time_to_words(hour, minute)`, `month_name(month)`.
- **Обыкновенные дроби**: `fraction(numer, denom)` → `"одна вторая"`, `"три пятых"`
  и `mixed_fraction(whole, numer, denom)` → `"одна целая две пятых"`.
- **Новые валюты**: `GBP`, `CNY`, `JPY`, `KZT`, `BYN`, `UAH`, `CHF`, `AED`
  (в дополнение к существующим `RUB`, `USD`, `EUR`).
- `Currency::from_iso(code)` — поиск встроенных валют по ISO 4217 коду
  (регистронезависимо).
- `Currency::new(...)` — удобный конструктор для пользовательских валют.
- `Currency::show_frac` — поле, позволяющее отключить вывод дробной части
  (используется для JPY, у которой нет массовой дробной единицы).
- `RoundingMode { Trunc, HalfUp, HalfEven }` и `money_from_str_rounded(s, cur, mode)`
  — явное округление при парсинге денежных сумм.
- `money_from_str`, `decimal_to_words` и `decimal_to_words_precision` теперь
  принимают как `.`, так и `,` в качестве десятичного разделителя.
- **`Display`-обёртки**: `Number::new(n).masculine()`, `.feminine()`, `.neuter()`,
  `.ordinal(g)`, `.with_noun(...)`, `.money(cents, cur)` — fluent API,
  реализующий `core::fmt::Display`, так что числа можно писать прямо в `write!`,
  `format!`, `println!` без промежуточного `String`.
- `core::error::Error` реализован безусловно (раньше был только под `std`),
  что позволяет использовать `?`-оператор в `no_std` приложениях.
- `decimal_value_to_words_precision(d, precision)` — конвертация
  `rust_decimal::Decimal` с точностью 1-9 знаков.
- Новые примеры: `examples/percent.rs`, `examples/duration.rs`, `examples/date.rs`.
- Criterion-бенчмарки в `benches/conversion.rs`.
- Фаззинг-таргеты для `decimal_to_words`, `money_from_str`, `percent_decimal`.

### Changed (breaking)

- `Gender` enum discriminants изменены с `1/2/3` на `0/1/2` (соответствует
  индексам внутренних таблиц). Затронет только код, который делал
  `gender as i32` — обычное использование `match`/`==` работает как раньше.
- `Currency` получила новое публичное поле `show_frac: bool`. Код, создающий
  `Currency` через struct-литерал, должен либо добавить это поле, либо
  переключиться на `Currency::new(...)`.
- Внутренняя таблица `ONES` ужата с `[[&str; 4]; 9]` до `[[&str; 3]; 9]`
  (был неиспользуемый индекс 0). Это внутреннее изменение, но упоминается
  для полноты.

### Internal

- Унифицирован парсинг дробной части в `src/parse.rs::parse_fractional_digits`
  (раньше три почти идентичные функции в `currency.rs` и `decimal.rs`).
- Документация уточняет фактический публичный диапазон: API принимает `i64`
  (до квинтиллионов), хотя словарь масштабов содержит названия вплоть до `10^39`.
- Унифицирована индексация рода: `Gender::index()` возвращает `0..=2`.
- Магические строки `"ноль"`, `"минус"`, `"целых"` вынесены в `dictionary.rs`.

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
