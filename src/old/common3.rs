use std::collections::HashMap;
use std::collections::HashSet;

use leptos::Scope;
use leptos::View;

use crate::owner::Owner;
use crate::lang::Lang;

use crate::langbridge::Langbridge;
use crate::langbridge::LangbridgeInfo;

use crate::ui::Ui;
use crate::ui::UiInfo;
use crate::layout::Layout;
use crate::layout::LayoutInfo;
use crate::render::Render;
use crate::render::RenderInfo;
use crate::intergfx::Intergfx;
use crate::intergfx::IntergfxInfo;
use crate::gfx::Gfx;
use crate::gfx::GfxInfo;
use crate::platform::Platform;
use crate::platform::PlatformInfo;

// use macroni::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bt{
    Root,
    AllFollowing,
    LangBridge(Langbridge),
    
    Ui(Ui),
    Layout(Layout),
    Render(Render),
    Gfx(Gfx),
    InterGfx(Intergfx),
    Platform(Platform),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Info {
    pub name: String,
    pub owner: Owner,
    pub description: String,
    pub imp_lang: Lang, // implementation language
    pub bind_langs: Vec<Lang>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExtraInfo {
    Root,
    LangBridge(LangbridgeInfo),

    Ui(UiInfo),
    Layout(LayoutInfo),
    Render(RenderInfo),
    Intergfx(IntergfxInfo),
    Gfx(GfxInfo),
    Platform(PlatformInfo),
}





#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Digraph<'a> {
    pub root: &'a Node<'a>,
    pub nodes: Vec<&'a Node<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node<'a> {
    pub blocktype: Bt,
    pub info: Info,
    pub extra: ExtraInfo,
    pub prevs: Vec<&'a Node<'a>>,
    pub nexts: Vec<&'a Node<'a>>,
    pub nexts_to_leaf: Vec<&'a Digraph<'a>>,
}

impl Node<'_> {
    fn has_in_nexts(&self, n: &Node) -> bool {
        for next in &self.nexts {
            if **next == *n {
                return true
            }
        }
        return false
    }

    fn has_in_prevs(&self, n: &Node) -> bool {
        for prev in &self.prevs {
            if **prev == *n {
                return true
            }
        }
        return false
    }
}

impl <'a>Digraph<'a> {

    fn add(self: &'a mut Digraph<'a>, node: Node<'a>) {
        if self.contains(&node) {
            panic!("Tried to add node to data structure twice")
        }
        self.nodes.push(&node);
    }

    fn remove(self: &'a mut Digraph<'a>, node: &Node<'a>) {
        if self.contains(node) {
            for i in 0..self.nodes.len()-1 {
                if self.nodes[i] == node {
                    self.nodes.remove(i);
                }
            }
        }
    }

    fn link(&mut self, up_node: &'a mut Node<'a>, down_node: &'a mut Node<'a>) {
        for n in self.nodes {
            if n == up_node {
                n.nexts.push(down_node);
            }
            if n == down_node {
                n.nexts.push(up_node);
            }
        }
        // (*up_node).nexts.push(down_node);
        // (*down_node).prevs.push(up_node);
    }

    fn contains(&self, node: &Node) -> bool {
        for n in self.nodes {
            if n == node {
                return true
            }
        }
        return false
    }

    fn assert_bidir_cons(&self) {
        for node in &self.nodes {
            for prev in &node.prevs {
                assert!(prev.has_in_nexts(&node))
            }
            for next in &node.nexts {
                assert!(next.has_in_prevs(&node))
            }
        }
    }

    fn assert_no_dangling_nodes(&self) {
        // assert that self.nodes contains all the nodes in the graph

        fn check_downstream_node_list<'a>(selfy: &Digraph, checked_nodes: &mut HashSet<&'a Node<'a>>, to_check_nodes: HashSet<&'a Node<'a>>) {
            let mut unchecked_nodes: HashSet<&Node> = HashSet::new();
            for node in to_check_nodes {
                for next in node.nexts.iter() {
                    if !checked_nodes.contains(next) {
                        unchecked_nodes.insert(next);
                    }
                    assert!(selfy.nodes.contains(next))
                }
                checked_nodes.insert(node);
            }
            check_downstream_node_list(&selfy, checked_nodes, unchecked_nodes)
        }

        let mut checked_nodes: HashSet<&Node> = HashSet::new();
        let mut to_check_nodes: HashSet<&Node> = HashSet::new();
        to_check_nodes.insert(self.root);
        check_downstream_node_list(&self, &mut checked_nodes, to_check_nodes);
    }
}




pub struct BuildGraph {
    node: Bt,
    nexts: Vec<BuildGraph>
}

pub struct DownTreeUi {
    node: Bt,
    nexts: Vec<DownTreeUi>,
}



/*
    addui!(
        info: Info {
            name: ..,
            owner: ..
        },
        extra: ExtraOne{
            softness: ...,
            rating: ...,
        },
        paths: vec![
            (Ui, [(Ui2, [(Render1, [(Gfx, ...), ...]), (Render2, [(Gfx, ...), ...])])), Ui3)
        ],
        paths: vec![
            (Ui1,[
                (Render1, [
                    (Gfx,[
                        (...[...
                            ...
                        )
                    ])
                ]),
                (Render2,[
                    (Gfx,[
                        (...[
                            ...
                        ])
                    ]),
                    (Gfx,[
                        (...[
                            ...
                        ])
                    ]),
                    ...
                ]),
            ]),
            (Ui2,[
                ALL
            ]),

        ]
    )
*/

pub fn getRoot() -> Node<'static> {
    return Node {
        blocktype: Bt::Root,
        info: Info {
            name: String::from("ROOT NODE"),
            owner: Owner::Erithax,
            description: String::from("ROOT NODE"),
            imp_lang: Lang::Rust,
            bind_langs: vec![Lang::Rust],
        },
        extra: ExtraInfo::Root,
        prevs: Vec::new(),
        nexts: Vec::new(),
        nexts_to_leaf: Vec::new(),
    };
}

pub trait Blockify {
    fn addAll(blocks: &mut HashMap<Bt, Node>);

    fn getTextRep(&self, cx: Scope) -> View;
}