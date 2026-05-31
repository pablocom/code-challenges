//! Add two binary strings, returning their binary sum.
//!
//! Like grade-school addition from the least-significant bit, carrying as we
//! go. The result keeps the width of the wider operand (preserving leading
//! zeros), plus one extra digit only when a final carry remains.

pub fn solve(a: &str, b: &str) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let width = a.len().max(b.len());

    let mut digits = vec![0u8; width];
    let mut carry = 0u8;
    let mut i = a.len() as isize - 1;
    let mut j = b.len() as isize - 1;
    let mut w = width as isize - 1;

    while i >= 0 || j >= 0 {
        let a_digit = if i >= 0 { a[i as usize] - b'0' } else { 0 };
        let b_digit = if j >= 0 { b[j as usize] - b'0' } else { 0 };

        let sum = carry + a_digit + b_digit;
        digits[w as usize] = b'0' + (sum & 1);
        carry = sum >> 1;

        i -= 1;
        j -= 1;
        w -= 1;
    }

    let mut result = String::with_capacity(width + 1);
    if carry == 1 {
        result.push('1');
    }
    result.push_str(std::str::from_utf8(&digits).unwrap());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("10110", "10101"), "101011");
        assert_eq!(solve("111", "1"), "1000");
        assert_eq!(solve("0", "0"), "0");
        assert_eq!(
            solve("10100110101010101", "1101011010011010"),
            "100010001111101111"
        );
        assert_eq!(solve("000000", "11010"), "011010");
    }
}
