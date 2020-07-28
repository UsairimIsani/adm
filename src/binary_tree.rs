type Branch = Option<Box<Node>>;
#[derive(Debug)]
struct Node {
    val: usize,
    left: Branch,
    right: Branch,
    root: Branch,
}
#[derive(Debug)]
struct Tree {
    root: Branch,
    length: usize,
}

impl Node {
    fn new(val: usize) -> Branch {
        Some(Box::new(Node {
            val,
            left: None,
            right: None,
            root: None,
        }))
    }
    fn add(&mut self, node: Branch) -> Branch {
        Some(Box::new(Node {
            val: self.val,
            left: node,
            right: None,
            root: None,
        }))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree() {
        let some = Node::new(4).unwrap().add(Node::new(5));
        println!("{:?}", some);
    }
}
