//! Reverse the digits of a signed 32-bit integer, returning 0 on overflow.
//!
//! Two takes: build the reversed value in a wider `i64` and range-check at the
//! end, or accumulate in `i32` and guard against overflow each step.

/// Reverse using a wider accumulator, then check it fits in `i32`.
pub fn reverse(input: i32) -> i32 {
    let sign: i64 = if input < 0 { -1 } else { 1 };
    let mut n = (input as i64).abs();
    let mut reversed: i64 = 0;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed *= sign;

    if (i32::MIN as i64..=i32::MAX as i64).contains(&reversed) {
        reversed as i32
    } else {
        0
    }
}

/// Reverse in `i32`, bailing out before any step would overflow.
pub fn reverse_optimized(mut input: i32) -> i32 {
    let mut result: i32 = 0;

    while input != 0 {
        let digit = input % 10;
        input /= 10;

        if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
            return 0;
        }
        if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
            return 0;
        }
        result = result * 10 + digit;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_examples() {
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(-2147483412), -2143847412);
        assert_eq!(reverse(-2147483648), 0);
        assert_eq!(reverse(2147483647), 0);
        assert_eq!(reverse(1534236469), 0);
    }

    #[test]
    fn reverse_optimized_examples() {
        assert_eq!(reverse_optimized(0), 0);
        assert_eq!(reverse_optimized(123), 321);
        assert_eq!(reverse_optimized(-123), -321);
        assert_eq!(reverse_optimized(120), 21);
        assert_eq!(reverse_optimized(-2147483412), -2143847412);
        assert_eq!(reverse_optimized(-2147483648), 0);
        assert_eq!(reverse_optimized(2147483641), 1463847412);
        assert_eq!(reverse_optimized(2147483647), 0);
        assert_eq!(reverse_optimized(1534236469), 0);
    }
}
