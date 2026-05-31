//! Convert a Roman numeral to its integer value.
//!
//! Greedily strip the largest numeral (including the subtractive pairs like
//! `CM` and `IV`) from the front, accumulating its value.

const NUMERALS: &[(&str, i32)] = &[
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

pub fn solve(s: &str) -> i32 {
    let mut rest = s;
    let mut total = 0;

    for &(numeral, value) in NUMERALS {
        while let Some(stripped) = rest.strip_prefix(numeral) {
            total += value;
            rest = stripped;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("III"), 3);
        assert_eq!(solve("IV"), 4);
        assert_eq!(solve("MMD"), 2500);
        assert_eq!(solve("MCMXCIV"), 1994);
    }
}
