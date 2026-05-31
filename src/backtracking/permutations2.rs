//! Permutations II — all *distinct* permutations of a multiset.
//!
//! Counting occurrences and recursing over distinct values (kept in ascending
//! order via a `BTreeMap`) naturally avoids generating duplicate permutations.

use std::collections::BTreeMap;

pub fn solve(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
    for &n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    let distinct: Vec<i32> = counts.keys().copied().collect();
    let mut result = Vec::new();
    let mut path = Vec::with_capacity(nums.len());
    permute(&distinct, &mut counts, nums.len(), &mut path, &mut result);
    result
}

fn permute(
    distinct: &[i32],
    counts: &mut BTreeMap<i32, usize>,
    total: usize,
    path: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if path.len() == total {
        result.push(path.clone());
        return;
    }

    for &num in distinct {
        if counts[&num] == 0 {
            continue;
        }
        *counts.get_mut(&num).unwrap() -= 1;
        path.push(num);
        permute(distinct, counts, total, path, result);
        path.pop();
        *counts.get_mut(&num).unwrap() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1]), vec![vec![1]]);
        assert_eq!(solve(&[1, 2]), vec![vec![1, 2], vec![2, 1]]);
        assert_eq!(
            solve(&[1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(solve(&[2, 2, 2]), vec![vec![2, 2, 2]]);
        assert_eq!(
            solve(&[1, 1, 2, 2]),
            vec![
                vec![1, 1, 2, 2],
                vec![1, 2, 1, 2],
                vec![1, 2, 2, 1],
                vec![2, 1, 1, 2],
                vec![2, 1, 2, 1],
                vec![2, 2, 1, 1],
            ]
        );
        assert_eq!(
            solve(&[-1, -1, 0]),
            vec![vec![-1, -1, 0], vec![-1, 0, -1], vec![0, -1, -1]]
        );
    }
}
