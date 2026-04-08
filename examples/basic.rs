//! Basic usage of the chislo library.
//!
//! Run with: cargo run --example basic

use chislo::{decimal_to_words, decline, int_to_words, int_to_words_gender, Gender};

fn main() {
    // Integer to words (masculine by default)
    println!("=== Целые числа ===");
    println!("0 = {}", int_to_words(0));
    println!("42 = {}", int_to_words(42));
    println!("-5 = {}", int_to_words(-5));
    println!("1000 = {}", int_to_words(1000));
    println!("1000000 = {}", int_to_words(1_000_000));

    // Gender-specific forms
    println!("\n=== Грамматический род ===");
    println!("1 (м.р.) = {}", int_to_words_gender(1, Gender::Masculine));
    println!("1 (ж.р.) = {}", int_to_words_gender(1, Gender::Feminine));
    println!("1 (ср.р.) = {}", int_to_words_gender(1, Gender::Neuter));
    println!("2 (м.р.) = {}", int_to_words_gender(2, Gender::Masculine));
    println!("2 (ж.р.) = {}", int_to_words_gender(2, Gender::Feminine));

    // Noun declension
    println!("\n=== Склонение ===");
    for n in [1, 2, 5, 11, 21, 100] {
        let form = decline(n, "рубль", "рубля", "рублей");
        println!("{n} {form}");
    }

    // Decimal numbers
    println!("\n=== Десятичные числа ===");
    println!("123.45 = {}", decimal_to_words("123.45").unwrap());
    println!("0.99 = {}", decimal_to_words("0.99").unwrap());
    println!("5.01 = {}", decimal_to_words("5.01").unwrap());
}
