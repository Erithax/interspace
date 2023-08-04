#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_braces)]

/*
## Web
WebDOM, WebSVG, Webcanvas, WebGL, WebGPU
* option to stop when encountering one of these

## Filter
## Sort
## Relation styling
### Persistent
* background color
* background repeating text
* border
### Hover/Focus/Emphasize
* highlight components (border/background)
* scale highlighted components
* fade non-related components

* show owner logo on hover, also on all owner components
* show superproject logo on hover, also on all all superproject components
### 
* Union blocks (OR)
* Intersection blocks (AND)
* way to show fallback (e.g. CPU fallback)
* show shell (Browser, Webview, Electron, winit, glazier)

### Clicking focussed item expands row below with all its info

### Popover component

### Dragging rows

### Comparator component (cfr. current pipeline/stage component)


*/

mod swig;
use swig::*;
mod bowl;
use bowl::*;

mod common;
mod lang;
mod owner;
mod langbridge;
mod ui;
mod layout;
mod render;
mod intergfx;
mod gfx;
mod platform;
mod image;

use common::*;
use crate::image::*;
use lang::*;
use owner::*;
use langbridge::*;
use ui::*;
use layout::*;
use render::*;
use intergfx::*;
use gfx::*;
use platform::*;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;


use core::fmt::LowerHex;
use log::{info, LevelFilter};
use dioxus::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use num_integer;
use sha2::{Sha256, Sha512, Digest};


use serde::Serialize;
use serde::Deserialize;

/*
size in bytes:
    Skia fulldowntree : 1384
        fulluptree: 1192
        splitdowntree: 808
        splituptree: 616

*/


pub fn init_dioxus() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init dioxus logger");
    dioxus_web::launch_cfg(App, dioxus_web::Config::new().rootname("entry"));
}

pub fn init_dioxus2() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init dioxus logger");
    dioxus_web::launch_cfg(AppV2, dioxus_web::Config::new().rootname("entry"));
}

pub fn init_dioxus3() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init dioxus logger");
    dioxus_web::launch_cfg(AppV3, dioxus_web::Config::new().rootname("entry"));
}

pub fn indent(s: &str) -> String {
    let res: String = s.replace("\n", "\n\t");
    return res
}

pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    return std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub fn is_focussed_class<T : PartialEq>(a: &T, b: &T) -> String {
    if *a == *b {
        return "active".to_string()
    } else {
        return "inactive".to_string()
    }
}

#[derive(PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}
impl Theme {

    pub fn set_favicon() {
        let prefers_dark_color_theme = web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)");
        let icon = web_sys::window().unwrap().document().unwrap().get_element_by_id("favicon").unwrap();

        if prefers_dark_color_theme.is_ok() && prefers_dark_color_theme.as_ref().unwrap().is_some() && prefers_dark_color_theme.unwrap().unwrap().matches() {
            icon.set_attribute("href", "/img/Erithax/Erithax_dark.ico");
        } else {
            icon.set_attribute("href", "/img/Erithax/Erithax_light.ico");
        }
    }

    pub fn toggle(&self) -> Self {
        match *self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    pub fn to_class(&self) -> String {
        match self {
            Theme::Dark => {return "th_dark".to_string()},
            Theme::Light => {return "th_light".to_string()}
        }
    }
}


fn App(cx: Scope) -> Element {

    // let now = wasm_timer::Instant::now();
    let con = common::Constellation::generate();
    // info!("Constellation::generate() in {:?}", now.elapsed());

    use_shared_state_provider(cx, || con);

    Theme::set_favicon();

    // let c = Constellation::load(include_str!("../res/state/constellation.ron")).unwrap();
    // use_shared_state_provider(cx, || c);

    // let con = use_shared_state::<Constellation>(cx).unwrap();
    // let _binding = con.read();
    let theme = use_state(cx, || Theme::Dark);
    let switching_themes_class = use_state(cx, || "".to_string());

    let switching = "";


    let switch_theme = move || {
        cx.spawn({
            let switching_themes_class = switching_themes_class.to_owned();
            switching_themes_class.set("switching_theme".to_string());

            async move {
                let i = async_std::task::sleep(std::time::Duration::from_millis(200)).await;
                switching_themes_class.set("".to_string());
            }
        })
    };

    cx.render(rsx!{
        div {
            class: "{theme.to_class()} {switching_themes_class}",
            Header{
                theme: theme,
                on_mouse: move |event: MouseEvent| {
                    switch_theme();                    
                    theme.set(theme.toggle());
                },
            },
            main {
                id: "main-content",
                DynaTable{},
            },
            
        }
        
    })
}


#[inline_props]
fn Header<'a>(
    cx: Scope<'a>, 
    theme: &'a Theme, 
    on_mouse: EventHandler<'a, MouseEvent>
) -> Element<'a> {
    cx.render(rsx!{
        header {
            div {
                class: "lef",
                a {
                    img {
                        src: "./img/Erithax/Erithax.svg",
                        alt: "erithax logo",
                    }
                }
            },
            div {
                class: "rig",
                a {
                    class: "code",
                    href: "https://github.com/erithax/ui-overview",
                    target: "_blank",
                    img {
                        src: "./img/code.svg",
                        alt: "repository",
                    },
                },
                div {
                    class: "seperator",
                }
                button {
                    class: "heart",
                    img {
                        src: "./img/heart.svg",
                        alt: "support me",
                    },
                },
                button {
                    class: "wrench",
                    img {
                        src: "./img/wrench.svg",
                        alt: "contribute",
                },
                },
                button {
                    class: "font",
                    img {
                        src: "./img/font.svg",
                        alt: "font",
                },
                },
                button {
                    class: "sunmoon",
                    onclick: move |event| on_mouse.call(event),
                    if **theme == Theme::Dark {
                        rsx!{img {
                            class: "sun",
                            src: "./img/sun.svg",
                            alt: "enable light mode",
                        }}
                    } else {
                        rsx!{img {
                            class: "moon",
                            src: "./img/moon.svg",
                            alt: "enable dark mode",
                        }}
                    }
                },
            }
        }
    })
}

pub fn inline_erithax<T: Into<f64>>(id: String, col: String, size: T, stroke_wid: T) -> String {
    let mut res = erithax(id, col, size, stroke_wid);
    res.retain(|char| char != '\n');
    return res
}

pub fn erithax<T: Into<f64>>(id: String, col: String, size: T, stroke_wid: T) -> String {
    let size: f64 = size.into();
    let stroke_wid: f64 = stroke_wid.into();

    let octa_side_len = (size-stroke_wid) / (1.0 + 2.0_f64.sqrt()); // top flat
    let octa_side_proj = 2.0_f64.sqrt()/2.0 * octa_side_len;

    let cs0: f64 = 0.0;
    let cs1: f64 = 0.0 + stroke_wid/2.0;
    let cs2: f64 = 0.0 + stroke_wid/2.0 + octa_side_proj;
    let cm: f64 = size/2.0;
    let ce2: f64 = size - cs2;
    let ce1: f64 = size - cs1;
    let ce0: f64 = size;

    let L = octa_side_len / 3.0; // theta and xi stripe length
    let th0 = cm-L/2.0;
    let th1 = cm+L/2.0;
    let xi0 = ce2 + octa_side_proj/2.0 + stroke_wid/2.0 - L/2.0;
    let xi1 = ce2 + octa_side_proj/2.0 + stroke_wid/2.0 + L/2.0;

    let res = format!(r#"
        <svg viewBox="0 0 {size} {size}" preserveAspectRatio="xMidYMid meet">
            <path d="M {cs2},{cs1}  {ce2},{cs1}  {ce2},{ce1}  {cs2},{ce1} Z M {th0},{cm}  {th1},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
            <path d="M {cs2},{cs2}  {cs1},{cs2}  {cs1},{ce2}  {cs2},{ce2} M {cs1},{cm}  {cs2},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
            <path d="M {ce2},{cs2}  {ce0},{cs2} M {ce0},{ce2}  {ce2},{ce2} M {xi0},{cm}  {xi1},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
       </svg>
    "#);

    return res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsableToggle {
    Collapsed,
    Expanded
}

impl CollapsableToggle {
    pub fn toggle(&self) -> Self {
        match self {
            CollapsableToggle::Collapsed => {CollapsableToggle::Expanded},
            CollapsableToggle::Expanded => {CollapsableToggle::Collapsed},
        }
    }

    pub fn is_collapsed(&self) -> bool {
        match self {
            Self::Collapsed => {true},
            Self::Expanded => {false},
        }
    }

    pub fn is_expanded(&self) -> bool {
        match self {
            Self::Collapsed => {false},
            Self::Expanded => {true},
        }
    }
}

impl std::fmt::Display for CollapsableToggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollapsableToggle::Collapsed => { write!(f, "collapsed")},
            CollapsableToggle::Expanded => {write!(f, "expanded")},
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BlockDirection {
    Up,
    Down,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum FocusColumn {
    Normal,
    Isolated, // focussed item gets own column
    Combined, // focussed item gets own column and multiple representations across rows are combined
}

impl FocusColumn {
    // only used because cant use :: in rsx so cant use FocusColumn::Normal
    pub fn getnormal(&self) -> Self {return FocusColumn::Normal}
    pub fn getisolated(&self) -> Self {return FocusColumn::Isolated}
    pub fn getcombined(&self) -> Self {return FocusColumn::Combined}
}

impl std::fmt::Display for FocusColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Normal => {"Normal"},
            Self::Isolated => {"Isolated"},
            Self::Combined => {"Combined"},
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq)]
pub enum DynaTreeType {
    FullDown,   // >
    FullUp,     // <
    Hourglass,  // >< (2 DynaTrees)
}


impl DynaTreeType {
    pub fn getfulldown(&self) -> Self {
        return Self::FullDown
    }
    pub fn getfullup(&self) -> Self {
        return Self::FullUp
    }
    pub fn gethourglass(&self) -> Self {
        return Self::Hourglass
    }
}

impl std::fmt::Display for DynaTreeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::FullDown => {"Down"},
            Self::FullUp => {"Up"},
            Self::Hourglass => {"Hourglass"},
        };
        write!(f, "{s}")
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Normal,
    DeleteCluster{
        clustered_nodes: Vec<SingleBlockId>,
    },
    GroupCluster{
        clustered_nodes: Vec<SingleBlockId>,
        clustered_prims: Vec<BlockType>,
    }, 
}

#[derive(Debug, Clone)]
pub struct DynaTree {
    pub tree: SingleBlockTree,
    pub dyna_nodes: HashMap<SingleBlockId, NodeType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynaRenderPrims {
    pub id: SingleBlockId,
    pub nt: NodeType,
    pub bt: BlockType,
    pub rel_stage_i: i32, // stages removed from root 
    pub inner_stage_depth_lcm: i32,
    pub inner_stage_col_start: i32,
    pub inner_stage_col_span: i32,
    pub wid_start_i: i32, 
    pub wid_end_i: i32,
}



impl DynaTree {
    pub fn new() -> DynaTree {
        return DynaTree {
            tree: SingleBlockTree::new_blank(),
            dyna_nodes: HashMap::new(),
        }
    }

    pub fn from(t: &SingleBlockTree) -> DynaTree {
        return DynaTree {
            tree: t.clone(),
            dyna_nodes: t.items.iter().map(|(id, _)| (*id, NodeType::Normal)).collect(),
        }
    }

    fn remove_hidden_bts_from_tree(t: &mut SingleBlockTree, targ: BlockType, before_hidden: &Vec<BlockType>, after_hidden: &Vec<BlockType>) {
        Self::remove_hidden_bts_from_tree_from_node(t, targ, before_hidden, after_hidden, t.root, true)
    }

    fn remove_hidden_bts_from_tree_from_node(t: &mut SingleBlockTree, targ: BlockType, before_hidden: &Vec<BlockType>, after_hidden: &Vec<BlockType>, id: SingleBlockId, before_targ: bool) {
        assert!(!(t.get(id).data != targ && t.get(id).children.len() == 0 && before_targ));
        let sb = t.get(id);

        if sb.data == targ {
            for ch_id in t.get(id).children {
                Self::remove_hidden_bts_from_tree_from_node(t, targ, before_hidden, after_hidden, ch_id, false);
            }
            return
        } else if    before_targ && before_hidden.iter().any(|hidden_bt| variant_eq(hidden_bt, &sb.data)) ||
                    !before_targ && after_hidden .iter().any(|hidden_bt| variant_eq(hidden_bt, &sb.data)) {
            t.remove(id);
        }

        for ch_id in sb.children {
            Self::remove_hidden_bts_from_tree_from_node(t,targ, before_hidden, after_hidden, ch_id, before_targ);
        }
    }

    pub fn from_tree_with_hidden_bts(
        t: &SingleBlockTree, 
        targ: BlockType, 
        before_hidden: &Vec<BlockType>, 
        after_hidden: &Vec<BlockType>
    ) -> DynaTree {
        // !!! before_hidden and after_hidden are in terms of the tree! (e.g. before=node between root and targ) !!!
        let mut t = t.clone();
        Self::remove_hidden_bts_from_tree(&mut t, targ, before_hidden, after_hidden);

        let dyna_nodes = t.items.iter().map(|(id, _)| (*id, NodeType::Normal)).collect();
        t.assert_valid();

        let r = t.root;
        let mut res = DynaTree {
            tree: t.clone(),
            dyna_nodes,
        };
        res.dedupe_from(r);
        return res
    }

    pub fn get_root_id(&self) -> SingleBlockId {
        return self.tree.root
    }

    pub fn get_root_bt(&self) -> BlockType {
        return self.tree.get(self.tree.root).data
    }

    pub fn get_dyna_node(&self, id: SingleBlockId) -> NodeType {
        assert!(self.dyna_nodes.contains_key(&id));
        return self.dyna_nodes.get(&id).unwrap().clone();
    }

    pub fn get_wid(&self) -> i32 {
        return self.get_wid_from(self.tree.root)
    }

    pub fn delete_node(&mut self, id: SingleBlockId) {
        assert!(*self.dyna_nodes.get(&id).unwrap() == NodeType::Normal);
        // DeleteCluster.clustered_nodes does not include id of itself (as opposed to GroupCluster)
        // self.tree is invalid when there is a delete cluster because clustered nodes are not in the children of any other node

        // check if there is a delete cluster sibling
        let del_clus_sib_id = self.tree.get(self.tree.get(id).parent).children.into_iter().find(
            |&sib_id| sib_id != id && 
            match self.dyna_nodes.get(&sib_id).unwrap() {NodeType::DeleteCluster{..} => {true}, _ => {false}}
        );

        match del_clus_sib_id {
            Some(sib_id) => {
                // add self to delete cluster
                match self.dyna_nodes.get_mut(&sib_id).unwrap() {
                    NodeType::DeleteCluster{clustered_nodes} => {
                        clustered_nodes.push(id); // add self to cluster
                        self.tree.get_mut(self.tree.get(id).parent).children.retain(|x| *x != id); // remove self from parent children
                    },
                    _ => {panic!("not supposed to reach")}
                }
            },
            None => {
                // turn self into cluster
                self.dyna_nodes.insert(id, NodeType::DeleteCluster{clustered_nodes: vec![]});
            }
        }
    }

    pub fn undelete_node(&mut self, id: SingleBlockId) {
        assert!(match self.dyna_nodes.get(&id).unwrap() {NodeType::DeleteCluster{..} => true, _ => false});

        let dn = self.dyna_nodes.get(&id).unwrap().clone();
        match dn {
            NodeType::DeleteCluster{mut clustered_nodes} => {
                self.tree.get_mut(self.tree.get(id).parent).children.append(&mut clustered_nodes);
            },
            _ => {panic!("no no no")}
        }

        self.dyna_nodes.insert(id, NodeType::Normal);
    }

    pub fn group_node(&mut self, id: SingleBlockId) {
        assert!(*self.dyna_nodes.get(&id).unwrap() == NodeType::Normal);

        // check if there is a cluster sibling
        let group_cluster_sib_Id = self.tree.get(self.tree.get(id).parent).children.into_iter().find(
            |&sib_id| sib_id != id && 
            match self.dyna_nodes.get(&sib_id).unwrap() {NodeType::GroupCluster{..} => {true}, _ => {false}}
        );

        
        match group_cluster_sib_Id {
            Some(sib_id) => {
                // add self to cluster
                match self.dyna_nodes.get_mut(&sib_id).unwrap() {
                    NodeType::GroupCluster{clustered_nodes, clustered_prims} => {
                        clustered_nodes.push(id); // add self to cluster
                        clustered_prims.push(self.tree.get(id).data);
                        let mut chs = self.tree.get(id).children.clone(); 
                        chs.iter().for_each(|&ch_id| self.tree.get_mut(ch_id).parent = sib_id); // set self children's .parent to sibling
                        self.tree.get_mut(sib_id).children.append(&mut chs);            // append self children to sibling children
                        self.tree.get_mut(self.tree.get(id).parent).children.retain(|x| *x != id); // remove self from parent children

                        self.tree.items.remove(&id);
        
                        
                        self.dedupe_from(sib_id);
                    },
                    _ => {panic!("not supposed to reach")}
                }
            },
            None => {
                // turn self into cluster
                self.dyna_nodes.insert(id, NodeType::GroupCluster{clustered_nodes: vec![id], clustered_prims: vec![self.tree.get(id).data]});
            }
        }
    }

    pub fn ungroup_node(&mut self, id: SingleBlockId, orig_tree: &SingleBlockTree, targ_bt: BlockType,  before_hidden: &Vec<BlockType>, after_hidden: &Vec<BlockType>) {
        assert!(match self.dyna_nodes.get(&id).unwrap() {NodeType::GroupCluster{..} => true, _ => false} );
        let mut orig_tree = orig_tree.clone();

        Self::remove_hidden_bts_from_tree(&mut orig_tree, targ_bt, before_hidden, after_hidden);

        let parent_id = self.tree.get(id).parent;
        let clustered_nodes = match self.dyna_nodes.get(&id).unwrap() {
            NodeType::GroupCluster{clustered_nodes, ..} => clustered_nodes.clone(),
            _ => {panic!("no no no no")}
        };
        assert!(clustered_nodes.contains(&id));
    
        for clustered_node_id in clustered_nodes {
            self.copy_subtree_at_from(clustered_node_id, &orig_tree);
            if clustered_node_id != id {self.tree.get_mut(parent_id).children.push(clustered_node_id);};
            self.tree.get_mut(clustered_node_id).parent = parent_id; // !!!
            self.dyna_nodes.insert(clustered_node_id, NodeType::Normal);
        }
        self.tree.assert_valid();
    }

    pub fn copy_subtree_at_from(&mut self, id: SingleBlockId, tree: &SingleBlockTree) {
        self.tree.items.insert(id, tree.get(id));
        self.dyna_nodes.insert(id, NodeType::Normal);
        for ch_id in self.tree.get(id).children {
            self.copy_subtree_at_from(ch_id, tree);
        }
    }

    pub fn dedupe_from(&mut self, id: SingleBlockId) {
        // Deduplicate branches by BlockType from id down
        // e.g. (A -> (B -> (C, D, E), B -> (D, E, F) ), G -> C)  with dedup_from(A) becomes (A -> (B -> (C, D, E, F), G -> C))
        let mut i = 0;
        if self.tree.get(id).children.len() == 0 {return}
        while i < self.tree.get(id).children.len()-1 { // last ele doesn't need dupe check
            let chs = self.tree.get(id).children;
            let ch_id = chs[i];
            // self.dedupe_from(ch_id);
            let ch_sb = self.tree.get(ch_id);
            
            // find sibling duplicates (same BlockType), link their children to ch_id, delete that duplicate
            let dupes: Vec<SingleBlockId> = chs[i+1..].to_vec().into_iter().filter(
                |&other_ch_id| {assert!(other_ch_id != ch_id); ch_sb.data == self.tree.get(other_ch_id).data}
            ).collect();
            
            for other_ch_id in dupes {
                // remove other from parent
                self.tree.get_mut(ch_sb.parent).children.retain(|&sib_id| sib_id != other_ch_id);

                // link children of other to ch_id
                self.tree.get(other_ch_id).children.iter().for_each(|&other_ch_ch| self.tree.get_mut(other_ch_ch).parent = ch_id);

                // append other children to ch_id
                let mut other_ch_chs: Vec<SingleBlockId> =  self.tree.get_mut(other_ch_id).children.clone();
                self.tree.get_mut(ch_id).children.append(&mut other_ch_chs);
                
                // remove other from tree.items
                self.tree.items.remove(&other_ch_id);
            }
            //self.tree.assert_valid(); // will assert false when using cluster nodes
            self.dedupe_from(ch_id);
            i+= 1;
        }
        
    }

    pub fn get_wid_from(&self, id: SingleBlockId) -> i32 {
        if self.tree.get(id).children.len() == 0 || (match self.get_dyna_node(id) {NodeType::DeleteCluster{..}=>true,_=>false}) {
            return 1
        }
        let mut res = 0;
        for ch_id in self.tree.get(id).children {
            res += self.get_wid_from(ch_id);
        }
        return res
    }

    pub fn is_before(&self, id: SingleBlockId, targ: BlockType) -> bool {
        assert!(self.tree.get(id).data != targ);
        if self.tree.get(id).children.len() == 0 {
            return false
        }
        if self.tree.get(id).children.iter().any(|ch_id| self.tree.get(*ch_id).data == targ) {
            return true
        }
        return self.tree.get(id).children.iter().any(|ch_id| self.is_before(*ch_id, targ))
    }

    pub fn get_render_prims(&self) -> Vec<DynaRenderPrims> {
        return self.get_render_prims_from(self.tree.root, 0);
    }

    pub fn get_render_prims_from(&self, id: SingleBlockId, wid_start_i: i32) -> Vec<DynaRenderPrims> {
        assert!(self.tree.get(id).data != BlockType::ALL);
        let mut res = Vec::<DynaRenderPrims>::new();

        let wid_end_i = if self.tree.get(id).children.len() == 0 || (match self.get_dyna_node(id) {NodeType::DeleteCluster{..}=>true,_=>false}) {
            wid_start_i
        } else {
            let mut child_wid_start = wid_start_i;
            for ch in self.tree.get(id).children {
                res.append(&mut self.get_render_prims_from(ch, child_wid_start));
                child_wid_start = res.last().unwrap().wid_end_i + 1;
            }
            child_wid_start - 1
        };

        let (s, e) = self.get_stage_inner_col_start_and_depth(id, None);
        
        res.push(DynaRenderPrims{
            id: id,
            nt: self.dyna_nodes.get(&id).unwrap().clone(),
            bt: self.tree.get(id).data,
            rel_stage_i: self.get_rel_stage_i(id),
            inner_stage_depth_lcm: self.get_stage_branches_depth_lcm(self.tree.get_stage(id).lvl()),
            inner_stage_col_start: s,
            inner_stage_col_span: e,
            wid_start_i,
            wid_end_i,
        });

        return res
    }

    pub fn get_rel_stage_i(&self, id: SingleBlockId) -> i32 {
        return (self.tree.get_stage(id).lvl() - self.tree.get_stage(self.tree.root).lvl()).abs()
    }

    pub fn get_max_inner_depth_of_branch_in_stage_at_node(&self, id: SingleBlockId) -> i32 {
        let st = self.tree.get_stage(id).lvl();
        let mut highest_node_in_branch = id;
        loop {
            let parent = self.tree.get(highest_node_in_branch).parent;
            if self.tree.get_stage(parent).lvl() == st && parent != self.tree.root {
                highest_node_in_branch = parent;
            } else {
                break;
            }
        }
        return self.get_max_dep_to_next_stage(highest_node_in_branch, st)
    }

    pub fn get_stage_max_inner_depth_of_stage(&self, st: i32) -> i32 {
        let mut max_stage_inner_dep: i32 = 0;
        for (id, sb) in self.tree.items.iter() {
            let other_st = self.tree.get_stage(*id).lvl();
            let parent_st = self.tree.get_stage(self.tree.get(*id).parent).lvl();
            if other_st == st && parent_st != other_st {
                max_stage_inner_dep = std::cmp::max(max_stage_inner_dep, self.get_max_dep_to_next_stage(*id, st));
            }
        }
        return max_stage_inner_dep
    }

    pub fn get_stage_inner_col_start_and_depth(&self, id: SingleBlockId, stopper_bt: Option<BlockType>) -> (i32, i32) {
        // stopper_bt is root when not splitting the focussed column
        let st = self.tree.get_stage(id).lvl();
        let lcm = self.get_stage_branches_depth_lcm(st);
        let mut space_left = lcm;
        let mut start = 0;
        let mut id_stack_to_branch_start = Vec::from(vec![id]);
        let mut curr_id = self.tree.get(id).parent;
        while self.tree.get_stage(curr_id).lvl() == st && curr_id != self.tree.root && (stopper_bt.is_none() || self.tree.get(curr_id).data != stopper_bt.unwrap())  {
            id_stack_to_branch_start.push(curr_id);
            curr_id = self.tree.get(curr_id).parent;
        }
        
        while id_stack_to_branch_start.len() > 1 {
            start += space_left / self.get_max_dep_to_next_stage(id_stack_to_branch_start.pop().unwrap(), st);
            space_left -= start;
        }
        let block_depth = space_left / self.get_max_dep_to_next_stage(id, st);
        return (start, block_depth)
    }

    pub fn get_stage_branches_depth_lcm(&self, st: i32) -> i32 {
        // least common multiple of all the depths of branches through this stage
        assert!(self.tree.items.iter().any(|(_, sb)| sb.data.stage().lvl() == st));
        let mut lcm = 1; // 
        for (id, sb) in self.tree.items.iter() {
            let id_st = self.tree.get_stage(*id).lvl();
            if id_st == st && self.tree.get_stage(sb.parent).lvl() != id_st {
                // id is highest node in a branch in the correct stage
                let branch_dep = self.get_max_dep_to_next_stage(*id, st);
                lcm = num_integer::lcm(lcm, branch_dep);
            }  
        }
        return lcm
    }

    pub fn get_max_dep_to_next_stage(&self, id: SingleBlockId, orig_st: i32) -> i32 {
        if self.tree.get_stage(id).lvl() != orig_st {
            return 0
        } else if self.tree.get(id).children.len() == 0  {
            return 1
        } else {
            return 1 + self.tree.get(id).children.iter().map(|&ch_id| self.get_max_dep_to_next_stage(ch_id, orig_st)).max().unwrap()
        }
    }

    pub fn get_isolated_render_prims(&self, targ: BlockType) -> Vec<DynaRenderPrims> {
        let mut res = self.get_render_prims();
        let targ_st = targ.stage();
        for prim in res.iter_mut() {
            if prim.bt != targ && !self.is_before(prim.id, targ) {
                prim.rel_stage_i += 2;
            }
            if self.tree.get_stage(prim.id) == targ_st {
                if prim.bt == targ {
                    prim.rel_stage_i += 1;
                    prim.inner_stage_col_start = 0;
                    prim.inner_stage_col_span = 1;
                } else if !self.is_before(prim.id, targ) {
                    let (s, d) = self.get_stage_inner_col_start_and_depth(prim.id, Some(targ));
                    prim.inner_stage_col_start = s;
                    prim.inner_stage_col_span = d;
                } 
            }
        }
        return res
    }

    pub fn get_combined_render_prims(&self, targ: BlockType) -> Vec<DynaRenderPrims> {
        let mut res = self.get_isolated_render_prims(targ);
        // save on targ node
        let m = res.iter().find(|prim| prim.bt == targ);
        match m {
            Some(m) => {
                let mut m = m.clone();
                // delete all the targ nodes
                res.retain(|prim| prim.bt != targ);
                // modify targ node to cover whole space
                m.wid_start_i = 0;
                m.wid_end_i = self.get_wid();
                // add targ node back in
                res.push(m);
            },
            None => {
                // delete node in tree before target
            }
        }
        return res
        
    }

    pub fn get_split_render_prims(&self) -> Vec<DynaRenderPrims> {
        return self.get_render_prims()
    }

    pub fn get_full_render_prims(&self, targ: BlockType, fc: FocusColumn) -> Vec<DynaRenderPrims> {
        match fc {
            FocusColumn::Normal =>   {self.get_render_prims()},
            FocusColumn::Isolated => {self.get_isolated_render_prims(targ)},
            FocusColumn::Combined => {self.get_combined_render_prims(targ)}
        }
    }

    pub fn get_inner_stage_i(&self, id: SingleBlockId) -> i32 {
        let mut curr_inner_stage_i = 0;
        let mut curr_id = id;
        let mut curr_sb = self.tree.get(curr_id);

        while curr_id != self.tree.root && self.tree.get_stage(curr_sb.parent) == self.tree.get_stage(curr_id) {
            curr_inner_stage_i += 1;
            curr_id = curr_sb.parent;
            curr_sb = self.tree.get(curr_id);
        }
        if curr_id == self.tree.root {
            curr_inner_stage_i -= 1;
        }
        return curr_inner_stage_i
    }

    pub fn dyna_leaf_count_from(&self, id: SingleBlockId) -> usize {
        if self.tree.get(id).children.len() == 0 {
            return 1
        }

        match self.dyna_nodes.get(&id).unwrap() {
            NodeType::DeleteCluster{..} => return 1,
            _=> {}
        }

        return self.tree.get(id).children.iter().fold(0, |acc, ch_id| acc + self.dyna_leaf_count_from(*ch_id))
    }

}






#[derive(Debug, Clone)]
pub enum StagSplit {
    Solid(Vec<BlockType>),
    Split(Vec<BlockType>, Vec<BlockType>, Vec<BlockType>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagSelect {
    Base,
    Selected,
}

pub struct Stagerino {
    selected_bt: BlockType,
    tt: DynaTreeType,
    fc: FocusColumn,
    items: std::collections::BTreeMap<Stage, (StagSplit, StagSelect)>,
    hovering: Option<(Stage, Option<usize>)>
}

impl Stagerino {

    pub fn new(select: BlockType) -> Self {
        assert!(Stage::iter_reals().any(|s| s==select.stage()));
        let mut items = std::collections::BTreeMap::new();
        for s in Stage::iter_reals() {
            if s == select.stage() {
                items.insert(s, (StagSplit::Split(vec![], vec![], vec![]), StagSelect::Selected));
            } else {
                items.insert(s, (StagSplit::Solid(vec![]), StagSelect::Base));
            }
        }
        return Stagerino {selected_bt: select, tt: DynaTreeType::Hourglass, fc: FocusColumn::Combined, items: items, hovering: None}
    }

    pub fn assert_valid(&self) {
        for s in Stage::iter_reals() {
            assert!(self.items.iter().any(|(my_s, _)| *my_s == s));
        }
        for (my_s, _) in self.items.iter() {
            assert!(Stage::iter_reals().any(|s| s == *my_s));
        }

        for (s, (sp, ss)) in self.items.iter() {
            match sp {
                StagSplit::Solid(bts) => {
                    bts.iter().all(|bt| bt.stage() == *s);
                },
                StagSplit::Split (pre, mid, post )=> {
                    pre.iter().all(|bt| bt.stage() == *s);
                    mid.iter().all(|bt| bt.stage() == *s);
                    post.iter().all(|bt| bt.stage() == *s);
                }
            }
        }

        assert!(self.items.iter().all(|(s, (_, ss))| *ss != StagSelect::Selected || *s == self.selected_bt.stage()));

        assert!(self.items.iter().all(|(s, (sp, ss))| *ss == StagSelect::Selected || match sp {StagSplit::Solid{..} => true, _=> false} ));

        assert!(self.items.iter().filter(|(s, (sp, ss))| *ss == StagSelect::Selected).count() == 1);

        if self.tt == DynaTreeType::Hourglass || self.fc != FocusColumn::Normal{
            assert!(self.items.iter().filter(|(s, (sp, ss))| match sp {StagSplit::Split{..}=>true, _=>false}).count() == 1);
        } 
    }  

    pub fn unhide_all_of_stage(&mut self, stage: Stage) {
        for (s, (split, ssel)) in self.items.iter_mut() {
            if *s == stage {
                match split {
                    StagSplit::Solid(h) => {h.clear();},
                    StagSplit::Split ( pre, mid, post ) => {
                        pre.clear();
                        mid.clear();
                        post.clear();
                    }
                }
            }
        }
    }

    pub fn unhide_at_least_mid_of_stage(&mut self, stage: Stage) {
        for (s, (split, ssel)) in self.items.iter_mut() {
            if *s == stage {
                match split {
                    StagSplit::Solid(h) => {h.clear();},
                    StagSplit::Split (pre, mid, post) => {
                        mid.clear();
                    }
                }
            }
        }
    }

    pub fn unhide_all_of_pseudo_stage(&mut self, stage: Stage, sub: Option<usize>) {
        for (s, (split, sel)) in self.items.iter_mut() {
            if *s == stage {
                match split {
                    StagSplit::Solid(hids) => {
                        assert!(sub.is_none());
                        hids.clear();
                    },
                    StagSplit::Split(pre, mid, post) => {
                        assert!(sub.is_some());
                        let sub = sub.unwrap();
                        if sub == 0 {
                            pre.clear();
                        } else if sub == 2 {
                            post.clear();
                        } else {
                            panic!("no");
                        }
                    }
                }
            }
        }
    }

    pub fn is_hovered(&self, (stage, sub): (Stage, Option<usize>)) -> bool {
        match self.hovering {
            Some((st, su)) => {
                return st == stage && su == sub
            },
            None => {
                return false
            }
        }
    }

    pub fn get_selected_stage(&self) -> Stage {
        for (s, (_, ss)) in self.items.iter() {
            if *ss == StagSelect::Selected {
                return *s
            }
        }
        panic!("no");
    }

    pub fn get_grid_cols(&self) -> Vec<String> {
        let mut grid_cols = vec![];
        for (s, (split, sel)) in self.items.iter() {
            match split {
                StagSplit::Solid(hids) => {
                    if hids.len() == s.iter_blocktypes().count() {
                        if self.hovering == Some((*s, None)) {
                            grid_cols.push("minmax(auto, 30px) ".to_string());
                        } else {
                            grid_cols.push("minmax(auto, 10px) ".to_string());  
                        }        
                    } else {
                        grid_cols.push("minmax(auto, 240px) ".to_string());     
                    }
                },
                StagSplit::Split(pre, mid, post) => {
                    if pre.len() == s.iter_blocktypes().count() {
                        if self.hovering == Some((*s, Some(0))) {
                            grid_cols.push("minmax(auto, 30px) ".to_string());
                        } else {
                            grid_cols.push("minmax(auto, 10px) ".to_string());  
                        }  
                    } else {
                        grid_cols.push("min-content ".to_string());
                    }
                    
                    grid_cols.push("minmax(auto, 240px) ".to_string());     

                    if post.len() == s.iter_blocktypes().count() {
                        if self.hovering == Some((*s, Some(2))) {
                            grid_cols.push("30px ".to_string());
                        } else {
                            grid_cols.push("10px ".to_string());  
                        }  
                    } else {
                        grid_cols.push("min-content ".to_string());
                    }
                },
            }
        }
        return grid_cols
    }

    pub fn get_tree_type_button_prims(&self) -> Vec<(String, bool, bool, DynaTreeType)> {
        // (text, active, clickable, callback_value)
        let mut res = vec![];
        for tt in DynaTreeType::iter() {
            res.push((tt.to_string(), tt == self.tt, tt != self.tt, tt));
        }
        return res
    }

    pub fn tree_type_button_callback(&mut self, tt: DynaTreeType) {
        assert!(self.tt != tt);
        if self.fc == FocusColumn::Normal && tt == DynaTreeType::Hourglass {
            self.items.get_mut(&self.get_selected_stage()).unwrap().0 = StagSplit::Split(vec![], vec![], vec![]);
        }
        self.tt = tt;
    }

    pub fn get_focus_type_button_prims(&self) -> Vec<(String, bool, bool, FocusColumn)> {
        // (button text, active, do_callback, callback)
        let mut res = vec![];
        for fc in FocusColumn::iter() {
            res.push((fc.to_string(), (fc == self.fc) && (self.tt != DynaTreeType::Hourglass), (fc != self.fc) && (self.tt != DynaTreeType::Hourglass), fc));
        }
        return res
    }
    
    pub fn focus_type_button_callback(&mut self, fc: FocusColumn) {
        assert!(self.tt != DynaTreeType::Hourglass);
        assert!(self.fc != fc);

        if fc == FocusColumn::Normal {
            self.items.get_mut(&self.get_selected_stage()).unwrap().0 = StagSplit::Solid(vec![]);
        } else if self.fc == FocusColumn::Normal {
            self.items.get_mut(&self.get_selected_stage()).unwrap().0 = StagSplit::Split(vec![], vec![], vec![]);
        }
        self.fc = fc;
    }

    pub fn get_blocktype_select_button_prims(&self) -> Vec<(String, bool, bool, BlockType)> {
        let mut res = vec![];
        for bt in BlockType::iter_reals() {
            res.push((bt.to_string(), bt.of_same_type(self.selected_bt), !bt.of_same_type(self.selected_bt), bt));
        }
        return res
    }

    pub fn blocktype_select_button_callback(&mut self, bt: BlockType) {
        assert!(!bt.of_same_type(self.selected_bt));

        match self.items.get(&self.selected_bt.stage()).unwrap().0 {
            StagSplit::Solid(_) => {
                self.items.get_mut( &bt.stage()).unwrap().0 = StagSplit::Solid(vec![]);
            },
            StagSplit::Split(_, _, _) => {
                self.items.get_mut(&bt.stage()).unwrap().0 = StagSplit::Split(vec![], vec![], vec![]);
            }
        }
        if bt.stage() != self.selected_bt.stage() {
            self.items.insert(
                self.selected_bt.stage(), (
                    StagSplit::Solid(vec![]),
                    StagSelect::Base,
                )
            );
        }

        self.items.get_mut(&bt.stage()).unwrap().1 = StagSelect::Selected;
        self.selected_bt = bt;
        self.assert_valid();
    }

    pub fn get_bt_hide_prims(&self) -> Vec<(String, bool, bool, (Stage, Option<usize>, BlockType))> {
        let mut res = vec![];
        for (s, (split, sele)) in self.items.iter() {
            match split {
                StagSplit::Solid(hids) => {
                    for bt in s.iter_blocktypes() {
                        res.push((bt.to_string(), hids.contains(&bt), self.selected_bt != bt, (*s, None, bt)));
                    }
                },
                StagSplit::Split(pre, mid, post) => {
                    for bt in s.iter_blocktypes() {
                        res.push(("pre-".to_string() + bt.to_string().as_str(), pre.contains(&bt), true, (*s, Some(0), bt)));
                    }
                    for bt in s.iter_blocktypes() {
                        res.push(("post-".to_string() + bt.to_string().as_str(), post.contains(&bt), true, (*s, Some(2), bt)));
                    }
                }
            }
        }
        return res
    }

    pub fn bt_hide_button_callback(&mut self, (stage, sub, bt): (Stage, Option<usize>, BlockType)) {
        assert!(bt.stage() == stage);
        let (sp, ss) = self.items.get_mut(&stage).unwrap();

        let toggle = |v: &mut Vec<BlockType>| {
            if v.contains(&bt) {
                v.retain(|b| *b != bt);
            } else {
                v.push(bt);
            }
        };

        match sp {
            StagSplit::Solid(hids) => {
                assert!(sub.is_none());
                toggle(hids);
            },
            StagSplit::Split(pre, mid, post) => {
                assert!(sub.is_some());
                let sub = sub.unwrap();
                assert!(sub <= 2);
                if sub == 0 {toggle(pre)}
                else if sub == 1 {toggle(mid)}
                else if sub == 2 {toggle(post)}
            }
        }

    }

    pub fn get_backerinos_prims(&self) -> Vec<(String, bool, (Stage, Option<usize>))> {
        // (Vec<(classes, do_callback, call_back)>)
        let mut res = vec![];
        for (s, (split, sel)) in self.items.iter() {
            match split {
                StagSplit::Solid(hids) => {
                    if hids.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, None)) {
                            res.push(("hidden hovered ".to_string(), true, (*s, None)));
                        } else {
                            res.push(("hidden ".to_string(), true, (*s, None)));
                        }
                    } else {
                        res.push(("".to_string(), false, (*s, None)));
                    }
                },
                StagSplit::Split(pre, mid, post) => {
                    if pre.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, Some(0))) {
                            res.push(("pre hidden hovered ".to_string(), true, (*s, Some(0))));
                        } else {
                            res.push(("pre hidden ".to_string(), true, (*s, Some(0))));
                        }
                    } else {
                        res.push(("pre ".to_string(), false, (*s, None)));
                    }

                    res.push(("mid".to_string(), false, (*s, None)));

                    if post.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, Some(2))) {
                            res.push(("post hidden hovered ".to_string(), true, (*s, Some(2))));
                        } else {
                            res.push(("post hidden ".to_string(), true, (*s, Some(2))));
                        }
                    } else {
                        res.push(("post ".to_string(), false, (*s, None)));
                    }
                },
            }
        }

        // add "prehidden " to items before a hidden one, and "posthidden " to items after a hidden one
        let mut i = 0;
        while i < res.len()-1 {
            if res[i+1].1 {
                res[i].0 += "prehidden ";
            }
            i += 1;
        }
        i = 1;
        while i < res.len() {
            if res[i-1].1 {
                res[i].0 += "posthidden ";
            }
            i += 1
        }

        return res
    }

    pub fn backerinos_callback(&mut self, (stage, sub): (Stage, Option<usize>)) {
        self.hovering = None;
        self.unhide_all_of_pseudo_stage(stage, sub);
    }

    pub fn get_headerino_prims(&self) -> Vec<(String, String, bool, (Stage, Option<usize>))> {
        // (text, classes, do_callback, callback)
        let mut res = vec![];
        for (s, (split, sel)) in self.items.iter() {
            match split {
                StagSplit::Solid(hids) => {
                    if hids.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, None)) {
                            res.push((s.to_string(), "hovered hidden ".to_string(), true, (*s, None)));
                        } else {
                            res.push((s.to_string(), "hidden ".to_string(), true, (*s, None)));
                        }
                    } else {
                        res.push((s.to_string(), " ".to_string(), false, (*s, None)));
                    }
                },
                StagSplit::Split(pre, mid, post) => {
                    if pre.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, Some(0))) {
                            res.push(("".to_string(), "pre hidden hovered ".to_string(), true, (*s, Some(0))));
                        } else {
                            res.push(("".to_string(), "pre hidden ".to_string(), true, (*s, Some(0))));
                        }
                    } else {
                        res.push(("".to_string(), "pre ".to_string(), false, (*s, None)));
                    }

                    res.push((s.to_string(), "mid".to_string(), false, (*s, None)));

                    if post.len() == s.iter_blocktypes().count() {
                        if self.is_hovered((*s, Some(2))) {
                            res.push(("".to_string(), "post hidden hovered ".to_string(), true, (*s, Some(2))));
                        } else {
                            res.push(("".to_string(), "post hidden ".to_string(), true, (*s, Some(2))));
                        }
                    } else {
                        res.push(("".to_string(), "post ".to_string(), false, (*s, None)));
                    }
                },
            }
            
        }

        // add "prehidden " to items before a hidden one, and "posthidden " to items after a hidden one
        let mut i = 0;
        while i < res.len()-1 {
            if res[i+1].2 {
                res[i].1 += "prehidden ";
            }
            i += 1;
        }
        i = 1;
        while i < res.len() {
            if res[i-1].2 {
                res[i].1 += "posthidden ";
            }
            i += 1
        }

        return res
    }

    pub fn backerinos_onhover_callback(&mut self, (stage, sub): (Stage, Option<usize>)) {
        self.hovering = Some((stage, sub));
    }

    pub fn backerinos_offhover_callback(&mut self, (stage, sub): (Stage, Option<usize>)) {
        self.hovering = None;
    }

    pub fn get_hidden_bts(&self) -> (Vec<BlockType>, Vec<BlockType>){
        // (left of select, right of select)
        // !!!  != (before select, after select) in tree  !!!
        let mut res: (Vec<BlockType>, Vec<BlockType>) = (vec![], vec![]);
        let mut side = vec![];

        for (s, (split, sel)) in self.items.iter() {
            match (split, sel) {
                (StagSplit::Solid(hids), StagSelect::Base) => {
                    side.append(&mut hids.clone());
                },
                (StagSplit::Solid(hids), StagSelect::Selected) => {
                    res.0 = side;
                    side = vec![];
                },
                (StagSplit::Split(pre, mid, post), StagSelect::Selected) => {
                    side.append(&mut pre.clone());
                    res.0 = side;
                    side = post.clone();
                },
                _ => {panic!("no");}
            }
        }

        res.1 = side;

        return res
    }

}



pub fn DynaTable(cx: Scope) -> Element {
    let con = use_shared_state::<Constellation>(cx).unwrap();
    // let _binding = con.read();

    let settings_coll = use_state(cx, || CollapsableToggle::Collapsed);
    let content_coll = use_state(cx, || CollapsableToggle::Expanded);

    let mut stagger = Stagerino::new(BlockType::Ui(Ui::META));
    stagger.bt_hide_button_callback((Stage::Layout, None, BlockType::Layout(Layout::META)));
    stagger.bt_hide_button_callback((Stage::Langbridge, None, BlockType::Langbridge(Langbridge::META)));
    stagger.assert_valid();
    let stagger = use_ref(cx, || stagger);

    let hidden_bts = stagger.with(|s| s.get_hidden_bts());

    let grid_temp_cols_string = stagger.with(|s| s.get_grid_cols().into_iter().fold("".to_string(), |acc, x| acc + x.as_str()).to_string());

    // let now = wasm_timer::Instant::now();    

    let stuff = cx.render(rsx!(
        section {
            class: "dynatable expanded{content_coll.get().is_expanded()}",
            onclick: move |_| {
                if content_coll.get().is_collapsed() {
                    content_coll.set(CollapsableToggle::Expanded);
                }
            },
            div {class: "header",
                onclick: move |_| {
                    if *content_coll.get() == CollapsableToggle::Collapsed && *settings_coll.get() == CollapsableToggle::Collapsed {
                        content_coll.set(CollapsableToggle::Expanded);
                    } else {
                        content_coll.set(CollapsableToggle::Collapsed);
                        settings_coll.set(CollapsableToggle::Collapsed)
                    }
                },
                div {
                    class: "title",
                    "Section Title"
                },
                button {
                    class: "settingstoggle shown{content_coll.get().is_expanded()}",
                    onclick: move |e| {
                        if *content_coll.get() == CollapsableToggle::Expanded {
                            settings_coll.set(settings_coll.toggle());
                            e.stop_propagation();
                        }
                    },
                    img {
                        src: "/img/gear.svg",
                        alt: "toggle section settings",
                    }
                }  
            },
            div{class:"collapsable {settings_coll}", div {class: "settings",
                div {
                    class: "selector",
                    div {class: "title", "Select Tree"},
                    div {for (text, active, clickable, callback) in stagger.with(|s| s.get_tree_type_button_prims()) {
                        div {
                            onclick: move |_| {if clickable {stagger.with_mut(|s| s.tree_type_button_callback(callback))}},
                            class: "active{active} clickable{clickable}",
                            "{text}"
                        }
                    }}
                },
                div {
                    class: "selector",
                    div {class: "title", "Select Focus"},
                    div {for (text, active, clickable, callback) in stagger.with(|s| s.get_focus_type_button_prims()) {
                        div {
                            onclick: move |_| {if clickable {stagger.with_mut(|s| s.focus_type_button_callback(callback))}},
                            class: "active{active} clickable{clickable}",
                            "{text}"
                        }
                    }}
                },
                div {
                    class: "selector",
                    div {class: "title", "Select Blocktype"},
                    div {for (text, active, clickable, callback) in stagger.with(|s| s.get_blocktype_select_button_prims()) {
                        div {
                            onclick: move |_| {if clickable {stagger.with_mut(|s| s.blocktype_select_button_callback(callback))}},
                            class: "active{active} clickable{clickable}",
                            "{text}"
                        }
                    }}
                },
                div {
                    class: "selector",
                    div {class: "title", "Hide Blocks"},
                    div {for (text, active, clickable, callback) in stagger.with(|s| s.get_bt_hide_prims()) {
                        div {
                            onclick: move |_| {if clickable {stagger.with_mut(|s| s.bt_hide_button_callback(callback))}},
                            class: "active{active} clickable{clickable}",
                            "{text}"
                        }
                    }}
                },
            }},
            div {class: "collapsable {content_coll}", div {class: "content",
                style: "
                    display: grid; 
                    grid-template-columns: {grid_temp_cols_string};
                    grid-template-rows: repeat({con.read().get_all_of_bt(&stagger.with(|s| s.selected_bt)).len()+1}, auto);
                    transition: grid-template-columns 200ms;
                ",
                div {style:"display: contents;", for (i, (class, do_callback, callback)) in stagger.with(|s| s.get_backerinos_prims()).into_iter().enumerate() {
                    div {
                        onclick: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_callback(callback))};},
                        onmouseover: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_onhover_callback(callback))};},
                        onmouseout: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_offhover_callback(callback))};},
                        class: "stage_back {class}",
                        style: "
                            grid-column: {i+1} / {i+2};
                            grid-row: 2 / -1;
                        ",
                    }
                }},
                div {style:"display: contents;", for (i, (name, class, do_callback, callback)) in stagger.with(|s| s.get_headerino_prims()).into_iter().enumerate() {
                    div {
                        onclick: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_callback(callback))};},
                        onmouseover: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_onhover_callback(callback))};},
                        onmouseout: move |_| {if do_callback {stagger.with_mut(|s| s.backerinos_offhover_callback(callback))};},
                        class: "stage_header {class}",
                        style: "
                            grid-column: {i+1} / {i+2};
                        ",
                        "{name}"
                    }
                }},
                {info!("{}\n KEY: {}\n{}", "=".repeat(30), format!("{hidden_bts:?}{}{}{}i", stagger.with(|s| s.selected_bt), stagger.with(|s| s.tt), stagger.with(|s| s.fc)), "=".repeat(30));},
                div {
                    style:"display: contents;", 
                    match stagger.with(|s| s.tt) {
                        DynaTreeType::FullDown  => {rsx!(
                            for (i, (primary_bt, _)) in con.read().get_all_of_bt(&stagger.with(|s| s.selected_bt)).into_iter().enumerate() {
                                PrimaryFull{
                                    orig_block: primary_bt, 
                                    prim_start_grid_line: (i+2) as i32,
                                    tree_type: stagger.with(|s| s.tt),
                                    focus_col: stagger.with(|s| s.fc),
                                    hidden_bts: hidden_bts.clone(),
                                    key: "{hidden_bts:?}{stagger.with(|s| s.selected_bt)}{stagger.with(|s| s.tt)}{stagger.with(|s| s.fc)}{i}"
                                }
                            })
                        }, DynaTreeType::FullUp => {rsx!(
                            for (i, (primary_bt, _)) in con.read().get_all_of_bt(&stagger.with(|s| s.selected_bt)).into_iter().enumerate() {
                                PrimaryFull{
                                    orig_block: primary_bt, 
                                    prim_start_grid_line: (i+2) as i32,
                                    tree_type: stagger.with(|s| s.tt),
                                    focus_col: stagger.with(|s| s.fc),
                                    hidden_bts: hidden_bts.clone(),
                                    key: "{hidden_bts:?}{stagger.with(|s| s.selected_bt)}{stagger.with(|s| s.tt)}{stagger.with(|s| s.fc)}{i}"
                                }
                            })
                        }, DynaTreeType::Hourglass => {rsx!(
                            for (i, (primary_bt, _)) in con.read().get_all_of_bt(&stagger.with(|s| s.selected_bt)).into_iter().enumerate() {
                                PrimarySplit{
                                    orig_block: primary_bt, 
                                    prim_start_grid_line: (i+2) as i32,
                                    tree_type: stagger.with(|s| s.tt),
                                    hidden_bts: hidden_bts.clone(),
                                    // timer: now,
                                    key: "{hidden_bts:?}{stagger.with(|s| s.selected_bt)}{stagger.with(|s| s.tt)}{stagger.with(|s| s.fc)}{i}"
                                }
                            })
                        },
                    }
                }
            }}
        },
    ));
    // info!("DYNATABLE RERENDEr in {:?}", now.elapsed());
    return stuff
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockBoxerEvent {
    Delete(SingleBlockId),
    Undelete(SingleBlockId),
    Cluster(SingleBlockId),
    Uncluster(SingleBlockId),
}


#[inline_props]
pub fn PrimaryFull(
    cx: Scope,
    orig_block: BlockType,
    prim_start_grid_line: i32,
    tree_type: DynaTreeType,
    focus_col: FocusColumn,
    hidden_bts: (Vec<BlockType>, Vec<BlockType>),
) -> Element {
    assert!(*tree_type == DynaTreeType::FullDown || *tree_type == DynaTreeType::FullUp);

    let con = use_shared_state::<Constellation>(cx).unwrap();

    let tree: &UseRef<_> = use_ref(cx, || 
        match tree_type {
            DynaTreeType::FullDown => {
                DynaTree::from_tree_with_hidden_bts(&con.read().get_node(orig_block).fulldowntree, *orig_block, &hidden_bts.0, &hidden_bts.1)
            }, DynaTreeType::FullUp => {
                DynaTree::from_tree_with_hidden_bts(&con.read().get_node(orig_block).fulluptree, *orig_block, &hidden_bts.1, &hidden_bts.0)
            }, _=> {panic!()}
        }
    );

    let line_count = tree.with(|t: &DynaTree| t.get_wid());
    let prims = tree.with(|t| t.get_full_render_prims(*orig_block, *focus_col));

    let col_count = if *focus_col == FocusColumn::Normal {Stage::last().lvl()+1} else {Stage::last().lvl()+3};
    let mut staged_prims: Vec<Vec<DynaRenderPrims>> = vec![Vec::new(); (col_count+1) as usize]; // +1 for root/leaf, skipped later

    for p in prims {
        assert!(p.rel_stage_i < staged_prims.len() as i32, "stage_i to high: {p:?}");
        staged_prims[p.rel_stage_i as usize].push(p);
    }

    // stage block col span
    let colspan = |i: usize| {
        if *tree_type == DynaTreeType::FullDown {
            format!("{} / span 1", i+1)
        } else {
            format!("{} / span 1", (col_count - (i as i32)) as i32)
        }
    };

    let class_name_dir = match tree_type {
        DynaTreeType::FullDown => {"down"},
        DynaTreeType::FullUp => {"up"},
        _ => {panic!("no")}
    };

    let class_focus = |i| {
        if *focus_col == FocusColumn::Normal && i as i32 == orig_block.stage().lvl() {
            "focussed"
        } else if *focus_col != FocusColumn::Normal && i as i32 == orig_block.stage().lvl() + 1 {
            "focussed"
        } else {
            ""
        }   
    };

    return cx.render(rsx!{
        div {
            class: "primary {class_name_dir}",
            style: "
                display: grid;
                grid-column: 1 / -1;
                grid-row: {prim_start_grid_line} / span 1;

                grid-template-columns: subgrid;
                grid-template-rows: repeat({line_count}, auto);
            ",
            for (i, prims) in staged_prims.into_iter().skip(1).enumerate() {
                div {
                    class: "stage_box {class_focus(i)}",
                    style: "
                        grid-column: {colspan(i)};
                    ",
                    for prim in prims.into_iter() {
                        BlockBox{
                            orig_block: *orig_block,
                            prim: prim,
                            on_bonk: move |e: BlockBoxerEvent| {
                                match e {
                                    BlockBoxerEvent::Cluster(id) => {tree.with_mut(|t| t.group_node(id));},
                                    BlockBoxerEvent::Uncluster(id) => {
                                        tree.with_mut(|t| 
                                            match tree_type {DynaTreeType::FullDown => {
                                                t.ungroup_node(id, &con.read().get_node(orig_block).fulldowntree, *orig_block, &hidden_bts.0, &hidden_bts.1);
                                            },
                                            DynaTreeType::FullUp => {
                                                t.ungroup_node(id, &con.read().get_node(orig_block).fulluptree, *orig_block, &hidden_bts.1, &hidden_bts.0);
                                            },
                                            _ => {panic!("no")},
                                            }
                                        );
                                    },
                                    BlockBoxerEvent::Delete(id) => {tree.with_mut(|t| t.delete_node(id));},
                                    BlockBoxerEvent::Undelete(id) => {tree.with_mut(|t| t.undelete_node(id));},
                                }
                            },
                            dir: if *tree_type == DynaTreeType::FullDown {BlockDirection::Down} else {BlockDirection::Up},
                            // timer: wasm_timer::Instant::now(),
                        }
                    }
                }
            }
        }
    })
}


#[inline_props]
pub fn PrimarySplit(
    cx: Scope,
    orig_block: BlockType,
    prim_start_grid_line: i32,
    tree_type: DynaTreeType,
    // timer: wasm_timer::Instant,
    hidden_bts: (Vec<BlockType>, Vec<BlockType>),
) -> Element {
    assert!(*tree_type == DynaTreeType::Hourglass);
    // {info!("START SPLIT PRIMARY in {:?}", timer.elapsed());}
    let con = use_shared_state::<Constellation>(cx).unwrap();

    let trees = (
        use_ref(cx, || DynaTree::from_tree_with_hidden_bts(&con.read().get_node(orig_block).splituptree,   *orig_block, &hidden_bts.1, &hidden_bts.0)),
        use_ref(cx, || DynaTree::from_tree_with_hidden_bts(&con.read().get_node(orig_block).splitdowntree, *orig_block, &hidden_bts.0, &hidden_bts.1)),
    );

    let line_counts = (trees.0.with(|t| t.get_wid()), trees.1.with(|t| t.get_wid()));
    let prims = (trees.0.with(|t| t.get_split_render_prims()), trees.1.with(|t| t.get_split_render_prims()));

    let col_counts = (orig_block.stage().lvl()+1, Stage::last().lvl() - orig_block.stage().lvl()+1); // up and down 

    let mut staged_prims: (Vec<Vec<DynaRenderPrims>>, Vec<Vec<DynaRenderPrims>>) = (
        vec![vec![]; (col_counts.0) as usize], 
        vec![vec![]; (col_counts.1) as usize]
    ); // +1 for root/leaf, skipped later

    let mut stage_inner_depths: (Vec<i32>, Vec<i32>) = (
        vec![0; (col_counts.0) as usize], 
        vec![0; (col_counts.1) as usize]
    ); // +1 for root/leaf, skipped later


    let mid = prims.0.iter().find(|prim| prim.bt == *orig_block).unwrap().clone();

    for p in prims.0.into_iter().filter(|prim| prim.bt != *orig_block) {
        stage_inner_depths.0[p.rel_stage_i as usize] = p.inner_stage_col_start + p.inner_stage_col_span;
        staged_prims.0[p.rel_stage_i as usize].push(p);
    }
    for p in prims.1.into_iter().filter(|prim| prim.bt != *orig_block) {
        stage_inner_depths.1[p.rel_stage_i as usize] = p.inner_stage_col_start + p.inner_stage_col_span;
        staged_prims.1[p.rel_stage_i as usize].push(p);
    }

    // column span for stage block
    let stage_colspan = |up: bool, i: usize| {
        if !up {
            format!("{} / span 1", i+1)
        } else {
            format!("{} / span 1", (col_counts.0 - (i as i32)))
        }
    };

    // let mut block_timer = wasm_timer::Instant::now();

    // {info!("IN SPLIT PRIMARY in {:?}", timer.elapsed());}
    return cx.render(rsx!{
        div {
            class: "primary",
            style: "
                display: grid;
                grid-column: 1 / -1;
                grid-row: {prim_start_grid_line} / span 1;

                grid-template-columns: subgrid;
                grid-template-rows: auto;
            ",
            div {
                class: "up",
                style: "
                    display: grid;
                    grid-column: 1 / span {col_counts.0};
                    grid-row: 1 / span 1;
                    
                    grid-template-rows: repeat({line_counts.0}, auto);
                    grid-template-columns: subgrid;
                ",
                for (i, prims) in staged_prims.0.into_iter().enumerate() {
                    div {
                        class: "stage_box",
                        style: "
                            grid-column: {stage_colspan(true, i)};
                        ",
                        for prim in prims.into_iter() {
                            BlockBox{
                                orig_block: *orig_block,
                                prim: prim,
                                on_bonk: move |e: BlockBoxerEvent| {
                                    match e {
                                        BlockBoxerEvent::Cluster(id) => {trees.0.with_mut(|t| t.group_node(id)); /*block_timer = wasm_timer::Instant::now()*/},
                                        BlockBoxerEvent::Uncluster(id) => {trees.0.with_mut(|t| t.ungroup_node(id, &con.read().get_node(orig_block).splituptree, *orig_block, &hidden_bts.1, &hidden_bts.0))}
                                        BlockBoxerEvent::Delete(id) => {trees.0.with_mut(|t| t.delete_node(id));},
                                        BlockBoxerEvent::Undelete(id) => {trees.0.with_mut(|t| t.undelete_node(id));},
                                    }
                                },
                                dir: if *tree_type == DynaTreeType::FullDown {BlockDirection::Down} else {BlockDirection::Up},
                                // timer: block_timer,
                            }
                        }
                    }
                }
            },
            div {
                class: "mid stage_box",
                style: "
                    grid-column: {col_counts.0+1} / span 1;
                    
                    grid-template-rows: auto;
                    grid-template-columns: subgrid;
                ",
                BlockBox{
                    orig_block: *orig_block,
                    prim: mid,
                    on_bonk: move |e: BlockBoxerEvent| {},
                    dir: BlockDirection::Up,
                    // timer: block_timer,
                }
            }
            div {
                class: "down",
                style: "
                    display: grid;
                    grid-column: {col_counts.0+2} / -1;
                    grid-row: 1 / span 1;
                    
                    grid-template-rows: repeat({line_counts.1}, auto);
                    grid-template-columns: subgrid;
                ",
                for (i, prims) in staged_prims.1.into_iter().enumerate() {
                    div {
                        class: "stage_box",
                        style: "
                            grid-column: {stage_colspan(false, i)};
                        ",
                        for prim in prims.into_iter() {
                            BlockBox{
                                orig_block: *orig_block,
                                prim: prim,
                                on_bonk: move |e: BlockBoxerEvent| {
                                    match e {
                                        BlockBoxerEvent::Cluster(id) => {trees.1.with_mut(|t| t.group_node(id)); /*block_timer = wasm_timer::Instant::now();*/},
                                        BlockBoxerEvent::Uncluster(id) => {trees.1.with_mut(|t| t.ungroup_node(id, &con.read().get_node(orig_block).splitdowntree, *orig_block, &hidden_bts.0, &hidden_bts.1))}
                                        BlockBoxerEvent::Delete(id) => {trees.1.with_mut(|t| t.delete_node(id));},
                                        BlockBoxerEvent::Undelete(id) => {trees.1.with_mut(|t| t.undelete_node(id));},
                                    }
                                },
                                dir: BlockDirection::Down,
                                // timer: block_timer,
                            }
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn BlockBox<'a>(
    cx: Scope<'a>, 
    orig_block: BlockType,
    prim: DynaRenderPrims,
    on_bonk: EventHandler<'a, BlockBoxerEvent>,
    dir: BlockDirection,
) -> Element<'a> {

    // info!("BLOCK BOX START: {:?}", timer.elapsed());
    let con = use_shared_state::<Constellation>(cx).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(con.read().get_node(&prim.bt).info.owner.to_string());
    let result = hasher.finalize();
    let color_nums = &result[1..=3].to_vec();

    // info!("{color_nums:?}");

    let mut color_str = format!("rgba({}, {}, {}, 0.2)", color_nums[0], color_nums[1], color_nums[2]);
    // info!("{color_str}");

    color_str = {
        match con.read().get_node(&prim.bt).info.owner.value().light_back {
            Some(c) => {c.to_string()},
            None => {color_str}
        }
    };

    let colspan: String = {
        if *dir == BlockDirection::Down {
            format!("grid-column: {} / span {};", prim.inner_stage_col_start+1, prim.inner_stage_col_span)
        } else {
            format!("grid-column: {} / span {};", -prim.inner_stage_col_start-1-prim.inner_stage_col_span, prim.inner_stage_col_span)
        }
    };


    let is_top_of_prim_line_class = if prim.wid_start_i == 0 {""} else {"draw_top_seperator"};

    let focus_class = if *orig_block == prim.bt {
        "focussed"
    } else {
        ""
    };

    match &prim.nt {
        NodeType::Normal => {
            return cx.render(rsx!(
                div {
                    class: "block_box normal {is_top_of_prim_line_class} {focus_class}",
                    style: "
                        grid-row: {prim.wid_start_i+1} / {prim.wid_end_i + 2};
                        {colspan};
                        position: relative;
                    ",
                    div {
                        class: "block_button delete",
                        onclick: move |_| {on_bonk.call(BlockBoxerEvent::Delete(prim.id));},
                    },
                    div {
                        class: "block_button group",
                        onclick: move |_| {on_bonk.call(BlockBoxerEvent::Cluster(prim.id));},
                    },
                    div {
                        style: "
                            position: absolute;
                            top: 0; bottom: 0; left: 0; right: 0;
                            background: {color_str};
                            opacity: 0.4;
                        "
                    }
                    {prim.bt.inner_to_string()}
                }
            ))
        },
        NodeType::DeleteCluster{..} => {
            return cx.render(rsx!(
                div {
                    class: "block_box deleted {is_top_of_prim_line_class}",
                    style: "
                        grid-row: {prim.wid_start_i+1} / {prim.wid_end_i + 2};
                        {colspan}
                    ",
                    div {
                        onclick: move |_| {
                            on_bonk.call(BlockBoxerEvent::Undelete(prim.id));
                        },
                    }
                }
            ))
        },
        NodeType::GroupCluster{clustered_nodes, clustered_prims} => {
            return cx.render(rsx!(
                div {
                    class: "block_box grouped {is_top_of_prim_line_class}",
                    style: "
                        grid-row: {prim.wid_start_i+1} / {prim.wid_end_i + 2};
                        {colspan}
                    ",
                    div {
                        onclick: move |_| {
                            on_bonk.call(BlockBoxerEvent::Uncluster(prim.id));
                        },
                        for (i, n) in clustered_nodes.iter().enumerate() {
                            div {"{clustered_prims[i].inner_to_string()}"}
                        }
                    }
                }
            ))
        }
    }
}








