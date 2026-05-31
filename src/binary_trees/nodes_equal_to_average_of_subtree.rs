//! Count nodes whose value equals the integer average of their subtree.
//!
//! One post-order pass returns each subtree's `(sum, count)` so a parent can
//! compute its own average in O(1).

use super::Tree;

pub fn solve(root: &Tree) -> i32 {
    let mut matches = 0;
    subtree_totals(root, &mut matches);
    matches
}

/// Returns `(sum, node_count)` of the subtree, incrementing `matches` for every
/// node equal to its subtree's integer average.
fn subtree_totals(link: &Tree, matches: &mut i32) -> (i32, i32) {
    let Some(node) = link else {
        return (0, 0);
    };

    let (left_sum, left_count) = subtree_totals(&node.left, matches);
    let (right_sum, right_count) = subtree_totals(&node.right, matches);

    let sum = left_sum + right_sum + node.val;
    let count = left_count + right_count + 1;

    if node.val == sum / count {
        *matches += 1;
    }

    (sum, count)
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn single_node_matches_itself() {
        assert_eq!(solve(&node(4).build()), 1);
    }

    #[test]
    fn full_tree_counts_all_matching_nodes() {
        let root = node(4)
            .left(node(8).left(node(0)).right(node(1)))
            .right(node(5).right(node(6)))
            .build();
        assert_eq!(solve(&root), 5);
    }

    #[test]
    fn three_node_tree_only_leaves_match() {
        let root = node(1).left(node(2)).right(node(3)).build();
        assert_eq!(solve(&root), 2);
    }

    #[test]
    fn unbalanced_tree_only_leaf_matches() {
        let root = node(10).left(node(3)).build();
        assert_eq!(solve(&root), 1);
    }
}
