#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = core::str::from_utf8(data) {
        let _ = chislo::percent_decimal(s);
        for precision in 1..=9u32 {
            let _ = chislo::percent_decimal_precision(s, precision);
        }
    }
});
