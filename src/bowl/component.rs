use serde;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::bowl::{*, owner::*, lang::*};
use crate::bowl::components::*;
use crate::bowl::stage::*;
use crate::bowl::components::{langbridge::*, ui::*, layout::*, render::*, intergfx::*, gfxapi::*, platform::*};
use crate::bowl::tree::*;
use crate::bowl::block::*;

pub  type ComponentId = usize;
pub type ComponentStrId = &'static str;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Componentus {
    pub id: ComponentId,
    pub str_id: ComponentStrId,
    pub typ: ComponentType,
    pub info: Info,
    pub extra: ExtraInfo,

    pub lefToRigTree: Tree,
    pub rigToLefTree: Tree,
    pub splitdowntree: Tree,
    pub splituptree: Tree,
}

impl Componentus {
    pub fn get_leaf_count(&self, tree_type: TreeType) -> usize {
        match tree_type {
            TreeType::LefToRig          => {self.lefToRigTree.leaf_count()},
            TreeType::RigToLef          => {self.rigToLefTree.leaf_count()},
            TreeType::HourglassToLef    => {self.splitdowntree.leaf_count()},
            TreeType::HourglassToRig    => {self.splituptree.leaf_count()}
        }
    }

    /*
    pub fn render(&self, tree_type: DynaTabTree) -> LazyNodes {
        
        match tree_type {
            DynaTabTree::LefToRig => {
                let rendered_blocks = self.lefToRigTree.render(1, true);
                rsx!{
                    div {
                        style: "
                            display: grid;
                        ",
                        div {
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 1 / span 1;
                                background-color: #0ff5;
                            ",
                            "{self.str_id}"
                        },
                        div {
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 2 / span 1;
                                display: grid;
                                grid-template-columns: {self.lefToRigTree.grid_cols_str};
                                grid-template-rows: {self.lefToRigTree.grid_rows_str};
                            ",
                            rendered_blocks,
                        }
                    },
                }

            },
            DynaTabTree::RigToLef => {
                let rendered_blocks = self.rigToLefTree.render(1, false);
                rsx!{
                    div {
                        style: "
                            display: grid;
                        ",
                        div {
                            style: "

                                grid-column: 1 / -1;
                                grid-row: 1 / span 1;
                                background-color: #0ff5;
                            ",
                            "{self.str_id}"
                        },
                        div {
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 2 / span 1;
                                display: grid;
                                grid-template-columns: {self.rigToLefTree.grid_cols_str};
                                grid-template-rows: {self.rigToLefTree.grid_rows_str};
                            ",
                            rendered_blocks,
                        }
                    },
                }
            },
            DynaTabTree::Hourglass => {
                let rendered_blocks_left = self.splituptree.render(1, false);
                let rendered_backs_left = self.splituptree.render_backings();
                let rendered_blocks_right = self.splitdowntree.render(1, true);
                let rendered_backs_right = self.splitdowntree.render_backings();
                let selected_grid_col_str = format!(
                    "grid-column: {}-mid-0-1 / {}-mid-1-1", 
                    Stage::from_comp_typ(self.typ).short_rep(), 
                    Stage::from_comp_typ(self.typ).short_rep(),
                );
                let targ_stage = Stage::from_comp_typ(self.typ);
                rsx!{
                    div {
                        style: "
                            display: grid;
                            grid-template-columns: auto auto auto;
                            grid-template-rows: auto auto;
                        ",
                        div {
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 1 / span 1;
                                background-color: #0ff5;
                            ",
                            "{self.str_id}"
                        },
                        
                        div {
                            style: "
                                grid-column: 1 / span 1;
                                grid-row: 2 / span 1;
                                display: grid;
                                grid-template-columns: {self.splituptree.grid_cols_str};
                                grid-template-rows: {self.splituptree.grid_rows_str};
                            ",
                            rendered_blocks_left,
                            rendered_backs_left,
                        },
                        div {
                            style: "
                                grid-column: 2 / span 1;
                                grid-row: 2 / span 1;
                                display: grid;
                            ",
                            div {
                                style: "
                                    grid-column: 1 / -1;
                                    grid-row: 1 / -1;
                                ",
                                Block{
                                    comp_id: self.id,
                                    on_bonk: move |e: BlockBoxerEvent| {
                                        match e {
                                            BlockBoxerEvent::Skip => {} /*block_timer = wasm_timer::Instant::now()*/,
                                            BlockBoxerEvent::WUD => {},
                                        }
                                    },
                                }
                            },
                            div {
                                class: "sub_stage_back-{targ_stage.short_rep()}-mid",
                                style: "
                                    z-index: -1;
                                    grid-column: 1 / -1;
                                    grid-row: 1 / -1;
                                ",
                            }
                            
                        },
                        div {
                            style: "
                                grid-column: 3 / span 1;
                                grid-row: 2 / span 1;
                                display: grid;
                                grid-template-columns: {self.splitdowntree.grid_cols_str};
                                grid-template-rows: {self.splitdowntree.grid_rows_str};
                            ",
                            rendered_blocks_right,
                            rendered_backs_right,
                        },
                    },
                }
            }
        }
      

    }
    */

    pub fn dimension_grid(&self) {
        
        let doc = web_sys::window().unwrap().document().unwrap();
        
    }
}

#[derive(Debug, Copy, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum ComponentType {
    Langbridge,
    Ui,
    Layout,
    Paint,
    Raster,
    Gfxapi,
    Intergfx,
    Platform,
}

impl ComponentType {
    pub fn iterator() -> impl Iterator<Item = ComponentType> {
        [
            ComponentType::Langbridge,
            ComponentType::Ui,
            ComponentType::Layout,
            ComponentType::Paint,
            ComponentType::Raster,
            ComponentType::Gfxapi,
            ComponentType::Intergfx,
            ComponentType::Platform,
        ].iter().copied()
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum SourceOpeness {
    NA,
    Superopen,          // MIT, Apache
    Copyleft,           // GPL
    Sourceavailable,    // 
    Closed 
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Info {
    pub name: &'static str,
    pub owner: Owner,
    pub description: &'static str,
    pub source_openess: SourceOpeness,
    pub website: &'static str,
    pub impl_langs: Vec<Lang>,
}




#[inline_props]
pub fn ComponentusComp(cx: Scope, comp_id: ComponentId, tree_type: DynaTabTree) -> Element {
    let comp = CONSTELLATION.get_comp(*comp_id);
    match tree_type {
        DynaTabTree::LefToRig => {
            cx.render(rsx!{
                div {
                    style: "
                        display: grid;
                    ",
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
                            background-color: #0ff5;
                        ",
                        "{comp.str_id}"
                    },
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 2 / span 1;
                            display: grid;
                            grid-template-columns: {comp.lefToRigTree.grid_cols_str};
                            grid-template-rows: {comp.lefToRigTree.grid_rows_str};
                        ",
                        TreeComp{
                            comp_id: comp.id,
                            tree_type: TreeType::LefToRig,
                        },
                    }
                },
            })
        },
        DynaTabTree::RigToLef => {
            cx.render(rsx!{
                div {
                    style: "
                        display: grid;
                    ",
                    div {
                        style: "

                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
                            background-color: #0ff5;
                        ",
                        "{comp.str_id}"
                    },
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 2 / span 1;
                            display: grid;
                            grid-template-columns: {comp.rigToLefTree.grid_cols_str};
                            grid-template-rows: {comp.rigToLefTree.grid_rows_str};
                        ",
                        TreeComp{
                            comp_id: comp.id,
                            tree_type: TreeType::RigToLef,
                        }
                    }
                },
            })
        },
        DynaTabTree::Hourglass => {
            // let rendered_backs_left = comp.splituptree.render_backings();
            // let rendered_backs_right = comp.splitdowntree.render_backings();
            info!("ComponentusComp: Hourglass");
            let selected_grid_col_str = format!(
                "grid-column: {}-mid-0-1 / {}-mid-1-1", 
                Stage::from_comp_typ(comp.typ).short_rep(), 
                Stage::from_comp_typ(comp.typ).short_rep(),
            );
            let targ_stage = Stage::from_comp_typ(comp.typ);
            cx.render(rsx!{
                div {
                    style: "
                        display: grid;
                        grid-template-columns: auto auto auto;
                        grid-template-rows: auto auto;
                    ",
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
                            background-color: #0ff5;
                        ",
                        "{comp.str_id}"
                    },
                    
                    div {
                        style: "
                            grid-column: 1 / span 1;
                            grid-row: 2 / span 1;
                            display: grid;
                            grid-template-columns: {comp.splituptree.grid_cols_str};
                            grid-template-rows: {comp.splituptree.grid_rows_str};
                        ",
                        TreeComp{
                            comp_id: comp.id,
                            tree_type: TreeType::HourglassToLef,
                        },
                        // rendered_backs_left,
                    },
                    div {
                        style: "
                            grid-column: 2 / span 1;
                            grid-row: 2 / span 1;
                            display: grid;
                        ",
                        div {
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 1 / -1;
                            ",
                            Block{
                                comp_id: comp.id,
                                on_bonk: move |e: BlockBoxerEvent| {
                                    match e {
                                        BlockBoxerEvent::Skip => {} /*block_timer = wasm_timer::Instant::now()*/,
                                        BlockBoxerEvent::WUD => {},
                                    }
                                },
                            }
                        },
                        div {
                            class: "sub_stage_back-{targ_stage.short_rep()}-mid",
                            style: "
                                z-index: -1;
                                grid-column: 1 / -1;
                                grid-row: 1 / -1;
                            ",
                        }
                        
                    },
                    div {
                        style: "
                            grid-column: 3 / span 1;
                            grid-row: 2 / span 1;
                            display: grid;
                            grid-template-columns: {comp.splitdowntree.grid_cols_str};
                            grid-template-rows: {comp.splitdowntree.grid_rows_str};
                        ",
                        TreeComp{
                            comp_id: comp.id,
                            tree_type: TreeType::HourglassToRig,
                        },
                    },
                },
            })
        }
    }
}