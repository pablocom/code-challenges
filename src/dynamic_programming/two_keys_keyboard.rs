//! Two Keys Keyboard — fewest Copy-All / Paste operations to grow a single
//! 'A' into `n` copies. Three takes on the same problem.

use std::collections::HashMap;

/// The closed form: the answer is the sum of `n`'s prime factors.
pub fn solve_with_magic(mut n: i32) -> i32 {
    let mut result = 0;
    let mut factor = 2;

    while factor <= n {
        while n % factor == 0 {
            result += factor;
            n /= factor;
        }
        factor += 1;
    }

    result
}

/// Bottom-up over divisors: reaching `i` costs the best `dp[j] + i / j` for any
/// divisor `j` of `i`.
pub fn solve_with_factors(n: i32) -> i32 {
    let size = (n + 1) as usize;
    let mut dp = vec![1000; size];
    dp[1] = 0;

    for i in 2..=n {
        for j in 1..=i / 2 {
            if i % j == 0 {
                dp[i as usize] = dp[i as usize].min(dp[j as usize] + i / j);
            }
        }
    }

    dp[n as usize]
}

/// Memoized DFS over `(current_count, clipboard_size)` states.
pub fn solve_with_dfs(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
    1 + helper(1, 1, n, &mut cache)
}

fn helper(count: i32, clipboard: i32, target: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
    const INFINITE: i32 = i32::MAX - 1001;

    if count == target {
        return 0;
    }
    if count > target {
        return INFINITE;
    }
    if let Some(&cached) = cache.get(&(count, clipboard)) {
        return cached;
    }

    let copying = 1 + helper(count + clipboard, clipboard, target, cache);
    let copy_pasting = 2 + helper(count + count, count, target, cache);

    let best = copying.min(copy_pasting);
    cache.insert((count, clipboard), best);
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(i32, i32)] = &[
        (1, 0),
        (2, 2),
        (3, 3),
        (4, 4),
        (6, 5),
        (9, 6),
        (12, 7),
        (37, 37),
    ];

    #[test]
    fn with_magic() {
        for &(n, expected) in SCENARIOS {
            assert_eq!(solve_with_magic(n), expected);
        }
    }

    #[test]
    fn with_factors() {
        for &(n, expected) in SCENARIOS {
            assert_eq!(solve_with_factors(n), expected);
        }
    }

    #[test]
    fn with_dfs() {
        for &(n, expected) in SCENARIOS {
            assert_eq!(solve_with_dfs(n), expected);
        }
    }
}
