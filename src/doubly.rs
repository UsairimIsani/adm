use std::cell::{Ref, RefCell};
use std::rc::Rc;
type Link = Option<Rc<RefCell<Node>>>;
#[derive(Debug)]
pub struct Node {
    val: Option<u64>,
    next: Link,
    prev: Link,
}
#[derive(Debug)]
pub struct DoublyLinkedList {
    head: Link,
    tail: Link,
    len: u64,
}

impl Node {
    pub fn new(val: u64) -> Self {
        Self {
            val: Some(val),
            next: None,
            prev: None,
        }
    }
    pub fn new_ref(val: u64) -> Link {
        Some(Rc::new(RefCell::new(Node::new(val))))
    }
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn push_back(&mut self, val: u64) {
        let new_node = Node::new_ref(val).unwrap();
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new_node.clone()),
        };
        self.inc_len();
        self.tail = Some(new_node);
    }
    pub fn push_front(&mut self, val: u64) {
        let new_node = Node::new_ref(val).unwrap();
        match self.head.take() {
            Some(old) => {
                new_node.borrow_mut().next = Some(old.clone());
                old.borrow_mut().prev = Some(new_node.clone());
            }
            None => self.tail = Some(new_node.clone()),
        };
        self.inc_len();
        self.head = Some(new_node);
    }
    pub fn pop_back(&mut self) -> Option<u64> {
        self.tail
            .take()
            .map(|tail| {
                if let Some(last) = tail.borrow_mut().prev.take() {
                    last.borrow_mut().next = None;
                    self.tail = Some(last)
                } else {
                    self.head.take();
                }
                Rc::try_unwrap(tail).ok().expect("CANT UW").into_inner().val
            })
            .unwrap()
    }
    pub fn pop_front(&mut self) -> Option<u64> {
        self.head
            .take()
            .map(|head| {
                if let Some(next) = head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }
                self.dec_len();
                Rc::try_unwrap(head)
                    .ok()
                    .expect("Something is terribly wrong")
                    .into_inner()
                    .val
            })
            .unwrap()
    }
    pub fn find(&mut self, val: u64) -> bool {
        match &self.head {
            Some(node) => self.find_r(node.borrow(), val),
            None => false,
        }
    }
    fn find_r(&self, node: Ref<Node>, val: u64) -> bool {
        if node.val.is_some() && node.val.unwrap() == val {
            true
        } else if node.val.is_none() {
            false
        } else {
            match &node.next {
                Some(next) => self.find_r(next.borrow(), val),
                None => false,
            }
        }
    }
    pub fn len(&mut self) -> u64 {
        self.len
    }
    fn inc_len(&mut self) {
        self.len += 1;
    }
    fn dec_len(&mut self) {
        self.len -= 1;
    }
    // pub fn insert(&mut self, val: T) {
    //     self.len += 1;
    //     let root = replace(&mut self.root, None); // Need More practise with this.
    //     self.root = self.insert_r(root, val);
    // }
    // fn insert_r(&mut self, node: Tree<T>, val: T) -> Tree<T> {
    //     match node {
    //         Some(mut n) => {
    //             if n.val <= val {
    //                 n.left = self.insert_r(n.left, val);
    //             } else {
    //                 n.right = self.insert_r(n.right, val);
    //             }
    //             Some(n)
    //         }
    //         _ => Node::new(val),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_insert() {
        use super::*;
        let mut dl = DoublyLinkedList::new();
        dl.push_back(4);
        dl.push_back(5);
        dl.push_front(3);
        dl.push_front(2);
        assert_eq!(true, dl.find(4));
        // assert_eq!(2, dl.pop_front().unwrap());
        // assert_eq!(3, dl.pop_front().unwrap());
        // assert_eq!(5, dl.pop_back().unwrap());
        // assert_eq!(4, dl.pop_back().unwrap());
    }
}
