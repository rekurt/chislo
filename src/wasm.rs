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

/// Format a money amount (whole units + cents) as Russian words.
///
/// `code` is an ISO 4217 currency code (e.g. "RUB", "USD"); unknown codes fall
/// back to the rouble.
#[wasm_bindgen(js_name = "money")]
pub fn money_js(whole: i64, cents: u32, code: &str) -> String {
    crate::money(whole, cents, currency_by_code(code))
}

/// Parse a decimal amount string ("1234.56") and format it as Russian words for
/// the given ISO 4217 currency code.
#[wasm_bindgen(js_name = "moneyFromStr")]
pub fn money_from_str_js(amount: &str, code: &str) -> Result<String, JsError> {
    crate::money_from_str(amount, currency_by_code(code)).map_err(|e| JsError::new(&e.to_string()))
}

/// Format an integer percentage as Russian words.
#[wasm_bindgen(js_name = "percent")]
pub fn percent_js(n: i64) -> String {
    crate::percent(n)
}

/// Format a decimal percentage string ("42.25") as Russian words.
#[wasm_bindgen(js_name = "percentDecimal")]
pub fn percent_decimal_js(s: &str) -> Result<String, JsError> {
    crate::percent_decimal(s).map_err(|e| JsError::new(&e.to_string()))
}

/// Format a common fraction (numerator/denominator) as Russian words.
#[wasm_bindgen(js_name = "fraction")]
pub fn fraction_js(numer: i64, denom: u32) -> Result<String, JsError> {
    crate::fraction(numer, denom).map_err(|e| JsError::new(&e.to_string()))
}

/// Format a mixed fraction (whole + numerator/denominator) as Russian words.
#[wasm_bindgen(js_name = "mixedFraction")]
pub fn mixed_fraction_js(whole: i64, numer: i64, denom: u32) -> Result<String, JsError> {
    crate::mixed_fraction(whole, numer, denom).map_err(|e| JsError::new(&e.to_string()))
}

/// Format hours/minutes/seconds as a Russian duration.
#[wasm_bindgen(js_name = "durationHms")]
pub fn duration_hms_js(hours: u64, minutes: u64, seconds: u64) -> String {
    crate::duration_hms(hours, minutes, seconds)
}

/// Format a total number of seconds as a Russian duration.
#[wasm_bindgen(js_name = "durationFromSecs")]
pub fn duration_from_secs_js(total_secs: u64) -> String {
    crate::duration_from_secs(total_secs)
}

/// Format a calendar date as Russian words.
#[wasm_bindgen(js_name = "dateToWords")]
pub fn date_to_words_js(year: i32, month: u32, day: u32) -> Result<String, JsError> {
    crate::date_to_words(year, month, day).map_err(|e| JsError::new(&e.to_string()))
}

/// Format a year as Russian ordinal words ("две тысячи двадцать шестого").
#[wasm_bindgen(js_name = "yearToWords")]
pub fn year_to_words_js(year: u64) -> String {
    crate::year_to_words(year)
}

/// Format a wall-clock time (hour:minute) as Russian words.
#[wasm_bindgen(js_name = "timeToWords")]
pub fn time_to_words_js(hour: u32, minute: u32) -> Result<String, JsError> {
    crate::time_to_words(hour, minute).map_err(|e| JsError::new(&e.to_string()))
}

fn parse_gender(s: &str) -> crate::Gender {
    match s {
        "feminine" | "f" | "fem" => crate::Gender::Feminine,
        "neuter" | "n" | "neu" => crate::Gender::Neuter,
        _ => crate::Gender::Masculine,
    }
}

/// Look up a currency by ISO 4217 code, falling back to the rouble.
fn currency_by_code(code: &str) -> &'static crate::Currency<'static> {
    crate::Currency::from_iso(code).unwrap_or(&crate::RUB)
}
