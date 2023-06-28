


// All indexed with BlockId:
const STAGES: &'static [(Stage, Stage)] = &[];
const BLOCKS: &'static [Block] = &[]; 

const NORMAL_TREE: &'static [Tree] = &[];
const REVERS_TREE: &'static [Tree] = &[];
const SPLITNORMALTREE: &'static [Tree] = &[];
const SPLITREVERSTREE: &'static [Tree] = &[];

const NORMAL_TREE_SUBSTAGE_COUNTS: &'static [sub_stage_count] = &[];
const REVERS_TREE_SUBSTAGE_COUNTS: &'static [sub_stage_count] = &[];
const SPLIT_NORMAL_TREE_SUBSTAGE_COUNTS: &'static [sub_stage_count] = &[];
const SPLIT_REVERS_TREE_SUBSTAGE_COUNTS: &'static [sub_stage_count] = &[];


struct Info {}
struct Style {}

pub enum Stage {
    ROOTSTAGE,
    Langbridge,
    Ui,
    Style,
    Layout,
    Paint,
    Composit,
    Gfx,
    Shell,
    Platform,
    LEAFSTAGE,
}



type BlockId = usize;
pub struct Block {
    sym: String,
    info: Info,
    style: Style,
}

pub struct SuperBlock {
    
}

pub enum NodeType {
    Normal,
    Hidden, 
    Filler,
    DeleteCluster {
        clustered_nodes: Vec<NodeId>,
    },
    GroupCluster {
        clustered_nodes: Vec<NodeId>,
    },
    SuperBlock {
        sub_blocks: Vec<NodeId>,
    },
}

/* 
# Hidden
Block not rendered, but affects tree normally.
# Filler
Filler block in
    A, B, C
    A, C
    is filled with style of A
    A -> B      -> C
      -> filler -> C

# DeleteCluster & GroupCluster
NodeId = new id
A -> X
  -> Y 
becomes
A -> cluster_node{clustered_nodes = [X, Y]} 
clustered_nodes are always siblings
uncluster is done with reference to original tree

# DeleteCluster 
all clustered nodes, and nodes after these clustered nodes are not renderered
A -> X -> B
  -> Y -> (B, C)
becomes
A -> cluster_node{clustered_nodes = [X, Y]} 

# GroupCluster
clustered nodes act like singular node in tree, possibly simplifying their downstream branches
A -> X -> B
  -> Y -> (B, C)
becomes
A -> cluster_node{clustered_nodes = [X, Y]} -> (B, C)

# Superblock
consecutive blocks part of a superblock can be rendered as superblock
A -> B1 -> B2 -> C
becomes
A -> super(B, (1, 2)) -> C

but
A -> B1 -> B2 -> C
        -> D
remains unchanged
*/

type NodeId = usize;
pub struct Node {
    parent: NodeId,
    children: Vec<NodeId>,
    nodetype: NodeType,
    block: BlockId,
}

pub enum sub_stage_count {
    Normal(usize),
    Split(usize, usize, usize),
}

pub struct Tree {
    items: Vec<Node>,
    root: NodeId,
}

pub struct ConstellationO {
    
}

impl ConstellationO {
    pub fn new() -> Self {
        return ConstellationO {  }
    }
}