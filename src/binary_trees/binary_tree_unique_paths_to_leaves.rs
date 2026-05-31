//! All root-to-leaf paths, rendered as `"a->b->c"` strings.
//!
//! A single mutable `String` is grown on the way down and truncated back on the
//! way up, so each path is built without re-allocating per node.

use super::Tree;

pub fn solve(root: &Tree) -> Vec<String> {
    let mut result = Vec::new();
    let mut path = String::new();
    traverse(root, &mut path, &mut result);
    result
}

fn traverse(link: &Tree, path: &mut String, result: &mut Vec<String>) {
    let Some(node) = link else {
        return;
    };

    let len_before = path.len();
    if !path.is_empty() {
        path.push_str("->");
    }
    path.push_str(&node.val.to_string());

    if node.left.is_none() && node.right.is_none() {
        result.push(path.clone());
    }

    traverse(&node.left, path, result);
    traverse(&node.right, path, result);

    path.truncate(len_before);
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn single_node() {
        assert_eq!(solve(&node(5).build()), vec!["5"]);
    }

    #[test]
    fn two_leaves() {
        let root = node(1).left(node(2)).right(node(3)).build();
        assert_eq!(solve(&root), vec!["1->2", "1->3"]);
    }

    #[test]
    fn left_only_chain() {
        let root = node(1).left(node(2).left(node(3))).build();
        assert_eq!(solve(&root), vec!["1->2->3"]);
    }

    #[test]
    fn asymmetric_tree() {
        let root = node(1).left(node(2).right(node(5))).right(node(3)).build();
        assert_eq!(solve(&root), vec!["1->2->5", "1->3"]);
    }

    #[test]
    fn full_three_level_tree() {
        let root = node(1)
            .left(node(2).left(node(4)).right(node(5)))
            .right(node(3).right(node(6)))
            .build();
        assert_eq!(solve(&root), vec!["1->2->4", "1->2->5", "1->3->6"]);
    }
}
