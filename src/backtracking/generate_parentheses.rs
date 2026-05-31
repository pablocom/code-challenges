//! Generate all combinations of `n` well-formed parentheses pairs.
//!
//! Add a `(` whenever opens remain, and a `)` only while it would stay
//! balanced. Exploring opens first yields the canonical ordering.

pub fn generate_parenthesis(n: u32) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    generate(n, &mut current, 0, 0, &mut result);
    result
}

fn generate(n: u32, current: &mut String, open: u32, close: u32, result: &mut Vec<String>) {
    if open == n && close == n {
        result.push(current.clone());
        return;
    }

    if open < n {
        current.push('(');
        generate(n, current, open + 1, close, result);
        current.pop();
    }

    if close < open {
        current.push(')');
        generate(n, current, open, close + 1, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }
}
