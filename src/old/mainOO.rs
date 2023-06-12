#![allow(dead_code)]
#![feature(stmt_expr_attributes)]

use leptos::*;
use strum::IntoEnumIterator;

mod lang;
mod owner;
mod langbridge;

mod ui;
mod layout;
mod render;
mod intergfx;
mod gfx;
mod platform;
mod common3;

use std::{collections::{LinkedList, HashMap, VecDeque}};
use stylers::style;
use crate::{ui::Ui};
use crate::common3::{Node, Digraph, Bt, getRoot, Blockify};



use std::str::FromStr;


fn mainO() {
    // let mut blocks: HashMap<Bt, Node> = HashMap::new();
    
    // Ui::addAll(&mut blocks);
    // //warn!("{blocks:?}");
    // let root = getRoot();

    // let mut nodes: Vec<&Node> = Vec::new();

    // for (bt, node) in blocks {
    //     nodes.push(&node)
    // }

    // println!("{nodes:?}");

    // let _constellation: Digraph = Digraph {root: &root, nodes : nodes};

    leptos::mount_to_body(|cx| view! { cx, <App/> });

    // println!("REACHED HERRE");
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    let mut blocks: HashMap<Bt, Node> = HashMap::new();
    
    Ui::addAll(&mut blocks);
    //warn!("{blocks:?}");
    let root = getRoot();

    let mut nodes: Vec<&Node> = Vec::new();
   

    for (bt, node) in &blocks {
        nodes.push(node)
    }

    println!("{nodes:?}");
    let _constellation: Digraph = Digraph {root: &root, nodes : nodes};
    

    // leptos::mount_to_body(|cx| view! { cx, <App/> });

    println!("REACHED HERRE");



    view!{cx,

    }

    /*
    let (count, set_count) = create_signal(cx, 0);

    let us = ui::Ui::iter();
    //let mut usv: Vec<Box<dyn IntoView>> = Vec::new();
    let mut usv: LinkedList<View> = LinkedList::new();
    let mut other_usv: LinkedList<View> = LinkedList::new();
    for u in Ui::iter() {
        usv.push_back(u.get_icon_rep(cx));
    }
    for u in Ui::iter() {
        other_usv.push_back(u.get_icon_rep(cx));
    }

    let _i = 888;
 
    view! { cx,
        <Header />

        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me2: "
            {count}
        </button>

        <MainSection heading = String::from("Introduction")>
            <p>"Understand all the relevant aspects of UI stacks. Including targets, state management paradigm, renderstack (-> performance!), and more."</p> 
        </MainSection>

        <MainSection heading = String::from("UI listing")>
            {
                us.clone()
                    .map(|n| view! { cx, <li>{n.value().name}</li>})
                    .collect::<Vec<_>>()
            }
        </MainSection>


       <MainSection heading = String::from("UI listing")>
            {
            usv.iter()
                .map(|_n| view!{cx, <p>{String::from("WOOP! ")}</p>})
                .collect::<Vec<_>>()
            }
        </MainSection>

        <MainSection heading = String::from("UI UI UI")>
            {
            other_usv.iter()
                .map(|n| n) //view!{cx, <p>{String::from("WOOsh!")}</p>})
                .collect::<Vec<_>>()
            }
        </MainSection>


    }
    */
}

// pub trait ProgRep:  Clone + Sized{
//     //fn getTextRep(self, cx: Scope) -> Box<dyn IntoView>;
//     fn get_icon_rep(self, cx: Scope) -> View;
//     //fn getIconTextRep(cx: Scope) -> Box<dyn IntoView>;
// }

#[component]
fn Header(cx: Scope) -> impl IntoView {

    let styler_class = style!{ "Header", 
        header {
            background-color: #222222;
            font-size: 32px;
        } 
    };

    view! { cx, class = styler_class, 
        <header>
           "Erithax' Great UI Overview"
        </header>
    }
}

#[component]
fn MainSection(cx: Scope, heading: String, children: Children) -> impl IntoView {
    let styler_class = style!{ "MainSection", 
        section {
            width: 80%;
            margin: 0 10% 20px 10%;
        } 
    };

    view! { cx, class = styler_class,
        <section>
            <h1>{heading}</h1>
            {children(cx)}
        </section>
        
    }
}