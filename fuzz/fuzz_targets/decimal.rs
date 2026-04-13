#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = core::str::from_utf8(data) {
        // Parser must never panic on arbitrary input.
        let _ = chislo::decimal_to_words(s);
        for precision in 1..=9u32 {
            let _ = chislo::decimal_to_words_precision(s, precision);
        }
    }
});
