use std::fmt::Debug;
use std::mem::replace;
type Tree<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
pub struct Node<T: PartialEq + PartialOrd> {
    val: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: PartialEq + PartialOrd + Debug> Node<T> {
    pub fn new(val: T) -> Tree<T> {
        Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }))
    }
}
#[derive(Debug)]
pub struct BinaryTree<T: PartialEq + PartialOrd + Debug> {
    root: Tree<T>,
    len: u64,
}
impl<T: PartialEq + PartialOrd + Debug> BinaryTree<T> {
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
}
