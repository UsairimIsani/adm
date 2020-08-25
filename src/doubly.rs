use std::cell::RefCell;
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
    pub fn push_back(&self) {
        unimplemented!();
    }
    pub fn push_front(&mut self, val: u64) {
        let val = Node::new_ref(val).unwrap();
        match self.head.take() {
            Some(node) => {
                println!("IN SOME");
                val.borrow_mut().prev = None;
                // node.borrow_mut().prev = Some(val.clone());
                val.borrow_mut().next = Some(node);
                self.head = Some(val);
                // self.tail = Some(node);
            }
            None => {
                val.borrow_mut().prev = None;
                val.borrow_mut().next = None;
                self.head = Some(val);
                // self.tail = Some(val.clone());
            }
        };
        self.inc_len();
    }
    pub fn pop_back(&self) -> u64 {
        unimplemented!();
    }
    pub fn pop_front(&self) -> u64 {
        unimplemented!();
    }
    pub fn find() -> u64 {
        unimplemented!();
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
        println!("{:#?}", dl);
        dl.push_front(4);
        println!("{:#?}", dl);
        dl.push_front(3);
        println!("{:#?}", dl);
    }
}
