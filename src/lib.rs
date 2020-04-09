use std::cmp::Eq;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

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

pub fn has_dupes<T>(items: Vec<T>) -> bool
where
    T: Eq + PartialEq + Hash,
{
    // assign the number of items to the variable before moving items into from_iter()
    let item_len = items.len();
    let hashed: HashSet<T> = HashSet::from_iter(items);

    // if the set and the original vector are different lengths, return false
    hashed.len() == item_len
}

// Given a non-empty array, find an element that does not repeat itself.
pub fn get_unique<T>(mut items: Vec<T>) -> Vec<T>
where
    T: Eq + PartialEq + Hash + Clone,
{
    // preallocate space for one
    // one set will have length equal to the number of elements in the input items
    // the other will not be preallocated because we don't know beforehand
    // how many uniques elements of type T will be in the input item array
    let mut set_a: HashSet<T> = HashSet::with_capacity(items.len());
    let mut set_b: HashSet<T> = HashSet::new();
    // keep popping elements from the items vector
    // if the current element is in set_a, add it to set_b
    // otherwise, add the element to set_a. This will let us calculate the
    // the symmetric difference between the two sets and surface all unique elements
    while let Some(el) = items.pop() {
        match set_a.contains(&el) {
            true => set_b.insert(el),
            false => set_a.insert(el),
        };
    }

    // Gets the symmetric difference between two sets in constant time, collects and
    // returns the difference (elements without duplicates in items) into a vector
    set_a
        .symmetric_difference(&set_b)
        .cloned()
        .collect::<Vec<T>>()
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

    #[test]
    fn test_dupe_bool() {
        let test1 = vec![1, 1, 2, 3, 4, 5];
        let test2 = vec![0, 1, 2, 3, 4, 5];
        let test3 = vec![1, 2];
        let test4 = vec!["loop", "zoop", "goop", "loop"];
        let test5 = vec!["lol", "jimmy", "johnny"];

        assert!(!has_dupes(test1));
        assert!(has_dupes(test2));
        assert!(has_dupes(test3));
        assert!(!has_dupes(test4));
        assert!(has_dupes(test5));
    }

    #[test]
    fn test_unique() {
        let test1 = vec![1, 1, 1, 4, 4, 5];
        let test2 = vec![1, 1, 1, 1, 4];
        let test3 = vec![1, 1, 2, 2, 5, 6];
        let test4 = vec![1];
        let test5 = vec!["foo", "bar", "baz", "foo", "ding"];

        assert_eq!(vec![5], get_unique(test1));
        assert_eq!(vec![4], get_unique(test2));
        assert!(get_unique(test3.clone()).contains(&5) && get_unique(test3.clone()).contains(&6));
        assert!(!get_unique(test3.clone()).contains(&1) && !get_unique(test3.clone()).contains(&2));
        assert_eq!(vec![1], get_unique(test4));
        assert!(!get_unique(test5.clone()).contains(&"foo"));
        assert!(
            get_unique(test5.clone()).contains(&"bar")
                && get_unique(test5.clone()).contains(&"baz")
        );
    }
}
