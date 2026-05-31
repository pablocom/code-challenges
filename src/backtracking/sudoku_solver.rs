//! Sudoku solver, two ways.
//!
//! - [`solve`] uses bitmask candidate sets plus the minimum-remaining-values
//!   heuristic (always fill the most-constrained cell next) for speed.
//! - [`solve_naive`] is the textbook cell-by-cell backtracker.
//!
//! Boards are `9 x 9` grids of `'1'..='9'` with `'.'` for blanks, solved in
//! place.

const BLANK: char = '.';

/// Bitmask + minimum-remaining-values backtracking.
pub fn solve(board: &mut [Vec<char>]) {
    let mut row_masks = [0u16; 9];
    let mut col_masks = [0u16; 9];
    let mut box_masks = [0u16; 9];
    let mut empty_cells: Vec<usize> = Vec::new();

    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == BLANK {
                empty_cells.push(r * 9 + c);
            } else {
                let bit = 1u16 << (board[r][c] as u8 - b'0');
                row_masks[r] |= bit;
                col_masks[c] |= bit;
                box_masks[box_index(r, c)] |= bit;
            }
        }
    }

    let count = empty_cells.len();
    backtrack(
        board,
        &mut empty_cells,
        count,
        &mut row_masks,
        &mut col_masks,
        &mut box_masks,
    );
}

fn box_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}

fn backtrack(
    board: &mut [Vec<char>],
    empty_cells: &mut [usize],
    empty_count: usize,
    row_masks: &mut [u16; 9],
    col_masks: &mut [u16; 9],
    box_masks: &mut [u16; 9],
) -> bool {
    if empty_count == 0 {
        return true;
    }

    // Pick the empty cell with the fewest candidates (MRV).
    let mut best_idx = 0;
    let mut min_options = 10;
    let mut best_mask = 0u16;
    for i in 0..empty_count {
        let cell = empty_cells[i];
        let (r, c) = (cell / 9, cell % 9);
        let used = row_masks[r] | col_masks[c] | box_masks[box_index(r, c)];
        let available = !used & 0x3FE; // bits 1..=9
        let options = available.count_ones();

        if options == 0 {
            return false;
        }
        if options >= min_options {
            continue;
        }
        min_options = options;
        best_idx = i;
        best_mask = available;
        if options == 1 {
            break;
        }
    }

    let cell = empty_cells[best_idx];
    let (r, c) = (cell / 9, cell % 9);
    let b = box_index(r, c);

    empty_cells.swap(best_idx, empty_count - 1);

    for num in 1..=9u16 {
        let bit = 1u16 << num;
        if best_mask & bit == 0 {
            continue;
        }

        board[r][c] = (b'0' + num as u8) as char;
        row_masks[r] |= bit;
        col_masks[c] |= bit;
        box_masks[b] |= bit;

        if backtrack(
            board,
            empty_cells,
            empty_count - 1,
            row_masks,
            col_masks,
            box_masks,
        ) {
            return true;
        }

        board[r][c] = BLANK;
        row_masks[r] &= !bit;
        col_masks[c] &= !bit;
        box_masks[b] &= !bit;
    }

    empty_cells.swap(best_idx, empty_count - 1);
    false
}

/// Straightforward cell-by-cell backtracking.
pub fn solve_naive(board: &mut [Vec<char>]) {
    backtrack_naive(board, 0, 0);
}

fn backtrack_naive(board: &mut [Vec<char>], row: usize, col: usize) -> bool {
    if row == 9 {
        return true;
    }

    let (next_row, next_col) = if col == 8 {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    if board[row][col] != BLANK {
        return backtrack_naive(board, next_row, next_col);
    }

    for num in b'1'..=b'9' {
        let candidate = num as char;
        if !is_valid(board, row, col, candidate) {
            continue;
        }

        board[row][col] = candidate;
        if backtrack_naive(board, next_row, next_col) {
            return true;
        }
        board[row][col] = BLANK;
    }

    false
}

fn is_valid(board: &[Vec<char>], row: usize, col: usize, num: char) -> bool {
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    let (box_row, box_col) = ((row / 3) * 3, (col / 3) * 3);
    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if board[r][c] == num {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE: [&str; 9] = [
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ];

    const SOLUTION: [&str; 9] = [
        "534678912",
        "672195348",
        "198342567",
        "859761423",
        "426853791",
        "713924856",
        "961537284",
        "287419635",
        "345286179",
    ];

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn bitmask_solver_finds_the_unique_solution() {
        let mut grid = board(&PUZZLE);
        solve(&mut grid);
        assert_eq!(grid, board(&SOLUTION));
    }

    #[test]
    fn naive_solver_finds_the_unique_solution() {
        let mut grid = board(&PUZZLE);
        solve_naive(&mut grid);
        assert_eq!(grid, board(&SOLUTION));
    }
}
