use std::fmt::Debug;
use std::mem::replace;
type Tree<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
pub struct Node<T: PartialEq + PartialOrd> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: PartialEq + PartialOrd + Debug + Clone> Node<T> {
    pub fn new(val: T) -> Tree<T> {
        Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }))
    }
}
#[derive(Debug)]
pub struct BinaryTree<T: PartialEq + PartialOrd + Debug + Clone> {
    root: Tree<T>,
    len: u64,
}
impl<T: PartialEq + PartialOrd + Debug + Clone> BinaryTree<T> {
    pub fn new(val: T) -> Self {
        BinaryTree {
            len: 0,
            root: Node::new(val),
        }
    }
    pub fn insert(&mut self, val: T) {
        self.len += 1;
        let root = replace(&mut self.root, None); // Need More practise with this.
        self.root = self.insert_r(root, val);
    }
    fn insert_r(&mut self, node: Tree<T>, val: T) -> Tree<T> {
        match node {
            Some(mut n) => {
                if n.val <= val {
                    n.left = self.insert_r(n.left, val);
                } else {
                    n.right = self.insert_r(n.right, val);
                }
                Some(n)
            }
            _ => Node::new(val),
        }
    }
    pub fn find(&mut self, val: T) -> Option<T> {
        self.find_r(&self.root, val)
    }
    fn find_r(&self, node: &Tree<T>, val: T) -> Option<T> {
        match node {
            Some(n) => {
                if n.val == val {
                    Some(n.val.clone())
                } else if n.val <= val {
                    self.find_r(&n.left, val)
                } else {
                    self.find_r(&n.right, val)
                }
            }
            _ => None,
        }
    }
}
