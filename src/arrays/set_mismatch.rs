//! Set Mismatch — one value `1..=n` is duplicated and one is missing.
//!
//! Use the array itself as a presence table: visiting value `v` flips the sign
//! at index `v - 1`. A slot already negative reveals the duplicate; the slot
//! still positive afterward reveals the missing value. Returns
//! `[duplicate, missing]`.

pub fn solve(nums: &[i32]) -> [i32; 2] {
    let mut nums = nums.to_vec();
    let mut duplicate = -1;
    let mut missing = 1;

    for i in 0..nums.len() {
        let index = (nums[i].abs() - 1) as usize;
        if nums[index] < 0 {
            duplicate = nums[i].abs();
        } else {
            nums[index] = -nums[index];
        }
    }

    for (i, &value) in nums.iter().enumerate() {
        if value > 0 {
            missing = i as i32 + 1;
            break;
        }
    }

    [duplicate, missing]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 3, 3, 4]), [3, 2]);
        assert_eq!(solve(&[2, 2, 3, 5, 4]), [2, 1]);
        assert_eq!(solve(&[1, 5, 3, 2, 7, 7, 6]), [7, 4]);
        assert_eq!(solve(&[1, 1]), [1, 2]);
        assert_eq!(solve(&[2, 2]), [2, 1]);
        assert_eq!(solve(&[1, 2, 3, 3]), [3, 4]);
    }
}
