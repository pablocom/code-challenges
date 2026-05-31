//! Maximum Area of Island — the largest connected group of `1`s (4-directional)
//! in a binary grid. Flood-fill from each land cell, marking cells visited.

pub fn solve(grid: &[Vec<i32>]) -> i32 {
    let mut grid: Vec<Vec<i32>> = grid.to_vec();
    let mut max_area = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                max_area = max_area.max(flood_fill(&mut grid, i as isize, j as isize));
            }
        }
    }

    max_area
}

fn flood_fill(grid: &mut [Vec<i32>], i: isize, j: isize) -> i32 {
    if i < 0 || i >= grid.len() as isize {
        return 0;
    }
    let (row, col) = (i as usize, j as usize);
    if j < 0 || j >= grid[row].len() as isize || grid[row][col] != 1 {
        return 0;
    }

    grid[row][col] = -1; // mark visited

    1 + flood_fill(grid, i + 1, j)
        + flood_fill(grid, i - 1, j)
        + flood_fill(grid, i, j + 1)
        + flood_fill(grid, i, j - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&[i32]]) -> Vec<Vec<i32>> {
        rows.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn leetcode_example() {
        let g = grid(&[
            &[0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            &[0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            &[0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]);
        assert_eq!(solve(&g), 6);
    }

    #[test]
    fn small_grids() {
        assert_eq!(solve(&grid(&[&[0, 0, 0], &[0, 0, 0]])), 0);
        assert_eq!(solve(&grid(&[&[0, 1, 0], &[0, 0, 0]])), 1);
        assert_eq!(solve(&grid(&[&[1, 1], &[1, 1]])), 4);
        assert_eq!(solve(&grid(&[&[1, 0], &[0, 1]])), 1);
        assert_eq!(solve(&grid(&[&[1, 0, 0], &[1, 0, 0], &[1, 1, 1]])), 5);
    }

    #[test]
    fn multiple_islands_returns_largest() {
        let g = grid(&[
            &[1, 1, 0, 0, 0],
            &[1, 1, 0, 0, 0],
            &[0, 0, 0, 1, 1],
            &[0, 0, 0, 1, 1],
            &[0, 0, 0, 1, 1],
        ]);
        assert_eq!(solve(&g), 6);
    }
}
