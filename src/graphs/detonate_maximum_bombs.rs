//! Detonate the Maximum Bombs — a bomb triggers any bomb within its blast
//! radius, which chains. Find the largest chain reachable from a single bomb.
//!
//! Build a directed graph (`i -> j` if `j` is within `i`'s radius), then DFS
//! from each bomb and take the largest reachable set. Distances are compared
//! squared to stay in integer arithmetic.

pub fn solve(bombs: &[[i32; 3]]) -> usize {
    let n = bombs.len();
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n];

    for i in 0..n {
        for j in i + 1..n {
            let [x1, y1, r1] = bombs[i];
            let [x2, y2, r2] = bombs[j];
            let dx = (x2 - x1) as i64;
            let dy = (y2 - y1) as i64;
            let distance_sq = dx * dx + dy * dy;

            if distance_sq <= (r1 as i64).pow(2) {
                adjacency[i].push(j);
            }
            if distance_sq <= (r2 as i64).pow(2) {
                adjacency[j].push(i);
            }
        }
    }

    let mut best = 0;
    for start in 0..n {
        let mut visited = vec![false; n];
        best = best.max(reachable(start, &adjacency, &mut visited));
    }
    best
}

fn reachable(bomb: usize, adjacency: &[Vec<usize>], visited: &mut [bool]) -> usize {
    if visited[bomb] {
        return 0;
    }
    visited[bomb] = true;

    let mut count = 1;
    for &next in &adjacency[bomb] {
        count += reachable(next, adjacency, visited);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[[2, 1, 3], [6, 1, 4]]), 2);
        assert_eq!(solve(&[[1, 1, 5], [10, 10, 5]]), 1);
        assert_eq!(
            solve(&[[1, 2, 3], [2, 3, 1], [3, 4, 2], [4, 5, 3], [5, 6, 4]]),
            5
        );
    }
}
