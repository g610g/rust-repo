use os_school::linked_list::{*};
use std::{process};
fn main(){
    let node = linkedList::create_node(1, None);
    let node1 = linkedList::create_node(2, Some(node));
    let mut head = Some(node1);
    if let Err(e) = linkedList::traverse_list(head.as_ref()){
        println!("Error traversing the list: {}", e);
        process::exit(1);
    };
}