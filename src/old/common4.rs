
use petgraph::prelude::*;

type Pgraph = Graph<Node, (), Directed>;


pub enum Bt {
    One(OneComps),
    Two(TwoComps)
}



pub enum OneComps {OneOne,OneTwo}
pub enum TwoComps {TwoOne,TwoTwo}

pub struct Constellation {
    graph: pgraph,
}

impl Constellation {

    fn add_ui(one_type: OneComps, info: Info, extra: ExtraOne, branches: Vec<Bt>) {
        
    }
}

/*
Ui(Dioxus)
> Ui(Dom)
> Ui(CustomTree)
> > Layout(Taffy)
> > > Render(Blitz)
> > > > Gfx(Opengl)
> > > > Gfx(Vulkan)
> > > Render(BONK)
> > > > ALL

fn add_all (c) {
    c.add_node(Bt::One(
            onet: OneOne,
            info: Info {...},
            extra: ExtraOne {...},
            branches: {
                let mut branches = pgraph::new();
                
                let tone, two = branches.add_nodes(vec![TwoOne, TwoTwo])
                branches.add_edges([
                    ()
                ])


                return branches
            }
        )
    )
}


*/

pub struct Info {s: String}

pub enum Extra {One(ExtraOne),Two(ExtraTwo)}
pub struct ExtraOne {t: String, o: Vec<String>}
pub struct ExtraTwo {t: i32, o: String}

pub struct Node {
    bt: Bt,
    info: Info,
    extra: Extra,
    branches: pgraph,
}

// pub struct Digraph {
//     nodes: Vec<Node>
// }

pub trait Blockify {
    fn add_all();
}

#[macro_export]
macro_rules! useall {
    () => {
        use Bt::*;
        use OneComps::*;
    };
}

fn main() {
    // use Bt::*;
    // use OneComps::*;
    useall!();

    let mut c = pgraph::new();

    let  q = c.add_node(Node {
        bt: One(OneOne),
        info: Info {s: "TODO".to_string()},
        extra: Extra::One(ExtraOne{t: "TODO".to_string(), o: vec![" ".to_string()]}),
        branches: pgraph::new(),
    });

    println!("{q:?}");
}


///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
///////////////////////////////////////////////////////
///////////////////////////////////////////////////////


impl Blockify for OneComps {
    fn add_all() {
        
    }
}