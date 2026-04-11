#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    get_declension(n, one, two, five).to_string()
}

pub(crate) fn get_declension<'a>(n: i64, one: &'a str, two: &'a str, five: &'a str) -> &'a str {
    let n = ((n as i128).unsigned_abs() % 100) as u64;

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
    fn test_decline() {
        assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
        assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
        assert_eq!(decline(11, "рубль", "рубля", "рублей"), "рублей");
        assert_eq!(decline(21, "рубль", "рубля", "рублей"), "рубль");
    }

    #[test]
    fn test_decline_all_boundaries() {
        let cases: [(i64, &str); 33] = [
            (0, "рублей"),
            (1, "рубль"),
            (2, "рубля"),
            (3, "рубля"),
            (4, "рубля"),
            (5, "рублей"),
            (6, "рублей"),
            (7, "рублей"),
            (8, "рублей"),
            (9, "рублей"),
            (10, "рублей"),
            (11, "рублей"),
            (12, "рублей"),
            (13, "рублей"),
            (14, "рублей"),
            (15, "рублей"),
            (16, "рублей"),
            (17, "рублей"),
            (18, "рублей"),
            (19, "рублей"),
            (20, "рублей"),
            (21, "рубль"),
            (22, "рубля"),
            (23, "рубля"),
            (24, "рубля"),
            (25, "рублей"),
            (100, "рублей"),
            (101, "рубль"),
            (102, "рубля"),
            (111, "рублей"),
            (112, "рублей"),
            (121, "рубль"),
            (200, "рублей"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                decline(n, "рубль", "рубля", "рублей"),
                expected,
                "decline({n})"
            );
        }
    }

    #[test]
    fn test_decline_negative_and_large() {
        let cases: [(i64, &str); 12] = [
            (-1, "рубль"),
            (-5, "рублей"),
            (-11, "рублей"),
            (-21, "рубль"),
            (-111, "рублей"),
            (1001, "рубль"),
            (1011, "рублей"),
            (1021, "рубль"),
            (1111, "рублей"),
            (10001, "рубль"),
            (100011, "рублей"),
            (1000021, "рубль"),
        ];
        for (n, expected) in cases {
            assert_eq!(
                decline(n, "рубль", "рубля", "рублей"),
                expected,
                "decline({n})"
            );
        }
    }

    #[test]
    fn test_decline_i64_min() {
        assert_eq!(decline(i64::MIN, "рубль", "рубля", "рублей"), "рублей");
    }
}
