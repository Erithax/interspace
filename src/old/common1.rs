
use std::collections::HashMap;

use crate::owners::Owner;
use crate::langs::Lang;

#[derive(PartialEq, Eq, Hash)]
pub enum BlockType {
    StateWidget,
    Layout,
    Render,
    Gfx,
}

pub struct Digraph {
    pub nodes: HashMap<String, Node>,
}

impl Digraph {
    pub fn add_node(&mut self, k: String, n: Node) {
        assert!(! self.nodes.contains_key(&k));
        self.nodes.insert(k, n);
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Node {
    pub info: Box<Info>,
    pub prev: Vec<Node>,
    pub next: Vec<Node>,
}


#[derive(PartialEq, Eq, Hash)]
pub struct Info {
    pub name: String,
    pub btype: BlockType,
    pub owner: Owner,
    pub desc: String,
    pub bindings: Vec<Lang>,
    pub lib_lang: Vec<(Lang, Option<i16>)>,
}

fn add_uis(g: &mut Digraph) {
    static KKK: &str =  "boink";
    g.add_node (  "KKK".to_string(), Node{
        info: Box::new(Info {
            name: String::from("yeya"),
            btype: BlockType::Render,
            owner: Owner::Google,
            desc: String::from("TODO"),
            bindings: Vec::new(),
            lib_lang: Vec::new(),
        }),
        prev: Vec::new(),
        next: Vec::new(),
    })
}