
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use front_of_house::hosting::add_to_waitlist;
mod customer {
    fn eat_at_restaurant(){
        super::add_to_waitlist();
    }
}
pub mod authentication{
    pub trait  Authenticatable{
        fn display_user_credentials(&self);
        fn username(&self)->&str;
    }
    #[derive(Debug)]
    pub struct User<'a>{
        username:&'a String,
        password:String,
   }
    impl<'a> Authenticatable for User<'a>{
        fn display_user_credentials(&self){
            println!("username: {}, password:{}", self.username, self.password);
        }
        fn username(&self)-> &str {
            &self.username[..]
        }
    }
    impl<'a> User<'a>{
        pub fn new(username: &'a String, password:String)->User{
            User{
                username,
                password
            }
        }
        // pub fn check_list<'a, 'b>(&'a self, vip_users: &'b [String]) -> &'b str{
        //     for (index, user) in vip_users.iter().enumerate(){
        //         if (user == &self.username){
        //             return vip_users.get(index).unwrap();
        //         }
        //     }
        //     return vip_users.first().unwrap(); 
        // }
    }
    pub fn verify_email<T: Authenticatable>(user1: &T){
    }

   pub struct Profile<T>{
        user:T,
        likes:u64,
        has_badge:bool
   }
   //implements only this method on inner T where T has a Authenticatable trait
   impl <T: Authenticatable> Profile<T>{
    pub fn show_profile(&self){
        if self.has_badge{
            println!("username:{} has {} likes and has a badge", self.user.username(), self.likes);
        }else{
            println!("user {} has no badge", self.user.username());
        }
    }
   }
   impl<T> Profile<T>{
    pub fn new(user:T)->Self{
        Profile{
            user,
            likes:120,
            has_badge:false
        }
    }
   }
}
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
pub mod tree{
    use std::{collections::VecDeque, error::Error};

    pub struct Tree{
        pub head:Option<Box<Node>>        
    }
    pub struct Node{
        val: i32,
        left: Option<Box<Node>>,
        right:Option<Box<Node>>    
    }
    impl Tree {
        pub fn new()->Tree{
            Tree{
                head:None
            }
        }
        //node insertion using iteration and queue
        pub fn insert(&mut self, new_node: Box<Node>){ 
            let mut queue = VecDeque::new();
            
            if let None = self.head{
                self.head = Some(new_node);
                return;
            }
            let mut temp: Option<&mut Box<Node>> = self.head.as_mut();
            queue.push_back(temp);
            loop {
               if queue.is_empty(){
                    return;
                }
                
                if let Some(node) = queue.pop_front().unwrap(){
                    if node.left.is_some() && node.right.is_some(){
                        queue.push_back(node.left.as_mut());
                        queue.push_back(node.right.as_mut());
                    }else if node.left.is_none(){
                        node.left = Some(new_node);
                        return;
                    }else if node.right.is_none(){
                        node.right = Some(new_node);
                        return;
                    }
                }
            }
        }
        pub fn insert_bst(&self, root: Option<Box<Node>>, new_node:Box<Node>)-> Option<Box<Node>>{
            if let None = root{
                let new_root = Some(new_node);
                return new_root;
            }
            if let Some(mut node) = root{
                if node.val > new_node.val{
                    node.left = self.insert_bst(node.left, new_node);
                    return Some(node);
                }else{
                    node.right = self.insert_bst(node.right, new_node);
                    return Some(node);
                }
            }
            return root;
             
        }
        pub fn inorder_traversal(&self, root: Option<&Box<Node>>){
            if let None = root{
                // println!("IS none");
                return;
            }
            if let Some(node) =  root {
                self.inorder_traversal(node.left.as_ref());
                println!("Node: {}", node.val);
                self.inorder_traversal(node.right.as_ref());
            }
        }
        //returns head as Option<&Box<Node>>
        pub fn as_ref(&self) -> Option<&Box<Node>>{
            self.head.as_ref()
        }
        //the api provided for inserting as a bst
        //calls the inner helper function insert_bst
        pub fn bst(&mut self, val:i32){
            let head = self.head.take();
            self.head = self.insert_bst(head, Node::new(val));
        }
        pub fn right_rotate(mut root:Box<Node>)->Option<Box<Node>>{
            //y as the new root of the rotation
            let mut y = root.left.unwrap();
            //rotation
            root.left = y.right;
            y.right = Some(root);
            Some(y)
        }
        pub fn left_rotate(mut root: Box<Node>) -> Option<Box<Node>>{
            //x as the new root of the rotation
            let mut x = root.right.unwrap();
            //rotation
            root.right = x.left;
            x.left = Some(root);
            Some(x)
        }
        //used for recalculating the height after balancing
        pub fn give_height(root: Option<&Box<Node>>){

        }
        pub fn check_balance(root: Option<Box<Node>>)-> (i32, i32, Option<Box<Node>>){
            if root.is_none(){
                return (0, 0, None);
            }
            let mut root = root.unwrap();
            let mut new_node: Option<Box<Node>>= None; 
            let (left_h, left_differ, left_node) = Tree::check_balance(root.left);
            root.left = left_node;
            let (right_h, right_differ, right_node) = Tree::check_balance(root.right);
            root.right = right_node;
            let height_differ = left_h - right_h;
            if height_differ > 1 && left_differ >= 0{
                //do necessary rotation for left left
                new_node = Tree::right_rotate(root);
            }
            else if height_differ > 1 && left_differ <= -1{
                //for left-right rotation
                root.left = Tree::left_rotate(root.left.unwrap());
                new_node = Tree::right_rotate(root);
            }
            //right right 
            else if height_differ < -1 && right_differ <= 0{
                new_node = Tree::left_rotate(root);
            }else if height_differ < -1 && right_differ > 0{
                //right-left
                root.right = Tree::right_rotate(root.right.unwrap());
                new_node = Tree::left_rotate(root);
            }

            return (left_h.max(right_h) + 1, height_differ, new_node);
            
        }

    }
    impl Node{
        // pub fn inorder_traversal(&self){
        //     if let None = self{

        //     }     
        // }
    }
    impl Node {
        //creates a Box<Node> initial improvements to return None on fail creation
        pub fn new(val:i32) -> Box<Node>{
            Box::new(Node{val, left:None, right:None})
        } 
    }
    // pub struct RopeTree<'a >{
    //     root:Option<Box<NodeTree<'a >>>

    // }
    // impl RopeTree {      
        
    // }
    // pub struct NodeTree<'a>{
    //     string:&'a str,
    //     weight:i32,

    // }
    
}
pub  mod string_manipulation{
    pub struct LeafNode<'a >{
        string:&'a str,
        weight:usize

    }
    //string node as leaf node
    impl<'a > LeafNode<'a> {
        pub fn new(string :&str) -> LeafNode{
           LeafNode{string, weight:string.len()} 
        }
    }    
}

