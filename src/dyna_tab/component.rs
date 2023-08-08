use serde;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::dyna_tab::{*, owner::*, lang::*};
use crate::dyna_tab::components::*;
use crate::dyna_tab::stage::*;
use crate::dyna_tab::components::{langbridge::*, ui::*, layout::*, render::*, intergfx::*, gfxapi::*, platform::*};
use crate::dyna_tab::tree::*;
use crate::dyna_tab::block::*;

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
    let hovereffectall = use_state(cx, || false);

    match tree_type {
        DynaTabTree::LefToRig => {
            cx.render(rsx!{
                div {
                    class: "primary",
                    style: "
                        display: grid;
                    ",
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
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
                    class: "primary",
                    style: "
                        display: grid;
                    ",
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
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
            let selected_grid_col_str = format!(
                "grid-column: {}-mid-0-1 / {}-mid-1-1", 
                Stage::from_comp_typ(comp.typ).short_rep(), 
                Stage::from_comp_typ(comp.typ).short_rep(),
            );
            let targ_stage = Stage::from_comp_typ(comp.typ);
            cx.render(rsx!{
                div {
                    class: "primary hovereffectall-{hovereffectall.get()}",
                    style: "
                        display: grid;
                        grid-template-columns: auto auto auto;
                        grid-template-rows: auto;
                    ",
                    div {
                        style: "
                            grid-column: 1 / span 1;
                            grid-row: 1 / span 1;
                            display: grid;
                            grid-template-columns: {comp.splituptree.grid_cols_str};
                            grid-template-rows: {comp.splituptree.grid_rows_str};
                        ",
                        TreeComp{
                            comp_id: comp.id,
                            tree_type: TreeType::HourglassToLef,
                        },
                    },
                    div {
                        style: "
                            grid-column: 2 / span 1;
                            grid-row: 1 / span 1;
                            display: grid;
                        ",
                        div {
                            onmouseenter: move |_| {hovereffectall.set(true)},
                            onmouseleave: move |_| {hovereffectall.set(false)},
                            style: "
                                grid-column: 1 / -1;
                                grid-row: 1 / -1;
                            ",
                            Block{
                                comp_id: comp.id,
                                is_focussed: true,
                                on_bonk: move |e: BlockBoxerEvent| {
                                    match e {
                                        BlockBoxerEvent::Snip => {},
                                        BlockBoxerEvent::Skip => {},
                                    }
                                },
                            }
                        },
                        div {
                            class: "ssb-{targ_stage.short_rep()}-mid",
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
                            grid-row: 1 / span 1;
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