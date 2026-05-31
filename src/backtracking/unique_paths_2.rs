//! Unique Paths II — count top-left → bottom-right paths (moving only right or
//! down) on a grid where `1` marks an obstacle.
//!
//! Memoized DFS: cache the path count from each cell so it is computed once.

use std::collections::HashMap;

pub fn unique_paths_with_obstacles(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
    count_paths(grid, 0, 0, rows, cols, &mut memo)
}

fn count_paths(
    grid: &[Vec<i32>],
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if let Some(&cached) = memo.get(&(x, y)) {
        return cached;
    }
    if grid[x][y] == 1 {
        return 0;
    }
    if x == rows - 1 && y == cols - 1 {
        return 1;
    }

    let mut paths = 0;
    if x < rows - 1 {
        paths += count_paths(grid, x + 1, y, rows, cols, memo);
    }
    if y < cols - 1 {
        paths += count_paths(grid, x, y + 1, rows, cols, memo);
    }

    memo.insert((x, y), paths);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&[i32]]) -> Vec<Vec<i32>> {
        rows.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn examples() {
        assert_eq!(
            unique_paths_with_obstacles(&grid(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]])),
            6
        );
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0, 1], &[1, 0]])), 0);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[1, 0], &[0, 0]])), 0);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0, 0], &[0, 1]])), 0);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0]])), 1);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[1]])), 0);
        assert_eq!(
            unique_paths_with_obstacles(&grid(&[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]])),
            2
        );
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0, 0, 0]])), 1);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0], &[0], &[0]])), 1);
        assert_eq!(unique_paths_with_obstacles(&grid(&[&[0, 1], &[0, 0]])), 1);
    }
}
