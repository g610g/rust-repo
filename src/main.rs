#[derive(Debug)]
struct linkedList{
    value:i32,
    next: Option<Box<linkedList>>,
}
impl linkedList {
    fn create_node(value:i32, next: Option<Box<linkedList>>) -> linkedList{
        linkedList{
            value,
            next 
        }
    }
}

fn insert_node(head: linkedList){

}
fn main(){
    let next_node = Box::new(linkedList{value:1, next:None});
    let head = linkedList::create_node(1, Some(next_node));
    println!("{:?}", head);
}