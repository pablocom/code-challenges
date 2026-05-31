//! Valid Mountain Array — strictly up to a single peak, then strictly down.

pub fn solve(arr: &[i32]) -> bool {
    if arr.len() < 3 {
        return false;
    }
    let last = arr.len() - 1;

    let peak = ascend_from(arr, 0);
    let is_interior_peak = peak != 0 && peak != last;

    is_interior_peak && descend_from(arr, peak) == last
}

fn ascend_from(arr: &[i32], mut i: usize) -> usize {
    while i + 1 < arr.len() && arr[i] < arr[i + 1] {
        i += 1;
    }
    i
}

fn descend_from(arr: &[i32], mut i: usize) -> usize {
    while i + 1 < arr.len() && arr[i] > arr[i + 1] {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(!solve(&[2, 1]));
        assert!(!solve(&[3, 5, 5]));
        assert!(solve(&[0, 3, 2, 1]));
        assert!(solve(&[0, 1, 2, 3, 4, 5, 4, 3, 2, 1]));
        assert!(!solve(&[1, 2, 3, 4, 5]));
        assert!(!solve(&[5, 4, 3, 2, 1]));
        assert!(solve(&[1, 3, 2]));
        assert!(!solve(&[1, 2, 2, 1]));
        assert!(!solve(&[1]));
    }
}
