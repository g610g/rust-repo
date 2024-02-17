use os_school::linked_list::{*};
use os_school::tree::{*};
use std::{process};
fn main(){
    // let mut head = None;
    // let mut new_node = linkedList::create_node(1, None);
    // head = linkedList::insert_node_constant(head, new_node);
    // head = linkedList::insert_node_constant(head, linkedList::create_node(2, None));
    
    // if let Err(e) = linkedList::insert_end(head.as_mut(), linkedList::create_node(3, None)){
    //     println!("Error while inserting at the end: {}", e);
    //     process::exit(1);
    // }
    
    // linkedList::traverse_list(head.as_ref());
    let mut tree = Tree::new();
    let node  = Node::new(20);
    tree.bst(10);
    tree.bst(5);
    tree.bst(11);
    tree.bst(12);
    tree.bst(13);
    tree.bst(14);
    tree.bst(15);
    // tree.head = Tree::right_rotate(tree.head.unwrap());
    // tree.head = Tree::left_rotate(tree.head.unwrap());
    // tree.inorder_traversal(tree.as_ref());
    tree.head = Tree::balance_tree(tree.head);
    println!("Height: {}", Tree::give_height(tree.head.as_ref()));
}