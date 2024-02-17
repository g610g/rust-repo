pub mod rope;
pub mod tree;
pub mod linked_list{
    #[derive(Debug)]
    pub struct linkedList{
        pub value:i32,
        pub next: Option<Box<linkedList>>,
    }
    impl linkedList {
        pub fn create_node(value:i32, next: Option<Box<linkedList>>) -> Box<linkedList>{
            Box::new(linkedList{
                value,
                next 
            })
        }
        pub fn traverse_list(head: Option<&Box<linkedList>>)-> Result<(), &str>{
            let mut temp = head;
            if let None = temp{
                return Err("Cannot traverse the list. Empty List!");
            }
            while let Some(node) = temp{
                println!("{}", node.value);
                temp = node.next.as_ref();
            }
            Ok(())
        }
        pub fn insert_node_constant(mut head:Option<Box<linkedList>>, mut new_node:Box<linkedList>)-> Option<Box<linkedList>>{
            if let None = head{
                head = Some(new_node);
                return head;
            
            }
            new_node.next = head;
            head = Some(new_node);
            return head;
        }
        pub fn insert_end(head: Option<&mut Box<linkedList>>, new_node: Box<linkedList>)->Result<(), &str>{
            if let None = head{
                return Err("Cannot insert at the end while list is empty at the moment");
            }
            let mut tmp = head;
            while let Some(node) = tmp {
                if let None = node.next{
                    node.next = Some(new_node);       
                    break;
                }
                tmp = node.next.as_mut();
            }
            Ok(())
        }
    }
}

