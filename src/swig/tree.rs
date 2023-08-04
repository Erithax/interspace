
use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::swig::component::*;
use crate::swig::constellation::*;
use crate::swig::stage;
use log::{info};

use super::CONSTELLATION;
use super::stage::Stage;
use super::block::*;

pub type BlockId = usize;

const ROOT_COMP_ID: ComponentId = 77777;


#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    Normal, 
    Dropped, // dropped because of group cluster
    SkipCluster{
        clustered_nodes: Vec<BlockId>,
    },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeType {
    LefToRig,
    RigToLef,
    Hourglass,
}

impl TreeType {
    pub fn get_lef_to_rig(&self) -> Self {return Self::LefToRig}
    pub fn get_rig_to_lef(&self) -> Self {return Self::RigToLef}
    pub fn get_hourglass(&self) -> Self {return Self::Hourglass}
}

impl std::fmt::Display for TreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::LefToRig => {"LefToRig"},
            Self::RigToLef => {"RigToLef"},
            Self::Hourglass => {"Hourglass"},
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
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Tree {
    pub blocks: BTreeMap<BlockId, Block>,
    pub root: BlockId,
    pub targ: ComponentId,
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
        });
        return Tree {
            blocks: blocks,
            root: block_id,
            targ: ROOT_COMP_ID,
        }
    }

    pub fn new(block_id: BlockId, block: Block, targ: ComponentId) -> Tree {
        let mut blocks: BTreeMap<BlockId, Block> = BTreeMap::new();
        blocks.insert(block_id, block);
        return Tree {
            blocks: blocks,
            root: block_id,
            targ: targ,
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
        return &self.blocks.get(&id).unwrap()
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
        }, targ
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
        }, targ);
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
        }, targ
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
        }, targ);
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

    // SETTING STAGE INDICES
    // Must pass reference &Constellation, because functions are called in lazy_static init of CONSTELLATION, so do not support recursion
    pub fn set_stage_indices(&mut self, constellation: &Constellation) {
        let mut stage_indices: BTreeMap<BlockId, (usize, usize, usize)> = BTreeMap::new();
        for (&block_id, block) in self.blocks.iter() {
            if block_id == self.root {continue}
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

    pub fn stage_subtrees_dep_len_max(&self, constellation: &Constellation) -> BTreeMap<Stage, usize> {
        let mut res: BTreeMap<Stage, usize> = BTreeMap::new();
        for (&block_id, block) in self.blocks.iter() {
            if block_id == self.root {continue}
            let stage = Stage::from_comp_typ(constellation.get_comp(block.comp).typ);
            if !res.contains_key(&stage) ||
                *res.get(&stage).unwrap() < 1 + block.stage_subtree_dep_len {
                res.insert(stage, 1 + block.stage_subtree_dep_len);
            }
        }
        return res
    }

    // RENDERING TREE
    pub fn render(&self, start_row_i: usize, going_to_rig: bool) -> (LazyNodes, usize) {
        info!("{}", self.pretty_string(&CONSTELLATION));
        let blocks_rendered = self.render_block(start_row_i, self.root, going_to_rig);
        return (rsx!{blocks_rendered.0}, blocks_rendered.1)
    }

    pub fn render_block(&self, start_row_i: usize, block_id: BlockId, going_to_rig: bool) -> (std::vec::IntoIter<LazyNodes<'_, '_>>, usize) {
        let mut curr_row_i: usize = start_row_i;
        let mut res_lazy_nodes = vec![].into_iter();

        for ch_id in self.get(block_id).children.iter() {
            let temp = self.render_block(curr_row_i, *ch_id, going_to_rig);
            res_lazy_nodes = res_lazy_nodes.chain(temp.0).collect::<Vec<LazyNodes>>().into_iter();
            info!("{}", temp.1);
            curr_row_i = temp.1;
        }

        if curr_row_i == start_row_i {
            curr_row_i += 1;
        }

        if block_id == self.root {
            return (
                res_lazy_nodes,
                curr_row_i,
            )
        } 

        // info!("{}, {}", self.get(block_id).comp, self.get(self.targ).comp);
        // info!("{}", self.pretty_string(&CONSTELLATION));
   

        let rel_pos = Stage::from_comp_typ(CONSTELLATION.get_comp(self.get(block_id).comp).typ)
            .cmp(&Stage::from_comp_typ(CONSTELLATION.get_comp(self.targ).typ));

        let pseudo_stage = 
            if block_id == self.root || rel_pos != std::cmp::Ordering::Equal {
            "mid"
            } else if self.get(block_id).before_targ == going_to_rig {
                "pre"
            } else {
                "post"
            };

        let start = if going_to_rig {self.get(block_id).stage_subtree_dep_start}
            else {self.get(block_id).stage_subtree_dep_len - self.get(block_id).stage_subtree_dep_start};
        let end = if going_to_rig {1+self.get(block_id).stage_subtree_dep_end}
            else {self.get(block_id).stage_subtree_dep_len - self.get(block_id).stage_subtree_dep_end-1};

        let grid_col_string = format!("{}-{}-{}-{} / {}-{}-{}-{}",
            Stage::from_comp_typ(CONSTELLATION.get_comp(self.get(block_id).comp).typ).short_rep(),
            pseudo_stage,
            start,
            self.get(block_id).stage_subtree_dep_len,
            Stage::from_comp_typ(CONSTELLATION.get_comp(self.get(block_id).comp).typ).short_rep(),
            pseudo_stage,
            end,
            self.get(block_id).stage_subtree_dep_len,
        );

        let cont_string = format!("{}#{}-{} / {}", 
            CONSTELLATION.get_comp(self.get(block_id).comp).str_id, 
            self.get(block_id).stage_subtree_dep_start,
            1+self.get(block_id).stage_subtree_dep_end,
            self.get(block_id).stage_subtree_dep_len,
        );

        res_lazy_nodes = res_lazy_nodes.chain(
            vec![rsx!{
                div {
                    style: "
                        grid-row: {start_row_i} / {curr_row_i};
                        grid-column: {grid_col_string};
                    ",
                    Block{
                        comp_id: self.get(block_id).comp,
                    }
                }
            }].into_iter()
        ).collect::<Vec<LazyNodes>>().into_iter();

        return (
            res_lazy_nodes,
            curr_row_i,
        )
    }

}


