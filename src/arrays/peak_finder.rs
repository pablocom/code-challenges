//! Count "peaks": positions where a reading jumps by at least 5 from the
//! previous one. A counted peak suppresses the immediately following sample so
//! a single spike is not double-counted.

pub fn count_peaks(values: &[f64]) -> i32 {
    if values.len() <= 2 {
        return 0;
    }

    let mut total = 0;
    let mut previous_was_peak = false;

    for window in values.windows(2) {
        if previous_was_peak {
            previous_was_peak = false;
            continue;
        }

        if (window[1] - window[0]).abs() >= 5.0 {
            total += 1;
            previous_was_peak = true;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_spikes() {
        let measures = [8.0, 10.7, 17.1, 11.2, 13.5, 9.9, 14.9, 9.4, 9.4, 3.1, 12.7];
        assert_eq!(count_peaks(&measures), 3);
    }
}
