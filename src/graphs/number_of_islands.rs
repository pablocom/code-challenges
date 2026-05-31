//! Number of Islands — count connected groups of `'1'` (4-directional) in a
//! grid of `'0'`/`'1'`. Each row is a string; the grid is flood-filled in place.

pub fn solve(grid: &[&str]) -> i32 {
    let mut grid: Vec<Vec<u8>> = grid.iter().map(|r| r.as_bytes().to_vec()).collect();
    let mut islands = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'1' {
                sink(&mut grid, i as isize, j as isize);
                islands += 1;
            }
        }
    }

    islands
}

fn sink(grid: &mut [Vec<u8>], i: isize, j: isize) {
    if i < 0 || i >= grid.len() as isize {
        return;
    }
    let (row, col) = (i as usize, j as usize);
    if j < 0 || j >= grid[row].len() as isize || grid[row][col] != b'1' {
        return;
    }

    grid[row][col] = b'X';
    sink(grid, i + 1, j);
    sink(grid, i - 1, j);
    sink(grid, i, j + 1);
    sink(grid, i, j - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&["11110", "11010", "11000", "00000"]), 1);
        assert_eq!(solve(&["11000", "11000", "00100", "00011"]), 3);
        assert_eq!(solve(&["00", "00"]), 0);
        assert_eq!(solve(&["11", "11"]), 1);
        assert_eq!(solve(&["10", "01"]), 2);
        assert_eq!(solve(&["1"]), 1);
    }
}
