//! Course Schedule — can all courses be finished given `[course, prereq]`
//! dependencies? Equivalent to "is the dependency graph acyclic?".
//!
//! Depth-first search with three colours: unvisited, in the current path, and
//! fully explored. Re-encountering an in-path node means a cycle.

const UNVISITED: u8 = 0;
const IN_PATH: u8 = 1;
const DONE: u8 = 2;

pub fn solve(num_courses: usize, prerequisites: &[[i32; 2]]) -> bool {
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); num_courses];
    for prereq in prerequisites {
        adjacency[prereq[0] as usize].push(prereq[1] as usize);
    }

    let mut state = vec![UNVISITED; num_courses];
    for course in 0..num_courses {
        if has_cycle(course, &adjacency, &mut state) {
            return false;
        }
    }
    true
}

fn has_cycle(course: usize, adjacency: &[Vec<usize>], state: &mut [u8]) -> bool {
    match state[course] {
        DONE => return false,
        IN_PATH => return true,
        _ => {}
    }

    state[course] = IN_PATH;
    for &next in &adjacency[course] {
        if has_cycle(next, adjacency, state) {
            return true;
        }
    }
    state[course] = DONE;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acyclic_schedules_can_finish() {
        assert!(solve(1, &[]));
        assert!(solve(2, &[[1, 0]]));
        assert!(solve(5, &[]));
        assert!(solve(4, &[[1, 0], [2, 1], [3, 2]]));
        assert!(solve(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]));
        assert!(solve(4, &[[1, 0], [3, 2]]));
        assert!(solve(4, &[[1, 0], [2, 0], [3, 0]]));
        assert!(solve(6, &[[5, 0], [5, 4], [4, 1], [3, 2], [2, 1]]));
    }

    #[test]
    fn cyclic_schedules_cannot_finish() {
        assert!(!solve(2, &[[0, 1], [1, 0]]));
        assert!(!solve(3, &[[1, 0], [2, 1], [0, 2]]));
        assert!(!solve(1, &[[0, 0]]));
        assert!(!solve(3, &[[0, 1], [1, 0]]));
        assert!(!solve(5, &[[1, 0], [2, 1], [3, 2], [1, 3]]));
        assert!(!solve(3, &[[0, 1], [1, 0], [2, 0]]));
    }
}
