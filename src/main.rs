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
    let node  = Node::new(2);
    tree.insert(node);
    tree.insert(Node::new(1));
    tree.insert(Node::new(3));
    tree.insert(Node::new(5));
    tree.insert(Node::new(10));
    tree.insert(Node::new(6));
    tree.inorder_traversal(tree.as_ref());
}