//! Plus One — add one to a number represented as a digit array (most
//! significant digit first), handling the carry across nines.

pub fn solve(digits: &[i32]) -> Vec<i32> {
    let mut digits = digits.to_vec();

    for digit in digits.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return digits;
        }
        *digit = 0;
    }

    // Every digit was 9 (now all 0): prepend a leading 1.
    let mut result = Vec::with_capacity(digits.len() + 1);
    result.push(1);
    result.extend(digits);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(solve(&[1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(solve(&[1, 9, 9]), vec![2, 0, 0]);
        assert_eq!(solve(&[9, 9, 9]), vec![1, 0, 0, 0]);
        assert_eq!(solve(&[0]), vec![1]);
        assert_eq!(solve(&[9]), vec![1, 0]);
    }
}
