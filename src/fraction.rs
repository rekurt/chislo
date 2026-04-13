//! Common fraction formatting (e.g. 1/2 → "одна вторая").

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::convert::convert_int_to_words;
use crate::decline::get_declension;
use crate::dictionary::{ORDINAL_HUNDREDS, ORDINAL_ONES, ORDINAL_TEENS, ORDINAL_TENS};
use crate::{Error, Gender};

const FEMININE: usize = 1;

/// Formats an ordinary fraction with numerator and denominator as a Russian
/// phrase like "одна вторая", "три пятых", "семь сотых".
///
/// Supported denominators: any in `2..=999` whose ordinal form has a feminine
/// nominative in the dictionary. Numerator may be negative.
///
/// # Examples
///
/// ```
/// use chislo::fraction;
///
/// assert_eq!(fraction(1, 2).unwrap(), "одна вторая");
/// assert_eq!(fraction(3, 5).unwrap(), "три пятых");
/// assert_eq!(fraction(1, 100).unwrap(), "одна сотая");
/// assert_eq!(fraction(22, 7).unwrap(), "двадцать две седьмых");
/// assert_eq!(fraction(-1, 2).unwrap(), "минус одна вторая");
/// ```
pub fn fraction(numer: i64, denom: u32) -> Result<String, Error> {
    if denom < 2 {
        return Err(Error::InvalidNumber(format!(
            "denominator must be >= 2, got {denom}"
        )));
    }

    let numer_words = convert_int_to_words(numer, Gender::Feminine);

    // Determine the declension form: "вторая" (1), "вторых" (2-4/5+/11-19).
    let last_two = (numer.unsigned_abs() % 100) as u32;
    let denom_ordinal = feminine_ordinal_of(denom)?;
    let denom_decl = if (11..=19).contains(&last_two) {
        &denom_ordinal.plural
    } else {
        match last_two % 10 {
            1 => &denom_ordinal.singular,
            _ => &denom_ordinal.plural,
        }
    };

    Ok(format!("{numer_words} {denom_decl}"))
}

/// Formats a mixed number: "две целых одна треть" style (one whole + fraction).
///
/// # Examples
///
/// ```
/// use chislo::mixed_fraction;
///
/// assert_eq!(
///     mixed_fraction(1, 1, 2).unwrap(),
///     "одна целая одна вторая"
/// );
/// assert_eq!(
///     mixed_fraction(3, 2, 5).unwrap(),
///     "три целых две пятых"
/// );
/// ```
pub fn mixed_fraction(whole: i64, numer: i64, denom: u32) -> Result<String, Error> {
    let whole_words = convert_int_to_words(whole, Gender::Feminine);
    let whole_decl = get_declension(whole, "целая", "целых", "целых");
    let frac = fraction(numer, denom)?;
    Ok(format!("{whole_words} {whole_decl} {frac}"))
}

struct FemOrdinalPair {
    singular: String, // e.g. "вторая" (agrees with 1)
    plural: String,   // e.g. "вторых" (agrees with 2-4/5+/11-19)
}

/// Builds the feminine singular and plural genitive ("X-ая" / "X-ых") forms
/// of the ordinal for the given integer denominator. Works for 2..=999 by
/// looking up the ordinal in nominative feminine form and transforming the
/// suffix.
fn feminine_ordinal_of(n: u32) -> Result<FemOrdinalPair, Error> {
    if !(2..=999).contains(&n) {
        return Err(Error::InvalidNumber(format!(
            "unsupported denominator: {n}"
        )));
    }

    let h = (n / 100) as usize;
    let t = ((n % 100) / 10) as usize;
    let o = (n % 10) as usize;

    // Generate the last ordinal word (feminine nominative) and transform to
    // plural genitive by replacing the trailing "ая" → "ых" or "яя" → "их".
    // For the compound part we need cardinals joined with the last ordinal.
    let mut parts: Vec<String> = Vec::new();
    let mut parts_plural: Vec<String> = Vec::new();

    if t == 1 {
        // Teens: "одиннадцатая" / "одиннадцатых"
        if h > 0 {
            parts.push(crate::dictionary::HUNDREDS[h - 1].to_string());
            parts_plural.push(crate::dictionary::HUNDREDS[h - 1].to_string());
        }
        let sing = ORDINAL_TEENS[o][FEMININE];
        parts.push(sing.to_string());
        parts_plural.push(pluralize_feminine(sing));
    } else if o > 0 {
        if h > 0 {
            parts.push(crate::dictionary::HUNDREDS[h - 1].to_string());
            parts_plural.push(crate::dictionary::HUNDREDS[h - 1].to_string());
        }
        if t > 0 {
            parts.push(crate::dictionary::TENS[t].to_string());
            parts_plural.push(crate::dictionary::TENS[t].to_string());
        }
        let sing = ORDINAL_ONES[o - 1][FEMININE];
        parts.push(sing.to_string());
        parts_plural.push(pluralize_feminine(sing));
    } else if t > 0 {
        if h > 0 {
            parts.push(crate::dictionary::HUNDREDS[h - 1].to_string());
            parts_plural.push(crate::dictionary::HUNDREDS[h - 1].to_string());
        }
        let sing = ORDINAL_TENS[t - 2][FEMININE];
        parts.push(sing.to_string());
        parts_plural.push(pluralize_feminine(sing));
    } else {
        let sing = ORDINAL_HUNDREDS[h - 1][FEMININE];
        parts.push(sing.to_string());
        parts_plural.push(pluralize_feminine(sing));
    }

    Ok(FemOrdinalPair {
        singular: parts.join(" "),
        plural: parts_plural.join(" "),
    })
}

/// Converts a feminine nominative ordinal ("вторая", "третья") to its
/// plural genitive form ("вторых", "третьих") used after 2+ in fractions.
fn pluralize_feminine(s: &str) -> String {
    // "третья" → "третьих"
    if let Some(stripped) = s.strip_suffix("ья") {
        return format!("{stripped}ьих");
    }
    // "вторая" / "сотая" / "десятая" → "вторых" / "сотых" / "десятых"
    if let Some(stripped) = s.strip_suffix("ая") {
        return format!("{stripped}ых");
    }
    // Fallback: return as-is (should not happen with dictionary data).
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_basic() {
        let cases: &[(i64, u32, &str)] = &[
            (1, 2, "одна вторая"),
            (1, 3, "одна третья"),
            (1, 4, "одна четвёртая"),
            (1, 5, "одна пятая"),
            (3, 5, "три пятых"),
            (1, 10, "одна десятая"),
            (1, 100, "одна сотая"),
            (2, 3, "две третьих"),
            (5, 7, "пять седьмых"),
            (22, 7, "двадцать две седьмых"),
            (-1, 2, "минус одна вторая"),
            (0, 5, "ноль пятых"),
            (11, 13, "одиннадцать тринадцатых"),
        ];
        for &(n, d, expected) in cases {
            assert_eq!(fraction(n, d).unwrap(), expected, "fraction({n}, {d})");
        }
    }

    #[test]
    fn test_fraction_large_denom() {
        assert_eq!(fraction(1, 25).unwrap(), "одна двадцать пятая");
        assert_eq!(fraction(1, 200).unwrap(), "одна двухсотая");
        assert_eq!(
            fraction(7, 999).unwrap(),
            "семь девятьсот девяносто девятых"
        );
    }

    #[test]
    fn test_fraction_errors() {
        assert!(fraction(1, 0).is_err());
        assert!(fraction(1, 1).is_err());
        assert!(fraction(1, 1000).is_err());
    }

    #[test]
    fn test_mixed_fraction() {
        assert_eq!(mixed_fraction(1, 1, 2).unwrap(), "одна целая одна вторая");
        assert_eq!(mixed_fraction(3, 2, 5).unwrap(), "три целых две пятых");
        assert_eq!(mixed_fraction(2, 1, 4).unwrap(), "две целых одна четвёртая");
    }
}
