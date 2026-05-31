//! Gray code sequence — successive integers differing by exactly one bit.
//!
//! Build it by reflection: for each new bit, mirror the existing sequence and
//! set that bit on the mirrored half.

pub fn solve(n: u32) -> Vec<i32> {
    let mut result = vec![0];

    for i in 0..n {
        let high_bit = 1 << i;
        for j in (0..result.len()).rev() {
            result.push(result[j] + high_bit);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(0), vec![0]);
        assert_eq!(solve(1), vec![0, 1]);
        assert_eq!(solve(2), vec![0, 1, 3, 2]);
        assert_eq!(solve(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
        assert_eq!(
            solve(4),
            vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8]
        );
    }
}
