

use core::fmt::LowerHex;
use std::{collections::BTreeMap, char::MAX};
use log::{info, LevelFilter};
use dioxus::{prelude::*, html::a};
use strum::{self, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use lazy_static::lazy_static;

pub mod constellation;
mod component;
mod components;
mod tree;
mod stage;
pub mod owner;
mod lang;
mod block;
mod filters;
use filters::*;

use crate::*;
use {constellation::*, component::*, owner::*, lang::*, stage::*, tree::*};
use components::{langbridge::*, ui::*, layout::*, render::*, gfxapi::*, intergfx::*, platform::*};
use filters::{component_type_filter::*};

  
lazy_static!{
    static ref CONSTELLATION: Constellation = Constellation::generate();
}   


pub trait Blockify {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, crate::dyna_tab::components::ExtraInfo, Vec<Vec<ComponentStrId>>)>;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynaTabTree {
    LefToRig,
    RigToLef,
    Hourglass,
}

impl DynaTabTree {
    pub fn iterator() -> impl Iterator<Item = DynaTabTree> {
        [
            DynaTabTree::LefToRig,
            DynaTabTree::Hourglass,
            DynaTabTree::RigToLef,
        ].iter().copied()
    }
}



#[inline_props]
pub fn DynaTabTreeComp(cx: Scope, self_: UseRef<DynaTabTree>) -> Element {

    cx.render(rsx!{
        div {
            class: "selector",
            div {
                "Select tree type",
            },
            div {
                for pot_dyna_tab_tree in DynaTabTree::iterator() {{
                    let active = if pot_dyna_tab_tree == *self_.read() {"active"} else {"inactive"};
                    rsx!{
                        div {
                            onclick: move |_| {
                                self_.set(pot_dyna_tab_tree);
                            },
                            class: "{active} clickabletrue",
                            "{pot_dyna_tab_tree:?}",
                        }
                    }
                }}
            }
        }
        
    })
}

pub enum ShowFullInfo {
    InTreeAlways,
    InTreeOnHover,
    InTreeOnClick,
    AfterTreeOnClick,
}

#[inline_props]
pub fn DynaTab(cx: Scope, id: usize) -> Element {

    // APPLY SELECTION FILTERS
    // APPLY SELECTION SORTS
    // TRAVERSE & RENDER SELECTION LIST PRIMARIES
        // TRAVERSE & RENDER TREE
            // APPLY IN-TREE FILTER
            // APPLY GROUPCLUSTER AND DELETECLUSTER


    info!("cloning Constellation");
    let now = wasm_timer::Instant::now();
    let constellation = CONSTELLATION.clone();
    info!("Constellation.clone() in {:?}", now.elapsed());
    
    let settings_coll = use_state(cx, || CollapsableToggle::Expanded);
    let content_coll = use_state(cx, || CollapsableToggle::Expanded);


    let tree_type = use_ref(cx, || DynaTabTree::Hourglass);

    let comp_type_filter = use_ref(cx, || 
        Box::new(ComponentTypeFilter{
            allowed: vec![ComponentType::Ui],
        })
    );
    

    let mut grid_template_rows_string = String::from("grid-template-rows: ");
    for (comp_id, comp) in constellation.comps.iter().enumerate() {
        grid_template_rows_string += &("[".to_owned() + comp.str_id);
        grid_template_rows_string += "-s] ";
        grid_template_rows_string += "auto ";
        grid_template_rows_string += &("[".to_owned() + comp.str_id);
        grid_template_rows_string += "-e] ";
        grid_template_rows_string += "auto ";
    } 

    let mut all_substages: Vec<SubStage> = Vec::new();
    for stage in Stage::iter_reals() {
        for pseudo in PseudoStage::iter() {
            all_substages.push(
                SubStage {
                    stage: stage,
                    pseudostage: pseudo,
                }
            );
        }
    }
    
    cx.render(rsx!{
        section {
            onmounted: move |_| {
                let create_eval = use_eval(cx);
                create_eval(r#"
                    console.log("doing the grid yo");
                    updateGridSizerList();
                "#).unwrap();
            },
            class: "dynatable expanded{content_coll.get().is_expanded()}",
            style: "
                margin: 10px 0;
                display: grid;
                grid-template-columns: 100%;
                grid-template-rows: auto auto auto;
                justify-content: stretch;
            ",
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
                DynaTabTreeComp{
                    self_: tree_type.clone(),
                }
                ComponentTypeFilterComp{
                    self_: comp_type_filter.clone(),
                },
            }}
            div {class: "collapsable {content_coll}", div {class: "content",
                style: "
                    display: grid;
                    width: min-content;
                    grid-template-columns: auto;
                    {grid_template_rows_string};
                    justify-content: start;
                    align-content: start;
                    align-items: stretch;
                    justify-items: start;
                ",
                style {
                    id: "grid_sizer_{id}",
                    class: "grid_sizer_anchor",
                    // innerHTML overridden
                },
                div {
                    for stage in Stage::iter_reals() {
                        div {
                            class: "ssb-{stage.short_rep()}-all",
                            style: "display: inline-block; text-align: center; box-sizing: border-box; border-left: 1px solid gray; border-right: 1px solid gray; overflow: hidden;",
                            "{stage:?}",
                        }
                    }
                },
                div {
                    class: "mutation_observer_container",
                    style: "display:contents;",
                    for comp in CONSTELLATION.comps.iter().filter(|comp| comp_type_filter.read().filter(comp)) {
                        ComponentusComp{
                            comp_id: comp.id,
                            tree_type: *tree_type.read(),
                        }
                    }
                }
            }},
            
        }
    })
}
