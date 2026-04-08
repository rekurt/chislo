//! Example: generating an invoice with feminine noun forms.
//!
//! Run with: cargo run --example invoice

use propisyu::{decline, int_to_words_gender, Gender};

fn format_quantity(count: i64, one: &str, two: &str, five: &str) -> String {
    let words = int_to_words_gender(count, Gender::Feminine);
    let unit = decline(count, one, two, five);
    format!("{words} {unit}")
}

fn main() {
    println!("========== СЧЁТ-ФАКТУРА ==========");
    println!();

    let items = [
        (21, "Гайка M8", "штука", "штуки", "штук"),
        (5, "Болт M8x40", "штука", "штуки", "штук"),
        (1, "Шайба M8", "штука", "штуки", "штук"),
        (103, "Саморез 4x50", "штука", "штуки", "штук"),
    ];

    for (count, name, one, two, five) in items {
        let qty = format_quantity(count, one, two, five);
        println!("  {name}: {qty}");
    }

    println!();
    println!("===================================");
}
