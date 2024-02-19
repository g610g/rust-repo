
use std::mem;

use crate::tree::Tree;
#[derive(Debug)]
pub enum TreeNode{
    LeafNode(LeafNode),
    WeightNode(WeightNode)
}
impl TreeNode {
    // pub fn new_leaf(string: &str)-> Box<TreeNode>{
    //     LeafNode::new(string)
    // }
    // pub fn new_weight() -> Box<TreeNode>{
    //     WeightNode::new()
    // }
    fn take_leaf(self) -> LeafNode{
        match self{
            TreeNode::LeafNode(l) => l,
            _ => {panic!("Error not a leafnode")}
        }
    }
    pub fn take_weight(self) -> WeightNode{
        match self{
            TreeNode::WeightNode(w) => w,
            _ => panic!("Error not a weightnode")
        }
    }
    
    pub fn append(&mut self, str: &str) -> Option<Box<TreeNode>>{
        let mut weight = self.take_weight();
        if weight.right.is_none(){
            let leaf_node = LeafNode::new(str);
            weight.right = Some(leaf_node);
            let mut weight_node = WeightNode::new();
            if let TreeNode::WeightNode(w) = weight_node.as_mut(){
                w.left = Some(Box::new(TreeNode::WeightNode(weight)));
            }
            return Some(weight_node);
        }
        let leaf_node = LeafNode::new(str);
        
    }
    pub fn read_rope(&self){
        
    }
}

#[derive(Debug)]
pub struct LeafNode{
    string: String,
    length:usize,
    start:usize
}

impl LeafNode{
    fn new(s:&str)->Box<TreeNode>{
        let leaf_node = TreeNode::LeafNode(LeafNode{
            string:s.to_string(),
            start:0,
            length:s.len() - 1
        });
        Box::new(leaf_node)
    }

}
#[derive(Debug)]
pub struct WeightNode{
    weight:i64,
    left: Option<Box<TreeNode>>,
    right:Option<Box<TreeNode>>
}
impl WeightNode{
    fn new()-> Box<TreeNode>{
       let weight_node = TreeNode::WeightNode(WeightNode{weight:64, left:None, right:None});
       Box::new(weight_node)
   }
}
pub struct Rope{
    root:Option<Box<TreeNode>>
}
impl Rope{
    pub fn new() -> Self{
        Rope{
            root:None
        }
    }
    pub fn append(&mut self, string: &str){
        match self.root.as_mut(){
            Some(root_ref) => self.root = root_ref.append(string),
            //mag kuan ka diria himo ka og new weight_node og leaf_node tapos ang created nga weight_node mao na ang imohang new root
            None => panic!("Wala rason pero ga panici")
        };
    }
    // pub fn print_string(&mut self){
    //    if let Some(head) =  self.root.as_ref(){
        
    //    };
    // }

}

