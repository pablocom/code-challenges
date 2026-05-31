//! Remove all occurrences of `val` in place, returning the new length.
//!
//! A write pointer keeps every element that is not `val`; ordering of the kept
//! prefix is preserved.

pub fn remove_element(nums: &mut [i32], val: i32) -> usize {
    let mut write = 0;
    for read in 0..nums.len() {
        if nums[read] != val {
            nums[write] = nums[read];
            write += 1;
        }
    }
    write
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kept_sorted(nums: &[i32], len: usize) -> Vec<i32> {
        let mut kept = nums[..len].to_vec();
        kept.sort_unstable();
        kept
    }

    #[test]
    fn empty_array_returns_zero() {
        let mut nums: [i32; 0] = [];
        assert_eq!(remove_element(&mut nums, 3), 0);
    }

    #[test]
    fn single_matching_element_returns_zero() {
        let mut nums = [3];
        assert_eq!(remove_element(&mut nums, 3), 0);
    }

    #[test]
    fn single_non_matching_element_returns_one() {
        let mut nums = [5];
        assert_eq!(remove_element(&mut nums, 3), 1);
        assert_eq!(nums[0], 5);
    }

    #[test]
    fn all_elements_match() {
        let mut nums = [2, 2, 2, 2];
        assert_eq!(remove_element(&mut nums, 2), 0);
    }

    #[test]
    fn leetcode_examples() {
        let mut a = [3, 2, 2, 3];
        let la = remove_element(&mut a, 3);
        assert_eq!(la, 2);
        assert_eq!(kept_sorted(&a, la), vec![2, 2]);

        let mut b = [0, 1, 2, 2, 3, 0, 4, 2];
        let lb = remove_element(&mut b, 2);
        assert_eq!(lb, 5);
        assert_eq!(kept_sorted(&b, lb), vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn scattered_duplicates() {
        let mut nums = [1, 7, 3, 7, 5, 7, 7, 9];
        let len = remove_element(&mut nums, 7);
        assert_eq!(len, 4);
        assert_eq!(kept_sorted(&nums, len), vec![1, 3, 5, 9]);
    }
}
