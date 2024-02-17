pub struct Rope{
    head:Option<Box<dyn TreeNode>>
}
pub trait TreeNode{
    fn length(&self) -> usize;
}
impl<'a> TreeNode for LeafNode<'a >{
    fn length(&self)->usize{
        self.length
    }
}
impl TreeNode for WeightNode{
    fn length(&self) -> usize{
        12
    }

}
pub struct LeafNode<'a >{
    string:&'a str,
    length:usize
}

pub struct WeightNode{
    weight:i64,
    left: Option<Box<dyn TreeNode>>,
    right:Option<Box<dyn TreeNode>>
}

impl Rope{
    pub fn new() -> Self{
        Rope{
            head:None
        }
    }
    pub fn append(&mut self, string: &'static str){
        let leaf_node = LeafNode::new(string);
        let mut weight_node = WeightNode::new();
        weight_node.left = Some(leaf_node);
        self.head = Some(weight_node);
    } 
}
impl <'a > LeafNode<'a >{
    pub fn new(string: &'a str)->Box<LeafNode>{
        Box::new(LeafNode{
            string,
            length:string.len()
        })
    }
}
impl WeightNode{
    pub fn new()-> Box<Self>{
        Box::new(
            WeightNode{weight:64, left:None, right:None}
        )
    }
}
