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

use std::cell::RefCell;
use std::rc::Rc;

type Link<T: Clone> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: Clone> {
    val: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug + Clone> Node<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

#[derive(Debug)]

pub struct List<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
    len: u64,
}

impl<T: std::fmt::Debug + Clone> List<T> {
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
    pub fn inc_len(&mut self) {
        self.len += 1;
    }
    pub fn dec_len(&mut self) {
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
        // Gives the last element but kills the List.
        let mut prev = None;
        let mut current = std::mem::replace(&mut self.head, None);
        while let Some(node) = current {
            prev = Some(node.borrow_mut().val.clone());
            current = std::mem::replace(&mut node.borrow_mut().next, None);
            self.dec_len();
        }
        self.tail.take();
        prev
    }
    pub fn length(&self) -> u64 {
        self.len
    }
    // Try 1

    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|head| {
    //         if let Some(next) = head.borrow_mut().next.take() {
    //             next.pop_r()
    //         } else {
    //             None
    //         }
    //     })
    // }
    // fn pop_r(&mut self, node: Rc<RefCell<Node<T>>>) -> Option<T> {
    //     node.take().map(|node_val|{
    //         if let Some(next) = node.borrow_mut().take() {
    //             next.pop_r()
    //         }else{
    //             Rc::try_unwrap(node).ok().expect("Phati win")
    //         }
    //     })
    // }

    // Try 2
    // pub fn pop(&mut self) -> Option<T> {
    //     let mut prev = None;
    //     let mut current = self.head.take();
    //     while let Some(node) = current {
    //         prev = Some(
    //             Rc::try_unwrap(node)
    //                 .ok()
    //                 .expect("GETTING INSIDE")
    //                 .into_inner(),
    //         );
    //         current = Some(Rc::new(RefCell::new(prev.unwrap()));
    //     }
    //     prev.map(|n| n.val)
    //     // Some(
    //     //     Rc::try_unwrap(prev.unwrap())
    //     //         .ok()
    //     //         .expect("h")
    //     //         .into_inner()
    //     //         .val,
    //     // )
    // }
    // Try 3
    // pub fn pop(&mut self) -> Option<T> {
    //     // let mut prev = None;
    //     let mut current = std::mem::replace(&mut self.head, None);
    //     while let Some(node) = current {
    //         // prev = Some(node);
    //         // prev = std::mem::replace(&mut , None);
    //         current = std::mem::replace(&mut node.borrow_mut().next, None);
    //     }
    //     // prev
    //     Some(
    //         Rc::try_unwrap(current.unwrap())
    //             .ok()
    //             .expect("h")
    //             .into_inner()
    //             .val,
    //     )
    //     // Some(current)
    // }
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
        println!("{}", li.pop().unwrap());
        println!("{:#?}", li);
    }
}
