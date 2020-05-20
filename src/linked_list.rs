use std::cmp::PartialEq;
use std::fmt::Debug;

// Need a Node type to hold data and reference to next Node in LL
struct Node<T: PartialEq + Debug> {
    data: T,
    next: Link<T>,
}

// Need a Link type to hold a reference to a boxed Node or None
type Link<T: PartialEq + Debug> = Option<Box<Node<T>>>;

// Need a LL type to join everything together by maintaining a pointer to a specific Link
struct LinkedList<T: PartialEq + Debug> {
    head: Link<T>,
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    // Method to push Nodes onto the Linked List
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node)
    }
    // Method to pop Nodes from the Linked List
    fn pop(&mut self) -> Option<T> {
        // Take the box stored at the head pointer and map over the node in a closure
        self.head.take().map(|node| {
            // link to the next node in the series
            self.head = node.next;
            // return the data stored in the popped noded
            node.data
        })
    }
    // Method to peek at the Node within the head pointer's Box
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // Method to obtain a mutable reference to box innards
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    // Method to find an element in the LinkedList and return its index
    fn find(&mut self, target: T) -> Option<usize> {
        let mut target_idx = Some(0);
        // borrow a reference to the head node 
        let mut curr_node = self.head.as_ref();

        // traverse all nodes in the linked list:
        // while there is Some data inside the current node
        while let Some(boxed_data) = curr_node {
            // check to see if the target value is contained in the node
            let node_data = curr_node.map(|node| &node.data);

            // if it does, return the index of said node
            if *node_data.unwrap() == target {
                return target_idx
            } else {
                // otherwise, increment the target_idx counter
                target_idx.as_mut().map(|idx| *idx+=1);
            }

            // Move to next node in linked list
            curr_node = boxed_data.next.as_ref()
        }

        // if the target is not found, will evaluate to None
        None
    }
}
// Need to implement the Drop trait because dropping boxes is not tail recursive
impl<T: PartialEq + Debug> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();
        while let Some(mut boxed_data) = curr_node {
            curr_node = boxed_data.next.take()
        }
    }
}

// Need to implement a way to iterate over the LinkedList
// Uses a Tuple Struct to wrap the LinkedList Type
pub struct IntoIter<T: PartialEq + Debug>(LinkedList<T>);

impl<T: PartialEq + Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T: PartialEq + Debug> LinkedList<T> {
    // Method to move self into an iterator
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct Iter<'a, T: PartialEq + Debug> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: PartialEq + Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.data
        })
    }
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn iter(&mut self) -> Iter<T> {
        Iter {
            // Deref the Box before taking the reference to underlying node
            next: self.head.as_ref().map(|node| &**node),
            // can also be written like so:
            // next: self.head.as_ref().map::<&Node<T>, _>(|node| &node)
        }
    }
}

pub struct IterMut<'a, T: PartialEq + Debug> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T: PartialEq + Debug> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.data
        })
    }
}

impl<T: PartialEq + Debug> LinkedList<T> {
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ll_push() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(5);
        list.push(6);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // Check to make sure you can mutate the mut ref
        // Correct way:
        list.peek_mut().map(|val| *val = 42);

        // Incorrect way:
        // list.peek_mut().map(|&mut val| val = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.push(2);
        list.push(3);
        list.push(1);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push(2);
        list.push(3);
        list.push(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push(2);
        list.push(3);
        list.push(1);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_node_finder() {
        let mut list = LinkedList::new();
        list.push(2);
        list.push(3);
        list.push(1);

        assert_eq!(list.find(1), Some(0));
        assert_eq!(list.find(3), Some(1));
        assert_eq!(list.find(2), Some(2));
        assert_eq!(list.find(5), None);

    }
}
