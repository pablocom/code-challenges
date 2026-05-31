//! Word Search — does `word` exist as a path of orthogonally adjacent cells,
//! using each cell at most once?
//!
//! DFS from every matching start cell, temporarily marking visited cells with
//! `#` so the path cannot revisit them.

pub fn solve(board: &[&str], word: &str) -> bool {
    if word.is_empty() {
        return true;
    }

    let mut grid: Vec<Vec<u8>> = board.iter().map(|row| row.as_bytes().to_vec()).collect();
    if grid.is_empty() {
        return false;
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let word = word.as_bytes();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == word[0] && traverse(&mut grid, i, j, word, 0) {
                return true;
            }
        }
    }

    false
}

fn traverse(grid: &mut [Vec<u8>], i: usize, j: usize, word: &[u8], position: usize) -> bool {
    if position == word.len() - 1 {
        return true;
    }

    let next = position + 1;
    let next_char = word[next];

    let original = grid[i][j];
    grid[i][j] = b'#';

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut found = false;

    if i + 1 < rows && grid[i + 1][j] == next_char {
        found = traverse(grid, i + 1, j, word, next);
    }
    if !found && j + 1 < cols && grid[i][j + 1] == next_char {
        found = traverse(grid, i, j + 1, word, next);
    }
    if !found && i >= 1 && grid[i - 1][j] == next_char {
        found = traverse(grid, i - 1, j, word, next);
    }
    if !found && j >= 1 && grid[i][j - 1] == next_char {
        found = traverse(grid, i, j - 1, word, next);
    }

    grid[i][j] = original;
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let board = ["ABCE", "SFCS", "ADEE"];
        assert!(solve(&board, "ABCCED"));
        assert!(solve(&board, "SEE"));
        assert!(!solve(&board, "ABCB"));

        assert!(solve(&["A"], "A"));
        assert!(!solve(&["A"], "AB"));
        assert!(solve(&["AB", "CD"], "ABDC"));
    }
}
