//! Multiply two non-negative integers given as strings, without converting the
//! whole values to a native integer type.
//!
//! Grade-school multiplication: digit `i` of `num1` times digit `j` of `num2`
//! contributes to result positions `i + j` and `i + j + 1`.

pub fn solve(num1: &str, num2: &str) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let a = num1.as_bytes();
    let b = num2.as_bytes();
    let mut product = vec![0u32; a.len() + b.len()];

    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            let mul = u32::from(a[i] - b'0') * u32::from(b[j] - b'0');
            let low = i + j + 1;
            let sum = mul + product[low];
            product[low] = sum % 10;
            product[i + j] += sum / 10;
        }
    }

    let first_significant = product
        .iter()
        .position(|&d| d != 0)
        .unwrap_or(product.len() - 1);
    product[first_significant..]
        .iter()
        .map(|&d| (d as u8 + b'0') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("1", "2"), "2");
        assert_eq!(solve("2", "3"), "6");
        assert_eq!(solve("99", "2"), "198");
        assert_eq!(solve("999", "22"), "21978");
        assert_eq!(solve("9999999999999999", "222"), "2219999999999999778");
        assert_eq!(
            solve("11111111111111111111111111111111111", "1000000"),
            "11111111111111111111111111111111111000000"
        );
        assert_eq!(solve("0", "12345"), "0");
    }
}
