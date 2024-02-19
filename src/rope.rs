use std::{error::Error as StdError, fmt::Display};

use super::rope::Rope::{*};
#[derive(Debug)]
struct Error{
    message: String
}
impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl StdError for Error {
    
}
#[derive(Debug)]
pub struct LeafNode{
    string: String,
    length:usize,
    start:usize
}
impl LeafNode{
    fn new(str: &str) -> Self{
        LeafNode{
            string:str.to_string(),
            length: str.len(),
            start: 0
        }
    }
}
pub struct WeightNode{
    weight:usize,
    left: Option<Box<Rope>>,
    right:Option<Box<Rope>>
}
impl WeightNode{
    fn new(weight:usize)-> Self{
        WeightNode{
            weight,
            left:None,
            right:None
        }
    }
}
pub enum Rope{
    LeafNode(LeafNode),
    WeightNode(WeightNode),
}
impl Rope{
    fn new_leaf_node(str: &str) -> Option<Box<Rope>>{
        Some(Box::new(
            LeafNode(LeafNode::new(str))
        ))
    }
    fn new_weight_node(weight:usize) -> Option<Box<Rope>>{
        Some(
            Box::new(
                WeightNode(WeightNode::new(weight))
            )
        )
    }
    pub fn new(string: &str) -> Result<Option<Box<Rope>>, &str>{
        let leaf_node = Self::new_leaf_node(string);
        if let Some(ln) = leaf_node{
            if let LeafNode(leaf) = *ln{
                let weight_node = Self::new_weight_node(leaf.length);
                return Ok(weight_node);
            }
           
        }
        return Err("error creating rope");
    }

}

