use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
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

// Computes the intersection of two vectors
pub fn get_intersection<T>(vec_a: Vec<T>, vec_b: Vec<T>) -> Vec<T>
where
    T: Eq + PartialEq + Hash + Clone,
{
    // consume vec_a and vec_b and store their respective hash set representations
    let set_a: HashSet<T> = HashSet::from_iter(vec_a);
    let set_b: HashSet<T> = HashSet::from_iter(vec_b);

    // Calculate the interesection and collect into a vector with elements of Type T
    set_a.intersection(&set_b).cloned().collect::<Vec<T>>()
}

// takes an input array of unsigned integers
// returns the array of unsigned integers + 1
// eg. [1, 3, 4, 5] -> [1, 3, 4, 6] or [1, 2, 6, 9] -> [1, 2, 7, 0]
pub fn add_one(mut input: Vec<u32>) -> Vec<u32> {
    let mut temp = String::new();
    while let Some(int) = input.pop() {
        temp.push_str(&int.to_string());
    }

    // Reverse the temp chars and collect into a string that we parse into a u32 int and add 1
    let plus_one = temp
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
        + 1;

    // cast the u32 int as a string after the 1 has been added, map over the characters
    // to return a new vector where each element is typed as a u32 int
    plus_one
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32)
        .collect()
}

// Sorts the array in place using a cached key for performance
// returns the same array with all zeroes moved to the end of the vector
pub fn zeroes_in_back(mut input: Vec<i32>) -> Vec<i32> {
    input.sort_by_cached_key(|k| k == &0);
    input
}

// Return indices of the two numbers in a list such that they add up to a specific target.
// You may not use the same integer more than once
// Runs in Quadratic time O(n^2)
pub fn add_to_target_brute(input: Vec<i32>, target: i32) -> Vec<(usize, usize)> {
    // store the indices that add up to the target in a hash set for O(1) lookup
    let mut found: HashSet<usize> = HashSet::new();
    // store the index pairs that add to the target in a vector of tuples
    let mut output: Vec<(usize, usize)> = Vec::new();

    for i in 0..(input.len() - 1) {
        if found.contains(&i) {
            continue;
        }
        for j in (i + 1)..input.len() {
            if found.contains(&j) || found.contains(&i) {
                continue;
            }
            match (input[i] + input[j]) == target {
                true => {
                    found.insert(i);
                    found.insert(j);
                    output.push((i, j));
                }
                false => continue,
            }
        }
    }

    output
}

// finds a single pair of indices that add up to the target
// This solution runs in O(2N) time with O(2N) space
pub fn add_to_target_mapped(input: Vec<i32>, target: i32) -> Vec<(usize, usize)> {
    let mut hashed: HashMap<i32, usize> = HashMap::new();
    let mut found: HashSet<usize> = HashSet::new();
    let mut output: Vec<(usize, usize)> = Vec::new();
    // Stores the input vector elements in a hashmap where
    // the key is the element and the value is the index
    for (i, t) in input.iter().enumerate() {
        hashed.insert(*t, i);
    }

    for i in 0..input.len() {
        let temp = &target - input[i];
        let index = match hashed.get(&temp) {
            Some(t) => t,
            None => continue,
        };

        // must add two separate indices, and only add indices
        // that haven't been added previously
        if i != *index && !found.contains(index) && !found.contains(&i) {
            output.push((i, *index));
            found.insert(*index);
        }
    }

    output
}

// Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated.
// A Sudoku board (partially filled) could be valid but is not necessarily solvable.
pub fn is_valid_sudoku(input: Vec<Vec<&str>>) -> bool {
    fn unroll_data(data: &[&str]) -> bool {
        let mut hashed: HashSet<u8> = HashSet::new();
        for x in data {
            // parse the vector into u8 (0 - 9) integers
            let num_rep = match x.parse::<u8>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // if the hash set contains the number, the sudoku is invalid
            if hashed.contains(&num_rep) {
                return false;
            } else {
                hashed.insert(num_rep);
            }
        }

        true
    }

    fn test_square(input: &[Vec<&str>], start: usize, window: usize) -> bool {
        let mut flat: Vec<&str> = Vec::new();

        for i in start..window {
            for j in start..window {
                flat.push(input[i][j]);
            }
        }
        unroll_data(&flat[..]) // return true if the square is valid
    }

    fn test_column(input: &[Vec<&str>], col: usize) -> bool {
        let mut flat: Vec<&str> = Vec::new();
        // for each row in the input matrix
        for i in 0..input.len() {
            flat.push(input[i][col]);
        }

        unroll_data(&flat[..])
    }

    // test the rows
    for row in 0..input.len() {
        match unroll_data(&input[row]) {
            true => continue,
            false => return false,
        }
    }

    // test the columns
    for col in 0..input.len() {
        match test_column(&input, col) {
            true => continue,
            false => return false,
        }
    }

    // test the sub-squares
    let chunk_size = input.len() / 3;
    let mut window = chunk_size;
    let mut start = 0;

    while window < input.len() {
        if test_square(&input, start, window) == false {
            return false;
        }

        window += chunk_size;
        start += chunk_size;
    }

    // return true if all tests pass
    true
}

// Given n non-negative integers representing an elevation map where
// the width of each bar is 1, compute how much water it is able to trap after raining.
pub fn rain_catcher(height: Vec<u32>) -> u32 {
    let mut left_tower_height: u32 = 0;
    let mut right_tower_height: u32 = 0;
    let mut water: u32 = 0;

    let mut lp: usize = 0; // left pointer
    let mut rp = height.len()-1; // right pointer

    while lp < rp {
        if height[lp] < height[rp] {
            if height[lp] > left_tower_height {
                left_tower_height = height[lp];
            } else {
                water += left_tower_height - height[lp];
            }
            lp += 1
        } else {
            if height[rp] > right_tower_height {
                right_tower_height = height[rp];
            } else {
                water += right_tower_height - height[rp];
            }
            rp -= 1
        }
    }

    water
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

    #[test]
    fn test_intersection() {
        let test1 = (vec![1, 2, 3], vec![1, 3, 4]);
        let test2 = (vec![1, 1, 2], vec![1, 2, 3]);
        let test3 = (vec![1, 1, 1], vec![2]);

        assert!(get_intersection(test1.0, test1.1).contains(&1));

        let (a, b) = test2.clone();
        assert!(get_intersection(a, b).contains(&1));

        let (a, b) = test2.clone();
        assert!(get_intersection(a, b).contains(&2));

        assert!(!get_intersection(test3.0, test3.1).contains(&1));
    }

    #[test]
    fn test_add_one() {
        let test1 = vec![1, 2, 3, 4];
        let test2 = vec![1, 2, 2, 9];
        let test3 = vec![1, 9, 9, 9];
        let test4 = vec![9];

        assert_eq!(vec![1, 2, 3, 5], add_one(test1));
        assert_eq!(vec![1, 2, 3, 0], add_one(test2));
        assert_eq!(vec![2, 0, 0, 0], add_one(test3));
        assert_eq!(vec![1, 0], add_one(test4));
    }

    #[test]
    fn test_ending_zeroes() {
        let test1 = vec![1, 0, 0, 4];
        let test2 = vec![1, 0, 0, 1, 0];
        let test3 = vec![1, 0, 0, 4, 0];
        let test4 = vec![0, 0, 0, 4, 0];
        let test5 = vec![0];

        assert_eq!(vec![1, 4, 0, 0], zeroes_in_back(test1));
        assert_eq!(vec![1, 1, 0, 0, 0], zeroes_in_back(test2));
        assert_eq!(vec![1, 4, 0, 0, 0], zeroes_in_back(test3));
        assert_eq!(vec![4, 0, 0, 0, 0], zeroes_in_back(test4));
        assert_eq!(vec![0], zeroes_in_back(test5));
    }

    #[test]
    fn test_add_to_target() {
        let test1 = (vec![1, 0, 0, 4], 5);
        let test2 = (vec![1, 4, 1, 4], 5);
        let test3 = (vec![1, 2, 3, 4], 5);
        let test4 = (vec![0, 0, 0, 7], 7);
        let test5 = (vec![1, 3, 2, 2], 4);

        assert_eq!(add_to_target_brute(test1.0, test1.1), vec![(0, 3)]);
        assert_eq!(add_to_target_brute(test2.0, test2.1), vec![(0, 1), (2, 3)]);
        assert_eq!(add_to_target_brute(test3.0, test3.1), vec![(0, 3), (1, 2)]);
        assert_eq!(add_to_target_brute(test4.0, test4.1), vec![(0, 3)]);
        assert_eq!(add_to_target_brute(test5.0, test5.1), vec![(0, 1), (2, 3)]);
    }

    #[test]
    fn test_mapped_add_to_target() {
        let test1 = (vec![1, 0, 0, 4], 5);
        let test2 = (vec![1, 4, 2, 3], 5);
        let test3 = (vec![1, 2, 3, 4], 5);
        let test4 = (vec![0, 0, 0, 7], 7);
        let test5 = (vec![1, 3, 2, 2], 4);

        assert_eq!(add_to_target_mapped(test1.0, test1.1), vec![(0, 3)]);
        assert_eq!(add_to_target_mapped(test2.0, test2.1), vec![(0, 1), (2, 3)]);
        assert_eq!(add_to_target_mapped(test3.0, test3.1), vec![(0, 3), (1, 2)]);
        assert_eq!(add_to_target_mapped(test4.0, test4.1), vec![(0, 3)]);
        assert_eq!(add_to_target_mapped(test5.0, test5.1), vec![(0, 1), (2, 3)]);
    }

    #[test]
    fn test_sudoku_validator() {
        let test1 = vec![
            vec!["8", "3", ".", ".", "7", ".", ".", ".", "."],
            vec!["6", ".", ".", "1", "9", "5", ".", ".", "."],
            vec![".", "9", "8", ".", ".", ".", ".", "6", "."],
            vec!["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            vec!["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            vec!["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            vec![".", "6", ".", ".", ".", ".", "2", "8", "."],
            vec![".", ".", ".", "4", "1", "9", ".", ".", "5"],
            vec![".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        let test2 = vec![
            vec!["5", "3", ".", ".", "7", ".", ".", ".", "."],
            vec!["6", ".", ".", "1", "9", "5", ".", ".", "."],
            vec![".", "9", "8", ".", ".", ".", ".", "6", "."],
            vec!["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            vec!["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            vec!["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            vec![".", "6", ".", ".", ".", ".", "2", "8", "."],
            vec![".", ".", ".", "4", "1", "9", ".", ".", "5"],
            vec![".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        assert!(!is_valid_sudoku(test1));
        assert!(is_valid_sudoku(test2));
    }

    #[test]
    fn test_rain_catcher() {
        let test1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let test2 = vec![0, 2, 1, 0, 3, 1, 3, 0, 0, 1, 2];
        let test3 = vec![0, 0, 0, 0, 0, 0, 0, 1];
        let test4 = vec![0, 0, 0, 0, 0, 0, 0];

        assert_eq!(rain_catcher(test1), 6);
        assert_eq!(rain_catcher(test2), 10);
        assert_eq!(rain_catcher(test3), 0);
        assert_eq!(rain_catcher(test4), 0);

    }
}
