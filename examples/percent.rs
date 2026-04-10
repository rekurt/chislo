//! Percentage formatting.
//!
//! Run with: cargo run --example percent

use chislo::{percent, percent_decimal};

fn main() {
    println!("=== Целые проценты ===");
    for n in [0, 1, 2, 5, 11, 21, 42, 100] {
        println!("{n}% = {}", percent(n));
    }

    println!("\n=== Дробные проценты ===");
    let inputs = ["1.5", "42.25", "0.01", "100.00", "3.14"];
    for s in inputs {
        println!("{s}% = {}", percent_decimal(s).unwrap());
    }
}
