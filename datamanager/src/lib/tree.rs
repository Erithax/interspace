
type BlockId = usize;
pub struct Block {
    sym: String,
    info: Info,
    style: Style,
}

pub struct SuperBlock {
    
}

type NodeId = usize;
pub struct Node {
    parent: NodeId,
    children: Vec<NodeId>,
    active: bool,
    sub_blocks: Vec<NodeId>,
    block: BlockId,
}

pub struct Tree {
    items: Vec<Node>,
    root: NodeId,
}


pub struct Constellation {
    blocks: Vec<Block>, // BlockId indexed
    stages: Vec<BlockId>,  // BlockId indexed
    trees: Vec<Tree>,
}