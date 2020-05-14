use crate::math;

use std::cmp::PartialEq;
use std::collections::LinkedList;
use std::fmt;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { val: data }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

#[derive(Debug)]
struct TestData<T> {
    data: LinkedList<Node<T>>,
}

// Generates a linked list from a vector of elements with generic type T
impl<T> TestData<Vec<T>> {
    fn gen_linked_list(input: Vec<T>) -> TestData<T> {
        let mut list = LinkedList::new();
        for el in input {
            list.push_back(Node::new(el))
        }

        TestData { data: list }
    }
}

// Generates a linked list from a random range of i32
impl TestData<i32> {
    fn gen_mock_dataset(start: i32, stop: i32, count: u32) -> TestData<i32> {
        let mut dataset = LinkedList::new();
        let rand_data = math::gen_random_vector(start, stop, count);
        for el in rand_data {
            dataset.push_back(Node::new(el))
        }

        TestData { data: dataset }
    }
}

pub fn remove_node<T>(linked_list: LinkedList<T>, target: T) -> LinkedList<T>
where
    T: fmt::Display + fmt::Debug + PartialEq<T>,
{
    let mut del_idx: usize;
    let mut curr_idx = 0;
    let mut iter = linked_list.iter();
    while let Some(el) = iter.next() {
        println!("current: {:?}", el);
        if *el == target {
            del_idx = curr_idx
        } else {
            curr_idx += 1
        }
    }

    println!("curr_idx: {} -- del_idx: {}", curr_idx, del_idx);

    linked_list
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new(12);
        assert!(node.val == 12);

        let node = Node::new("loop");
        assert!(node.val == "loop");
    }

    #[test]
    fn test_mock_gen() {
        let mock_list = TestData::gen_mock_dataset(0, 10, 5);

        println!("{:?}", mock_list);
    }

    #[test]
    fn test_ll_generator() {
        // create a vector with values 1-5
        let test1 = vec![1, 2, 3, 4, 5];

        // create a linked list from the test vector using
        // the standard library implementation
        let mut std_ll_1 = LinkedList::new();
        for el in &test1 {
            std_ll_1.push_back(Node::new(*el))
        }

        // create a linked list using the TestData Struct
        let data1 = TestData::gen_linked_list(test1);

        // create an iterator over the std library LL and TestData LL
        let mut iter1 = data1.data.iter();
        let mut ll_iter_1 = std_ll_1.iter();

        // While Some element is yielded from the iterator over TestData LL
        // check to make sure it matches with the value yielded from
        // the iterator over the std lib LL
        while let Some(el) = iter1.next() {
            assert_eq!(el.val, ll_iter_1.next().unwrap().val);
        }
    }

    #[test]
    fn test_node_removal() {
        let test_ll = TestData::gen_linked_list(vec![1, 2, 3, 4, 5]);

        remove_node(test_ll.data, Node::new(2));
    }
}
