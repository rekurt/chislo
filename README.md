# chislo

[![Crates.io](https://img.shields.io/crates/v/chislo.svg)](https://crates.io/crates/chislo)
[![Documentation](https://docs.rs/chislo/badge.svg)](https://docs.rs/chislo)
[![CI](https://github.com/rekurt/propisyu/actions/workflows/ci.yml/badge.svg)](https://github.com/rekurt/propisyu/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

**Числа прописью на русском языке** — Rust-библиотека для преобразования чисел в слова с правильным грамматическим родом и склонением существительных.

Порт библиотеки [go-propisyu](https://github.com/rekurt/go-propisyu) на Rust.

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

Добавьте в `Cargo.toml`:

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

### Счёт-фактура с количеством

```rust
use chislo::{int_to_words_gender, decline, Gender};

let count = 21;
let words = int_to_words_gender(count, Gender::Feminine);
let unit = decline(count, "штука", "штуки", "штук");
println!("{words} {unit}");
// двадцать одна штука
```

### Платёжное поручение с копейками

```rust
use chislo::decimal_to_words;

let result = decimal_to_words("42350.50").unwrap();
println!("{result}");
// сорок две тысячи триста пятьдесят целых пятьдесят сотых
```

### Проценты

```rust
use chislo::{percent, percent_decimal};

percent(42);                         // "сорок два процента"
percent(1);                          // "один процент"
percent_decimal("1.5").unwrap();     // "одна целая пятьдесят сотых процента"
percent_decimal("42.25").unwrap();   // "сорок две целых двадцать пять сотых процента"
```

### Длительности

```rust
use chislo::{duration_hms, duration_from_secs};

duration_hms(1, 30, 0);          // "один час тридцать минут"
duration_hms(0, 5, 10);          // "пять минут десять секунд"
duration_from_secs(3_661);       // "один час одна минута одна секунда"
duration_from_secs(90_061);      // "один день один час одна минута одна секунда"
```

### Даты и время

```rust
use chislo::{date_to_words, time_to_words, year_to_words};

date_to_words(2026, 4, 10).unwrap();
// "десятое апреля две тысячи двадцать шестого года"

date_to_words(1945, 5, 9).unwrap();
// "девятое мая одна тысяча девятьсот сорок пятого года"

time_to_words(14, 30).unwrap();
// "четырнадцать часов тридцать минут"

year_to_words(2000); // "двухтысячного"
```

### Обыкновенные дроби

```rust
use chislo::{fraction, mixed_fraction};

fraction(1, 2).unwrap();          // "одна вторая"
fraction(3, 5).unwrap();          // "три пятых"
fraction(22, 7).unwrap();         // "двадцать две седьмых"
mixed_fraction(3, 2, 5).unwrap(); // "три целых две пятых"
```

### Fluent API (`Display`-обёртки)

```rust
use chislo::{Gender, Number, RUB};

println!("{}", Number::new(42).masculine());
// сорок два

println!("{}", Number::new(2026).ordinal(Gender::Masculine));
// две тысячи двадцать шестой

println!("{}", Number::new(5).with_noun("рубль", "рубля", "рублей"));
// пять рублей

println!("{}", Number::new(1234).money(56, &RUB));
// одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек
```

### Порядковые числительные

```rust
use chislo::{ordinal, Gender};

ordinal(1, Gender::Masculine);    // "первый"
ordinal(1, Gender::Feminine);     // "первая"
ordinal(42, Gender::Masculine);   // "сорок второй"
ordinal(2000, Gender::Masculine); // "двухтысячный"
ordinal(2026, Gender::Masculine); // "две тысячи двадцать шестой"
```

### Форматирование валют

```rust
use chislo::{money, money_from_str, money_from_str_rounded, Currency, RoundingMode,
             RUB, USD, EUR, GBP, CNY, JPY, KZT};

let rub = money(1234, 56, &RUB);
// "одна тысяча двести тридцать четыре рубля пятьдесят шесть копеек"

money(100, 0, &USD);                                  // "сто долларов ноль центов"
money(10, 50, &GBP);                                  // "десять фунтов стерлингов пятьдесят пенсов"
money(100, 0, &JPY);                                  // "сто иен" (без дробной части)
money_from_str("99,99", &EUR).unwrap();               // запятая как разделитель
money_from_str_rounded("100.995", &RUB, RoundingMode::HalfUp).unwrap();
// "сто один рубль ноль копеек"

// Поиск по ISO 4217
let cur = Currency::from_iso("USD").unwrap();
money(1, 0, cur);  // "один доллар ноль центов"
```

### Настраиваемая точность

```rust
use chislo::decimal_to_words_precision;

decimal_to_words_precision("3.14", 2).unwrap();
// "три целых четырнадцать сотых"

decimal_to_words_precision("3.14159", 5).unwrap();
// "три целых четырнадцать тысяч сто пятьдесят девять стотысячных"
```

### С `rust_decimal::Decimal`

```rust
use chislo::decimal_value_to_words;
use rust_decimal::Decimal;
use std::str::FromStr;

let d = Decimal::from_str("99.99").unwrap();
let result = decimal_value_to_words(d).unwrap();
// девяносто девять целых девяносто девять сотых
```

## Неочевидные примеры

Библиотека полезна не только для бухгалтерии — вот несколько неожиданных use cases.

### Текстовый квест — все три рода разом

```rust
use chislo::{int_to_words_gender, decline, Gender};

let loot = [
    (247, Gender::Feminine,  ("монета", "монеты", "монет")),
    (3,   Gender::Neuter,    ("зелье",  "зелья",  "зелий")),
    (1,   Gender::Masculine, ("меч",    "меча",   "мечей")),
];

println!("⚔ Добыча из подземелья:");
for (count, gender, (one, two, five)) in loot {
    let words = int_to_words_gender(count, gender);
    let unit = decline(count, one, two, five);
    println!("  → {words} {unit}");
}
// ⚔ Добыча из подземелья:
//   → двести сорок семь монет
//   → три зелья
//   → один меч
```

Один цикл — и все три грамматических рода с правильным склонением.

### Исторические даты

```rust
use chislo::{ordinal, Gender};

let events = [
    (1961, "Гагарин в космосе"),
    (1945, "День Победы"),
    (2026, "Сегодня"),
];

for (year, event) in events {
    println!("{event}: {} год", ordinal(year, Gender::Masculine));
}
// Гагарин в космосе: одна тысяча девятьсот шестьдесят первый год
// День Победы:       одна тысяча девятьсот сорок пятый год
// Сегодня:           две тысячи двадцать шестой год
```

### Кассовый чек с позициями

```rust
use chislo::{int_to_words_gender, decline, money, Gender, RUB};

struct Item { name: &'static str, qty: i64, price: i64 }

let items = [
    Item { name: "Молоко 3.2%",      qty: 2, price: 89  },
    Item { name: "Хлеб бородинский", qty: 1, price: 65  },
    Item { name: "Сыр Российский",   qty: 3, price: 245 },
];

for item in &items {
    let qty = int_to_words_gender(item.qty, Gender::Feminine);
    let unit = decline(item.qty, "штука", "штуки", "штук");
    println!("  {} × {} = {} ₽  ({qty} {unit})",
        item.name, item.qty, item.qty * item.price);
}

let total: i64 = items.iter().map(|i| i.qty * i.price).sum();
println!("ИТОГО: {}", money(total, 0, &RUB));
//   Молоко 3.2%      × 2 = 178 ₽  (две штуки)
//   Хлеб бородинский × 1 = 65 ₽   (одна штука)
//   Сыр Российский   × 3 = 735 ₽  (три штуки)
// ИТОГО: девятьсот семьдесят восемь рублей ноль копеек
```

### Научные константы прописью

```rust
use chislo::{int_to_words, decimal_to_words_precision, decline};

let c = 299_792_458i64; // скорость света, м/с
let c_words = int_to_words(c);
let m = decline(c, "метр", "метра", "метров");
println!("Скорость света: {c_words} {m} в секунду");
// Скорость света: двести девяносто девять миллионов семьсот
// девяносто две тысячи четыреста пятьдесят восемь метров в секунду

let pi = decimal_to_words_precision("3.14159265", 8).unwrap();
println!("π ≈ {pi}");
// π ≈ три целых четырнадцать миллионов сто пятьдесят девять
// тысяч двести шестьдесят пять стомиллионных
```

## API

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
| `decimal_to_words_precision(s: &str, precision: u32) -> Result<String, Error>` | Десятичное число с указанной точностью (1–9 знаков) |
| `decimal_value_to_words(d: Decimal) -> Result<String, Error>` | `rust_decimal::Decimal` в слова (feature `decimal`) |
| `fraction(numer: i64, denom: u32) -> Result<String, Error>` | Обыкновенная дробь ("три пятых") |
| `mixed_fraction(whole, numer, denom) -> Result<String, Error>` | Смешанная дробь ("три целых две пятых") |

### Проценты, длительности, даты, время

| Функция | Описание |
|---------|----------|
| `percent(n: i64) -> String` | Целый процент ("сорок два процента") |
| `percent_decimal(s: &str) -> Result<String, Error>` | Дробный процент |
| `duration_hms(h, m, s) -> String` | Длительность из часов/минут/секунд |
| `duration_from_secs(secs: u64) -> String` | Длительность из секунд (с днями) |
| `date_to_words(year, month, day) -> Result<String, Error>` | Дата прописью |
| `year_to_words(year: u64) -> String` | Год в родительном падеже |
| `time_to_words(hour, minute) -> Result<String, Error>` | Время суток прописью |

### Валюта

| Функция / тип | Описание |
|---------------|----------|
| `money(whole, cents, &Currency) -> String` | Сумма прописью |
| `money_from_str(amount, &Currency) -> Result<String, Error>` | Разбор строки (точка или запятая) |
| `money_from_str_rounded(amount, &Currency, RoundingMode)` | С явным режимом округления |
| `Currency::from_iso(code)` | Поиск встроенной валюты по ISO 4217 |
| `RoundingMode` | `Trunc`, `HalfUp`, `HalfEven` |
| Встроенные константы | `RUB`, `USD`, `EUR`, `GBP`, `CNY`, `JPY`, `KZT`, `BYN`, `UAH`, `CHF`, `AED` |

### Fluent API

`Number::new(n).masculine()` / `.feminine()` / `.neuter()` / `.ordinal(g)` /
`.with_noun(one, two, five)` / `.money(cents, &cur)` — все они реализуют
`core::fmt::Display`, так что числа можно писать прямо в `write!`, `format!`,
`println!` без промежуточного `String`.

### Типы

| Тип | Описание |
|-----|----------|
| `Gender` | Грамматический род: `Masculine`, `Feminine`, `Neuter` |
| `Error` | Ошибки: `InvalidNumber(String)`, `NumberTooLarge` (реализует `core::error::Error`) |
| `Currency` | Описание валюты (поле `show_frac` управляет выводом дробной части) |
| `RoundingMode` | Режим округления для `money_from_str_rounded` |
| `Number` и сопутствующие | `Display`-обёртки для fluent API |

## Грамматика десятичных дробей (0.3.0)

До версии `0.3.0` целая часть дроби выводилась в мужском роде:

```text
decimal_to_words("1.01") → "один целых одна сотая"    // грамматически неверно
```

Это ошибка: при чтении дроби целая часть согласуется с подразумеваемым словом
"целая" (женский род). Начиная с 0.3.0:

```text
decimal_to_words("1.01") → "одна целая одна сотая"
decimal_to_words("2.5")  → "две целых пятьдесят сотых"
decimal_to_words("5.5")  → "пять целых пятьдесят сотых"
```

Меняется только вывод для чисел, чья целая часть оканчивается на 1 или 2
(кроме 11/12). Все прочие выходы остаются прежними.

## Поддерживаемые масштабы

Публичный API принимает `i64`, поэтому максимум — `i64::MAX` ≈ 9.22 × 10^18
(квинтиллионы). Словарь масштабов включает названия вплоть до дуодециллионов
(10^39) и используется, например, при чтении составных словесных форм.

| Масштаб | Значение | Пример | Доступно через API |
|---------|----------|--------|-------------------|
| Единицы | 10^0 | один | ✓ |
| Тысячи | 10^3 | одна тысяча | ✓ |
| Миллионы | 10^6 | один миллион | ✓ |
| Миллиарды | 10^9 | один миллиард | ✓ |
| Триллионы | 10^12 | один триллион | ✓ |
| Квадриллионы | 10^15 | один квадриллион | ✓ |
| Квинтиллионы | 10^18 | один квинтиллион | ✓ (≤ `i64::MAX`) |
| Секстиллионы | 10^21 | один секстиллион | только в словаре |
| Септиллионы | 10^24 | один септиллион | только в словаре |
| Октиллионы | 10^27 | один октиллион | только в словаре |
| Нониллионы | 10^30 | один нониллион | только в словаре |
| Дециллионы | 10^33 | один дециллион | только в словаре |
| Ундециллионы | 10^36 | один ундециллион | только в словаре |
| Дуодециллионы | 10^39 | один дуодециллион | только в словаре |

## Правила склонения

| Последняя цифра | Форма | Пример |
|-----------------|-------|--------|
| 1 (кроме 11) | единственное число | 1 рубль, 21 рубль |
| 2-4 (кроме 12-14) | родительный ед. | 2 рубля, 23 рубля |
| 0, 5-9, 11-19 | родительный мн. | 5 рублей, 11 рублей |

## `no_std`

Библиотека поддерживает `no_std` с аллокатором:

```toml
[dependencies]
chislo = { version = "0.3", default-features = false }
```

Все основные функции работают с `alloc`. Feature `std` включён по умолчанию.
`Error` реализует `core::error::Error`, поэтому `?`-оператор работает и без `std`.

## WASM

Библиотека поддерживает WebAssembly через `wasm-bindgen`:

```toml
[dependencies]
chislo = { version = "0.3", features = ["wasm"] }
```

Доступные JS-функции: `intToWords`, `intToWordsGender`, `ordinal`, `decimalToWords`, `decimalToWordsPrecision`, `decline`.

## Сравнение с аналогами

| Возможность | chislo (Rust) | go-propisyu (Go) |
|-------------|----------------|-----------------|
| Целые числа | i64 | int |
| Грамматический род | 3 рода | 3 рода |
| Порядковые числительные | `ordinal()` | — |
| Форматирование валют | `money()` / `money_from_str()` | — |
| Десятичные числа | String + Decimal | String + Decimal |
| Точность десятичных | 1-9 знаков | 2 знака |
| Склонение | `decline()` | `Decline()` |
| Масштаб | i64 (квинтиллионы); словарь до 10^39 | до 10^39 |
| Аллокации словаря | 0 (const) | каждый вызов |
| `no_std` | да | — |
| WASM | да | — |
| Feature flags | `decimal`, `std`, `wasm` | — |

## Связанные проекты

- [go-propisyu](https://github.com/rekurt/go-propisyu) — Go-версия библиотеки

## Лицензия

[MIT](LICENSE)

---

**Ключевые слова:** числа прописью, число в текст, русский язык, склонение, конвертер чисел, Rust, number to words russian, russian number converter, declension, chislo, propisyu
