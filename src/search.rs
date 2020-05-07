use std::cmp::Ordering;

// Binary search function for sorted vector of integers.
// If the vector is not sorted, this will not work.
pub fn binary_search(arr: Vec<i32>, mut left: u32, mut right: u32, target: i32) -> Option<usize> {
    while left <= right {
        let mid_idx = left + (right - left) / 2;

        let curr_val = arr.get(mid_idx as usize).unwrap();

        match curr_val.cmp(&target) {
            Ordering::Less => left = mid_idx + 1,
            Ordering::Greater => right = mid_idx - 1,
            Ordering::Equal => return Some(mid_idx as usize),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let test1 = vec![1, 3, 5, 7, 9, 12, 22, 500, 7000];
        let right_pointer = (test1.len() - 1) as u32;

        assert_eq!(binary_search(test1.clone(), 0, right_pointer, 5), Some(2));
        assert_eq!(binary_search(test1.clone(), 0, right_pointer, 1), Some(0));
        assert_eq!(
            binary_search(test1.clone(), 0, right_pointer, 7000),
            Some(8)
        );
        assert_eq!(binary_search(test1.clone(), 0, right_pointer, 17), None);
        assert_eq!(binary_search(test1.clone(), 0, right_pointer, 23), None);
    }
}
