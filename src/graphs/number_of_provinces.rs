//! Number of Provinces — connected components in an adjacency matrix where
//! `is_connected[i][j] == 1` means cities `i` and `j` are directly linked.

pub fn solve(is_connected: &[Vec<i32>]) -> i32 {
    let n = is_connected.len();
    let mut visited = vec![false; n];
    let mut provinces = 0;

    for city in 0..n {
        if !visited[city] {
            explore(city, is_connected, &mut visited);
            provinces += 1;
        }
    }

    provinces
}

fn explore(city: usize, is_connected: &[Vec<i32>], visited: &mut [bool]) {
    visited[city] = true;
    for other in 0..is_connected[city].len() {
        if other != city && !visited[other] && is_connected[city][other] == 1 {
            explore(other, is_connected, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(rows: &[&[i32]]) -> Vec<Vec<i32>> {
        rows.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn examples() {
        assert_eq!(solve(&matrix(&[&[1, 1, 0], &[1, 1, 0], &[0, 0, 1]])), 2);
        assert_eq!(solve(&matrix(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]])), 3);
        assert_eq!(solve(&matrix(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]])), 1);
        assert_eq!(solve(&matrix(&[&[1]])), 1);
        assert_eq!(
            solve(&matrix(&[
                &[1, 1, 0, 0],
                &[1, 1, 1, 0],
                &[0, 1, 1, 0],
                &[0, 0, 0, 1]
            ])),
            2
        );
    }
}
