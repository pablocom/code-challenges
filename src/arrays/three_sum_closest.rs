//! 3Sum Closest — the triplet sum nearest to `target`.
//!
//! Sort, then for each anchor run a two-pointer scan, tracking the smallest
//! absolute distance to the target seen so far.

pub fn solve(array: &[i32], target: i32) -> i32 {
    let mut array = array.to_vec();
    array.sort_unstable();

    let mut result = 0;
    let mut best_diff = i32::MAX;

    for i in 0..array.len() {
        if i != 0 && array[i - 1] == array[i] {
            continue;
        }

        let (mut left, mut right) = (i + 1, array.len() - 1);
        while left < right {
            let sum = array[i] + array[left] + array[right];
            let diff = (sum - target).abs();

            if diff < best_diff {
                best_diff = diff;
                result = sum;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[-1, 2, 1, -4], 1), 2);
        assert_eq!(solve(&[0, 0, 0], 1), 0);
        assert_eq!(solve(&[1, 1, 1, 0], 1), 2);
        assert_eq!(solve(&[0, 2, 1, -3], 1), 0);
    }
}
