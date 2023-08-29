

use std::collections::BTreeMap;
use log::info;
use dioxus::prelude::*;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

pub mod constellation;
pub mod component;
mod components;
mod tree;
mod stage;
pub mod owner;
mod lang;
mod block;
mod grid_sizer;
mod filters;

use crate::*;
use constellation::*;
use component::*;
use stage::*;
use filters::{component_type_filter::*, stage_filter::*};
use filters::*;
use grid_sizer::*;

  
lazy_static!{
    pub static ref CONSTELLATION: Constellation = Constellation::load();
}


pub trait Blockify {
    fn add_all(&self) -> Vec<(&'static str, Info, crate::dyna_tab::components::ExtraInfo, Vec<Vec<&'static str>>)>;
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
pub fn DynaTabTreeComp(cx: Scope, dynatab_id: usize, self_: UseRef<DynaTabTree>) -> Element {


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
                                if *self_.read() != pot_dyna_tab_tree {
                                    self_.set(pot_dyna_tab_tree);
                                    // size_grid(*dynatab_id);
                                }
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
    AfterTreeAlways,
    AfterTreeOnClick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageState {
    NA,
    Content,
    Empty,
    EmptyHovered,
}

impl std::fmt::Display for StageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "{}",
            match self {
                Self::NA => {panic!("NA stage state display")},
                Self::Content => {"content"},
                Self::Empty => {"empty"},
                Self::EmptyHovered => {"emptyhovered"},
            }
        )
    }
}

#[inline_props]
pub fn DynaTab(cx: Scope, id: usize) -> Element {
    info!("rerender DynaTab[{}]", id);
    let _ = use_state(cx, || {info!("initializing DynaTab[{}] hooks", id); true});
    
    let settings_coll = use_state(cx, || CollapsableToggle::Expanded);
    let content_coll = use_state(cx, || CollapsableToggle::Expanded);

    let tree_type = use_ref(cx, || DynaTabTree::Hourglass);

    let selection_comp_type_filter = use_ref(cx, || 
        ComponentTypeFilter{allowed: vec![ComponentType::Ui],}
    );

    let in_tree_comp_type_filter = use_ref(cx, || 
        ComponentTypeFilter{allowed: ComponentType::iterator().collect()}
    );

    let in_tree_stage_filter = use_ref(cx, ||
        StageFilter{allowed: Stage::iter_reals().collect()}
    );


    let mut grid_template_rows_string = String::from("grid-template-rows: ");
    for (_, comp) in CONSTELLATION.comps.iter().enumerate() {
        grid_template_rows_string += &("[".to_owned() + &comp.str_id);
        grid_template_rows_string += "-s] ";
        grid_template_rows_string += "auto ";
        grid_template_rows_string += &("[".to_owned() + &comp.str_id);
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
    
    // GRID RESIZE
    use_effect(
        cx, 
        (in_tree_comp_type_filter, in_tree_stage_filter, selection_comp_type_filter, tree_type), 
        |(in_tree_comp_type_filter, in_tree_stage_filter, selection_comp_type_filter, tree_type)| 
        {
            to_owned![id];
            async move {
                size_grid(id);
            }
        }
    );

    let comps_stage_states = use_ref(cx, || {
        BTreeMap::from_iter(
            CONSTELLATION.comps.iter().enumerate()
                .map(|(comp_id, _)| (
                    comp_id, 
                    BTreeMap::from_iter(Stage::iter_reals().map(|stage| (stage, StageState::NA)))
                )
            )
        )
    });

    // update stage_states
    let mut shown_comps: Vec<ComponentId> = vec![];
    let mut stage_states: BTreeMap<Stage, StageState> = BTreeMap::from_iter(Stage::iter_reals().map(|stage| (stage, StageState::Empty)));
    for (comp_id, comp_stage_states) in comps_stage_states.read().iter() {
        if selection_comp_type_filter.read().filter(CONSTELLATION.get_comp(*comp_id)) {
            shown_comps.push(*comp_id);
            for (stage, &stage_state) in comp_stage_states {
                if 
                    stage_state != StageState::NA && 
                    stage_state != StageState::Empty && 
                    *stage_states.get(&stage).unwrap() != StageState::Content 
                {
                    *stage_states.get_mut(&stage).unwrap() = stage_state;
                }
            }
        }
        
    }    

    let stage_states_classes = || -> String {
        if stage_states.iter().all(|(s, ss)| *ss == StageState::Empty || *ss == StageState::EmptyHovered) {
            return stage_states.iter().fold("".to_string(), |acc, nex| acc + &format!(" stage-{}-{}", nex.0.short_rep(), StageState::Content))
        }
        return stage_states.iter().fold("".to_string(), |acc, nex| acc + &format!(" stage-{}-{}", nex.0.short_rep(), nex.1))
    };

    cx.render(rsx!{
        section {
            onmounted: move |_| {
                size_grid(*id);
            },
            class: "dynatable expanded{content_coll.get().is_expanded()}",
            style: "
                margin: 10px 0;
                display: grid;
                grid-template-columns: 100%;
                grid-template-rows: auto auto auto;
                justify-content: stretch;
            ",
            div {
                onclick: move |_| {
                    size_grid(*id);
                },
                id: "resize_observer_ele_{id}",
                class: "resize_observer_ele",
                style: "display: none;",
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
                    "Dynamic Partition Graph"
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
                    h3 {
                        "General settings",
                    },
                    DynaTabTreeComp{
                        dynatab_id: *id,
                        self_: tree_type.clone(),
                    }
                },
                div {
                    h3 {
                        "Selection component filter",
                    },
                    ComponentTypeFilterComp{
                        dynatab_id: *id,
                        self_: selection_comp_type_filter.clone(),
                    },
                },
                div {
                    h3 {
                        "In-tree component filter",
                    },
                    ComponentTypeFilterComp{
                        dynatab_id: *id,
                        self_: in_tree_comp_type_filter.clone(),
                    },
                    StageFilterComp{
                        dynatab_id: *id,
                        self_: in_tree_stage_filter.clone(),
                    },
                },
            }}
            div {class: "collapsable {content_coll}", div {id: "content_{id}", 
                class: "content {stage_states_classes()}",
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
                style {
                    class: "stage_hover_style",
                    // innerHMTL overridden
                },
                div {
                    class: "stage_headers",
                    for stage in Stage::iter_reals() {
                        div {
                            class: "ssb-{stage.short_rep()}-all stage_header",
                            div {
                                "{stage:?}",
                            }
                        }
                    }
                },
                div {
                    id: "mutation_observer_container_{id}",
                    class: "mutation_observer_container",
                    style: "display:contents;",
                    for comp_id in shown_comps {
                        ComponentusComp{
                            dynatab_id: *id,
                            comp_id: comp_id,
                            tree_type: *tree_type.read(),
                            comp_type_filter: in_tree_comp_type_filter.clone(),
                            stage_filter: in_tree_stage_filter.clone(),
                            stage_states: comps_stage_states.clone(),
                        }
                    }
                }
            }},
            
        }
    })
}
