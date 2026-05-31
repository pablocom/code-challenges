//! `atoi` — parse a leading optionally-signed integer, clamping to `i32` range.
//!
//! Skip surrounding whitespace, read an optional sign, then consume digits
//! until a non-digit, saturating at `i32::MIN` / `i32::MAX` on overflow.

pub fn my_atoi(s: &str) -> i32 {
    let bytes = s.trim().as_bytes();

    let (sign, start): (i64, usize) = match bytes.first() {
        Some(b'-') => (-1, 1),
        Some(b'+') => (1, 1),
        _ => (1, 0),
    };

    let mut value: i64 = 0;
    for &c in &bytes[start..] {
        if !c.is_ascii_digit() {
            break;
        }
        value = value * 10 + (c - b'0') as i64;

        let signed = value * sign;
        if signed > i32::MAX as i64 {
            return i32::MAX;
        }
        if signed < i32::MIN as i64 {
            return i32::MIN;
        }
    }

    (value * sign) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(my_atoi("42"), 42);
        assert_eq!(my_atoi("-42"), -42);
        assert_eq!(my_atoi("   -42"), -42);
        assert_eq!(my_atoi("4193 with words"), 4193);
        assert_eq!(my_atoi("words and 987"), 0);
        assert_eq!(my_atoi("+1"), 1);
        assert_eq!(my_atoi(""), 0);
        assert_eq!(my_atoi("   "), 0);
        assert_eq!(my_atoi("0"), 0);
        assert_eq!(my_atoi("7"), 7);
        assert_eq!(my_atoi("91283472332"), i32::MAX);
    }
}
