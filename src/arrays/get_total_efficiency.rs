//! Pair the smallest skill with the largest, second-smallest with
//! second-largest, and so on. Every pair must share the same sum; otherwise no
//! valid pairing exists (`-1`). Returns the sum of each pair's product.

pub fn get_total_efficiency(skill: &[i32]) -> i64 {
    if skill.len() < 2 {
        return 0;
    }

    let mut skill = skill.to_vec();
    skill.sort_unstable();

    let (mut lo, mut hi) = (0usize, skill.len() - 1);
    let target_sum = skill[lo] + skill[hi];
    let mut accumulated: i64 = 0;

    while lo < hi {
        if skill[lo] + skill[hi] != target_sum {
            return -1;
        }
        accumulated += skill[lo] as i64 * skill[hi] as i64;
        lo += 1;
        hi -= 1;
    }

    accumulated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_pairs_accumulate_products() {
        assert_eq!(get_total_efficiency(&[1, 2, 3, 2]), 7);
    }

    #[test]
    fn unbalanced_pairs_return_minus_one() {
        assert_eq!(get_total_efficiency(&[1, 1, 1, 2]), -1);
    }
}
