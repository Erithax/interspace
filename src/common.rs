use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use log::info;
use macroni::{self};
use sha2::digest::block_buffer::Block;
use strum::{IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use dioxus::prelude::*;

use crate::indent;
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
/// * enforce alphabetical variants in block enums & add_all



#[derive(PartialEq, Eq, EnumIter, PartialOrd, Ord, Display, Debug, Hash, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    ALL, // used in node branches to leaf with parsetree! to include all the branches to leaf of the previous Block
    ROOT, // most upstream node
    LEAF, // most downstream node
    SELF, // used in node branches to denote itself in the branch

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
            BlockType::ALL =>  {"ALL".to_string()},
            BlockType::SELF => {"SELF".to_string()},
            BlockType::ROOT => {"ROOT".to_string()},
            BlockType::LEAF => {"LEAF".to_string()},
            BlockType::Langbridge(i) => {"Langbridge(".to_string()  + &i.to_string() + ")"},
            BlockType::Ui(i) =>                 {"Ui(".to_string()          + &i.to_string() + ")"},
            BlockType::Layout(i) =>         {"Layout(".to_string()      + &i.to_string() + ")"},
            BlockType::Render(i) =>         {"Render(".to_string()      + &i.to_string() + ")"},
            BlockType::Intergfx(i) =>     {"Intergfx(".to_string()    + &i.to_string() + ")"},
            BlockType::Gfx(i) =>               {"Gfx(".to_string()         + &i.to_string() + ")"},
            BlockType::Platform(i) =>     {"Platform(".to_string()    + &i.to_string() + ")"},
        }
    }

    pub fn from_i<T: Into<i32> + std::fmt::Debug>(i : T) -> BlockType {
        match i.into() {
            -1 => {BlockType::ROOT},
            0 => {BlockType::Langbridge(Langbridge::META)},
            1 => {BlockType::Ui(Ui::META)},
            2 => {BlockType::Layout(Layout::META)},
            3 => {BlockType::Render(Render::META)},
            4 => {BlockType::Intergfx(Intergfx::META)},
            5 => {BlockType::Gfx(Gfx::META)},
            6 => {BlockType::Platform(Platform::META)},
            7 => {BlockType::LEAF},
            _ => {panic!("called BlockType.from_i(i) with i out of range")},
        }
    }

    pub fn inner_to_string(&self) -> String {
        match self {
            Self::Langbridge(i) => {i.to_string()},
            Self::Ui(i) => {i.to_string()},
            Self::Layout(i) => {i.to_string()},
            Self::Render(i) => {i.to_string()},
            Self::Intergfx(i) => {i.to_string()},
            Self::Gfx(i) => {i.to_string()},
            Self::Platform(i) => {i.to_string()},
            _ => {self.stringify()},
        }
    }

    pub fn stage(&self) -> Stage {
        match self {
            Self::ROOT                           => {Stage::ROOT},
            Self::Langbridge(i)     => {Stage::Langbridge},
            Self::Ui(i)                     => {Stage::Ui},
            Self::Layout(i)             => {Stage::Layout},
            Self::Render(i)             => {Stage::Render},
            Self::Intergfx(i)         => {Stage::Gfx},
            Self::Gfx(i)                   => {Stage::Gfx},
            Self::Platform(i)         => {Stage::Platform},
            Self::LEAF                           => {Stage::LEAF}
            _ => {panic!("called BlockType.stage() on {}", self)},
        }
    }

    pub fn is_real(&self) -> bool {
        match self {
            Self::Langbridge(_) => {true},
            Self::Ui(_) => {true},
            Self::Layout(_) => {true},
            Self::Render(_) => {true},
            Self::Intergfx(_) => {true},
            Self::Gfx(_) => {true},
            Self::Platform(_) => {true},
            _ => {false},
        }
    }

    pub fn from_stage(s: Stage) -> BlockType {
        for bt in BlockType::iter_reals() {
            if bt.stage() == s {
                return bt
            }
        }
        panic!("should not be reached");
    }

    pub fn of_same_type(&self, other: BlockType) -> bool {
        if *self == other {return true}
        match (self, other) {
            (BlockType::ALL, BlockType::ALL) =>{true},
            (BlockType::ROOT, BlockType::ROOT) => {true},
            (BlockType::LEAF, BlockType::LEAF) => {true},
            (BlockType::SELF, BlockType::SELF) => {true},
            (BlockType::Langbridge(_), BlockType::Langbridge(_)) =>{true},
            (BlockType::Ui(_), BlockType::Ui(_)) =>{true},
            (BlockType::Layout(_), BlockType::Layout(_)) =>{true},
            (BlockType::Render(_), BlockType::Render(_)) =>{true},
            (BlockType::Intergfx(_), BlockType::Intergfx(_)) =>{true},
            (BlockType::Gfx(_), BlockType::Gfx(_)) =>{true},
            (BlockType::Platform(_), BlockType::Platform(_)) =>{true},
            _ => {false}
        }
    }

    pub fn iter_reals() -> impl Iterator<Item = BlockType>  {
        return BlockType::iter().filter(|&bt|
            match bt {
                BlockType::ALL => {false},
                BlockType::ROOT => {false},
                BlockType::LEAF => {false},
                BlockType::SELF => {false},
                _ => {true}
            }
        ).collect::<Vec<BlockType>>().into_iter()
        
    }
}


#[derive(Debug, PartialEq, Eq, EnumIter, Display, Hash, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum Stage {
    ROOT,
    Langbridge,
    Ui,
    Layout,
    Render,
    Gfx,
    Platform,
    LEAF,
}

impl Stage {
    pub fn lvl(&self) -> i32 {
        match self {
            Stage::ROOT => {-1},
            Stage::Langbridge => {0},
            Stage::Ui => {1},
            Stage::Layout => {2},
            Stage::Render => {3},
            Stage::Gfx => {4},
            Stage::Platform => {5},
            Stage::LEAF => {6},
        }
    }

    pub fn from_i<T: Into<i32> + std::fmt::Debug>(i : T) -> Stage {
        match i.into() {
            -1 => {Stage::ROOT},
            0 => {Stage::Langbridge},
            1 => {Stage::Ui},
            2 => {Stage::Layout},
            3 => {Stage::Render},
            4 => {Stage::Gfx},
            5 => {Stage::Platform},
            6 => {Stage::LEAF},
            _ => {panic!("called stage.from_i(i) with i out of range")},
        }
    }

    pub fn next(&self) -> Stage {
        match self {
            Stage::ROOT => {Stage::Langbridge},
            Stage::Langbridge => {Stage::Ui},
            Stage::Ui => {Stage::Layout},
            Stage::Layout => {Stage::Render},
            Stage::Render => {Stage::Gfx},
            Stage::Gfx => {Stage::Platform},
            Stage::Platform => {Stage::LEAF},
            _ => {panic!("tried calling stage.next on {}", self)},
        }
    }

    pub fn prev(&self) -> Stage {
        match self {
            Stage::Langbridge => {Stage::ROOT},
            Stage::Ui => {Stage::Langbridge},
            Stage::Layout => {Stage::Ui},
            Stage::Render => {Stage::Layout},
            Stage::Gfx => {Stage::Render},
            Stage::Platform => {Stage::Gfx},
            Stage::LEAF => {Stage::Platform},
            _ => {panic!("called stage.prev on {}", self)},
        }
    }

    pub fn first() -> Stage {
        return Stage::Langbridge
    }

    pub fn last() -> Stage {
        return Stage::Platform
    }

    pub fn iter_reals() -> impl Iterator<Item = Stage> {
        return Stage::iter().filter(|&bt|
            match bt {
                Stage::ROOT => {false},
                Stage::LEAF => {false},
                _ => {true}
            }
        ).collect::<Vec<Stage>>().into_iter()
    }

    pub fn iter_blocktypes(&self) -> impl Iterator<Item = BlockType> {
        return BlockType::iter_reals().filter(|&bt|
            bt.stage() == *self
        ).collect::<Vec<BlockType>>().into_iter()
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
impl Ord for Stage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.lvl() < other.lvl() {
            return std::cmp::Ordering::Less
        } else if self.lvl() == other.lvl() {
            return std::cmp::Ordering::Equal
        } else {
            return std::cmp::Ordering::Greater
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
impl Default for Info {
    fn default() -> Self {
        return Info {
            name: "DEFAULT".to_string(),
            owner: Owner::Erithax,
            desc: "DEFAULT".to_string(),
            imp_lang: Lang:: TODO,
            bind_langs: vec![Lang::TODO],
        }
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

pub type SingleBlockId = usize;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SingleBlockTree {
    pub items: std::collections::BTreeMap<SingleBlockId, SingleBlock>,
    pub root: SingleBlockId,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SingleBlock {
    pub parent: SingleBlockId,
    pub data: BlockType,
    pub children: Vec<SingleBlockId>,
}

impl SingleBlockTree {
    pub fn new_blank() -> SingleBlockTree {
        return SingleBlockTree {
            items: std::collections::BTreeMap::new(),
            root: 0,
        };
    }

    pub fn assert_valid(&self) {
        // bidirectional consistency
        for (id, sb) in self.items.iter() {
            if *id != self.root {
                assert!(self.get(self.get(*id).parent).children.contains(id));
            }
            for ch in self.get(*id).children {
                assert!(self.get(ch).parent == *id);
            }
        }

        // every node in self.items is in tree
        for (id, sb) in self.items.iter() {
            assert!(self.crawl_tree_contains(self.root, *id));
        }
        // root in self.items
        assert!(self.items.contains_key(&self.root));

        // every node in tree is in self.items
        assert!(self.crawl_items_contains(self.root));

        // every id appears only once in tree
        for (id, _) in self.items.iter() {
            assert!(self.crawl_tree_count(self.root, *id) == 1);
        }
        
    }

    fn crawl_tree_contains(&self, id: SingleBlockId, targ: SingleBlockId) -> bool {
        if id == targ {return true}
        return self.get(id).children.iter().any(|ch_id| self.crawl_tree_contains(*ch_id, targ))
    }

    fn crawl_items_contains(&self, id: SingleBlockId) -> bool {
        return self.items.contains_key(&id);
    }

    fn crawl_tree_count(&self, curr_id: SingleBlockId, targ: SingleBlockId) -> usize {
        let child_count: usize = self.get(curr_id).children.iter().map(|ch_id| self.crawl_tree_count(*ch_id, targ)).sum();
        let my_count = if curr_id == targ {1} else {0};
        return my_count + child_count
    }

    fn new(root_id: SingleBlockId, root_data: SingleBlock) -> SingleBlockTree {
        let mut res =  SingleBlockTree {
            items: std::collections::BTreeMap::new(),
            root: root_id,
        };
        res.items.insert(root_id, root_data);
        return res
    }

    pub fn get_root(&self) -> SingleBlockId {
        return self.root
    }

    pub fn get(&self, id: SingleBlockId) -> SingleBlock {
        assert!(self.contains(id), "NO GOOD: {self}, {id}");
        return self.items.get(&id).unwrap().clone()
    }

    pub fn get_mut(&mut self, id: SingleBlockId) -> &mut SingleBlock {
        assert!(self.contains(id));
        return self.items.get_mut(&id).unwrap()
    }

    pub fn contains(&self, id: SingleBlockId) -> bool {
        return self.items.contains_key(&id);
    }

    pub fn add_item(&mut self, id: SingleBlockId, sb: SingleBlock) {
        assert!(!self.contains(id));
        self.items.insert(id, sb);
    }

    pub fn full_trees_from_paths_containing(paths: &Vec<Vec<BlockType>>, target_block: BlockType) -> (SingleBlockTree, SingleBlockTree) {
        assert!(paths.iter().enumerate().all(|(i, path)| paths.iter().enumerate().any(
            |(j, other_path)| i == j || path != other_path
        )));
        let mut id: SingleBlockId = 0;

        let root_block = SingleBlock {
            parent: id,
            data: BlockType::LEAF,
            children: vec![],
        };
        let mut uptree = Self::new(id, root_block);
        id += 1;

        let root_block = SingleBlock {
            parent: id,
            data: BlockType::ROOT,
            children: vec![],
        };
        let mut downtree = Self::new(id, root_block);
        id += 1;

        let mut paths: Vec<&Vec<BlockType>> = paths.iter().collect();

        // remove paths without target
        paths.retain(|path: &&Vec<BlockType>| path.contains(&target_block));

        assert!(paths.iter().all(|path| path.contains(&target_block)), "{}, {paths:?}", target_block.inner_to_string());

        let sole_targ_path = vec![target_block];
        if paths.len() == 0 {paths.push(&sole_targ_path);}

        // remove duplicate subpaths
        let mut new_paths: Vec<&Vec<BlockType>> = vec![];
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
            
            // ADD PATH TO UPTREE
            let mut curr_node_id: usize = uptree.root;
            for i in (0..path.len()).rev() {
                let already_in_tree: Option<usize> = uptree.get(curr_node_id).children.into_iter().find(|id| uptree.get(*id).data == path[i]);
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        uptree.add_item(id, SingleBlock {
                            parent: curr_node_id,
                            children: Vec::new(),
                            data: path[i],
                        });
                        uptree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }

            // ADD PATH TO DOWNTREE
            let mut curr_node_id: usize = downtree.root;
            for i in 0..path.len(){
                let already_in_tree: Option<usize> = downtree.get(curr_node_id).children.into_iter().find(|id| downtree.get(*id).data == path[i]);
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        downtree.add_item(id, SingleBlock {
                            parent: curr_node_id,
                            children: Vec::new(),
                            data: path[i],
                        });
                        downtree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }
        }
        return (downtree, uptree)
    }

    pub fn split_trees_from_paths_with_target_root(paths: &Vec<Vec<BlockType>>, root_bt: BlockType) -> (SingleBlockTree, SingleBlockTree) {
        let mut id: SingleBlockId = 0;

        let root_block = SingleBlock {
            parent: id,
            data: root_bt,
            children: vec![],
        };
        let mut uptree = Self::new(id, root_block);
        id += 1;

        let root_block = SingleBlock {
            parent: id,
            data: root_bt,
            children: vec![],
        };
        let mut downtree = Self::new(id, root_block);
        id += 1;

        for path in paths.into_iter().filter(|path| path.contains(&root_bt)) {
            assert!(path.contains(&root_bt));
            
            let root_index = path.iter().position(|bt| *bt == root_bt)
                .expect(&format!("invalid path {path:?} does not contain root bt {root_bt}"));

            
            // ADD PATH TO UPTREE
            let mut curr_node_id: usize = uptree.root;
            for i in (0..root_index).rev() {
                let already_in_tree: Option<usize> = uptree.get(curr_node_id).children.into_iter().find(|id| uptree.get(*id).data == path[i]);
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        uptree.add_item(id, SingleBlock {
                            parent: curr_node_id,
                            children: Vec::new(),
                            data: path[i],
                        });
                        uptree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }


            // ADD PATH TO DOWNTREE
            let mut curr_node_id: usize = downtree.root;
            for i in root_index+1..path.len(){
                let already_in_tree: Option<usize> = downtree.get(curr_node_id).children.into_iter().find(|id| downtree.get(*id).data == path[i]);
                match already_in_tree {
                    Some(matching_id) => {
                        curr_node_id = matching_id;
                    },
                    None => {
                        downtree.add_item(id, SingleBlock {
                            parent: curr_node_id,
                            children: Vec::new(),
                            data: path[i],
                        });
                        downtree.get_mut(curr_node_id).children.push(id);
                        curr_node_id = id;
                        id += 1;
                    }
                }
            }
        }

        return (downtree, uptree)
    }

    pub fn leaf_count(&self) -> usize {
        return self.items.iter().fold(0, |acc, (_, sb)| if sb.children.len() == 0 {acc + 1} else {acc})
    }

    pub fn leaf_count_from(&self, id: SingleBlockId) -> usize {
        if self.get(id).children.len() == 0 {
            return 1
        }
        return self.get(id).children.iter().fold(0, |acc, ch_id| acc + self.leaf_count_from(*ch_id))
    }

    pub fn max_depth_from_id_for_which<F: Fn(SingleBlock) -> bool>(&self, id: SingleBlockId, f: &F) -> usize {
        // maximum depth from node id to any child (and grandchild etc.) node that satisfies the condition
        let mut max_dep = 0;
        if !f(self.get(id)) {
            return 0;
        }
 
        for ch_id in self.get(id).children {
            let dep = self.max_depth_from_id_for_which(ch_id, f);
            if dep > max_dep {
                max_dep = dep;
            }  
        }

        return max_dep + 1
    }

    pub fn acc_wid_range(&self, id: StageBlockId, start_i: usize, entries: &mut HashMap<StageBlockId, (usize, usize)>) -> usize {
        let mut end_i = start_i;
        if self.get(id).children.len() == 0 {
            end_i += 1;
            entries.insert(id, (start_i, start_i+1));
        } else {
            for ch_id in self.get(id).children {
                end_i = self.acc_wid_range(ch_id, end_i, entries);
            }
            entries.insert(id, (start_i, end_i));
        }
        return end_i
    }

    pub fn get_stage(&self, id: SingleBlockId) -> Stage {
        let sb = self.get(id);
        assert!(sb.data != BlockType::ALL);
        return sb.data.stage()
    }

    pub fn get_path_to_leaf_strings(&self, id: SingleBlockId) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let me: String = format!("({}){}", id, self.get(id).data.inner_to_string());
        if self.get(id).children.len() == 0 {
            return vec![me]
        }

        let mut very_first = true;
        for ch in self.get(id).children.iter(){
            let nexts = self.get_path_to_leaf_strings(*ch);
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

    pub fn remove(&mut self, id: SingleBlockId) {
        assert!(id != self.root);
        assert!(self.items.contains_key(&id));
        let sb = self.get(id);
        {
            let parent = self.get_mut(sb.parent);

            // remove id from parent
            parent.children.retain(|sib_id| *sib_id != id);
            
            // add id's children to parent
            for ch_id in sb.children.iter() {
                assert!(!parent.children.contains(ch_id));
                parent.children.push(*ch_id);
            }
        }
        // change parent of id's children
        for ch_id in sb.children {
            self.get_mut(ch_id).parent = sb.parent;
        }

        self.items.remove(&id);
    }
}

impl std::fmt::Display for SingleBlockTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "SingleBlockTree:\n{}", self.get_path_to_leaf_strings(self.root).iter().fold("".to_owned(), |acc, e| acc + e + "\n"))
    }
}

impl Default for SingleBlockTree {
    fn default() -> Self {
        return SingleBlockTree::new_blank()
    }
}


pub type StageBlockId = usize;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct StageBlockTree {
    descendants: std::collections::BTreeMap<StageBlockId, StageBlock>,
    root: Option<StageBlockId>,
}

impl StageBlockTree {
    fn new() -> StageBlockTree {
        return StageBlockTree{
            descendants: std::collections::BTreeMap::<StageBlockId, StageBlock>::new(),
            root: None,
        }
    }

    pub fn from_root_to_targ_paths(paths: Vec<&[Vec<BlockType>]>, bt: BlockType) -> StageBlockTree {
        return Self::from_paths_with_dir(paths, bt, false)
    }

    pub fn from_targ_to_leaf_paths(paths: Vec<&[Vec<BlockType>]>, bt: BlockType) -> StageBlockTree{
        return Self::from_paths_with_dir(paths, bt, true)
    }

    fn from_paths_with_dir(paths: Vec<&[Vec<BlockType>]>, bt: BlockType, down: bool) -> StageBlockTree {
        // assert!(paths.len() > 0, "{}, {down}", bt.inner_to_string());
        let mut res = Self::new();
        let mut id: StageBlockId = 0;

        res.descendants.insert(id, StageBlock{
            parent: id,
            children: Vec::new(),
            data: vec![bt.clone()],
        });
        res.root = Some(id);
        id += 1;

        if paths.len() == 0 {
            return res
        }

        let path_root_i: i32 = 0;
        let path_leaf_i: i32 = (paths[0].len()-1) as i32;
        let path_inc: i32 = 1;
        let mut new_paths: Vec<Vec<Vec<BlockType>>> = Vec::new();

        for path in paths.iter() {
            let mut new_path = path.to_vec();
            if !down {
                new_path.reverse();
                new_path.iter_mut().for_each(|x| x.reverse());
            }

            
            let mut base = new_path[path_root_i as usize].clone();
            assert!(base.contains(&bt));
            base = base[base.iter().position(|b| *b==bt).unwrap()+1..].to_vec();
            assert!(!base.contains(&bt), "{path:?}, {new_path:?}");

            if base.len() == 0 && new_path.iter().skip(1).all(|sbp| sbp.is_empty()) {continue}; // skip fully empty paths to leaf
           
            if !res.contains_data(&base) {
                res.descendants.insert(id, StageBlock{
                    parent: res.root.unwrap(),
                    children: Vec::new(),
                    data: base.clone(),
                });
                res.get_mut_by_id(&res.root.unwrap()).unwrap().children.push(id);
                id += 1;
            };
            new_path[path_root_i as usize] = base;
            new_paths.push(new_path);
        }

        if paths[0].len() == 1 {
            // paths are from first stage up or last stage down
            return res
        }

        if new_paths.len() == 0 {
            assert!(res.descendants.len() == 1);
            // only empty paths to leaf exist, which were all filtered out, now adding one back in
            
            res.descendants.insert(id, StageBlock{
                parent: res.root.unwrap(),
                children: Vec::new(),
                data: vec![],
            });
            res.get_mut_by_id(&res.root.unwrap()).unwrap().children.push(id);
            id += 1;
            
            let mut emp_vec = Vec::new();
            for i in 0..paths[0].len() {
                emp_vec.push(vec![]);
            }
            new_paths.push(emp_vec);
        }


        for path in &new_paths {
            // print!("{}",res.pretty_string());
            // dbg!(&bt);
            // dbg!(&path);
            let (mut last_matching_path_i, mut last_matching_id) = res.get_corresponding_indices_of_longest_matching_datapath(path, down);
            // dbg!(format!("newpath"));
            while last_matching_path_i != path_leaf_i as usize {
                // dbg!(format!("{path_root_i}, {path_leaf_i} || {last_matching_path_i}, {last_matching_id}"));
                // assert!(path.len() > (last_matching_path_i as i32 +path_inc) as usize, "\n\tpath: {path:?}\n\tlast_path_i: {last_matching_path_i}\n\tlast_id: {last_matching_id}\n{}", res.pretty_string());
                res.get_mut_by_id(&last_matching_id).unwrap().children.push(id);
                res.descendants.insert(id, StageBlock{
                    parent: last_matching_id,
                    children: vec![],
                    data: path[(last_matching_path_i as i32 +path_inc) as usize].to_vec(),
                });
                last_matching_id = id;
                id += 1;
                last_matching_path_i = (last_matching_path_i as i32 + path_inc) as usize;
     
            }
        }
        return res
    }

    pub fn contains_stage_block(&self, stageblock: &StageBlock) -> bool {
        for (id, sb) in self.descendants.iter() {
            if *sb == *stageblock {return true}
        }
        return false
    }

    pub fn contains_id(&self, stage_block_id: &StageBlockId) -> bool {
        for (id, sb) in self.descendants.iter() {
            if *id == *stage_block_id {return true}
        }
        return false
    }

    pub fn contains_data(&self, data: &Vec<BlockType>) -> bool {
        for (id, sb) in self.descendants.iter() {
            if sb.data == *data {return true}
        }
        return false
    }

    pub fn get_corresponding_indices_of_longest_matching_datapath(&self, data_path: &[Vec<BlockType>], down: bool) -> (usize, StageBlockId) {
        assert!(data_path.len() > 0);
        
        let start_i: i32 = 0;
        let end_i: i32 = (data_path.len()-1) as i32;
        let inc: i32 = 1;
        
        let mut last_matching_path_i: i32 = start_i - inc;
        let mut candidate_path_i: i32;
        let mut last_matching_node: StageBlockId = self.root.unwrap();
        let mut candidate_nodes: Vec<StageBlockId>;

        // assert_eq!(data_path[0], self.descendants.get(&self.root.unwrap()).unwrap().data);

     

        // println!("doing: {start_i} -> {end_i}");
        loop {
            candidate_path_i = last_matching_path_i + inc;
            candidate_nodes = self.get_owned_by_id(&last_matching_node).unwrap().children.clone();
            candidate_nodes.retain(|id| self.get_owned_by_id(id).unwrap().data == data_path[candidate_path_i as usize]);
            
            assert!(candidate_nodes.len() == 1 || candidate_nodes.len() == 0, "\n\tdata_path: {data_path:?} di_prev: {candidate_path_i}\n\tprev_node: {last_matching_node:?} {:?} \n\tcandidate_nodes: {candidate_nodes:?} {:?} \nDOWN: {}\n{}", self.descendants.get(&last_matching_node).unwrap().data, candidate_nodes.iter().map(|id| &self.descendants.get(&last_matching_node).unwrap().data).collect::<Vec<_>>(), down, self.pretty_string());
            
            // println!("{last_matching_path_i} -> {candidate_path_i}\n{last_matching_node} -> {candidate_nodes:?}");

            if candidate_nodes.len() == 0 {break}

            last_matching_node = candidate_nodes[0];
            last_matching_path_i = candidate_path_i;

            if candidate_path_i == end_i {break}
        }
        assert!(last_matching_path_i != start_i - inc, "{data_path:?} {}", self.pretty_string());
        return (last_matching_path_i as usize, last_matching_node)
    }

    pub fn get_ref_by_id(&self, stage_block_id: &StageBlockId) -> Option<&StageBlock> {
        for (id, sb) in self.descendants.iter() {
            if *id == *stage_block_id {return Some(sb)}
        }
        return None
    }

    pub fn get_owned_by_id(&self, stage_block_id: &StageBlockId) -> Option<StageBlock> {
        for (id, sb) in self.descendants.iter() {
            if *id == *stage_block_id {return Some((*sb).clone())}
        }
        return None
    }

    pub fn get_mut_by_id(&mut self, stage_block_id: &StageBlockId) -> Option<&mut StageBlock> {
        for (id, sb) in self.descendants.iter_mut() {
            if *id == *stage_block_id {return Some(sb)}
        }
        return None
    }

    pub fn get_owned_by_data(&self, stage_block_data: Vec<BlockType>) -> Vec<StageBlock> {
        let mut res: Vec<StageBlock> = Vec::new();
        for (id, sb) in self.descendants.iter() {
            if sb.data == stage_block_data {res.push(sb.clone())}
        }
        return res
    }

    pub fn assert_constant_depth(&self) {
        let mut leafs = vec![self.root.unwrap()];
        let depth = 1;
        loop {
            assert!(leafs.len() > 0);
            let deeper_node_found = self.get_ref_by_id(&leafs[0]).unwrap().children.len() > 0;
            let mut new_leafs = vec![];
            for leaf_id in leafs.iter() {
                assert!((self.get_ref_by_id(&leaf_id).unwrap().children.len() > 0) == deeper_node_found, "{}", self.pretty_string());
                new_leafs.append(&mut self.get_ref_by_id(&leaf_id).unwrap().children.clone());
            }
            assert!(deeper_node_found == (new_leafs.len() > 0));
            if !deeper_node_found {break}
            leafs = new_leafs;
        }
    }

    pub fn get_depth(&self) -> i32 {
        let mut node = &self.root.unwrap();
        let mut depth = 1;
        while !self.descendants.get(node).unwrap().children.is_empty() {
            node = self.descendants.get(node).unwrap().children.first().unwrap();
            depth += 1;
        }
        return depth
    }

    pub fn better_print(&self) {
        println!("TREE");
        for (id, sb) in self.descendants.iter() {
            let mut d : String = String::new();
            for b in sb.data.iter() {
                d = d + b.inner_to_string().as_str() + ", ";
            }
            println!("\t{} -> {:?} :: {}", id, sb.children, d)
        }
    }

    pub fn pretty_string(&self) -> String {
        let mut res: String = String::new();
        res += "TREE: -----------\n";
        let lines = self.get_path_to_leaf_strings(self.root.unwrap());
        for line in lines {
            res = res + line.as_str() + "\n";
        }
        res += "\n-----------------\n";
        return res
    }

    pub fn pretty_print(&self) {
        let lines = self.get_path_to_leaf_strings(self.root.unwrap());
        for line in lines {
            println!("{line}");
        }
    }

    pub fn get_path_to_leaf_strings(&self, id: StageBlockId) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let me: String = format!("{:?}", self.get_ref_by_id(&id).unwrap().data);
        for ch in self.get_ref_by_id(&id).unwrap().children.iter() {
            let nexts = self.get_path_to_leaf_strings(*ch);
            for n in nexts {
                res.push(me.clone() + "->" + n.as_str())
            }
        }
        if res.len() == 0 {
            return vec![me]
        }
        return res
    }

}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct StageBlock {
    parent: StageBlockId,
    pub data: Vec<BlockType>,
    children: Vec<StageBlockId>,
}

impl Default for StageBlockTree {
    fn default() -> Self {
        return StageBlockTree::new()
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Node {
    pub bt: BlockType,
    #[serde(skip)]
    pub info: Info,
    pub extra: ExtraInfo,

    pub paths: Vec<usize>,
    pub staged_paths: Vec<usize>,
    pub fulldowntree: SingleBlockTree,
    pub fulluptree: SingleBlockTree,
    pub splitdowntree: SingleBlockTree,
    pub splituptree: SingleBlockTree,
    pub blockdowntree: StageBlockTree,
    pub blockuptree: StageBlockTree,
}

impl Node {
    pub fn get_info(&self) -> &Info {
        return &self.info
    }

    pub fn assert_valid(&self) {

        // assert!(self.own_downtree_stage_blocks().len() == self.own_uptree_stage_blocks().len(), "DOWN: {}\n{}, \nUP: {}\n{}", self.own_downtree_stage_blocks().len(), self.downtree.pretty_string(), self.own_uptree_stage_blocks().len(), self.uptree.pretty_string());
        // for id_down in self.own_downtree_stage_blocks() {
        //     let mut also_in_uptree = false;
        //     for id_up in self.own_uptree_stage_blocks() {
        //         if self.downtree.get_owned_by_id(id_down).unwrap().data == self.uptree.get_owned_by_id(id_up).unwrap().data {
        //             also_in_uptree = true
        //         }
        //     }
        //     assert!(also_in_uptree, "\nDOWN: {} \nUP: {} \nSB:{id_down}: {:?}", self.downtree.pretty_string(), self.uptree.pretty_string(), self.downtree.get_owned_by_id(id_down));
        // }

        let st = self.bt.stage().lvl();
        let up_depth = st + 1 + 1;

        let down_depth = Stage::last().lvl() - st + 1 + 1;

        self.blockdowntree.assert_constant_depth();
        self.blockuptree.assert_constant_depth();

        // assert_eq!(self.downtree.get_depth(), down_depth);
        // assert_eq!(self.uptree.get_depth(), up_depth)

        //TODO: check parent-children consistency
    }

    pub fn get_extra(&self) -> &ExtraInfo {
        return &self.extra
    }

    pub fn get_up_wid(&self) -> usize {
        return self.splituptree.leaf_count();
    }

    pub fn get_down_wid(&self) -> usize {
        return self.splitdowntree.leaf_count();
    }

    pub fn get_up_wids(&self) -> HashMap<SingleBlockId, (usize, usize)> {
        let mut res = HashMap::new();
        self.splituptree.acc_wid_range(self.splituptree.root, 0, &mut res);
        return res;
    }

    pub fn get_down_wids(&self) -> HashMap<SingleBlockId, (usize, usize)> {
        let mut res = HashMap::new();
        self.splitdowntree.acc_wid_range(self.splitdowntree.root, 0, &mut res);
        return res;
    }

    pub fn get_up_nei_single_blocks(&self) -> Vec<(SingleBlockId, BlockType)> {
        return self.get_up_nei_single_blocks_of(self.splituptree.root)
    }

    pub fn get_up_nei_single_blocks_of(&self, id: SingleBlockId) -> Vec<(SingleBlockId, BlockType)> {
        let mut res = vec![];
        for ch in self.splituptree.get(id).children {
            res.push((ch, self.splituptree.get(ch).data));
        }
        return res
    }

    pub fn get_down_nei_single_blocks(&self) -> Vec<(SingleBlockId, BlockType)> {
        return self.get_down_nei_single_blocks_of(self.splitdowntree.root)
    }

    pub fn get_down_nei_single_blocks_of(&self, id: SingleBlockId) -> Vec<(SingleBlockId, BlockType)> {
        let mut res = vec![];
        for ch in self.splitdowntree.get(id).children {
            res.push((ch, self.splitdowntree.get(ch).data));
        }
        return res
    }

    pub fn own_uptree_stage_blocks(&self) -> &Vec<StageBlockId> {
        return &self.blockuptree.descendants.get(&self.blockuptree.root.unwrap()).unwrap().children
    }

    pub fn own_downtree_stage_blocks(&self) -> &Vec<StageBlockId> {
        return &self.blockdowntree.descendants.get(&self.blockdowntree.root.unwrap()).unwrap().children
    }

    pub fn get_up_nei_stage_blocks(&self) -> Vec<(StageBlockId, Vec<BlockType>)> {
        let mut res: Vec<(StageBlockId, Vec<BlockType>)> = Vec::new();
        for sb in self.own_uptree_stage_blocks() {
            res.push((*sb, self.blockuptree.descendants.get(sb).expect(&format!("{sb}, self")).data.clone()));
        }
        return res
    }

    pub fn get_down_nei_stage_blocks(&self) -> Vec<(StageBlockId, Vec<BlockType>)> {
        let mut res: Vec<(StageBlockId, Vec<BlockType>)> = Vec::new();
        for sb in self.own_downtree_stage_blocks() {
            res.push((*sb, self.blockdowntree.descendants.get(sb).unwrap().data.clone()));
        }
        return res
    }

    pub fn get_up_nei_sb_ids_of(&self, target: StageBlockId) -> &Vec<StageBlockId> {
        return &self.blockuptree.descendants.get(&target).unwrap().children
    }

    pub fn get_down_nei_sb_ids_of(&self, target: StageBlockId) -> &Vec<StageBlockId> {
        return &self.blockdowntree.descendants.get(&target).unwrap().children
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {}\n\t{}\n\tinfo: {:?}\n\textra: {:?}\n\tpaths: {:?}\n\tstage_blocks:{:?}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            self.bt.to_string(), 
            self.bt.inner_to_string(), 
            self.info, 
            self.extra, 
            self.paths, 
            self.staged_paths,
            indent(&self.fulldowntree.to_string()),
            indent(&self.fulluptree.to_string()),
            indent(&self.splitdowntree.to_string()),
            indent(&self.splituptree.to_string()),
            indent(&self.blockdowntree.pretty_string()),
            indent(&self.blockuptree.pretty_string()),
        )
    }
}

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// pub struct Digraph {
//     graph: Graph<BlockType, (), Directed>,
// }

// impl Digraph {
    
//     fn add(&mut self, bt: &BlockType) {
//         assert!(!self.contains(&bt));
//         self.graph.add_node(*bt);
//     }

//     fn contains(&self, bt: &BlockType) -> bool {
//         return self.graph.node_indices().any(|x| self.graph.node_weight(x).unwrap() == bt)
//     }

//     fn index_from_blocktype(&self, bt: &BlockType) -> NodeIndex {
//         assert!(self.contains(bt));

//         for i in self.graph.node_indices() {
//             if self.graph.node_weight(i).unwrap() == bt {
//                 return i
//             }
//         }
//         panic!("node with BlockType {} not in graph", bt);
//     }

//     fn link(&mut self, from: &BlockType, to: &BlockType) {
//         assert!(from != to, "from: {} is not equal to to: {}", from, to);
//         assert!(self.contains(from), "self: {:?} does not contain from: {}", self, from);
//         assert!(self.contains(to), "self: {:?} does not contain to: {}, {}", self, to, to.stringify());

//         self.graph.add_edge(self.index_from_blocktype(from), self.index_from_blocktype(to), ());
//     }

//     fn get_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
//         let mut res: Vec<BlockType> = Vec::new();
//         for i in self.graph.neighbors(self.index_from_blocktype(bt)) {
//             res.push(*self.graph.node_weight(i).unwrap());
//         }
//         return res
//     }

//     fn get_up_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
//         let mut res: Vec<BlockType> = Vec::new();
//         for i in self.graph.neighbors_directed(self.index_from_blocktype(bt), Direction::Outgoing) {
//             res.push(*self.graph.node_weight(i).unwrap());
//         }
//         return res
//     }

//     fn get_down_neighbors(&self, bt: &BlockType) -> Vec<BlockType> {
//         let mut res: Vec<BlockType> = Vec::new();
//         for i in self.graph.neighbors_directed(self.index_from_blocktype(bt), Direction::Incoming) {
//             res.push(*self.graph.node_weight(i).unwrap());
//         }
//         return res
//     }

//     fn is_downstream_from(&self, one: &BlockType, two: &BlockType) -> bool {
//         assert!(self.contains(one));
//         assert!(self.contains(two));
//         assert!(one != two);
//         for bt in self.get_down_neighbors(two) {
//             if bt == *two {
//                 return true
//             } else if self.is_downstream_from(one, &bt){
//                 return true
//             }
//         }
//         return false
//     }

//     fn is_upstream_from(&self, one: &BlockType, two: &BlockType) -> bool {
//         assert!(self.contains(one));
//         assert!(self.contains(two));
//         assert!(one != two);
//         for bt in self.get_up_neighbors(two) {
//             if bt == *two {
//                 return true
//             } else if self.is_upstream_from(one, &bt){
//                 return true
//             }
//         }
//         return false
//     }

//     pub fn get_all_downstream_from(&self, block: &BlockType) -> Vec<BlockType> {
//         assert!(self.contains(block));
//         let mut res: Vec<BlockType> = Vec::new();
//         for other in self.graph.node_weights() {
//             if self.is_downstream_from(other, block) {
//                 res.push(*other);
//             }
//         }
//         return res
//     }

//     pub fn get_all_upstream_from(&self, block: &BlockType) -> Vec<BlockType> {
//         assert!(self.contains(block));
//         let mut res: Vec<BlockType> = Vec::new();
//         for other in self.graph.node_weights() {
//             if self.is_upstream_from(other, block) {
//                 res.push(*other);
//             }
//         }
//         return res
//     }

//     /// Insert shadow nodes BlockType::SHADOW between linked nodes with non-sequential stages
//     /// e.g. between node1 of stage Langbridge and node2 of stage Render, 2 shadow nodes get inserted
//     pub fn insert_shadow_nodes(&mut self) {

//     }

//     /// Get subgraph of self, including `block` and all its downstream nodes
//     pub fn get_downstream_subgraph(&self, block: &BlockType) -> Digraph {
//         let mut res: Digraph = Digraph {
//             graph: Graph::<BlockType, (), Directed>::new()
//         };
//         res.add(block);
//         for bt in self.get_all_downstream_from(block) {
//             res.add(&bt);
//         }

//         for bt in self.get_all_downstream_from(block) {
//             for e in self.graph.edges_directed(self.index_from_blocktype(&bt), Direction::Incoming) {
//                 res.link(self.graph.node_weight(e.source()).unwrap(), self.graph.node_weight(e.target()).unwrap());
//             }
//         }
//         return res
//     }

//     /// Get subgraph of self, including `block` and all its upstream nodes
//     pub fn get_upstream_subgraph(&self, block: &BlockType) -> Digraph {
//         let mut res: Digraph = Digraph {
//             graph: Graph::<BlockType, (), Directed>::new()
//         };
//         res.add(block);
//         for bt in self.get_all_upstream_from(block) {
//             res.add(&bt);
//         }

//         for bt in self.get_all_upstream_from(block) {
//             for e in self.graph.edges_directed(self.index_from_blocktype(&bt), Direction::Incoming) {
//                 res.link(self.graph.node_weight(e.source()).unwrap(), self.graph.node_weight(e.target()).unwrap());
//             }
//         }
//         return res
//     }

//     /// Returns true if  block has no outgoing edges
//     pub fn is_leaf(&self, block: &BlockType) -> bool {
//         return self.graph.edges_directed(self.index_from_blocktype(block), Direction::Outgoing).next().is_none()
//     }

//     /// Returns true if  block has no outgoing edges
//     pub fn is_root(&self, block: &BlockType) -> bool {
//         return self.graph.edges_directed(self.index_from_blocktype(block), Direction::Incoming).next().is_none()
//     }

// }

/*
Constellation is an interface for the graph using BlockType instances instead of the internally used node indices.
*/
#[derive(Debug, serde::Deserialize, serde::Serialize)] 
pub struct Constellation {
    data: std::collections::BTreeMap<BlockType, Node>, // MUST NOT BE HASHMAP, ORDER OF ITERATION MUST BE FIXED
    paths: Vec<Vec<BlockType>>,
    staged_paths: Vec<Vec<Vec<BlockType>>>,
}

impl Constellation {

    pub fn generate() -> Constellation {
        // info!("generating tree");
        let mut blockies: Vec<(BlockType, Info, ExtraInfo, Vec<Vec<BlockType>>)> = Vec::new();
    
        #[macro_export]
        macro_rules! validate_enum_and_add_all {
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

    
        validate_enum_and_add_all!(Langbridge);
        validate_enum_and_add_all!(Ui);
        validate_enum_and_add_all!(Layout);
        validate_enum_and_add_all!(Render);
        validate_enum_and_add_all!(Intergfx);
        validate_enum_and_add_all!(Gfx);
        validate_enum_and_add_all!(Platform);
    
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
            data: std::collections::BTreeMap::<BlockType, Node>::new(),
            paths: Vec::new(),
            staged_paths: Vec::new(),
        };
    
        for (bt, _, _, paths) in blockies.iter_mut() {
            for (path_i, path) in paths.iter_mut().enumerate() {
                let self_b = path.iter_mut().find(|b| **b == BlockType::SELF).unwrap();
                *self_b = *bt;
            }
        }

        assert!(blockies.iter().all(|(b, _, _, _)| *b != BlockType::SELF));

        for (bt, info, extra, paths) in blockies.into_iter() {
            c.add(bt, info, extra, paths);
        }

        c.expand_paths();
        // c.gen_staged_paths();
        c.gen_single_block_trees();
        // c.gen_stage_block_trees();

        for (bt, n) in &c.data {
            // n.assert_valid();
        }

        info!("Blocktype size {}", std::mem::size_of::<BlockType>());
        info!("Blocktype size {}", std::mem::size_of::<Langbridge>());

        return c
    }
    
    pub fn get_node(&self, bt: &BlockType) -> &Node {
        assert!(self.data.contains_key(bt), "called Constellation.getNode() with bt = {}", bt);
        return self.data.get(bt).unwrap();
    }

    fn add(&mut self, block: BlockType, info: Info, extra: ExtraInfo, mut paths: Vec<Vec<BlockType>>) {
        assert!(!self.data.contains_key(&block), "constellation already contains block {}", block);
        assert!(block != BlockType::ALL && block != BlockType::ROOT && block != BlockType::LEAF);
        assert!(paths.iter().all(|path| {
            let all_count =  path.iter().fold(0, |acc, x| acc + if *x == BlockType::ALL {1} else {0});
            if all_count > 1 {return false}
            if all_count == 1 {return *path.last().unwrap() == BlockType::ALL}
            return true
        }));
        
        let mut node_paths = Vec::new();
        for path in paths.into_iter() {
            node_paths.push(self.paths.len());
            self.paths.push(path);
        }

        self.data.insert(block, Node {
            bt: block,
            info: info,
            extra: extra,
            paths: node_paths,
            staged_paths: vec![],
            fulldowntree: SingleBlockTree::new(0, SingleBlock{parent: 0, data: BlockType::ROOT, children: vec![]}),
            fulluptree: SingleBlockTree::new(0, SingleBlock{parent: 0, data: BlockType::ROOT, children: vec![]}),
            splitdowntree: SingleBlockTree::new(0, SingleBlock{parent: 0, data: BlockType::ROOT, children: vec![]}),
            splituptree: SingleBlockTree::new(0, SingleBlock{parent: 0, data: BlockType::ROOT, children: vec![]}),
            blockdowntree: StageBlockTree::new(),
            blockuptree: StageBlockTree::new(),
        });
    }

    pub fn get_all_of_bt(&self, blocktype: &BlockType) -> std::collections::BTreeMap<BlockType, &Node> {
        // MUST HAVE FIXED ITERATION ORDER! -> no hashmap
        let mut res: std::collections::BTreeMap<BlockType, &Node> = std::collections::BTreeMap::new();
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
        
        match blocktype {
            BlockType::Langbridge(_) => {add_nodes_of_bt_type!(self, res, Langbridge)},
            BlockType::Ui(_) => {add_nodes_of_bt_type!(self, res, Ui)},
            BlockType::Layout(_) => {add_nodes_of_bt_type!(self, res, Layout)},
            BlockType::Render(_) => {add_nodes_of_bt_type!(self, res, Render)},
            BlockType::Intergfx(_) => {add_nodes_of_bt_type!(self, res, Intergfx)},
            BlockType::Gfx(_) => {add_nodes_of_bt_type!(self, res, Gfx)},
            BlockType::Platform(_) => {add_nodes_of_bt_type!(self, res, Platform)},
            _ => {panic!("called get_all_of_bt() {}", blocktype)},
        }
        return res
    }

    pub fn get_all_of_stage(&self, st: Stage) -> Vec<&BlockType> {
        assert!(st != Stage::ROOT && st != Stage::LEAF);
        let mut res: Vec<&BlockType> = Vec::new();
        for (bt, _) in self.data.iter() {
            if *bt != BlockType::ALL && bt.stage() == st {
                res.push(bt);
            }
        }
        return res
    }

    pub fn get_all_of_owner(&self, o: Owner) -> HashSet<BlockType> {
        let mut res: HashSet<BlockType> = HashSet::new();
        for (bt, n) in &self.data {
            if n.info.owner == o {
                res.insert(*bt);
            }
        }
        return res
    }

    pub fn get_all_block_names_of_owner(&self, o: Owner) -> HashSet<String> {
        let mut res: HashSet<String> = HashSet::new();
        for (bt, n) in &self.data {
            if n.info.owner == o {
                res.insert(bt.inner_to_string());
            }
        }
        return res
    }

    fn expand_paths(&mut self) {
        let mut new_paths = Vec::new();
        for (bt, n) in self.data.iter() {
            new_paths.append(&mut self.get_expanded_branches_of_node(&bt));
        }

        assert!(new_paths.iter().all(|path| !path.contains(&BlockType::ALL)));

        // delete any path with same block twice or more
        new_paths.retain(
            |path| path.iter().enumerate().all(
                |(i1, e1)| path.iter().enumerate().all(
                    |(i2, e2)| e1 != e2 || i1 == i2
                )
            )
        );
        
        // delete any duplicate paths
        let mut uniques = Vec::new();
        new_paths.retain(|path| {
            let is_dupe = uniques.contains(path);
            uniques.push(path.clone()); 
            return !is_dupe
        });

        self.paths = new_paths;

        // Add references (indices of self.paths) in all the Nodes to paths containing it
        for (bt, n) in self.data.iter_mut() {
            n.paths = Vec::new();
            for (i, path) in self.paths.iter().enumerate() {
                if path.contains(&bt) {
                    n.paths.push(i);
                }
            }
        }
    }

    pub fn get_expanded_branches_of_node(&self, block: &BlockType) -> Vec<Vec<BlockType>> {
        let mut new_paths: Vec<Vec<BlockType>> = Vec::new();
        for u in self.data.get(block).unwrap().paths.iter() {
            let path = &self.paths[*u];
            assert!(path.len() > 0, "every path length must be > 0, found {:?}", path);
            let mut base_path: Vec<BlockType> = path.clone();
            if *path.last().unwrap() == BlockType::ALL {
                assert!(path.len() > 2, "there must be a node in front of BlockType::ALL in a branch");
                base_path.remove(base_path.len()-1);
                let last_node = base_path.last().unwrap();
                assert!(*last_node != BlockType::ALL, "invalid branch contains consecutive ALL {:?}", path);
                let expanded_path: Vec<BlockType>;
                for sub_path in self.get_expanded_branches_of_node(last_node) {
                    let mut new_path = base_path.clone();
                    new_path.append(&mut (&sub_path[1..]).to_vec());
                    new_paths.push(new_path);
                }
            } else {
                new_paths.push(base_path);
            }
            
        }
        return new_paths;
    }

    fn gen_staged_paths(&mut self) {
        let mut stage_block_paths: Vec<Vec<Vec<BlockType>>> = Vec::new();
        for path in self.paths.iter() {
            let mut stage_block_path: Vec<Vec<BlockType>> = Vec::new();
            let mut curr_stage: Stage = Stage::Langbridge;
            let mut curr_stage_block: Vec<BlockType> = Vec::new();
            for block in path {
                if block.stage() == curr_stage {
                    curr_stage_block.push(*block);
                } else if block.stage().lvl() > curr_stage.lvl() {
                    stage_block_path.push(curr_stage_block);
                    for empty_stage in curr_stage.next().lvl()..block.stage().lvl() {
                        stage_block_path.push(Vec::new());
                    }
                    curr_stage_block = Vec::new();
                    curr_stage_block.push(*block);
                    curr_stage = block.stage();
                } else {
                    panic!("path with decreasing stages: {:?}", path);
                }
            }
            stage_block_path.push(curr_stage_block);
            for st in curr_stage.lvl()..Stage::Platform.lvl() {
                stage_block_path.push(Vec::new());
            }

            assert!(stage_block_path.len() == (Stage::last().lvl()+1) as usize, "staged_block_path length not right: \n\tpath : {path:?} \n\tstage_block_path: {stage_block_path:?}");

            for stage_block in stage_block_path.iter() {
                for block in stage_block {
                    self.data.get_mut(&block).unwrap().staged_paths.push(stage_block_paths.len())
                }
            }
            stage_block_paths.push(stage_block_path);
        }
        self.staged_paths = stage_block_paths;
    }

    fn gen_single_block_trees(&mut self) {
        for (block, n) in self.data.iter_mut() {
            (n.fulldowntree, n.fulluptree) = SingleBlockTree::full_trees_from_paths_containing(&self.paths, *block); 
            (n.splitdowntree, n.splituptree) = SingleBlockTree::split_trees_from_paths_with_target_root(&self.paths, *block);
        }   
    }

    fn gen_stage_block_trees(&mut self) {
        for (bt, n) in self.data.iter_mut() {
            // if bt.of_same_type(BlockType::Langbridge(Langbridge::META)) {continue};
            assert!(*bt != BlockType::ALL, "block {bt} found in Constellation.data");
            let mut root_to_targ_paths: Vec<&[Vec<BlockType>]> = Vec::new();
            for u in n.staged_paths.iter() {
                root_to_targ_paths.push(&self.staged_paths[*u][..=bt.stage().lvl() as usize]);
            }
            n.blockuptree = StageBlockTree::from_root_to_targ_paths(root_to_targ_paths, *bt);
        }
  
        for (bt, n) in self.data.iter_mut() {
            // if bt.of_same_type(BlockType::Platform(Platform::META)) {continue};
            assert!(*bt != BlockType::ALL, "block {bt} found in Constellation.data");
            let mut targ_to_leaf_paths: Vec<&[Vec<BlockType>]> = Vec::new();
            for u in n.staged_paths.iter() {
                targ_to_leaf_paths.push(&self.staged_paths[*u][bt.stage().lvl() as usize..]);
            }
            n.blockdowntree = StageBlockTree::from_targ_to_leaf_paths(targ_to_leaf_paths, *bt)
        }
    }

    pub fn store(&self) {
        let mut f = std::fs::File::create("./res/state/constellation.bincode").unwrap();
        bincode::serialize_into(&mut f, &self).expect("unable to store constellation");
        // std::fs::write("./res/state/constellation.ron", ron::to_string(self).unwrap())
        //     .expect("was unable to store constellation");
    }

    pub fn load() -> Self {
        bincode::deserialize::<Constellation>(&std::fs::read("./res/state/constellation.bincode").unwrap()).unwrap()
        // ron::from_str::<Constellation>(&std::fs::read_to_string("./res/state/constellation.ron").unwrap()).unwrap()
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



