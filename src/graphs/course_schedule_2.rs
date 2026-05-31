//! Course Schedule II — return a valid order to take all courses, or an empty
//! vec if a cycle makes that impossible.
//!
//! Depth-first topological sort: a course is appended only after all of its
//! prerequisites, so prerequisites land earlier in the output.

const UNVISITED: u8 = 0;
const IN_PATH: u8 = 1;
const DONE: u8 = 2;

pub fn solve(courses: usize, prerequisites: &[[i32; 2]]) -> Vec<usize> {
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); courses];
    for prereq in prerequisites {
        adjacency[prereq[0] as usize].push(prereq[1] as usize);
    }

    let mut state = vec![UNVISITED; courses];
    let mut order = Vec::with_capacity(courses);

    for course in 0..courses {
        if visit(course, &adjacency, &mut state, &mut order) {
            return Vec::new(); // cycle: no valid ordering
        }
    }

    order
}

/// Returns `true` if a cycle is detected.
fn visit(
    course: usize,
    adjacency: &[Vec<usize>],
    state: &mut [u8],
    order: &mut Vec<usize>,
) -> bool {
    match state[course] {
        IN_PATH => return true,
        DONE => return false,
        _ => {}
    }

    state[course] = IN_PATH;
    for &prereq in &adjacency[course] {
        if visit(prereq, adjacency, state, order) {
            return true;
        }
    }
    state[course] = DONE;
    order.push(course);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_topological_order(
        order: &[usize],
        courses: usize,
        prerequisites: &[[i32; 2]],
    ) -> bool {
        if order.len() != courses {
            return false;
        }
        let mut position = vec![usize::MAX; courses];
        for (i, &course) in order.iter().enumerate() {
            position[course] = i;
        }
        prerequisites
            .iter()
            .all(|p| position[p[1] as usize] < position[p[0] as usize])
    }

    #[test]
    fn exact_orderings() {
        assert_eq!(solve(1, &[]), vec![0]);
        assert_eq!(solve(2, &[[1, 0]]), vec![0, 1]);
        assert_eq!(solve(2, &[[0, 1]]), vec![1, 0]);
        assert_eq!(solve(4, &[[1, 0], [2, 1], [3, 2]]), vec![0, 1, 2, 3]);
        assert_eq!(solve(4, &[[0, 1], [1, 2], [2, 3]]), vec![3, 2, 1, 0]);
    }

    #[test]
    fn independent_courses_returns_all() {
        let order = solve(5, &[]);
        assert_eq!(order.len(), 5);
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn valid_orderings_for_branching_graphs() {
        let cases: &[(usize, &[[i32; 2]])] = &[
            (4, &[[1, 0], [2, 0], [3, 1], [3, 2]]),
            (4, &[[1, 0], [3, 2]]),
            (4, &[[1, 0], [2, 0], [3, 0]]),
            (6, &[[5, 0], [5, 4], [4, 1], [3, 2], [2, 1]]),
            (5, &[[4, 0], [4, 1], [4, 2], [4, 3]]),
        ];
        for &(courses, prerequisites) in cases {
            let order = solve(courses, prerequisites);
            assert!(is_valid_topological_order(&order, courses, prerequisites));
        }
    }

    #[test]
    fn cycles_return_empty() {
        assert!(solve(2, &[[0, 1], [1, 0]]).is_empty());
        assert!(solve(3, &[[1, 0], [2, 1], [0, 2]]).is_empty());
        assert!(solve(1, &[[0, 0]]).is_empty());
        assert!(solve(5, &[[1, 0], [2, 1], [3, 2], [1, 3]]).is_empty());
        assert!(solve(3, &[[0, 1], [1, 0], [2, 0]]).is_empty());
    }
}
