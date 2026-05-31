//! Network Delay Time — time for a signal from node `k` to reach all `n` nodes
//! (1-indexed), or `-1` if some node is unreachable.
//!
//! Dijkstra's algorithm with a min-heap; the answer is the largest shortest-path
//! distance once every node is settled.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve(times: &[[i32; 3]], n: usize, k: usize) -> i32 {
    let mut adjacency: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
    for edge in times {
        adjacency[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    let mut visited = vec![false; n + 1];
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, k)));

    let mut max_delay = 0;
    let mut settled = 0;

    while let Some(Reverse((delay, node))) = heap.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        settled += 1;
        max_delay = max_delay.max(delay);

        for &(destination, weight) in &adjacency[node] {
            if !visited[destination] {
                heap.push(Reverse((delay + weight, destination)));
            }
        }
    }

    if settled == n { max_delay } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2), 2);
        assert_eq!(solve(&[[1, 2, 1]], 3, 1), -1);
        assert_eq!(solve(&[], 1, 1), 0);
        assert_eq!(solve(&[[1, 2, 1], [1, 3, 10], [2, 3, 2]], 3, 1), 3);
        assert_eq!(solve(&[[1, 2, 3], [2, 3, 4], [3, 4, 5]], 4, 1), 12);
        assert_eq!(solve(&[[2, 1, 5]], 2, 1), -1);
    }
}
