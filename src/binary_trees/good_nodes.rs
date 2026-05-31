//! Count "good" nodes — those with no strictly greater ancestor on their path
//! from the root.

use super::Tree;

pub fn good_nodes(root: &Tree) -> i32 {
    fn go(link: &Tree, max_so_far: i64) -> i32 {
        match link {
            None => 0,
            Some(node) => {
                let value = node.val as i64;
                let is_good = i32::from(value >= max_so_far);
                let max_so_far = max_so_far.max(value);
                is_good + go(&node.left, max_so_far) + go(&node.right, max_so_far)
            }
        }
    }

    go(root, i64::MIN)
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn examples() {
        let tree1 = node(3)
            .left(node(1).left(node(3)))
            .right(node(4).left(node(1)).right(node(5)))
            .build();
        assert_eq!(good_nodes(&tree1), 4);

        let tree2 = node(3).left(node(3).left(node(4)).right(node(2))).build();
        assert_eq!(good_nodes(&tree2), 3);

        let tree3 = node(3).left(node(4).left(node(3))).build();
        assert_eq!(good_nodes(&tree3), 2);
    }
}
