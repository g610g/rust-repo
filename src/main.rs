use os_school::tree::{*};
fn main(){
    let mut tree = Tree::new();
    let node  = Node::new(20);
    tree.bst(10);
    tree.bst(5);
    tree.bst(11);
    tree.bst(4);
    tree.bst(3);
    tree.bst(2);
    tree.bst(1);
    tree.inorder_traversal(tree.as_ref());
    println!("Height: {}", Tree::give_height(tree.head.as_ref()));
    tree.head = Tree::balance_tree(tree.head);
    tree.inorder_traversal(tree.as_ref());
    println!("Height: {}", Tree::give_height(tree.head.as_ref()));
}