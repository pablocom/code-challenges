//! Maximize protected population. `unit` is a `'0'`/`'1'` mask of protected
//! positions; the protected total is the baseline. Each "`0` immediately before
//! a run of `1`s" lets the `0`'s population replace the weakest `1` in that run
//! if doing so is a net gain.

pub fn solve(population: &[i32], unit: &str) -> i32 {
    let unit = unit.as_bytes();
    let mut max_population = sum_protected(unit, population);

    for i in 1..unit.len() {
        if unit[i] != b'1' || unit[i - 1] != b'0' {
            continue;
        }

        let mut weakest_in_run = population[i];
        let mut j = i + 1;
        while j < unit.len() && unit[j] == b'1' {
            weakest_in_run = weakest_in_run.min(population[j]);
            j += 1;
        }

        let gain = population[i - 1] - weakest_in_run;
        if gain > 0 {
            max_population += gain;
        }
    }

    max_population
}

fn sum_protected(unit: &[u8], population: &[i32]) -> i32 {
    unit.iter()
        .zip(population)
        .filter(|&(&mask, _)| mask == b'1')
        .map(|(_, &pop)| pop)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[10, 5, 8, 9, 6], "01101"), 27);
        assert_eq!(solve(&[20, 10, 9, 30, 20, 19], "011011"), 80);
        assert_eq!(solve(&[5, 4, 5, 1], "0111"), 14);
        assert_eq!(
            solve(&[20, 10, 9, 30, 20, 19, 3, 42, 54, 66], "0110110110"),
            176
        );
    }
}
