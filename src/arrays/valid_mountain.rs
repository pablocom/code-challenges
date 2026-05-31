//! Valid Mountain Array — strictly up to a single peak, then strictly down.

pub fn solve(arr: &[i32]) -> bool {
    let n = arr.len();
    let mut i = 0;

    // Climb.
    while i + 1 < n && arr[i] < arr[i + 1] {
        i += 1;
    }

    // The peak cannot be the first or last element.
    if i == 0 || i == n - 1 {
        return false;
    }

    // Descend.
    while i + 1 < n && arr[i] > arr[i + 1] {
        i += 1;
    }

    i == n - 1
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
