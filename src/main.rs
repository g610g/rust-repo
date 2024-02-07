use os_school::linked_list::{*};
use std::{process};
fn main(){
    let mut head = None;
    let mut new_node = linkedList::create_node(1, None);
    head = linkedList::insert_node_constant(head, new_node);
    head = linkedList::insert_node_constant(head, linkedList::create_node(2, None));
    println!("{:?}", head);



}