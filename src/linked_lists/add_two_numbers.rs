//! Add two numbers stored as linked lists, least-significant digit first.
//!
//! Walk both lists together, summing digit-by-digit with a carry, and build the
//! result in the same least-significant-first order.

use super::{Link, from_slice};

pub fn solve(l1: &Link, l2: &Link) -> Link {
    let mut a = l1;
    let mut b = l2;
    let mut carry = 0;
    let mut digits = Vec::new();

    while a.is_some() || b.is_some() || carry > 0 {
        let mut sum = carry;
        if let Some(node) = a {
            sum += node.val;
            a = &node.next;
        }
        if let Some(node) = b {
            sum += node.val;
            b = &node.next;
        }
        digits.push(sum % 10);
        carry = sum / 10;
    }

    from_slice(&digits)
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn examples() {
        let cases: &[(&[i32], &[i32], &[i32])] = &[
            (&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]),
            (&[0], &[0], &[0]),
            (
                &[9, 9, 9, 9, 9, 9, 9],
                &[9, 9, 9, 9],
                &[8, 9, 9, 9, 0, 0, 0, 1],
            ),
            (&[5], &[5], &[0, 1]),
            (&[1], &[9, 9], &[0, 0, 1]),
        ];

        for &(a, b, expected) in cases {
            let result = solve(&from_slice(a), &from_slice(b));
            assert_eq!(to_vec(&result), expected.to_vec());
        }
    }
}
