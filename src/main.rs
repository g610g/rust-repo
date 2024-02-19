use core::num;
// use os_school::{tree::{*}, rope::{*}};
use std::{borrow::Borrow, cell::RefCell, mem, ops::Deref, rc::Rc}; 
use crate::List::{*};
struct DropTest{
    string: String
}
impl Drop for DropTest {
    fn drop(&mut self){
        println!("The struct has been deallocated");
    }
}
impl DropTest {
    fn new() -> Self{
        DropTest{
            string:String::from("Test")
        }
    }
}

#[derive(Debug)]
enum List{
    Cons(RefCell<i32>, Rc<List>),
    Nil
}
impl List {
    fn new_cons() -> List{
        Cons(RefCell::new(2), Rc::new(Nil))
    }
    fn test_ref_cell(&self){
        match self {
            Cons(ref_c, rc) => {
                *ref_c.borrow_mut() += 10;
            }
            _ => {panic!("Test panic")}
        }
    }
    fn print_val(&self){
        match self {
            Cons(ref_c, rc) => {
                println!("{}", ref_c.borrow());
            }
            _ => {println!("Nil")}
        }
    }
}
fn main(){
    let cons = List::new_cons();
    cons.test_ref_cell();
    cons.print_val();
    // let text_string = "This is a string test only!";
    // let mut rope = Rope::new();
    // rope.append(text_string);
    // let mut tree = Tree::new();
    // let node  = Node::new(20);
    // tree.bst(10);
    // tree.bst(5);
    // tree.bst(11);
    // tree.bst(4);
    // tree.bst(3);
    // tree.bst(2);
    // tree.bst(1);
    // tree.inorder_traversal(tree.as_ref());
    // println!("Height: {}", Tree::give_height(tree.head.as_ref()));
    // tree.head = Tree::balance_tree(tree.head);
    // tree.inorder_traversal(tree.as_ref());
    // println!("Height: {}", Tree::give_height(tree.head.as_ref()));
}