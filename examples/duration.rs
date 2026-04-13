//! Duration formatting.
//!
//! Run with: cargo run --example duration

use chislo::{duration_from_secs, duration_hms};

fn main() {
    println!("=== duration_hms ===");
    let cases = [
        (0u64, 0u64, 0u64),
        (1, 0, 0),
        (2, 30, 0),
        (0, 5, 10),
        (23, 59, 59),
    ];
    for (h, m, s) in cases {
        println!("{h}:{m:02}:{s:02} = {}", duration_hms(h, m, s));
    }

    println!("\n=== duration_from_secs ===");
    for secs in [60u64, 90, 3_600, 3_661, 86_400, 90_061] {
        println!("{secs:>7}s = {}", duration_from_secs(secs));
    }
}
