//! Remove duplicates from a sorted array in place.
//!
//! A slow/fast pointer pair compacts unique values to the front and returns the
//! new logical length; elements past that point are left as scratch.

pub fn remove_duplicates(array: &mut [i32]) -> usize {
    if array.is_empty() {
        return 0;
    }

    let mut write = 1;
    for read in 1..array.len() {
        if array[read - 1] != array[read] {
            array[write] = array[read];
            write += 1;
        }
    }

    write
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_single_duplicate() {
        let mut array = [1, 2, 2];
        let length = remove_duplicates(&mut array);
        assert_eq!(length, 2);
        assert_eq!(&array[..length], &[1, 2]);
    }

    #[test]
    fn removes_duplicate_keeping_tail() {
        let mut array = [1, 2, 2, 3];
        let length = remove_duplicates(&mut array);
        assert_eq!(length, 3);
        assert_eq!(&array[..length], &[1, 2, 3]);
    }

    #[test]
    fn empty_array_returns_zero() {
        let mut array: [i32; 0] = [];
        assert_eq!(remove_duplicates(&mut array), 0);
    }
}
