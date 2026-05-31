//! Level-order (breadth-first) traversal, grouped by depth.

use super::Tree;
use std::collections::VecDeque;

pub fn level_order(root: &Tree) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut queue: VecDeque<&Tree> = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }

    while !queue.is_empty() {
        let mut level = Vec::with_capacity(queue.len());
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap().as_ref().unwrap();
            level.push(node.val);
            if node.left.is_some() {
                queue.push_back(&node.left);
            }
            if node.right.is_some() {
                queue.push_back(&node.right);
            }
        }
        result.push(level);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn three_level_tree() {
        let root = node(3)
            .left(node(9))
            .right(node(20).left(node(15)).right(node(7)))
            .build();
        assert_eq!(level_order(&root), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn single_node() {
        assert_eq!(level_order(&node(1).build()), vec![vec![1]]);
    }

    #[test]
    fn left_skewed_tree() {
        let root = node(1).left(node(2).left(node(3))).build();
        assert_eq!(level_order(&root), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn complete_binary_tree() {
        let root = node(1)
            .left(node(2).left(node(4)).right(node(5)))
            .right(node(3).left(node(6)).right(node(7)))
            .build();
        assert_eq!(
            level_order(&root),
            vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]
        );
    }
}
