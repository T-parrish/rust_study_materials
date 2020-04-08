use std::fmt;

pub fn remove_dupes(mut target: Vec<i32>) -> Vec<i32> {
    // start at 1 to current index to previous index
    // will make it easier to remove elements from the vector
    // since the results 'slide' to the left
    let mut i = 1;

    while i < target.len() {
        if &target[i] == &target[i - 1] {
            // remove the current index if it is == value at prev index
            target.remove(i);
            // continue for another iteration without incrementing i
            // since the remainder of the vector slides left
            continue;
        }
        // if it the current and previous values are not equal,
        // step into the next vector position
        i = i + 1;
    }
    target
}

// Takes an array of prices where prices[i] is the price on day i
// Assuming you can complete as many buy / sell ops as you'd like
// this function returns the maximum profit you can yield from the input
pub fn max_return(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        // if the price from the day before is less than the current price
        // we can add it to max_profit. Will eventually return
        // the max profit from the input array
        if &prices[i] > &prices[i - 1] {
            max_profit = max_profit + (prices[i] - prices[i - 1]);
        }
    }

    max_profit
}

pub fn rotate_array<T>(items: Vec<T>, rotations: u16) -> Vec<T>
where
    T: fmt::Display + fmt::Debug + Clone,
{
    // get the pivot point by getting the remainder of rotations % number 
    // of elements in vector to account for situations where there are
    // more rotations than elements in the vector
    let pivot = rotations % items.len() as u16;
    // Get the rotation index by pivoting from the right (last element) of array 
    let rot = items.len() - pivot as usize;
    
    // split at the rotation point
    let (left, right) = items.split_at(rot);

    // swap the right and left to yield the rotated array
    [right, left].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dupes() {
        let test1 = vec![1, 2, 3, 4, 5];
        let test2 = vec![1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 5];
        let test3 = vec![0, 0, 1, 2, 3, 4, 4, 6, 6];
        let test4 = vec![0];
        let test5 = vec![1, 1, 1, 1, 1, 1, 1, 1, 2];

        assert_eq!(vec![1, 2, 3, 4, 5], remove_dupes(test1));
        assert_eq!(vec![1, 2, 3, 4, 5], remove_dupes(test2));
        assert_eq!(vec![0, 1, 2, 3, 4, 6], remove_dupes(test3));
        assert_eq!(vec![0], remove_dupes(test4));
        assert_eq!(vec![1, 2], remove_dupes(test5));
    }

    #[test]
    fn test_profits() {
        let test1 = vec![7, 1, 5, 3, 6, 4];
        let test2 = vec![1, 2, 3, 4, 5];
        let test3 = vec![7, 6, 4, 3, 1];
        let test4 = vec![1, 7, 3, 4, 6, 9];

        assert_eq!(max_return(test1), 7);
        assert_eq!(max_return(test2), 4);
        assert_eq!(max_return(test3), 0);
        assert_eq!(max_return(test4), 12);
    }

    #[test]
    fn test_rotation() {
        let test1 = (vec![1, 2, 3, 4, 5], 3);
        let test2 = (vec![1, 2, 3, 4, 5], 2);
        let test3 = (vec![1, 2, 3, 4, 5], 4);
        let test4 = (vec![1], 2);

        assert_eq!(rotate_array(test1.0, test1.1), vec![3, 4, 5, 1, 2]);
        assert_eq!(rotate_array(test2.0, test2.1), vec![4, 5, 1, 2, 3]);
        assert_eq!(rotate_array(test3.0, test3.1), vec![2, 3, 4, 5, 1]);
        assert_eq!(rotate_array(test4.0, test4.1), vec![1]);
    }
}
