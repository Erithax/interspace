#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use macroni::{self};
use sha2::digest::block_buffer::Block;
use strum::{IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use petgraph::prelude::*;
use dioxus::prelude::*;

use crate::lang::*;
use crate::owner::*;
use crate::langbridge::*;
use crate::ui::*;
use crate::layout::*;
use crate::render::*;
use crate::intergfx::*;
use crate::gfx::*;
use crate::platform::*;


/////////////////////////////////////////
/// TODO
/// * implement enum checks in proc macro
/// * deliver good errors in proc macro via spans
/// * enforce alphabetical variants in enums & add_all

#[derive(PartialEq, Eq, EnumIter, Display, Debug, Hash, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    ROOT,
    ALL, // used in node branches to leaf with parsetree! to include all the branches to leaf of the previous Block
    TODO, // used in node branches to leaf as a placeholder node
    SHADOW,

    Langbridge(Langbridge),
    Ui(Ui),
    Layout(Layout),
    Render(Render),
    Intergfx(Intergfx),
    Gfx(Gfx),
    Platform(Platform),
}

impl BlockType {

    fn stringify(&self) -> String {
        return match self {
            BlockType::ROOT =>                       {"ROOT".to_string()},
            BlockType::TODO =>                       {"TODO".to_string()},
            BlockType::Langbridge(i) => {"Langbridge(".to_string()  + &i.to_string() + ")"},
            BlockType::Ui(i) =>                 {"Ui(".to_string()          + &i.to_string() + ")"},
            BlockType::Layout(i) =>         {"Layout(".to_string()      + &i.to_string() + ")"},
            BlockType::Render(i) =>         {"Render(".to_string()      + &i.to_string() + ")"},
            BlockType::Intergfx(i) =>     {"Intergfx(".to_string()    + &i.to_string() + ")"},
            BlockType::Gfx(i) =>               {"Gfx(".to_string()         + &i.to_string() + ")"},
            BlockType::Platform(i) =>     {"Platform(".to_string()    + &i.to_string() + ")"},
            _ => {panic!("not supposed to reach")}
        }
    }

    pub fn inner_to_string(&self) -> String {
        match self {
            Self::ROOT => {"ROOT".to_string()},
            Self::ALL => {"ALL".to_string()},
            Self::TODO => {"TODO".to_string()},
            Self::SHADOW => {"SHADOW".to_string()},
            Self::Langbridge(i) => {i.to_string()},
            Self::Ui(i) => {i.to_string()},
            Self::Layout(i) => {i.to_string()},
            Self::Render(i) => {i.to_string()},
            Self::Intergfx(i) => {i.to_string()},
            Self::Gfx(i) => {i.to_string()},
            Self::Platform(i) => {i.to_string()},
        }
    }

    pub fn stage(&self) -> Stage {
        match self {
            Self::ROOT => {Stage::Root},
            Self::Langbridge(i) => {Stage::Langbridge},
            Self::Ui(i) => {Stage::Ui},
            Self::Layout(i) => {Stage::Layout},
            Self::Render(i) => {Stage::Render},
            Self::Intergfx(i) => {Stage::Gfx},
            Self::Gfx(i) => {Stage::Gfx},
            Self::Platform(i) => {Stage::Platform},
            Self::TODO => {Stage::NA},
            Self::ALL => {Stage::NA},
            Self::SHADOW => {Stage::NA},
        }
    }
}

#[derive(Debug, PartialEq, Eq, EnumIter, Display, Hash, Copy, Clone)]
pub enum Stage {
    NA,
    Root,
    Langbridge,
    Ui,
    Layout,
    Render,
    Gfx,
    Platform,
}

impl Stage {
    pub fn lvl(&self) -> i32 {
        match self {
            Stage::NA => {-1},
            Stage::Root => {0},
            Stage::Langbridge => {1},
            Stage::Ui => {2},
            Stage::Layout => {3},
            Stage::Render => {4},
            Stage::Gfx => {5},
            Stage::Platform => {6}
        }
    }

    pub fn next(&self) -> Stage {
        match self {
            Stage::NA => {Stage::NA},
            Stage::Root => {Stage::NA},
            Stage::Langbridge => {Stage::Ui},
            Stage::Ui => {Stage::Layout},
            Stage::Layout => {Stage::Render},
            Stage::Render => {Stage::Gfx},
            Stage::Gfx => {Stage::Platform},
            Stage::Platform => {Stage::NA},
        }
    }

    pub fn prev(&self) -> Stage {
        match self {
            Stage::NA => {Stage::NA},
            Stage::Root => {Stage::NA},
            Stage::Langbridge => {Stage::NA},
            Stage::Ui => {Stage::Langbridge},
            Stage::Layout => {Stage::Ui},
            Stage::Render => {Stage::Layout},
            Stage::Gfx => {Stage::Render},
            Stage::Platform => {Stage::Gfx},
        }
    }
}

impl PartialOrd for Stage {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lvl() == other.lvl() {
            return Some(std::cmp::Ordering::Equal)
        } else if self.lvl() < other.lvl() {
            return Some(std::cmp::Ordering::Less)
        } else {
            return Some(std::cmp::Ordering::Greater)
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Info {
    pub name: String,
    pub owner: Owner,
    pub desc: String,
    pub imp_lang: Lang, // implementation language
    pub bind_langs: Vec<Lang>,
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.name && self.owner == other.owner { return true};
        return false
    }
}
impl Eq for Info {}
impl Hash for Info {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.owner.hash(state);
        self.desc.hash(state);
        self.imp_lang.hash(state);
        self.bind_langs.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ExtraInfo {
    ROOT,
    TODO,

    Langbridge(LangbridgeInfo),
    Ui(UiInfo),
    Layout(LayoutInfo),
    Render(RenderInfo),
    Intergfx(IntergfxInfo),
    Gfx(GfxInfo),
    Platform(PlatformInfo),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Node {
    pub bt: BlockType,
    pub info: Info,
    pub extra: ExtraInfo,
    // pub nexts: Vec<Vec<BlockType>>, // paths to leaf, cannot be subgraph because want to distinguish between A -> B -> C and A -> D-> C e.g. IntergfxA->IntergfxB->Gfx1->Windows, IntergfxA->IntergfxC->Gfx1->Linux
    // pub prevs: Vec<Vec<BlockType>>, // paths 
    pub paths: Vec<Vec<BlockType>>,
}

impl Node {
    pub fn getInfo(&self) -> &Info {
        return &self.info
    }

    pub fn getExtra(&self) -> &ExtraInfo {
        return &self.extra
    }

    // pub fn get_subgraph(&self) -> &Digraph {
    //     return &self.subgraph
    // }

    // pub fn get_subgraph_mut(&mut self) -> &mut Digraph {
    //     return &mut self.subgraph
    // }

    // pub fn set_subgraph(&mut self, subgraph: Digraph) {
    //     self.subgraph = subgraph;
    // }

    pub fn get_all_in_prev_stage(&self) -> Vec<Vec<BlockType>> {
        assert!(self.bt.stage() != Stage::Platform, "stage is Stage::Platform");
        let mut res: Vec<Vec<BlockType>> = Vec::new();
        for nei in self.subgraph.get_up_neighbors(&self.bt) {
            
 
        }
        return res
    }

    pub fn get_all_in_same_stage(&self) -> Vec<BlockType> {
        let mut res: Vec<BlockType> = Vec::new();
        for b in self.subgraph.graph.node_weights() {
            if b.stage() == self.bt.stage() && !res.contains(b) {
                res.push(*b);
            }
        }
        return res
    }

    pub fn has_in_next_stage(&self, other: &BlockType) -> bool {
        return self.subgraph.graph.node_weights().any(|bt| 
            *bt == *other && bt.stage() > self.bt.stage()
        )
    }

 

}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Digraph {
    graph: Graph<BlockType, (), Directed>,
}

impl Digraph {
    
    fn add(&mut self, bt: &BlockType) {
        assert!(!self.contains(&bt));
        self.graph.add_node(*bt);
    }

    fn contains(&self, bt: &BlockType) -> bool {
        return self.graph.node_indices().any(|x| self.graph.node_weight(x).unwrap() == bt)
    }

    fn index_from_blocktype(&self, bt: &BlockType) -> NodeIndex {
        assert!(self.contains(bt));

        for i in self.graph.node_indices() {
            if self.graph.node_weight(i).unwrap() == bt {
                return i
            }
        }
        panic!("node with BlockType {} not in graph", bt);
    }

    fn link(&mut self, from: &BlockType, to: &BlockType) {
        assert!(from != to, "from: {} is not equal to to: {}", from, to);
        assert!(self.contains(from), "self: {:?} does not contain from: {}", self, from);
        assert!(self.contains(to), "self: {:?} does not contain to: {}, {}", self, to, to.stringify());

        self.graph.add_edge(self.index_from_blocktype(from), self.index_from_blocktype(to), ());
    }

    fn get_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
        let mut res: Vec<BlockType> = Vec::new();
        for i in self.graph.neighbors(self.index_from_blocktype(bt)) {
            res.push(*self.graph.node_weight(i).unwrap());
        }
        return res
    }

    fn get_up_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
        let mut res: Vec<BlockType> = Vec::new();
        for i in self.graph.neighbors_directed(self.index_from_blocktype(bt), Direction::Outgoing) {
            res.push(*self.graph.node_weight(i).unwrap());
        }
        return res
    }

    fn get_down_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
        let mut res: Vec<BlockType> = Vec::new();
        for i in self.graph.neighbors_directed(self.index_from_blocktype(bt), Direction::Incoming) {
            res.push(*self.graph.node_weight(i).unwrap());
        }
        return res
    }

    fn is_downstream_from(&self, one: &BlockType, two: &BlockType) -> bool {
        assert!(self.contains(one));
        assert!(self.contains(two));
        assert!(one != two);
        for bt in self.get_down_neighbors(two) {
            if bt == *two {
                return true
            } else if self.is_downstream_from(one, &bt){
                return true
            }
        }
        return false
    }

    fn is_upstream_from(&self, one: &BlockType, two: &BlockType) -> bool {
        assert!(self.contains(one));
        assert!(self.contains(two));
        assert!(one != two);
        for bt in self.get_up_neighbors(two) {
            if bt == *two {
                return true
            } else if self.is_upstream_from(one, &bt){
                return true
            }
        }
        return false
    }

    pub fn get_all_downstream_from(&self, block: &BlockType) -> Vec<BlockType> {
        assert!(self.contains(block));
        let mut res: Vec<BlockType> = Vec::new();
        for other in self.graph.node_weights() {
            if self.is_downstream_from(other, block) {
                res.push(*other);
            }
        }
        return res
    }

    pub fn get_all_upstream_from(&self, block: &BlockType) -> Vec<BlockType> {
        assert!(self.contains(block));
        let mut res: Vec<BlockType> = Vec::new();
        for other in self.graph.node_weights() {
            if self.is_upstream_from(other, block) {
                res.push(*other);
            }
        }
        return res
    }

    /// Insert shadow nodes BlockType::SHADOW between linked nodes with non-sequential stages
    /// e.g. between node1 of stage Langbridge and node2 of stage Render, 2 shadow nodes get inserted
    pub fn insert_shadow_nodes(&mut self) {

    }

    /// Get subgraph of self, including `block` and all its downstream nodes
    pub fn get_downstream_subgraph(&self, block: &BlockType) -> Digraph {
        let mut res: Digraph = Digraph {
            graph: Graph::<BlockType, (), Directed>::new()
        };
        res.add(block);
        for bt in self.get_all_downstream_from(block) {
            self.add(&bt);
        }

        for bt in self.get_all_downstream_from(block) {
            for e in self.graph.edges_directed(self.index_from_blocktype(&bt), Direction::Incoming) {
                res.link(self.graph.node_weight(e.source()).unwrap(), self.graph.node_weight(e.target()).unwrap());
            }
        }
        return res
    }

    /// Get subgraph of self, including `block` and all its upstream nodes
    pub fn get_upstream_subgraph(&self, block: &BlockType) -> Digraph {
        let mut res: Digraph = Digraph {
            graph: Graph::<BlockType, (), Directed>::new()
        };
        res.add(block);
        for bt in self.get_all_upstream_from(block) {
            self.add(&bt);
        }

        for bt in self.get_all_upstream_from(block) {
            for e in self.graph.edges_directed(self.index_from_blocktype(&bt), Direction::Incoming) {
                res.link(self.graph.node_weight(e.source()).unwrap(), self.graph.node_weight(e.target()).unwrap());
            }
        }
        return res
    }

    /// Returns true if  block has no outgoing edges
    pub fn is_leaf(&self, block: &BlockType) -> bool {
        return self.graph.edges_directed(self.index_from_blocktype(block), Direction::Outgoing).next().is_none()
    }

    /// Returns true if  block has no outgoing edges
    pub fn is_root(&self, block: &BlockType) -> bool {
        return self.graph.edges_directed(self.index_from_blocktype(block), Direction::Incoming).next().is_none()
    }

}




#[macro_export]
macro_rules! add_nodes_of_bt_type {
    ($self:ident, $map:ident, $bt_enu:ident) => {
        for bt in $bt_enu::iter() {
            if bt.to_string() != "META" {
                dbg!(&bt);
                dbg!(stringify!($bt_enu));
                $map.insert(BlockType::$bt_enu(bt), $self.data.get(&BlockType::$bt_enu(bt)).unwrap());
            }
        }
    }
}

/*
Constellation is an interface for the graph using BlockType instances instead of the internally used node indices.
*/
#[derive(serde::Deserialize, serde::Serialize)] 
pub struct Constellation {
    data: HashMap<BlockType, Node>,
    graph: Digraph,
}

impl Constellation {
    pub fn getNode(&self, bt: &BlockType) -> &Node {
        assert!(self.data.contains_key(bt), "called Constellation.getNode() with bt = {}", bt);
        return self.data.get(bt).unwrap();
    }

    fn add(&mut self, bt: BlockType, node: Node) {
        if self.data.contains_key(&bt) {
            panic!("tried to add node with blocktype {bt:?} to graph twice");
        }
        self.graph.add(&bt);
        self.data.insert(bt, node);
    }

    // add edges to Contellation.graph given by direct neighbors of nodes in Constellation.data.node
    fn add_all_edges(&mut self) {
        for (from, node) in &self.data {
            for nei in node.subgraph.get_neighbors(&node.bt) {
                // if nei.len() > 0 {
                //     // assert!(branch.iter().enumerate().all(|(i, bt)| *bt != BlockType::ALL || i == branch.len()-1), "only the last node in a branch can be ALL, found branch {:?}", branch);
                //     self.graph.link(&from, &branch[0]);
                // }
                self.graph.link(&from, &nei);
            }
        }
    }

    pub fn getAllOfBt(&self, blocktype: &BlockType) -> HashMap<BlockType, &Node> {
        let mut res: HashMap<BlockType, &Node> = HashMap::new();
        
        match blocktype {
            BlockType::ROOT => {panic!("should not call getAllOfBt with BlockType::ROOT")},
            BlockType::ALL => {panic!("should not call getAllOfBt with BlockType::ALL")},
            BlockType::TODO => {panic!("should not call getAllOfBt with BlockType::TODO")},
            BlockType::SHADOW => {panic!("should not call getAllOfBt with BlockType::SHADOW")},
            BlockType::Langbridge(_) => {add_nodes_of_bt_type!(self, res, Langbridge)},
            BlockType::Ui(_) => {add_nodes_of_bt_type!(self, res, Ui)},
            BlockType::Layout(_) => {add_nodes_of_bt_type!(self, res, Layout)},
            BlockType::Render(_) => {add_nodes_of_bt_type!(self, res, Render)},
            BlockType::Intergfx(_) => {add_nodes_of_bt_type!(self, res, Intergfx)},
            BlockType::Gfx(_) => {add_nodes_of_bt_type!(self, res, Gfx)},
            BlockType::Platform(_) => {add_nodes_of_bt_type!(self, res, Platform)}
        }
        return res
    }

    pub fn getAllOfStage(&self, st: Stage) -> Vec<&BlockType> {
        let mut res: Vec<&BlockType> = Vec::new();
        for (bt, _) in self.data.iter() {
            if bt.stage() == st {
                res.push(bt);
            }
        }
        return res
    }

    pub fn getAllOfOwner(&self, o: Owner) -> HashSet<BlockType> {
        let mut res: HashSet<BlockType> = HashSet::new();
        for (bt, n) in &self.data {
            if n.info.owner == o {
                res.insert(*bt);
            }
        }
        return res
    }

    pub fn getAllBlockNamesOfOwner(&self, o: Owner) -> HashSet<String> {
        let mut res: HashSet<String> = HashSet::new();
        for (bt, n) in &self.data {
            if n.info.owner == o {
                res.insert(bt.inner_to_string());
            }
        }
        return res
    }

    pub fn GetExpandedBranchesOfNode(&self, block: &BlockType) -> Vec<Vec<BlockType>> {
        let mut new_branches: Vec<Vec<BlockType>> = Vec::new();
        for bt in self.data.get(block).expect(&format!("block {} not in self.data??", block)).subgraph.get_up_neighbors(block) {
            assert!(branch.len() > 0, "branch length must be > 0");
            let mut base_branch: Vec<BlockType> = branch.clone();
            if *branch.last().unwrap() == BlockType::ALL {
                assert!(branch.len() > 1, "there must be a node in front of BlockType::ALL in a branch");
                base_branch.remove(base_branch.len()-1);
                let last_node = base_branch.last().unwrap();
                assert!(*last_node != BlockType::ALL, "invalid branch {:?}", branch);
                let expanded_branch: Vec<BlockType>;
                for sub_branch in self.GetExpandedBranchesOfNode(last_node) {
                    let mut new_branch = base_branch.clone();
                    new_branch.append(&mut sub_branch.clone());
                    new_branches.push(new_branch);
                }
            } else {
                new_branches.push(base_branch);
            }
            
        }
        return new_branches;
    }

    pub fn expand_branches_ALL(&mut self) {
        let mut new_branches: HashMap<BlockType, Vec<Vec<BlockType>>> = HashMap::new();
        for (bt, n) in self.data.iter() {
            new_branches.insert(*bt, self.GetExpandedBranchesOfNode(&bt));
        }

        for (bt, n) in self.data.iter_mut() {
            n.set_subgraph(new_branches.remove(bt).unwrap());
        }
    }

    pub fn expand_ALL_node(&mut self, block: &BlockType) {
        let mut new_branches: Vec<Vec<BlockType>> = Vec::new();
        for bt in self.data.get(block).unwrap().subgraph.get_up_neighbors(block) {
            assert!(bt != BlockType::ALL || self.graph.is_leaf(&bt));
            if bt != BlockType::ALL {continue}

        }
    }

    pub fn get_all_of_prev_stage(&self, bt: BlockType) -> Vec<BlockType> {
        let mut res: Vec<BlockType> = Vec::new();
        for (b, n) in self.data.iter() {
            if n.has_in_next_stage(b.stage(), bt) {
                res.push(*b);
            }
        }
        return res
    }

    pub fn store(&self) {
        std::fs::write("./res/state/constellation.ron", ron::to_string(self).unwrap())
            .expect("was unable to store constellation");
    }

}


#[inline_props]
pub fn RepName(cx: Scope, bt: BlockType, name: String, owner: String) -> Element {
    cx.render(rsx! {
        div {
            class: "block {bt.to_string()}",
            div {
                class: "underlay",
                svg {
                    class: "lef",
                    view_box: "0 0 1 2",
                    path {class: "fill"},
                    path {class: "stroke"},
                },
                svg {
                    class: "mid",
                    view_box: "0 0 2 2",
                    preserve_aspect_ratio: "none",
                    path {class: "fill", d: "M 0,0  2,0  2,2  0,2  Z"},
                    path {class: "stroke", d: "M 0,0  2,0"},
                    path {class: "stroke", d: "M 0,2  2,2"},
                },
                svg {
                    class: "rig",
                    view_box: "0 0 1 2",
                    path {class: "fill"},
                    path {class: "stroke"},
                },
            },

            div {
                class: "content",
                img {
                    src: "./img/{owner}/{bt.inner_to_string()}.svg",
                    alt: "{owner} {bt.inner_to_string()} logo",
                    "onerror": "this.style.display='none'",
                },
                img {
                    src: "./img/{owner}/{bt.inner_to_string()}.png",
                    alt: "{owner} {bt.inner_to_string()} logo",
                    "onerror": "this.style.display='none'",
                },
                img {
                    src: "./img/{owner}/{bt.inner_to_string()}.jpg",
                    alt: "{owner} {bt.inner_to_string()} logo",
                    "onerror": "this.style.display='none'",
                },
                div {
                    {cx.props.name.clone()},
                }
            },
        }
    })
}

pub trait Blockify<BT, EI> {
    fn add_all() -> Vec<(BT, Info, EI, Vec<Vec<BlockType>>)>;
}


#[macro_export]
macro_rules! verf {
    ($en:ident) => {
        let mut has_meta = false;
        let mut implemented = false;
        let l = $en::iter().len();
        let mut i = 0;
        for var in $en::iter() {
            if var.to_string() == "META" {has_meta = true}
            if i == l-1 && ! has_meta {
                panic!("Block Type Enum {} does not have META variant", var.to_string());
            }
            i += 1;

            for entry in $en::add_all() {
                if entry.0.to_string() == var.to_string() {
                    implemented = true;
                }
            }
            if ! implemented && var.to_string() != "META" {
                panic!("Block Type Variant {} is not implemented in add_all", var.to_string());
            }
        }

        // if $i::add_all().len()+1 != $i::iter().len() {panic!("Not all declared variants in {} are implemented in add_all()", $i)}
    };
}

pub fn setupConstellation() -> Constellation {

    let mut blockies: Vec<(BlockType, Info, ExtraInfo, Vec<Vec<BlockType>>)> = Vec::new();

    blockies.push((
        BlockType::ROOT, 
        Info{name:"ROOT".to_string(), owner: Owner::Erithax, desc: "ROOT NODE".to_string(), imp_lang: Lang::Rust, bind_langs: vec![Lang::Rust]},
        ExtraInfo::ROOT,
        vec![],
    ));

    blockies.push((
        BlockType::TODO,
        Info{name:"TODO".to_string(), owner: Owner::Erithax, desc: "TODO NODE".to_string(), imp_lang: Lang::Rust, bind_langs: vec![Lang::Rust]},
        ExtraInfo::TODO,
        vec![],
    ));

    verf!(Langbridge);
    verf!(Ui);
    verf!(Layout);
    verf!(Render);
    verf!(Intergfx);
    verf!(Gfx);
    verf!(Platform);

    if Langbridge::add_all().len()+1 != Langbridge::iter().len()    {panic!("Not all declared variants in Langbridge are implemented in add_all()")}
    if Ui::add_all().len()+1 != Ui::iter().len()                    {panic!("Not all declared variants in Ui are implemented in add_all()")}
    if Layout::add_all().len()+1 != Layout::iter().len()            {panic!("Not all declared variants in Layout are implemented in add_all()")}
    if Render::add_all().len()+1 != Render::iter().len()            {panic!("Not all declared variants in Render are implemented in add_all()")}
    if Intergfx::add_all().len()+1 != Intergfx::iter().len()        {panic!("Not all declared Intergfx in Langbridge are implemented in add_all()")}
    if Gfx::add_all().len()+1 != Gfx::iter().len()                  {panic!("Not all declared variants in Gfx are implemented in add_all()")}
    if Platform::add_all().len()+1 != Platform::iter().len()        {panic!("Not all declared variants in Platform are implemented in add_all()")}
    


    for i in Langbridge::add_all() {
        blockies.push((BlockType::Langbridge(i.0), i.1, ExtraInfo::Langbridge(i.2), i.3));
    }
    for i in Ui::add_all() {
        blockies.push((BlockType::Ui(i.0), i.1, ExtraInfo::Ui(i.2), i.3));
    }
    for i in Layout::add_all() {
        blockies.push((BlockType::Layout(i.0), i.1, ExtraInfo::Layout(i.2), i.3));
    }
    for i in Render::add_all() {
        blockies.push((BlockType::Render(i.0), i.1, ExtraInfo::Render(i.2), i.3));
    }
    for i in Intergfx::add_all() {
        blockies.push((BlockType::Intergfx(i.0), i.1, ExtraInfo::Intergfx(i.2), i.3));
    }
    for i in Gfx::add_all() {
        blockies.push((BlockType::Gfx(i.0), i.1, ExtraInfo::Gfx(i.2), i.3));
    }
    for i in Platform::add_all() {
        blockies.push((BlockType::Platform(i.0), i.1, ExtraInfo::Platform(i.2), i.3));
    }

    let mut c = Constellation {
        data: HashMap::<BlockType, Node>::new(),
        graph: Digraph {
            graph: Graph::<BlockType, (), Directed>::new()
        }
    };

    for block in blockies {
        c.add(block.0, Node {
            info: block.1,
            extra: block.2,
            nexts: block.3,
        })
    }

    c.add_all_edges();
    c.expand_branches_ALL();

    return c
}
