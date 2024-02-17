
    use std::{collections::VecDeque, error::Error};
    #[derive(Debug)]
    pub struct Tree{
        pub head:Option<Box<Node>>        
    }
    #[derive(Debug)]
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
        pub fn right_rotate(mut root:Box<Node>)->(Option<Box<Node>>, i32){
            //y as the new root of the rotation
            let mut y = root.left.unwrap();
            //rotation
            root.left = y.right;
            y.right = Some(root);
            let left_h = Self::give_height(y.left.as_ref());
            let right_h = Self::give_height(y.right.as_ref());
            (Some(y), left_h - right_h)
        }
        pub fn left_rotate(mut root: Box<Node>) -> (Option<Box<Node>>, i32){
            //x as the new root of the rotation
            let mut x = root.right.unwrap();
            //rotation
            root.right = x.left;
            x.left = Some(root);
            let left_h = Self::give_height(x.left.as_ref());
            let right_h = Self::give_height(x.right.as_ref());

            (Some(x), left_h - right_h)
        }
        //used for recalculating the height after balancing
        pub fn give_height(root: Option<&Box<Node>>)->i32{
            if root.is_none(){
                return -1;
            }
            let root = root.unwrap();
            let left_h = Tree::give_height(root.left.as_ref());
            let right_h = Tree::give_height(root.right.as_ref());

            return left_h.max(right_h) + 1;
        }
        fn prv_balance_tree(root: Option<Box<Node>>)-> (i32, i32, Option<Box<Node>>){
            if root.is_none(){
                return (-1, 0, None);
            }
            let mut root = root.unwrap();
            let mut new_node: Option<Box<Node>> = None; 
            let (left_h, left_differ, left_node) = Self::prv_balance_tree(root.left);
            root.left = left_node;
            let (right_h, right_differ, right_node) = Self::prv_balance_tree(root.right);
            root.right = right_node;
            let mut height_differ = left_h - right_h;
            let mut return_height = left_h.max(right_h) + 1;
            if height_differ > 1 && left_differ >= 0{
                //do necessary rotation for left left
                (new_node, height_differ) = Self::right_rotate(root);                
            }
            else if height_differ > 1 && left_differ <= -1{
                //for left-right rotation
                (root.left, _) = Self::left_rotate(root.left.unwrap());
                (new_node, height_differ) = Self::right_rotate(root);
                return_height = Self::give_height(new_node.as_ref());
            }
            //right right 
            else if height_differ < -1 && right_differ <= 0{
                (new_node, height_differ) = Self::left_rotate(root);
                return_height = Self::give_height(new_node.as_ref());
            }else if height_differ < -1 && right_differ > 0{
                //right-left
                (root.right, _) = Self::right_rotate(root.right.unwrap());
                (new_node, height_differ) = Self::left_rotate(root);
                return_height = Self::give_height(new_node.as_ref());
            }else{
                return (return_height, height_differ, Some(root));
            }
            return (return_height, height_differ, new_node);
            
        }
        pub fn balance_tree(root: Option<Box<Node>>)-> Option<Box<Node>>{
            let (_height, _differ, root) = Self::prv_balance_tree(root);
            return root;
        }


    }
    impl Node {
        //creates a Box<Node> initial improvements to return None on fail creation
        pub fn new(val:i32) -> Box<Node>{
            Box::new(Node{val, left:None, right:None})
        } 
    }
  
    
