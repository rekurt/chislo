//! Micro-benchmarks for the main conversion paths.
//!
//! Run with: `cargo bench`

use chislo::{
    Gender, RUB, date_to_words, decimal_to_words_precision, duration_from_secs, int_to_words,
    int_to_words_gender, money, ordinal, percent,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_int_to_words(c: &mut Criterion) {
    let mut g = c.benchmark_group("int_to_words");
    g.bench_function("small_42", |b| b.iter(|| int_to_words(black_box(42))));
    g.bench_function("thousand", |b| b.iter(|| int_to_words(black_box(1_234))));
    g.bench_function("million", |b| {
        b.iter(|| int_to_words(black_box(21_304_015)))
    });
    g.bench_function("i64_max", |b| b.iter(|| int_to_words(black_box(i64::MAX))));
    g.bench_function("negative", |b| {
        b.iter(|| int_to_words(black_box(-987_654_321)))
    });
    g.bench_function("feminine_21", |b| {
        b.iter(|| int_to_words_gender(black_box(21), Gender::Feminine))
    });
    g.finish();
}

fn bench_ordinal(c: &mut Criterion) {
    let mut g = c.benchmark_group("ordinal");
    g.bench_function("year_2026", |b| {
        b.iter(|| ordinal(black_box(2026), Gender::Masculine))
    });
    g.bench_function("42", |b| {
        b.iter(|| ordinal(black_box(42), Gender::Masculine))
    });
    g.bench_function("round_thousand", |b| {
        b.iter(|| ordinal(black_box(2_000_000), Gender::Masculine))
    });
    g.finish();
}

fn bench_decimal(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal");
    g.bench_function("simple_2dp", |b| {
        b.iter(|| decimal_to_words_precision(black_box("123.45"), 2).unwrap())
    });
    g.bench_function("large_5dp", |b| {
        b.iter(|| decimal_to_words_precision(black_box("987654.12345"), 5).unwrap())
    });
    g.finish();
}

fn bench_money(c: &mut Criterion) {
    let mut g = c.benchmark_group("money");
    g.bench_function("small", |b| {
        b.iter(|| money(black_box(5), black_box(50), &RUB))
    });
    g.bench_function("large", |b| {
        b.iter(|| money(black_box(1_234_567), black_box(89), &RUB))
    });
    g.finish();
}

fn bench_specialised(c: &mut Criterion) {
    let mut g = c.benchmark_group("specialised");
    g.bench_function("percent", |b| b.iter(|| percent(black_box(42))));
    g.bench_function("duration_from_secs", |b| {
        b.iter(|| duration_from_secs(black_box(3_661)))
    });
    g.bench_function("date_to_words", |b| {
        b.iter(|| date_to_words(black_box(2026), 4, 10).unwrap())
    });
    g.finish();
}

criterion_group!(
    benches,
    bench_int_to_words,
    bench_ordinal,
    bench_decimal,
    bench_money,
    bench_specialised
);
criterion_main!(benches);
