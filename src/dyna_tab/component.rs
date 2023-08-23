use serde;

use crate::dyna_tab::{*, owner::*, lang::*};
use crate::dyna_tab::components::*;
use crate::dyna_tab::stage::*;
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
pub enum SourceOpenness {
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
    pub code_openness: SourceOpenness,
    pub website: &'static str,
    pub impl_langs: Vec<Lang>,
}




#[inline_props]
pub fn ComponentusComp(
    cx: Scope, 
    dynatab_id: usize,
    comp_id: ComponentId, 
    tree_type: DynaTabTree, 
    comp_type_filter: UseRef<ComponentTypeFilter>,
    stage_filter: UseRef<StageFilter>,
    stage_states: UseRef<BTreeMap<ComponentId, BTreeMap<Stage, StageState>>>,
) -> Element {
    let comp = CONSTELLATION.get_comp(*comp_id);
    let hovereffectall = use_state(cx, || false);

    *stage_states.write_silent().get_mut(comp_id).unwrap().get_mut(&Stage::from_comp_typ(CONSTELLATION.get_comp(*comp_id).typ)).unwrap() = StageState::Content;

    let primary_hovered = use_state(cx, || false);

    let tree_remount_trigger = use_state(cx, || 0);

    match tree_type {
        DynaTabTree::LefToRig => {
            cx.render(rsx!{
                div {
                    onmouseenter: move |_| {primary_hovered.set(true);},
                    onmouseleave: move |_| {primary_hovered.set(false);},
                    class: "primary hovered-{primary_hovered.get()}",
                    style: "
                        display: grid;
                    ",
                    div {
                        onclick: move |_| {tree_remount_trigger.set(tree_remount_trigger.get() + 1); info!("bonk {}", tree_remount_trigger.get());},
                        class: "component_reset_button shown-{primary_hovered.get()}",
                    },
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
                            display: grid;
                            grid-template-columns: {comp.lefToRigTree.grid_cols_str};
                            grid-template-rows: {comp.lefToRigTree.grid_rows_str};
                        ",
                        TreeComp{
                            dynatab_id: *dynatab_id,
                            comp_id: comp.id,
                            tree_type: TreeType::LefToRig,
                            comp_type_filter: comp_type_filter.clone(),
                            stage_filter: stage_filter.clone(),
                            stage_states: stage_states.clone(),
                        },
                    }
                },
            })
        },
        DynaTabTree::RigToLef => {
            cx.render(rsx!{
                div {
                    onmouseenter: move |_| {primary_hovered.set(true);},
                    onmouseleave: move |_| {primary_hovered.set(false);},
                    class: "primary hovered-{primary_hovered.get()}",
                    style: "
                        display: grid;
                    ",
                    div {
                        onclick: move |_| {tree_remount_trigger.set(tree_remount_trigger.get() + 1); info!("bonk {}", tree_remount_trigger.get());},
                        class: "component_reset_button shown-{primary_hovered.get()}",
                    },
                    div {
                        style: "
                            grid-column: 1 / -1;
                            grid-row: 1 / span 1;
                            display: grid;
                            grid-template-columns: {comp.rigToLefTree.grid_cols_str};
                            grid-template-rows: {comp.rigToLefTree.grid_rows_str};
                        ",
                        TreeComp{
                            dynatab_id: *dynatab_id,
                            comp_id: comp.id,
                            tree_type: TreeType::RigToLef,
                            comp_type_filter: comp_type_filter.clone(),
                            stage_filter: stage_filter.clone(),
                            stage_states: stage_states.clone(),
                        }
                    }
                },
            })
        },
        DynaTabTree::Hourglass => {
            let targ_stage = Stage::from_comp_typ(comp.typ);
            cx.render(rsx!{
                div {
                    onmouseenter: move |_| {primary_hovered.set(true);},
                    onmouseleave: move |_| {primary_hovered.set(false);},
                    class: "primary hovereffectall-{hovereffectall.get()} hovered-{primary_hovered.get()}",
                    style: "
                        display: grid;
                        grid-template-columns: auto auto auto;
                        grid-template-rows: auto;
                        display: relative;
                    ",
                    div {
                        onclick: move |_| {tree_remount_trigger.set(tree_remount_trigger.get() + 1); info!("bonk {}", tree_remount_trigger.get());},
                        class: "component_reset_button shown-{primary_hovered.get()}",
                    },
                    div {
                        style: "
                            grid-column: 1 / span 1;
                            grid-row: 1 / span 1;
                            display: grid;
                            grid-template-columns: {comp.splituptree.grid_cols_str};
                            grid-template-rows: {comp.splituptree.grid_rows_str};
                        ",
                        for _ in 0..1 {
                            TreeComp{
                                dynatab_id: *dynatab_id,
                                comp_id: comp.id,
                                tree_type: TreeType::HourglassToLef,
                                comp_type_filter: comp_type_filter.clone(),
                                stage_filter: stage_filter.clone(),
                                stage_states: stage_states.clone(),
                                key: "{tree_remount_trigger.get()}",
                            }
                        }
                    },
                    div {
                        style: "
                            grid-column: 2 / span 1;
                            grid-row: 1 / span 1;
                            display: grid;
                        ",
                        div {
                            onmouseenter: move |_| {
                                hovereffectall.set(true);
                            },
                            onmouseleave: move |_| {
                                hovereffectall.set(false);
                            },
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
                        for _ in 0..1 {
                            TreeComp{
                                dynatab_id: *dynatab_id,
                                comp_id: comp.id,
                                tree_type: TreeType::HourglassToRig,
                                comp_type_filter: comp_type_filter.clone(),
                                stage_filter: stage_filter.clone(),
                                stage_states: stage_states.clone(),
                                key: "{tree_remount_trigger.get()}",
                            }
                        }
                    }
                },
            })
        }
    }
}