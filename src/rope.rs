use super::rope::Rope::{*};
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
    fn new_weight_node(weight_node: WeightNode) -> Option<Box<Rope>>{
        Some(
            Box::new(
                WeightNode(weight_node)
            )
        )
    }
    pub fn new(string: &str) -> Result<Rope, &str>{
        let leaf_node = Self::new_leaf_node(string);
        if leaf_node.is_some(){
            //creates the struct weightNode as mutable
            let mut w = WeightNode::new(string.len());
            //assign the weight_node to the newly leaf_node
            w.left = leaf_node;
            //wrap enum weight_node the struct weight_node
            let weight_node = WeightNode(w);
            return Ok(weight_node);
        }
        Err("error creating rope")
    }
    pub fn append(self, string: &str)-> Result<Rope, &str>{
        match self{
            WeightNode(mut w) => {
                let new_ln = Self::new_leaf_node(string);
                w.right = new_ln;
                let mut new_w = WeightNode::new(w.weight + string.len());
                new_w.left = Self::new_weight_node(w);
                //rebalance should be made here
                return Ok(WeightNode(new_w).balance()); 
            }
            _ => {Err("Error appending a string")}
        }
    }
    fn balance(self) -> Rope{
        // match self {
        //     WeightNode(w) => {

        //     },
        //     _ => {}
        // }
        return self;
    }

}

