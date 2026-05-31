//! Combination Sum — all combinations of candidates (each usable unlimited
//! times) that add up to `target`.
//!
//! At each position, branch on "use this candidate again" versus "move past
//! it", pruning once the running sum reaches or exceeds the target.

pub fn solve(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    traverse(candidates, 0, 0, target, &mut current, &mut result);
    result
}

fn traverse(
    candidates: &[i32],
    position: usize,
    sum: i32,
    target: i32,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if position >= candidates.len() {
        return;
    }
    if sum == target {
        result.push(current.clone());
        return;
    }
    if sum > target {
        return;
    }

    current.push(candidates[position]);
    traverse(
        candidates,
        position,
        sum + candidates[position],
        target,
        current,
        result,
    );
    current.pop();
    traverse(candidates, position + 1, sum, target, current, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[2, 3, 6, 7], 7), vec![vec![2, 2, 3], vec![7]]);
        assert_eq!(
            solve(&[2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(solve(&[2], 3), Vec::<Vec<i32>>::new());
        assert_eq!(solve(&[1], 1), vec![vec![1]]);
        assert_eq!(solve(&[1], 3), vec![vec![1, 1, 1]]);
    }
}
