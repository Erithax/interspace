
use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::bowl::component::*;
use crate::bowl::constellation::*;
use crate::bowl::stage;
use log::{info};

use super::CONSTELLATION;
use super::stage::*;
use super::block::*;

pub type BlockId = usize;

const ROOT_COMP_ID: ComponentId = 77777;


#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    Normal, 
    Dropped, // dropped because of group cluster
    Skipped,
    DeleteCluster{
        clustered_nodes: Vec<BlockId>,
    },
    GroupCluster{
        clustered_nodes: Vec<BlockId>,
    }
}
/*
Starting with
    A -> B -> C
      -> D -> C
      -> E -> F
# Skipped cluster
    e.g skip B & D
        A -> E -> F
          -> C
    e.g. skip B & D & E (cfr. hide stage)
        A -> C
          -> F
# GroupCluster .
    e.g. groupcluster D & E
        A -> B -> C
          -> (D & E) -> C
                     -> F
# DeleteCluster 
    e.g. deletecluster B & D
        A -> ...
          -> E -> F

*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedCells {
    Isolated,  
    Combined, // labels & background of selected cells in different branches are merged
}

impl SelectedCells {
    // because can't use :: in rsx! macro
    pub fn getIsolated(&self) -> Self {return SelectedCells::Isolated}
    pub fn getCombined(&self) -> Self {return SelectedCells::Combined}
}

impl std::fmt::Display for SelectedCells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Isolated => {"Isolated"},
            Self::Combined => {"Combined"},
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum TreeType {
    LefToRig,
    RigToLef,
    HourglassToRig,
    HourglassToLef,
}

impl TreeType {
    pub fn get_lef_to_rig(&self) -> Self {return Self::LefToRig}
    pub fn get_rig_to_lef(&self) -> Self {return Self::RigToLef}
    pub fn get_hourglass_to_lef(&self) -> Self {return Self::HourglassToLef}
    pub fn get_hourglass_to_rig(&self) -> Self {return Self::HourglassToRig}
}

impl std::fmt::Display for TreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::LefToRig => {"LefToRig"},
            Self::RigToLef => {"RigToLef"},
            Self::HourglassToLef => {"HourglassToLef"},
            Self::HourglassToRig => {"HourglassToRig"},
        };
        write!(f, "{s}")
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Block {
    parent: BlockId,
    children: Vec<BlockId>,
    comp: ComponentId,
    block_type: BlockType,
    before_targ: bool,
    stage_subtree_dep_len: usize,
    stage_subtree_dep_start: usize,
    stage_subtree_dep_end: usize,
    breadth_interval: (usize, usize),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Tree {
    pub blocks: BTreeMap<BlockId, Block>,
    pub root: BlockId,
    pub tree_type: TreeType,
    pub targ: ComponentId,
    pub grid_rows_str: String,
    pub grid_cols_str: String,
}

impl Tree {
    pub fn new_blank(block_id: BlockId) -> Tree {
        let mut blocks: BTreeMap<BlockId, Block> = BTreeMap::new();
        blocks.insert(block_id, Block {
            parent: block_id,
            children: Vec::new(),
            comp: ROOT_COMP_ID,
            block_type: BlockType::Normal,
            before_targ: true,
            stage_subtree_dep_len: 0,
            stage_subtree_dep_start: 0,
            stage_subtree_dep_end: 0,
            breadth_interval: (0, 0),
        });
        return Tree {
            blocks: blocks,
            root: block_id,
            tree_type: TreeType::HourglassToLef,
            targ: ROOT_COMP_ID,
            grid_rows_str: "".to_string(),
            grid_cols_str: "".to_string(),
        }
    }

    pub fn new(block_id: BlockId, block: Block, tree_type: TreeType, targ: ComponentId) -> Tree { 
        let mut blocks: BTreeMap<BlockId, Block> = BTreeMap::new();
        blocks.insert(block_id, block);
        return Tree {
            blocks: blocks,
            root: block_id,
            tree_type: tree_type,
            targ: targ,
            grid_rows_str: "".to_string(),
            grid_cols_str: "".to_string(),
        }
    }

    pub fn assert_valid(&self) {
        // bidirectional consistency
        for (id, block) in self.blocks.iter().enumerate() {
            assert!(self.get(self.get(id).parent).children.contains(&id));
            for ch in self.get(id).children.iter() {
                assert!(self.get(*ch).parent == id);
            }
        }

        // every node in self.items is in tree
        for (id, block) in self.blocks.iter().enumerate() {
            assert!(self.tree_contains(id));
        }
        // // root in self.items
        // assert!(self.items.contains_key(&self.root));

        // every node in tree is in self.items
        assert!((0..self.blocks.len()-1).into_iter().all(|i| self.blocks_contains(i)));

        // every id appears only once in tree
        for (id, _) in self.blocks.iter().enumerate(){
            assert!(self.tree_count(id) == 1);
        }
    }

    pub fn tree_contains(&self, id: BlockId) -> bool {
        return self.get(self.root).children.iter().any(|&ch_id| self.down_tree_from_contains(ch_id, id))
    }

    pub fn tree_count(&self, id: BlockId) -> usize {
        if self.get(id).children.len() == 0 {
            return 1
        } else {
            return 1 + self.get(id).children
                .iter().map(|&ch|
                    self.tree_count(ch)
                ).fold(0, |acc, next| 
                    acc + next
                );
        }
    }

    pub fn down_tree_from_contains(&self, id: BlockId, targ: BlockId) -> bool {
        if id == targ {return true}
        return self.get(id).children.iter().any(|ch_id| self.down_tree_from_contains(*ch_id, targ));
    }

    pub fn blocks_contains(&self, id: BlockId) -> bool {
        return id < self.blocks.len();
    }

    pub fn get(&self, id: BlockId) -> &Block {
        return &self.blocks.get(&id).expect(&format!("tree.get({}) failed: id not in tree\n{}", id, 0));
    }

    pub fn get_mut(&mut self, id: BlockId) -> &mut Block {
        return self.blocks.get_mut(&id).unwrap();
    }

    pub fn add(&mut self, id: BlockId, block: Block) {
        self.blocks.insert(id, block);
    }

    pub fn full_trees_from_paths_of(con: &Constellation, paths: &Vec<Vec<ComponentId>>, targ: ComponentId) -> (Tree, Tree) {
        assert!(paths.iter().enumerate().all(|(i, path)| paths.iter().enumerate().any(
            |(j, other_path)| i == j || path != other_path
        )));


        let mut id: BlockId = 0;

        let mut to_lef_tree = Self::new(id, Block{
            parent: id,
            children: Vec::new(),
            comp: ROOT_COMP_ID,
            block_type: BlockType::Normal,
            before_targ: true,
            stage_subtree_dep_len: 0,
            stage_subtree_dep_start: 0,
            stage_subtree_dep_end: 0,
            breadth_interval: (0, 0),
        }, TreeType::LefToRig, targ
        );
        id += 1;
        let mut to_rig_tree = Self::new(id, Block{
            parent: id,
            children: Vec::new(),
            comp: ROOT_COMP_ID,
            block_type: BlockType::Normal,
            before_targ: true,
            stage_subtree_dep_len: 0,
            stage_subtree_dep_start: 0,
            stage_subtree_dep_end: 0,
            breadth_interval: (0, 0),
        }, TreeType::RigToLef, targ);
        id += 1;

        let mut paths: Vec<&Vec<ComponentId>> = paths.iter().collect();

        // remove paths without target
        paths.retain(|path: &&Vec<ComponentId>| path.contains(&targ));

        assert!(paths.iter().all(|path| path.contains(&targ)), "{}, {paths:?}", targ.to_string());

        let sole_targ_path = vec![targ];
        if paths.len() == 0 {paths.push(&sole_targ_path);}

        // remove duplicate subpaths
        let mut new_paths: Vec<&Vec<ComponentId>> = vec![];
        for (i, path) in paths.iter().enumerate() {
            let mut include = true;
            for (j, other_path) in paths.iter().enumerate() { 
                if i == j || other_path.len() <= path.len() {continue}
                let other_i = other_path.iter().position(|e| *e == path[0]);
                if other_i.is_none() {continue}
                let mut other_i = other_i.unwrap();

                assert!(path[0] == other_path[other_i]);
                
                for i in 0..path.len() {
                    if other_i >= other_path.len() {continue}
                    if other_path[other_i] != path[i] {continue}
                    other_i += 1;
                }
                include = false
            }
            if include {
                new_paths.push(*path)
            }
        }

        for path in new_paths {
            
            // ADD PATH TO to_lef_tree
            let mut curr_node_id: usize = to_lef_tree.root;
            let mut before_targ = true;
            for i in (0..path.len()).rev() {
                let already_in_tree: Option<usize> = to_lef_tree.get(curr_node_id).children.iter().find(|&id| to_lef_tree.get(*id).comp == path[i]).copied();
                if path[i] == targ {
                    before_targ = false;
                }
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        to_lef_tree.add(id, Block {
                            parent: curr_node_id,
                            children: Vec::new(),
                            comp: path[i],
                            block_type: BlockType::Normal,
                            before_targ: before_targ,
                            stage_subtree_dep_len: 0,
                            stage_subtree_dep_start: 0,
                            stage_subtree_dep_end: 0,
                            breadth_interval: (0, 0),
                        });
                        to_lef_tree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }

            // ADD PATH TO to_rig_tree
            let mut curr_node_id: usize = to_rig_tree.root;
            let mut before_targ = true;
            for i in 0..path.len(){
                let already_in_tree: Option<usize> = to_rig_tree.get(curr_node_id).children.iter().find(|&id| to_rig_tree.get(*id).comp == path[i]).copied();
                if path[i] == targ {
                    before_targ = false;
                }
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        to_rig_tree.add(id, Block {
                            parent: curr_node_id,
                            children: Vec::new(),
                            comp: path[i],
                            block_type: BlockType::Normal,
                            before_targ: before_targ,
                            stage_subtree_dep_len: 0,
                            stage_subtree_dep_start: 0,
                            stage_subtree_dep_end: 0,
                            breadth_interval: (0, 0),
                        });
                        to_rig_tree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }
        }

        return (to_rig_tree, to_lef_tree)
    }
    
    pub fn split_trees_from_paths_of(paths: &Vec<Vec<ComponentId>>, targ: ComponentId) -> (Tree, Tree) {
        assert!(paths.iter().enumerate().all(|(i, path)| paths.iter().enumerate().any(
            |(j, other_path)| i == j || path != other_path
        )));
        
        let mut id: BlockId = 0;
        let mut to_lef_tree = Self::new(id, Block{
            parent: id,
            children: Vec::new(),
            comp: ROOT_COMP_ID,
            block_type: BlockType::Normal,
            before_targ: true,
            stage_subtree_dep_len: 0,
            stage_subtree_dep_start: 0,
            stage_subtree_dep_end: 0,
            breadth_interval: (0, 0),
        }, TreeType::HourglassToLef, targ
        );
        id += 1;
        let mut to_rig_tree = Self::new(id, Block{
            parent: id,
            children: Vec::new(),
            comp: ROOT_COMP_ID,
            block_type: BlockType::Normal,
            before_targ: true,
            stage_subtree_dep_len: 0,
            stage_subtree_dep_start: 0,
            stage_subtree_dep_end: 0,
            breadth_interval: (0, 0),
        }, TreeType::HourglassToRig, targ);
        id += 1;


        for path in paths.into_iter().filter(|path| path.contains(&targ)) {
            assert!(path.contains(&targ));
            
            let root_index = path.iter().position(|bt| *bt == targ)
                .expect(&format!("invalid path {path:?} does not contain root bt {targ}"));

            
            // ADD PATH TO to_lef_tree
            let mut curr_node_id: usize = to_lef_tree.root;
            for i in (0..root_index).rev() {
                let already_in_tree: Option<usize> = to_lef_tree.get(curr_node_id).children
                    .iter().find(|&id| to_lef_tree.get(*id).comp == path[i]).copied();
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        to_lef_tree.add(id, Block {
                            parent: curr_node_id,
                            children: Vec::new(),
                            comp: path[i],
                            block_type: BlockType::Normal,
                            before_targ: false,
                            stage_subtree_dep_len: 0,
                            stage_subtree_dep_start: 0,
                            stage_subtree_dep_end: 0,
                            breadth_interval: (0, 0),
                        });
                        to_lef_tree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }


            // ADD PATH TO to_rig_tree
            let mut curr_node_id: usize = to_rig_tree.root;
            for i in root_index+1..path.len(){
                let already_in_tree: Option<usize> = to_rig_tree.get(curr_node_id).children
                    .iter().find(|&id| to_rig_tree.get(*id).comp == path[i]).copied();
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        to_rig_tree.add(id, Block {
                            parent: curr_node_id,
                            children: Vec::new(),
                            comp: path[i],
                            block_type: BlockType::Normal,
                            before_targ: false,
                            stage_subtree_dep_len: 0,
                            stage_subtree_dep_start: 0,
                            stage_subtree_dep_end: 0,
                            breadth_interval: (0, 0),
                        });
                        to_rig_tree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }
        }

        return (to_rig_tree, to_lef_tree)
    }

    pub fn leaf_count(&self) -> usize {
        return self.blocks.iter().fold(0, |acc, (id, block)| if block.children.len() == 0 {acc + 1} else {acc})
    }

    pub fn leaf_count_from(&self, id: BlockId) -> usize {
        if self.get(id).children.len() == 0 {
            return 1
        }
        return self.get(id).children.iter().fold(0, |acc, ch_id| acc + self.leaf_count_from(*ch_id))
    }

    pub fn get_path_to_leaf_strings(&self, con: &Constellation, id: BlockId) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let me = if id == self.root {
            format!("({})ROOT", id)
        } else {
            format!("({}){}", id, con.get_comp(self.get(id).comp).str_id)
        };

        if self.get(id).children.len() == 0 {
            return vec![me]
        }

        let mut very_first = true;
        for ch in self.get(id).children.iter(){
            let nexts = self.get_path_to_leaf_strings(con, *ch);
            let mut first_for_child = true;
            for n in nexts {
                if very_first {
                    res.push(me.clone() + " -> " + n.as_str());
                    very_first = false;
                    first_for_child = false;
                } else if first_for_child {
                    res.push(" ".repeat(me.len()) + " -> " + n.as_str());
                    first_for_child = false;
                } else {
                    res.push(" ".repeat(me.len()) + "    " + n.as_str());
                }
            }
        }
        return res
    }

    pub fn pretty_string(&self, con: &Constellation) -> String {
        return format!("Tree:\n{}", self.get_path_to_leaf_strings(con, self.root)
            .iter().fold("".to_owned(), |acc, next| acc + next + "\n"));
    }

    pub fn pretty_id_string(&self) -> String {
        return format!("Tree:\n{}", self.get_path_to_leaf_id_strings(self.root)
            .iter().fold("".to_owned(), |acc, next| acc + next + "\n"));
    }

    pub fn get_path_to_leaf_id_strings(&self, id: BlockId) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let me = if id == self.root {
            format!("({})ROOT", id)
        } else {
            format!("({})", id)
        };

        if self.get(id).children.len() == 0 {
            return vec![me]
        }

        let mut very_first = true;
        for ch in self.get(id).children.iter(){
            let nexts = self.get_path_to_leaf_id_strings(*ch);
            let mut first_for_child = true;
            for n in nexts {
                if very_first {
                    res.push(me.clone() + " -> " + n.as_str());
                    very_first = false;
                    first_for_child = false;
                } else if first_for_child {
                    res.push(" ".repeat(me.len()) + " -> " + n.as_str());
                    first_for_child = false;
                } else {
                    res.push(" ".repeat(me.len()) + "    " + n.as_str());
                }
            }
        }
        return res
    }

    pub fn get_block_pseudo_stage(&self, constellation: &Constellation, block_id: BlockId) -> SubStage {
        let targ_stage = Stage::from_comp_typ(constellation.get_comp(self.targ).typ);
        let block_stage = Stage::from_comp_typ(constellation.get_comp(self.get(block_id).comp).typ);

        if targ_stage != block_stage {
            return SubStage {
                stage: block_stage,
                pseudostage: PseudoStage::All,
            }
        }

        if self.get(block_id).comp == self.targ {
            return SubStage {
                stage: block_stage,
                pseudostage: PseudoStage::Mid,
            }
        }
  
        if 
            self.tree_type == TreeType::HourglassToLef || 
            (self.tree_type == TreeType::RigToLef && self.is_block_after_target(block_id)) 
        {
            return SubStage {
                stage: block_stage,
                pseudostage: PseudoStage::Pre,
            }
        } else {
            return SubStage {
                stage: block_stage,
                pseudostage: PseudoStage::Post,
            }
        }
    }

    pub fn is_block_after_target(&self, mut block_id: BlockId) -> bool {
        if self.tree_type == TreeType::HourglassToLef || self.tree_type == TreeType::HourglassToRig {
            return true
        }
        while block_id != self.root {
            if self.get(block_id).comp == self.targ {
                return true
            }
            block_id = self.get(block_id).parent;
        }
        return false
    }

    pub fn get_all_substages(&self, constellation: &Constellation) -> Vec<SubStage> {
        let mut res: Vec<SubStage> = Vec::new();
        if self.tree_type == TreeType::LefToRig || self.tree_type == TreeType::RigToLef {
            let targ_stage = Stage::from_comp_typ(constellation.get_comp(self.targ).typ);
            for stage in Stage::iter_reals() {
                if stage != targ_stage {
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::All});
                } else {  
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::Pre});
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::Mid});
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::Post});
                }
            }
        } else if self.tree_type == TreeType::HourglassToRig {
            for stage in Stage::iter_reals() {
                let targ_stage = Stage::from_comp_typ(constellation.get_comp(self.targ).typ);
                if stage ==  targ_stage {
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::Post});
                } else if targ_stage < stage {
                    res.push(SubStage { stage: stage, pseudostage: PseudoStage::All })
                }
            }
        } else if self.tree_type == TreeType::HourglassToLef {
            for stage in Stage::iter_reals() {
                let targ_stage = Stage::from_comp_typ(constellation.get_comp(self.targ).typ);
                if stage ==  targ_stage {
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::Pre});
                } else if targ_stage > stage {
                    res.push(SubStage{stage: stage, pseudostage: PseudoStage::All})
                }
            }
        }
        return res
    }

    // SET TREE GRID LINES
    pub fn set_grid_lines(&mut self, constellation: &Constellation) {
        let epsilon = 0.01;
        
        let mut lines: Vec<(SubStage, Vec<Line>)> = Vec::new();

        #[derive(Debug, Clone, PartialEq)]
        struct Line {
            frac: f64,
            frac_strs: Vec<String>,
        }

        for (substage, mut stage_max_sub_div_lvl) in self.sub_stage_subtrees_dep_len_max(constellation) {

            stage_max_sub_div_lvl = stage_max_sub_div_lvl.max(1);

            let mut stage_lines: Vec<Line> = Vec::new();
            for i in 1..=stage_max_sub_div_lvl {
                
                for j in 0..=i {
                   
                    let line_str = format!("{}-{}", j, i);
                    let frac: f64 = j as f64 / i as f64;
                    
                    let mut line_already_present = false;
                    for line in stage_lines.iter_mut() {
                        if (line.frac-frac).abs() < epsilon {
                            line.frac_strs.push(line_str.clone());
                            line_already_present = true;
                            break
                        }
                    }
                    if !line_already_present {
                        stage_lines.push(Line {frac: frac, frac_strs: vec![line_str]});
                    }
                }
            }
            stage_lines.sort_unstable_by(|a: &Line, b: &Line| a.frac.partial_cmp(&b.frac).unwrap());
            assert!(stage_lines.len() > 0);
            lines.push((substage, stage_lines.clone()));
        }

        // move stage and pseudo line in names
        let mut lines: Vec<Vec<Vec<String>>> = lines.into_iter().map(|(substage, lines)| 
            lines.iter().map(|line|
                line.frac_strs.iter().map(|frac_str|
                    format!("{}-{}-{} ", substage.stage.short_rep(), substage.pseudostage.short_rep(), frac_str)
                ).collect()
            ).collect()
        ).collect();

        // merge last line of pseudostage with first line of next pseudostage
        for substage_i in 1..lines.len() {
            let mut follower_line = lines[substage_i][0].clone();
            lines[substage_i-1].last_mut().unwrap().append(&mut follower_line);
            lines[substage_i].remove(0);
        }

        let grid_cols_str: Vec<Vec<String>> = lines.into_iter().map(|lines| 
            lines.iter().map(|line|
                "[".to_string() + line.iter().map(|name|
                    format!("{} ", name)
                ).fold("".to_owned(), |acc, next| acc + next.as_str()).as_str() + "]"
            ).collect()
        ).collect();

        self.grid_cols_str = grid_cols_str.into_iter().flatten().collect::<Vec<String>>().join(" auto ");

        self.grid_rows_str = format!("repeat({}, auto); ", self.leaf_count().to_string().as_str());
    }

    // SET BREADTH INTERVALS
    pub fn set_breadth_intervals(&mut self, constellation: &Constellation) {
        self.set_breadth_intervals_from(constellation, self.root, 0);
    }

    pub fn set_breadth_intervals_from(&mut self, constellation: &Constellation, block_id: BlockId, start_i: usize) -> usize {
        let mut curr_i = start_i;
        for ch_id in self.get(block_id).children.clone() {
            curr_i = self.set_breadth_intervals_from(constellation, ch_id, curr_i);
        }
        if curr_i == start_i {
            curr_i += 1;
        }
        self.get_mut(block_id).breadth_interval = (start_i, curr_i);
        return curr_i
    }

    // SET STAGE INDICES
    // Must pass reference &Constellation, because functions are called in lazy_static init of CONSTELLATION, so do not support recursion
    pub fn set_stage_indices(&mut self, constellation: &Constellation) {
        let mut stage_indices: BTreeMap<BlockId, (usize, usize, usize)> = BTreeMap::new();
        for (&block_id, block) in self.blocks.iter() {
            if block_id == self.root || block.block_type == BlockType::Skipped {continue}
            let dep_len = self.stage_subtree_dep_len(block_id, constellation);
            stage_indices.insert(
                block_id,
                (
                    dep_len,
                    self.stage_subtree_dep_start(block_id, constellation),
                    self.stage_subtree_dep_end(block_id, dep_len, constellation),
                ),
            );
        }
        for (block_id, (len, start, end)) in stage_indices {
            let mut block = self.get_mut(block_id);
            block.stage_subtree_dep_len = len;
            block.stage_subtree_dep_start = start;
            block.stage_subtree_dep_end = end;
        }
    }

    pub fn stage_subtree_dep_start(&self, id: BlockId, constellation: &Constellation) -> usize {
        assert!(id != self.root);
        let self_stage = Stage::from_comp_typ(constellation.get_comp(self.get(id).comp).typ);
        let parent_id = self.get(id).parent;
        if parent_id != self.root && self_stage == Stage::from_comp_typ(constellation.get_comp(self.get(parent_id).comp).typ) {
            1 + self.stage_subtree_dep_start(parent_id, constellation)
        } else {
            0
        }
    }

    pub fn stage_subtree_dep_end(&self, block_id: BlockId, stage_subtree_dep_len: usize, constellation: &Constellation) -> usize {
        assert!(block_id != self.root);
        let ini_stage = Stage::from_comp_typ(constellation.get_comp(self.get(block_id).comp).typ);
        
        let mut active_blocks_in_subtree = vec![block_id];
        let mut init_to_end_len = 0;
        while active_blocks_in_subtree.len() > 0 {
            init_to_end_len += 1;
            let old = active_blocks_in_subtree.clone();
            active_blocks_in_subtree = vec![];
            for oldy in old {
                active_blocks_in_subtree.append(&mut self.get(oldy).children.clone().into_iter()
                    .filter(|&ch_id| Stage::from_comp_typ(constellation.get_comp(self.get(ch_id).comp).typ) == ini_stage)
                    .collect()
                );
            }
        }
        if init_to_end_len > stage_subtree_dep_len {
            info!("bad: {init_to_end_len} > {stage_subtree_dep_len} id: {block_id}, {}", self.pretty_string(constellation));
        }
        return stage_subtree_dep_len - init_to_end_len;
    }

    pub fn stage_subtree_dep_len(&self, block_id: BlockId, constellation: &Constellation) -> usize {
        // cannot use CONSTELLATION because cant use recursion with lazy static
        // e.g. A1 -> B1 -> B2 ->        C1
        //         -> B3 ->              C2
        //               -> B5 -> B6     C3
        //      A2 -> B7 ->              C1
        // stage_subtree_max_depth of B1, B2 = 2
        // stage_subtree_max_depth of B3, B5, B6 = 3
        // stage_subtree_max_depth of B7 = 1
        assert!(block_id != self.root);
        let ini_stage = Stage::from_comp_typ(constellation.get_comp(self.get(block_id).comp).typ);
        let mut stage_subtree_entry_id = block_id;
        let mut parent_id: BlockId = self.get(stage_subtree_entry_id).parent;

        while 
            parent_id != self.root && 
            Stage::from_comp_typ(constellation.get_comp(self.get(parent_id).comp).typ) == ini_stage 
        {
            stage_subtree_entry_id = parent_id;
            parent_id = self.get(stage_subtree_entry_id).parent;
        }
        
        // stage_subtree_entry_id now set
        let mut active_blocks_in_subtree = vec![stage_subtree_entry_id];
        let mut stage_subtree_dep_len = 0;
        while active_blocks_in_subtree.len() > 0 {
            stage_subtree_dep_len += 1;
            let old = active_blocks_in_subtree.clone();
            active_blocks_in_subtree = vec![];
            for oldy in old {
                active_blocks_in_subtree.append(&mut self.get(oldy).children.clone().into_iter()
                    .filter(|&ch_id| Stage::from_comp_typ(constellation.get_comp(self.get(ch_id).comp).typ) == ini_stage)
                    .collect()
                );
            }
        }
        return stage_subtree_dep_len
    }
 
    pub fn sub_stage_subtrees_dep_len_max(&self, constellation: &Constellation) -> BTreeMap<SubStage, usize> {
        let mut res: BTreeMap<SubStage, usize> = BTreeMap::new();
        let all_stages = self.get_all_substages(constellation);

        for sub in all_stages {
            res.insert(sub, 0);
        }
        
        for (sub, dep_len_max) in res.iter_mut() {
            let mut curr_max_dep_len = 0;
            for (&block_id, block) in self.blocks.iter().filter(|(id, b)| **id != self.root && Stage::from_comp_typ(constellation.get_comp(b.comp).typ) == sub.stage) {
                curr_max_dep_len = curr_max_dep_len.max(block.stage_subtree_dep_len)
            }
            *dep_len_max = curr_max_dep_len;
            
        }
        return res
    }

    // RENDERING TREEj
    /*
    pub fn render(&self, start_row_i: usize, going_to_rig: bool) -> LazyNodes {
        let blocks_rendered = Self::render_block(RefCell::new(self), start_row_i, self.root, going_to_rig);
        return rsx!{blocks_rendered.0}
    }

    pub fn render_block<'a>(self_: RefCell<&Self>, start_row_i: usize, block_id: BlockId, going_to_rig: bool) -> (std::vec::IntoIter<LazyNodes<'a, 'a>>, usize) {
        let mut curr_row_i: usize = start_row_i;
        let mut res_lazy_nodes = vec![].into_iter();

        let ch_ids = self_.borrow().get(block_id).children.clone();
        for ch_id in ch_ids{
            let temp = Self::render_block(self_, curr_row_i, ch_id, going_to_rig);
            res_lazy_nodes = res_lazy_nodes.chain(temp.0).collect::<Vec<LazyNodes>>().into_iter();
            curr_row_i = temp.1;
        }

        if curr_row_i == start_row_i {
            curr_row_i += 1;
        }

        if block_id == self_.borrow().root {
            return (
                res_lazy_nodes,
                curr_row_i,
            )
        } 

        if self_.borrow().get(block_id).block_type == BlockType::Normal {
            let rel_pos = Stage::from_comp_typ(CONSTELLATION.get_comp(self_.borrow().get(block_id).comp).typ)
            .cmp(&Stage::from_comp_typ(CONSTELLATION.get_comp(self_.borrow().targ).typ));

            let pseudo_stage = 
                if rel_pos != std::cmp::Ordering::Equal {
                    "all"
                } else {
                    if self_.borrow().get(block_id).comp == self_.borrow().targ {
                        "mid"
                    } else if self_.borrow().get(block_id).before_targ == going_to_rig {
                        "pre"
                    } else {
                        "post"
                    }
                };

            let (start, end) = if going_to_rig {
                (
                    self_.borrow().get(block_id).stage_subtree_dep_start,
                    1+self_.borrow().get(block_id).stage_subtree_dep_end,
                )
            } else {
                (
                    self_.borrow().get(block_id).stage_subtree_dep_len - self_.borrow().get(block_id).stage_subtree_dep_end-1,
                    self_.borrow().get(block_id).stage_subtree_dep_len - self_.borrow().get(block_id).stage_subtree_dep_start,  
                )
            };


            let grid_col_string = format!("{}-{}-{}-{} / {}-{}-{}-{}",
                Stage::from_comp_typ(CONSTELLATION.get_comp(self_.borrow().get(block_id).comp).typ).short_rep(),
                pseudo_stage,
                start,
                self_.borrow().get(block_id).stage_subtree_dep_len,
                Stage::from_comp_typ(CONSTELLATION.get_comp(self_.borrow().get(block_id).comp).typ).short_rep(),
                pseudo_stage,
                end,
                self_.borrow().get(block_id).stage_subtree_dep_len,
            );

            let cont_string = format!("{}#{}-{} / {}", 
                CONSTELLATION.get_comp(self_.borrow().get(block_id).comp).str_id, 
                self_.borrow().get(block_id).stage_subtree_dep_start,
                1+self_.borrow().get(block_id).stage_subtree_dep_end,
                self_.borrow().get(block_id).stage_subtree_dep_len,
            );

            let comp_id = self_.borrow().get(block_id).comp;

            res_lazy_nodes = res_lazy_nodes.chain(
                vec![rsx!{
                    div {
                        style: "
                            grid-row: {start_row_i} / {curr_row_i};
                            grid-column: {grid_col_string};
                        ",
                        Block{
                            comp_id: comp_id,
                            on_bonk: |e: BlockBoxerEvent| {
                                match e {
                                    BlockBoxerEvent::Skip => {
                                        // self.skip(block_id);
                                    },
                                    BlockBoxerEvent::WUD => {info!("WUD?")},
                                }
                            },
                        }
                    }
                }].into_iter()
            ).collect::<Vec<LazyNodes>>().into_iter();
        }
        
        return (
            res_lazy_nodes,
            curr_row_i,
        )
    }
    */

    pub fn render_backings(&self) -> LazyNodes {

        rsx!{
            for sub_stage in self.get_all_substages(&CONSTELLATION) {
                div {
                    class: "sub_stage_back-{sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()} sub_stage_back",
                    style: "
                        z-index: -1;
                        grid-column: {sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()}-0-1 / {sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()}-1-1;
                        grid-row: 1 / -1;
                    ",
                }
            }
        }
    }

    // TREE OPERATIONS

    pub fn skip(&mut self, block_id: BlockId) {
        assert!(block_id != self.root);
        assert!(self.get(block_id).block_type == BlockType::Normal);
        info!("before skip: {}", self.pretty_string(&CONSTELLATION));
        self.get_mut(block_id).block_type = BlockType::Skipped;
        let block_children = self.get(block_id).children.clone();
        {
            let parent_block = self.get_mut(self.get(block_id).parent);
            parent_block.children.remove(parent_block.children.iter().position(|&ch_id| ch_id == block_id).unwrap());
            for ch_id in block_children.iter() {
                assert!(!parent_block.children.contains(&ch_id));
                parent_block.children.push(*ch_id);
            }
        }
        for ch_id in block_children {
            self.get_mut(ch_id).parent = self.get(block_id).parent;
        }
        
        info!("dedupe");
        self.dedupe_branches_from(self.get(block_id).parent);
        info!("stage inds");
        self.set_stage_indices(&CONSTELLATION);
        info!("grid lines");
        self.set_grid_lines(&CONSTELLATION);
        info!("breadth intervals");
        self.set_breadth_intervals_from(&CONSTELLATION, self.get(block_id).parent, self.get(block_id).breadth_interval.0);
        info!("after skip: {}", self.pretty_string(&CONSTELLATION));
    }

    pub fn skip_batch(&mut self, block_ids: Vec<BlockId>) {
        // dedupes only once, from root
    }

    pub fn cluster(&mut self, block_id: BlockId) {
        assert!(block_id != self.root);
        assert!(self.get(block_id).block_type == BlockType::Normal);
 
        // check if has cluster sibling
        let siblings = self.get(self.get(block_id).parent).children.clone();
        let mut sib_cluster_id = 987;
        for sib_id in siblings {
            match self.get_mut(sib_id).block_type {
                BlockType::GroupCluster{ref mut clustered_nodes} => {
                    clustered_nodes.push(block_id);
                    sib_cluster_id = sib_id;
                    break;
                },
                BlockType::DeleteCluster{ref mut clustered_nodes} => {
                    clustered_nodes.push(block_id);
                    sib_cluster_id = sib_id;
                    break;
                },
                _ => {}
            }
        }
        if !sib_cluster_id == 987 {
            self.get_mut(self.get(block_id).parent).children.retain(|&sib_id| sib_id != block_id);
            for ch_id in self.get(block_id).children.clone() {
                self.get_mut(sib_cluster_id).children.push(ch_id);
            }
            
        } else {
            self.get_mut(block_id).block_type = BlockType::GroupCluster {clustered_nodes: vec![block_id]};
        }

        info!("dedupe");
        self.dedupe_branches_from(self.get(block_id).parent);
        info!("stage inds");
        self.set_stage_indices(&CONSTELLATION);
        info!("grid lines");
        self.set_grid_lines(&CONSTELLATION);
        info!("breadth intervals");
        self.set_breadth_intervals_from(&CONSTELLATION, self.get(block_id).parent, self.get(block_id).breadth_interval.0);
        info!("after skip: {}", self.pretty_string(&CONSTELLATION));
    }

    pub fn dedupe_branches_from(&mut self, from_block_id: BlockId) {
        // Deduplicate branches by ComponentId from id down
        // e.g. (A -> (B -> (C, D, E), B -> (D, E, F) ), G -> C)  with dedup_from(A) becomes (A -> (B -> (C, D, E, F), G -> C))
        let mut ch_ind = 0;
        if self.get(from_block_id).children.len() == 0 {return}
        while ch_ind < self.get(from_block_id).children.len()-1 { // last ele doesn't need dupe check
            let chs = &self.get(from_block_id).children;
            let ch_id = chs[ch_ind];
            
            // find sibling duplicates (same BlockType), link their children to ch_id, delete that duplicate
            let dupes: Vec<BlockId> = chs[ch_ind+1..].to_vec().into_iter().filter(
                |&other_ch_id| {
                    assert!(other_ch_id != ch_id); 
                    self.get(ch_id).comp == self.get(other_ch_id).comp
                }
            ).collect();
            
            for other_ch_id in dupes {
                // remove other from parent
                self.get_mut(self.get(ch_id).parent).children.retain(|&sib_id| sib_id != other_ch_id);

                // link children of other to ch_id
                self.get(other_ch_id).children.clone().iter().for_each(|&other_ch_ch| self.get_mut(other_ch_ch).parent = ch_id);

                // append other children to ch_id
                let mut other_ch_chs: Vec<BlockId> =  self.get_mut(other_ch_id).children.clone();
                self.get_mut(ch_id).children.append(&mut other_ch_chs);
                
                // remove other from tree.items
                self.blocks.remove(&other_ch_id);
            }
            //self.tree.assert_valid(); // will assert false when using cluster nodes
            self.dedupe_branches_from(ch_id);
            ch_ind+= 1;
        }
    }

    pub fn reattach_constellation_at(&mut self, block_id: BlockId) {

    }

}


pub enum TreeEvent {
    Skip(BlockId),
    WUD,
}

#[inline_props]
pub fn TreeComp(cx: Scope, comp_id: ComponentId, tree_type: TreeType) -> Element {
    let tree_use_ref = use_ref(cx, || 
        match tree_type {
            TreeType::LefToRig => {CONSTELLATION.get_comp(*comp_id).lefToRigTree.clone()},
            TreeType::RigToLef => {CONSTELLATION.get_comp(*comp_id).rigToLefTree.clone()},
            TreeType::HourglassToLef => {CONSTELLATION.get_comp(*comp_id).splituptree.clone()},
            TreeType::HourglassToRig => {CONSTELLATION.get_comp(*comp_id).splitdowntree.clone()},
        }
    );

    let update_flag = use_state(cx, || 0);    

    // let tree = tree_use_ref.read();
    // let tree_illegal_borrow = tree_use_ref.try_b

    let substages = tree_use_ref.read().get_all_substages(&CONSTELLATION);
    
    cx.render(rsx!{
        for ch_id in tree_use_ref.read().get(tree_use_ref.read().root).children.iter() {
            BlockComp{
                block_id: *ch_id,
                tree_use_ref: tree_use_ref.clone(),
                on_tree_event: move |e| {
                    match e {
                        TreeEvent::Skip(id) => {
                            info!("try tree.skip({})", id);
                            // info!("{:?}", tree_use_ref);
                            tree_use_ref.write().skip(id);
                            tree_use_ref.needs_update();
                            update_flag.set((update_flag.get() + 1) % 2);
                            // tree_use_ref.with_mut(|t| t.skip(id));
                            info!("tree.skip() succeeded");
                        },
                        TreeEvent::WUD => {}
                    }
                },
                key: "{ch_id}{update_flag.get()}"

            }
        },
        for sub_stage in substages {
            div {
                class: "sub_stage_back-{sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()} sub_stage_back",
                style: "
                    z-index: -1;
                    grid-column: {sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()}-0-1 / {sub_stage.stage.short_rep()}-{sub_stage.pseudostage.short_rep()}-1-1;
                    grid-row: 1 / -1;
                ",
            }
        }
    })
}

#[inline_props]
pub fn BlockComp<'a>(cx: Scope<'a>, block_id: BlockId, tree_use_ref: UseRef<Tree>, on_tree_event: EventHandler<'a, TreeEvent>) -> Element<'a> {
        
    // let tree = tree.read();
    // info!("rendering block: ({}){} in tree with targ: {}", 
    //     block_id, 
    //     CONSTELLATION.get_comp(tree.get(*block_id).comp).str_id, 
    //     CONSTELLATION.get_comp(tree.targ).str_id,
    // );
    let tree = tree_use_ref.read();

    // info!("render block: {}", CONSTELLATION.get_comp(tree.get(*block_id).comp).str_id);

    let going_to_rig = tree.tree_type == TreeType::LefToRig || tree.tree_type == TreeType::HourglassToRig;

    if tree.get(*block_id).block_type == BlockType::Normal {
        let rel_pos = Stage::from_comp_typ(CONSTELLATION.get_comp(tree.get(*block_id).comp).typ)
            .cmp(&Stage::from_comp_typ(CONSTELLATION.get_comp(tree.targ).typ));

        let pseudo_stage = 
            if rel_pos != std::cmp::Ordering::Equal {
                "all"
            } else {
                if tree.get(*block_id).comp == tree.targ {
                    "mid"
                } else if tree.get(*block_id).before_targ == going_to_rig {
                    "pre"
                } else {
                    "post"
                }
            };

        let (start, end) = if going_to_rig {
            (
                tree.get(*block_id).stage_subtree_dep_start,
                1+tree.get(*block_id).stage_subtree_dep_end,
            )
        } else {
            (
                tree.get(*block_id).stage_subtree_dep_len - tree.get(*block_id).stage_subtree_dep_end-1,
                tree.get(*block_id).stage_subtree_dep_len - tree.get(*block_id).stage_subtree_dep_start,  
            )
        };
        // info!("(start, end) = ({}, {})", start, end);

        let grid_col_string = format!("{}-{}-{}-{} / {}-{}-{}-{}",
            Stage::from_comp_typ(CONSTELLATION.get_comp(tree.get(*block_id).comp).typ).short_rep(),
            pseudo_stage,
            start,
            tree.get(*block_id).stage_subtree_dep_len,
            Stage::from_comp_typ(CONSTELLATION.get_comp(tree.get(*block_id).comp).typ).short_rep(),
            pseudo_stage,
            end,
            tree.get(*block_id).stage_subtree_dep_len,
        );

        let cont_string = format!("{}#{}-{} / {}", 
            CONSTELLATION.get_comp(tree.get(*block_id).comp).str_id, 
            tree.get(*block_id).stage_subtree_dep_start,
            1+tree.get(*block_id).stage_subtree_dep_end,
            tree.get(*block_id).stage_subtree_dep_len,
        );

        let comp_id = tree.get(*block_id).comp;

        let mut breadth_interval = tree.get(*block_id).breadth_interval;
        breadth_interval = (breadth_interval.0 + 1, breadth_interval.1 + 1);
        // info!("breadth_interval = {:?}", breadth_interval);

        cx.render(rsx!{
            rsx!{
                div {
                    style: "
                        grid-row: {breadth_interval.0} / {breadth_interval.1};
                        grid-column: {grid_col_string};
                    ",
                    Block{
                        comp_id: comp_id,
                        on_bonk: |e: BlockBoxerEvent| {
                            match e {
                                BlockBoxerEvent::Skip => {
                                    info!("skip block");
                                    on_tree_event.call(TreeEvent::Skip(*block_id));
                                },
                                BlockBoxerEvent::WUD => {info!("WUD?")},
                            }
                        },
                        debug_info: format!("{block_id}"),
                    },
                }
                for ch_id in tree.get(*block_id).children.iter() {
                    BlockComp{
                        block_id: *ch_id,
                        tree_use_ref: tree_use_ref.clone(),
                        on_tree_event: move |e| {on_tree_event.call(e)},
                        key: "{ch_id}",
                    }
                }
                
            }
        })
        
    } else {
        cx.render(rsx!{
            div {"BAD"}
        })
    }
    

}


