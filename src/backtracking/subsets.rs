//! All subsets (the power set) of a slice of distinct numbers.
//!
//! At each index, branch on excluding then including that element, yielding all
//! `2^n` subsets.

pub fn solve(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    traverse(nums, 0, &mut current, &mut result);
    result
}

fn traverse(nums: &[i32], position: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if position == nums.len() {
        result.push(current.clone());
        return;
    }

    traverse(nums, position + 1, current, result);

    current.push(nums[position]);
    traverse(nums, position + 1, current, result);
    current.pop();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array_yields_only_the_empty_subset() {
        let result = solve(&[]);
        assert_eq!(result.len(), 1);
        assert!(result[0].is_empty());
    }

    #[test]
    fn single_element() {
        let result = solve(&[1]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
    }

    #[test]
    fn two_elements() {
        let result = solve(&[1, 2]);
        assert_eq!(result.len(), 4);
        for subset in [vec![], vec![1], vec![2], vec![1, 2]] {
            assert!(result.contains(&subset));
        }
    }

    #[test]
    fn count_is_two_to_the_n() {
        assert_eq!(solve(&[1, 2, 3]).len(), 8);
        assert_eq!(solve(&[-1, 0, 1]).len(), 8);
        let sixteen: Vec<i32> = (1..=16).collect();
        assert_eq!(solve(&sixteen).len(), 65536);
    }
}
