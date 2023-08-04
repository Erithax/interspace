

use core::fmt::LowerHex;
use std::{collections::BTreeMap, char::MAX};
use log::{info, LevelFilter};
use dioxus::{prelude::*, html::a};
use strum::{self, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use lazy_static::lazy_static;

mod constellation;
mod component;
mod components;
mod tree;
mod stage;
mod owner;
mod lang;
mod block;
mod filters;
use filters::*;


use {constellation::*, component::*, owner::*, lang::*, stage::*, tree::*};
use components::{langbridge::*, ui::*, layout::*, render::*, gfxapi::*, intergfx::*, platform::*};
use filters::{component_type_filter::*};

  
lazy_static!{
    static ref CONSTELLATION: Constellation = Constellation::generate();
}   


pub fn AppV3(cx: Scope) -> Element {

    // cx.render(rsx!{
    //     SuperComp2{}
    // })

    cx.render(rsx!{
        DynaTab{}
        // MySuperComp{}
    })
} 

pub struct StuffHolder {
    pub stuffs: Vec<String>,
}

pub struct DupeEvent {
    pub id: usize,
}

#[inline_props]
pub fn SuperComp2(cx: Scope) -> Element {
    
    // let stuff_holder_use_ref = use_ref(cx, || stuff);

    // let raw_tree_ptr = stuff_holder_use_ref.with_mut(|sh: &mut StuffHolder| sh as *mut StuffHolder);


    // let stuff_holder = stuff_holder_use_ref.read();
    // let stuff_enumerated = stuff_holder.stuffs.iter().enumerate();
    // info!("{:?}", stuff_enumerated);
    // let mut y = 10;
    // let raw_mut = &mut y as *mut i32;
    // let points_at = unsafe { *raw_mut };
    let stuff_holder = use_ref(cx, || std::cell::UnsafeCell::new(StuffHolder{stuffs: vec!["yeya".to_string(), "boop".to_string()]}));
    
    let raw_tree_ptr = stuff_holder.read().get();
    // info!("{:?}", stuff_enumerated);

    // let bonk = use_state(cx, || stuff);

    unsafe { render!{
        div {}
        for (id, _) in  (*stuff_holder.read().get()).stuffs.iter().enumerate() {
            SubComp2{
                id: id,
                stuff_holder: raw_tree_ptr,
                on_update: move |_| {stuff_holder.needs_update()},
                // on_dupe: move |e: DupeEvent| {
                //     info!("DupeEvent::Yes");
                //     let item = stuff_holder_use_ref.read().stuffs[e.id].clone();
                //     stuff_holder_use_ref.write().stuffs.push(item);
                // }
            }
            // div {}
        }
    }}
}



#[inline_props]
pub fn SubComp2<'a>(
    cx: Scope<'a>, 
    id: usize, 
    stuff_holder: *mut StuffHolder, 
    on_update: EventHandler<'a, usize>,
    // on_dupe: EventHandler<'a, (DupeEvent, std::cell::RefMut<'a, StuffHolder>)>
) -> Element {
    let name = unsafe {(*(*stuff_holder)).stuffs[cx.props.id].clone()};
    info!("{}", name);
    render!{
        div {
            onclick: |_| {
                unsafe {(*(*stuff_holder)).stuffs.push("boom".to_string())};
                on_update.call(0);
            },
            "{name}",
        }
    }
}


pub enum UpdateSuper {
    Nah,
    Yeh,
}

#[inline_props]
pub fn MySuperComp(cx: Scope) -> Element {
	let get_wid = || {
        	let doc = web_sys::window().unwrap().document().unwrap();
        	match doc.get_element_by_id("bob") {
                Some(e) => {e.scroll_width()},
                None => {0}
            }
	};

    // let curr_wid = use_state(
    //     cx, || get_wid());

    let curr_wid = use_future(cx, (), |_| async move {
        async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        get_wid()
    });



	
	cx.render(rsx!{
        match curr_wid.value() {
            Some(w) => rsx! {
                div {
                    "current width of label: {w}\n"
                },
            },
            None => rsx! { div { "huh?" } },
        }
        // div {"current width of label: {curr_wid.get()}"},
		MyComp{on_something_changed: move |e: UpdateSuper| {
            info!("updating");
            curr_wid.set(
                match web_sys::window().unwrap().document().unwrap().get_element_by_id("bob") {
                    Some(e) => {e.scroll_width()},
                    None => {0}
                }
            );
        }}
	})
}

#[inline_props]
pub fn MyComp<'a>(cx: Scope<'a>, on_something_changed: EventHandler<'a, UpdateSuper>) -> Element<'a> {
    let long_label_enabled = use_state(cx, || false);

	cx.render(rsx!{
        div {
            onclick: move |_| {
                long_label_enabled.set(!long_label_enabled.get());
                // on_something_changed.call(UpdateSuper::Yeh);
            },
            "click me!",
        },
		span {
            onmounted: move |_| {
                on_something_changed.call(UpdateSuper::Yeh);
            },
			id: "bob",
            if *long_label_enabled.get() {"Dioxus is very good."
            } else                       {"dioxus gud"}
		}
	})
}










pub struct DynaTabSettings {
    selected_comp_typ: ComponentType,
    tree_typ: DynaTabTree,
    selected_cells: SelectedCells,
    // hovering: Option<Stage>,
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
        for pot_dyna_tab_tree in DynaTabTree::iterator() {{
            let active = if pot_dyna_tab_tree == *self_.read() {"active"} else {"inactive"};
            rsx!{
                div {
                    onclick: move |_| {
                        self_.set(pot_dyna_tab_tree);
                    },
                    class: "{active}",
                    "{pot_dyna_tab_tree:?}",
                }
            }
        }}
    })
}

// #[inline_props]
pub fn DynaTab(cx: Scope) -> Element {

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
    
    let tree_type = use_ref(cx, || DynaTabTree::Hourglass);

    

    // info!("setting settings");
    // let settings = use_ref(cx, || DynaTabSettings{
    //     selected_comp_typ: ComponentType::Ui,
    //     tree_typ: DynaTabTree::Hourglass,
    //     selected_cells: SelectedCells::Combined,
    // }); 

    let comp_type_filter = use_ref(cx, || 
        Box::new(ComponentTypeFilter{
            allowed: vec![ComponentType::Ui, ComponentType::Paint],
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
    // info!("{grid_template_rows_string}");

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
        if false {
            rsx!{style {
                "
                    .sub_stage_back-lb-pre {{background: rgb(255, 0, 0);}}
                    .sub_stage_back-ui-pre {{background: rgb(255, 40, 0);}}
                    .sub_stage_back-ly-pre {{background: rgb(255, 80, 0);}}
                    .sub_stage_back-pa-pre {{background: rgb(255, 120, 0);}}
                    .sub_stage_back-ra-pre {{background: rgb(255, 160, 0);}}
                    .sub_stage_back-gx-pre {{background: rgb(255, 200, 0);}}
                    .sub_stage_back-pl-pre {{background: rgb(255, 240, 0);}}

                    .sub_stage_back-lb-mid {{background: rgb(255, 0, 0);}}
                    .sub_stage_back-ui-mid {{background: rgb(255, 40, 0);}}
                    .sub_stage_back-ly-mid {{background: rgb(255, 80, 0);}}
                    .sub_stage_back-pa-mid {{background: rgb(255, 120, 0);}}
                    .sub_stage_back-ra-mid {{background: rgb(255, 160, 0);}}
                    .sub_stage_back-gx-mid {{background: rgb(255, 200, 0);}}
                    .sub_stage_back-pl-mid {{background: rgb(255, 240, 0);}}

                    .sub_stage_back-lb-post {{background: rgb(255, 0, 0);}}
                    .sub_stage_back-ui-post {{background: rgb(255, 40, 0);}}
                    .sub_stage_back-ly-post {{background: rgb(255, 80, 0);}}
                    .sub_stage_back-pa-post {{background: rgb(255, 120, 0);}}
                    .sub_stage_back-ra-post {{background: rgb(255, 160, 0);}}
                    .sub_stage_back-gx-post {{background: rgb(255, 200, 0);}}
                    .sub_stage_back-pl-post {{background: rgb(255, 240, 0);}}

                    .sub_stage_back-lb-all {{background: rgb(255, 0, 0);}}
                    .sub_stage_back-ui-all {{background: rgb(255, 40, 0);}}
                    .sub_stage_back-ly-all {{background: rgb(255, 80, 0);}}
                    .sub_stage_back-pa-all {{background: rgb(255, 120, 0);}}
                    .sub_stage_back-ra-all {{background: rgb(255, 160, 0);}}
                    .sub_stage_back-gx-all {{background: rgb(255, 200, 0);}}
                    .sub_stage_back-pl-all {{background: rgb(255, 240, 0);}}

                    .sub_stage_back-lb-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-ui-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-ly-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-pa-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-ra-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-gx-pre {{border-top: 3px solid red;}}
                    .sub_stage_back-pl-pre {{border-top: 3px solid red;}}

                    .sub_stage_back-lb-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-ui-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-ly-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-pa-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-ra-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-gx-mid {{border-top: 3px solid green;}}
                    .sub_stage_back-pl-mid {{border-top: 3px solid green;}}

                    .sub_stage_back-lb-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-ui-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-ly-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-pa-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-ra-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-gx-post {{border-top: 3px solid blue;}}
                    .sub_stage_back-pl-post {{border-top: 3px solid blue;}}

                    .sub_stage_back {{
                        grid-row: 1 / -1;
                        border: 0px solid lime;
                        width: 100%;
                        z-index: -1;
                    }}
                ",
            }}
        },


        DynaTabTreeComp{
            self_: tree_type.clone(),
        }
        ComponentTypeFilterComp{
            self_: comp_type_filter.clone(),
        },
        div {
            class: "dynatab",
            style: "
                display: grid;
                border: 3px solid purple;
                min-width: 80%;
                width: fit-content;
                margin-left: 10%;
                grid-template-columns: auto;
                {grid_template_rows_string};
                justify-content: start;
                align-content: start;
                align-items: stretch;
                justify-items: start;
            ",
            style {
                class: "gridsizer_style",
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
        }
    })
}

pub trait Blockify {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, crate::bowl::components::ExtraInfo, Vec<Vec<ComponentStrId>>)>;
}