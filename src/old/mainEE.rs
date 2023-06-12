use std::collections::HashMap;

use lazy_static::lazy_static;


#[derive(PartialEq, Eq, Hash)]
pub enum Bt {
    One,
    Two
}

#[derive(PartialEq, Eq)]
pub struct Info {
    s: String
}

#[derive(PartialEq, Eq)]
pub struct Node {
    info: Info,
    prevs: Vec<&'static Node>,
    nexts: Vec<&'static Node>,
    branches: Digraph,
}

#[derive(PartialEq, Eq)]
pub struct Digraph {
    nodes: Vec<Node>
}

impl Digraph {
    fn new() -> Digraph {
        return Digraph {
            nodes: Vec::new()
        }
    }
}

fn addAllOne(c: &mut HashMap<Bt, Node>) {
    c.insert(Bt::One, Node {
        info: Info{s: "TODO".to_string()},
        prevs: Vec::new(),
        nexts: Vec::new(),
        branches: Digraph::new(),
    });
}

fn main() {
    lazy_static! {
        static ref CONSTELLATION: Digraph = {
            let mut c: Digraph = Digraph::new();
            
            return c
        };
    }
}