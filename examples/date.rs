//! Date and time formatting.
//!
//! Run with: cargo run --example date

use chislo::{date_to_words, time_to_words, year_to_words};

fn main() {
    println!("=== Даты прописью ===");
    let dates = [
        (2026, 4, 10),
        (1945, 5, 9),
        (2000, 1, 1),
        (1961, 4, 12),
        (2024, 2, 29),
    ];
    for (y, m, d) in dates {
        println!("{y:04}-{m:02}-{d:02} = {}", date_to_words(y, m, d).unwrap());
    }

    println!("\n=== Год в родительном падеже ===");
    for y in [1000u64, 1900, 1945, 2000, 2026] {
        println!("{y} = {}", year_to_words(y));
    }

    println!("\n=== Время прописью ===");
    for (h, m) in [(0u32, 0u32), (1, 0), (14, 30), (23, 59)] {
        println!("{h:02}:{m:02} = {}", time_to_words(h, m).unwrap());
    }
}
