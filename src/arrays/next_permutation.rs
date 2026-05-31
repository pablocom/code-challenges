//! Next Permutation — rearrange in place to the next lexicographically greater
//! arrangement, wrapping to the smallest when already the largest.
//!
//! 1. Walk from the right to the first index `i` where `nums[i] < nums[i+1]`.
//! 2. Swap `nums[i]` with the smallest element to its right that still exceeds it.
//! 3. Reverse the suffix after `i` so it becomes ascending.

pub fn to_next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    if n < 2 {
        return;
    }

    // `pivot` walks down to -1 (represented by `None`) when the array is the
    // largest permutation.
    let mut pivot = n as isize - 2;
    while pivot >= 0 && nums[pivot as usize] >= nums[(pivot + 1) as usize] {
        pivot -= 1;
    }

    if pivot >= 0 {
        let p = pivot as usize;
        let mut j = n - 1;
        while nums[p] >= nums[j] {
            j -= 1;
        }
        nums.swap(p, j);
    }

    nums[(pivot + 1) as usize..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next(nums: &[i32]) -> Vec<i32> {
        let mut nums = nums.to_vec();
        to_next_permutation(&mut nums);
        nums
    }

    #[test]
    fn examples() {
        assert_eq!(next(&[2, 1]), vec![1, 2]);
        assert_eq!(next(&[1, 2]), vec![2, 1]);
        assert_eq!(next(&[3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(next(&[1, 2, 3]), vec![1, 3, 2]);
        assert_eq!(next(&[1, 1, 5]), vec![1, 5, 1]);
        assert_eq!(next(&[4, 7, 1, 4, 3, 2]), vec![4, 7, 2, 1, 3, 4]);
        assert_eq!(next(&[4, 7, 1, 4, 2, 3]), vec![4, 7, 1, 4, 3, 2]);
        assert_eq!(next(&[4, 7, 4, 9, 8, 7]), vec![4, 7, 7, 4, 8, 9]);
        assert_eq!(next(&[1, 9, 9, 6]), vec![6, 1, 9, 9]);
        assert_eq!(next(&[1, 3, 2]), vec![2, 1, 3]);
        assert_eq!(next(&[2, 3, 1]), vec![3, 1, 2]);
    }
}
