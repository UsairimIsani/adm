// type Link<T> = Option<Box<Node<T>>>;
// #[derive(Debug)]
// pub struct Node<T> {
//     val: Option<T>,
//     next: Link<T>,
// }
// #[derive(Debug)]
// pub struct List<T> {
//     len: u64,
//     head: Link<T>,
// }
// impl<T> Node<T> {
//     pub fn new(val: T) -> Self {
//         Self {
//             val: Some(val),
//             next: None,
//         }
//     }
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         Self { len: 0, head: None }
//     }
//     pub fn unshift(&mut self, mut val: Node<T>) {
//         if self.head.is_none() {
//             self.head = Some(Box::new(val));
//         } else {
//             let head = self.head.take();
//             val.next = head;
//             self.head = Some(Box::new(val));
//         }
//         self.len += 1;
//     }
//     pub fn shift(&mut self) -> Node<T> {
//         let head = std::mem::replace(&mut self.head, None);
//         let next = head.next;
//         self.head = next;
//         *head
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_list() {
//         use super::*;
//         let mut li = List::new();
//         li.unshift(Node::new(5));
//         li.unshift(Node::new(6));
//         li.unshift(Node::new(7));
//         li.unshift(Node::new(8));
//         println!("{:#?}", li);
//     }
// }

use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: Clone> {
    val: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug + Clone> Node<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
    pub fn new_n(val: T) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug)]

pub struct List<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
    len: u64,
}

impl<T: std::fmt::Debug + Clone + PartialEq + PartialOrd + Copy> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn append(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.tail.take() {
            Some(prev) => prev.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        };
        self.inc_len();
        self.tail = Some(new_node);
    }
    pub fn unshift(&mut self, val: T) {
        let new_node = Node::new(val);
        if let Some(prev) = self.head.take() {
            new_node.borrow_mut().next = Some(prev);
        }
        self.inc_len();
        self.head = Some(new_node);
    }
    fn inc_len(&mut self) {
        self.len += 1;
    }
    fn dec_len(&mut self) {
        self.len -= 1;
    }
    pub fn shift(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next); // The next node is now is the head.
            } else {
                self.tail.take(); // head and tail Both are now None
            }
            self.dec_len();
            Rc::try_unwrap(head) // Rc -> Reference Counted Type return a val if not more than 1 references.
                .ok()
                .expect("PHAT GAYA")
                .into_inner()
                .val
        })
    }
    pub fn pop(&mut self) -> Option<T> {
        match &self.head {
            Some(node) => self.pop_r(node.borrow()),
            None => None,
        }
    }
    fn pop_r(&self, node: Ref<Node<T>>) -> Option<T> {
        match &node.next {
            Some(next) => self.pop_r(next.borrow()),
            None => Some(node.val),
        }
    }
    pub fn pop_new(&mut self) -> Option<T> {
        let mut list = vec![];
        while let Some(n) = self.head.take() {
            self.head = n.borrow_mut().next.take();
            list.push(n);
        }
        let mut pop = list.pop();
        let mut new_list = List::new();
        list.iter()
            .rev()
            .for_each(|no| new_list.append(no.borrow_mut().val));
        self.head = new_list.head;
        self.len = new_list.len;
        self.tail = new_list.tail;
        pop.take().map(|node| {
            Rc::try_unwrap(node)
                .ok()
                .expect("msg: &str")
                .into_inner()
                .val
        })
    }
    pub fn length(&self) -> u64 {
        self.len
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_list() {
        use super::*;
        let mut li = List::new();
        li.append(5);
        li.append(6);
        li.unshift(4);
        li.unshift(3);
        li.append(7);
        println!("{:#?}", li);
        println!("{}", li.shift().unwrap());
        println!("{}", li.shift().unwrap());
        println!("{:?}", li.pop());
        println!("{:#?}", li);
    }
}
