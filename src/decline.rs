#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

pub fn decline(n: i64, one: &str, two: &str, five: &str) -> String {
    get_declension(n, one, two, five).to_string()
}

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
    fn test_decline() {
        assert_eq!(decline(1, "рубль", "рубля", "рублей"), "рубль");
        assert_eq!(decline(5, "рубль", "рубля", "рублей"), "рублей");
        assert_eq!(decline(11, "рубль", "рубля", "рублей"), "рублей");
        assert_eq!(decline(21, "рубль", "рубля", "рублей"), "рубль");
    }
}
