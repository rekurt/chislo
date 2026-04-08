/// Returns the correct Russian noun declension form based on a number.
///
/// Russian nouns change form depending on the number:
/// - `one`: form for 1, 21, 31, 41... (ends with 1, except 11)
/// - `two`: form for 2-4, 22-24, 32-34... (ends with 2-4, except 12-14)
/// - `five`: form for 0, 5-20, 25-30... (everything else)
///
/// # Examples
///
/// ```
/// use chislo::decline;
///
/// assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
/// assert_eq!(decline(2, "рубль", "рубля", "рублей"), "рубля");
/// assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
/// assert_eq!(decline(11, "рубль", "рубля", "рублей"), "рублей");
/// assert_eq!(decline(21, "рубль", "рубля", "рублей"), "рубль");
/// ```
pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    get_declension(n, one, two, five).to_string()
}

/// Internal declension logic returning a &str reference.
pub(crate) fn get_declension<'a>(n: i64, one: &'a str, two: &'a str, five: &'a str) -> &'a str {
    let n = (n.abs() % 100) as u64;

    if (11..=19).contains(&n) {
        return five;
    }

    match n % 10 {
        1 => one,
        2..=4 => two,
        _ => five,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decline_common_currencies() {
        let cases = [
            (1, "рубль"),
            (2, "рубля"),
            (5, "рублей"),
            (11, "рублей"),
            (21, "рубль"),
            (104, "рубля"),
            (111, "рублей"),
            (1234, "рубля"),
        ];

        for (n, expected) in cases {
            assert_eq!(
                decline(n, "рубль", "рубля", "рублей"),
                expected,
                "decline({n}) failed"
            );
        }
    }

    #[test]
    fn test_decline_dollars() {
        let cases = [
            (1, "доллар"),
            (2, "доллара"),
            (5, "долларов"),
            (11, "долларов"),
            (21, "доллар"),
        ];

        for (n, expected) in cases {
            assert_eq!(
                decline(n, "доллар", "доллара", "долларов"),
                expected,
                "decline({n}) for dollars failed"
            );
        }
    }

    #[test]
    fn test_decline_indeclinable() {
        // "евро" doesn't change form
        for n in [1, 2, 5, 11, 21] {
            assert_eq!(decline(n, "евро", "евро", "евро"), "евро");
        }
    }

    #[test]
    fn test_get_declension_edge_cases() {
        assert_eq!(get_declension(0, "one", "two", "five"), "five");
        assert_eq!(get_declension(2, "one", "two", "five"), "two");
        assert_eq!(get_declension(3, "one", "two", "five"), "two");
        assert_eq!(get_declension(7, "one", "two", "five"), "five");
        assert_eq!(get_declension(111, "one", "two", "five"), "five");
        assert_eq!(get_declension(219, "one", "two", "five"), "five");
    }

    #[test]
    fn test_decline_negative() {
        assert_eq!(decline(-1, "рубль", "рубля", "рублей"), "рубль");
        assert_eq!(decline(-2, "рубль", "рубля", "рублей"), "рубля");
        assert_eq!(decline(-5, "рубль", "рубля", "рублей"), "рублей");
        assert_eq!(decline(-11, "рубль", "рубля", "рублей"), "рублей");
    }
}
