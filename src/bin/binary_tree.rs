use adm::binary_tree::BinaryTree;
fn main() {
    let mut tree = BinaryTree::new(5);
    tree.insert(2);
    tree.insert(5);
    tree.insert(-1);
    println!("{:#?}", tree);
}
