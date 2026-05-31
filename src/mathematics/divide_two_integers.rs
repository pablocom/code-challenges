//! Integer division without using `/` or `%`, clamping the one overflow case
//! (`i32::MIN / -1`) to `i32::MAX`.
//!
//! This keeps the original repeated-subtraction approach: add the divisor until
//! it would exceed the dividend.

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }
    if dividend == i32::MIN && divisor == 1 {
        return i32::MIN;
    }

    let sign: i64 = if (dividend < 0) ^ (divisor < 0) {
        -1
    } else {
        1
    };
    let abs_dividend = (dividend as i64).abs();
    let abs_divisor = (divisor as i64).abs();

    let mut quotient: i64 = 0;
    let mut accumulated: i64 = 0;
    while accumulated < abs_dividend {
        accumulated += abs_divisor;
        quotient += 1;
    }
    if accumulated > abs_dividend {
        quotient -= 1;
    }

    (quotient * sign) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(divide(10, 5), 2);
        assert_eq!(divide(16, 5), 3);
        assert_eq!(divide(7, -3), -2);
        assert_eq!(divide(-2147483648, -1), 2147483647);
        assert_eq!(divide(2147483647, 2147483646), 1);
    }
}
