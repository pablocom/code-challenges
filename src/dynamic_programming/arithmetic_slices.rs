//! Count arithmetic slices — contiguous subarrays of length ≥ 3 with a constant
//! difference. Each time the run of equal differences grows, it adds that many
//! new slices ending at the current index.

pub fn solve(nums: &[i32]) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut result = 0;
    let mut run = 0;

    for i in 2..nums.len() {
        if nums[i - 2] - nums[i - 1] == nums[i - 1] - nums[i] {
            run += 1;
            result += run;
        } else {
            run = 0;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 2]), 0);
        assert_eq!(solve(&[1, 2, 3]), 1);
        assert_eq!(solve(&[1, 2, 3, 4]), 3);
        assert_eq!(solve(&[1, 2, 3, 4, 5]), 6);
        assert_eq!(solve(&[1, 2, 3, 1, 2]), 1);
    }
}
