use wasm_bindgen::prelude::*;

/// Convert integer to Russian words (masculine gender).
#[wasm_bindgen(js_name = "intToWords")]
pub fn int_to_words_js(n: i64) -> String {
    crate::int_to_words(n)
}

/// Convert integer to Russian words with gender ("masculine", "feminine", "neuter").
#[wasm_bindgen(js_name = "intToWordsGender")]
pub fn int_to_words_gender_js(n: i64, gender: &str) -> String {
    let g = parse_gender(gender);
    crate::int_to_words_gender(n, g)
}

/// Convert number to ordinal form.
#[wasm_bindgen(js_name = "ordinal")]
pub fn ordinal_js(n: i64, gender: &str) -> String {
    let g = parse_gender(gender);
    crate::ordinal(n, g)
}

/// Convert decimal string to Russian words.
#[wasm_bindgen(js_name = "decimalToWords")]
pub fn decimal_to_words_js(s: &str) -> Result<String, JsError> {
    crate::decimal_to_words(s).map_err(|e| JsError::new(&e.to_string()))
}

/// Convert decimal string with precision to Russian words.
#[wasm_bindgen(js_name = "decimalToWordsPrecision")]
pub fn decimal_to_words_precision_js(s: &str, precision: u32) -> Result<String, JsError> {
    crate::decimal_to_words_precision(s, precision).map_err(|e| JsError::new(&e.to_string()))
}

/// Russian noun declension.
#[wasm_bindgen(js_name = "decline")]
pub fn decline_js(n: i64, one: &str, two: &str, five: &str) -> String {
    crate::decline(n, one, two, five)
}

fn parse_gender(s: &str) -> crate::Gender {
    match s {
        "feminine" | "f" | "fem" => crate::Gender::Feminine,
        "neuter" | "n" | "neu" => crate::Gender::Neuter,
        _ => crate::Gender::Masculine,
    }
}
