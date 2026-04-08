//! Example: generating a fiscal receipt with amounts in words.
//!
//! Run with: cargo run --example receipt

use chislo::{decline, int_to_words};

fn format_amount(amount: i64) -> String {
    let words = int_to_words(amount);
    let currency = decline(amount, "рубль", "рубля", "рублей");
    format!("{words} {currency}")
}

fn main() {
    println!("========== КАССОВЫЙ ЧЕК ==========");
    println!("Молоко 2.5%       x1    89 руб.");
    println!("Хлеб белый        x2    54 руб.");
    println!("Масло сливочное   x1   145 руб.");
    println!("-----------------------------------");

    let total = 89 + 54 + 145;
    println!("ИТОГО: {} руб.", total);
    println!("Прописью: {}", format_amount(total));
    println!("===================================");
}
