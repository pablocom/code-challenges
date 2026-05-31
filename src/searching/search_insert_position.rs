//! Search Insert Position — the index of `target`, or where it would be
//! inserted to keep the array sorted. This is exactly a lower-bound search.

pub fn solve(nums: &[i32], target: i32) -> usize {
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if nums[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[2, 4, 6, 8, 10], 1), 0);
        assert_eq!(solve(&[1, 3, 7, 9, 11], 10), 4);
        assert_eq!(solve(&[5, 8, 12], 12), 2);
        assert_eq!(solve(&[1, 3, 5, 6], 5), 2);
        assert_eq!(solve(&[1, 3, 5, 6], 2), 1);
        assert_eq!(solve(&[1, 3, 5, 6], 7), 4);
        assert_eq!(solve(&[1, 3, 5, 6], 0), 0);
        assert_eq!(solve(&[1], 1), 0);
        assert_eq!(solve(&[1, 3], 3), 1);
        assert_eq!(solve(&[1, 3, 5], 4), 2);
    }
}
