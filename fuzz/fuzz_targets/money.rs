#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = core::str::from_utf8(data) {
        use chislo::{RUB, USD, EUR, JPY, RoundingMode};
        let _ = chislo::money_from_str(s, &RUB);
        let _ = chislo::money_from_str(s, &USD);
        let _ = chislo::money_from_str(s, &EUR);
        let _ = chislo::money_from_str(s, &JPY);
        for mode in [RoundingMode::Trunc, RoundingMode::HalfUp, RoundingMode::HalfEven] {
            let _ = chislo::money_from_str_rounded(s, &RUB, mode);
        }
    }
});
