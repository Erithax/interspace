use serde;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::swig::{*, owner::*, lang::*};
use crate::swig::components::*;
use crate::swig::stage::*;
use crate::swig::components::{langbridge::*, ui::*, layout::*, render::*, intergfx::*, gfxapi::*, platform::*};
use crate::swig::tree::*;
use crate::swig::block::*;

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
            TreeType::LefToRig => {self.lefToRigTree.leaf_count()},
            TreeType::RigToLef => {self.rigToLefTree.leaf_count()},
            TreeType::Hourglass => {
                return self.splitdowntree.leaf_count().max(self.splituptree.leaf_count())
            }
        }
    }

    pub fn get_max_subchain_len(&self) -> BTreeMap<Stage, usize> {
        let mut max_subdivide_lvls_per_tree: Vec<BTreeMap<Stage, usize>> = Vec::new();
        max_subdivide_lvls_per_tree.push(self.lefToRigTree.stage_subtrees_dep_len_max(&CONSTELLATION));
        max_subdivide_lvls_per_tree.push(self.rigToLefTree.stage_subtrees_dep_len_max(&CONSTELLATION));
        max_subdivide_lvls_per_tree.push(self.splitdowntree.stage_subtrees_dep_len_max(&CONSTELLATION));
        max_subdivide_lvls_per_tree.push(self.splituptree.stage_subtrees_dep_len_max(&CONSTELLATION));

        let mut res: BTreeMap<Stage, usize> = BTreeMap::new();
        for stage in Stage::iter() {
            res.insert(stage, 
                *max_subdivide_lvls_per_tree.iter()
                    .map(|l| l.get(&stage))
                    .filter(|x| x.is_some())
                    .max()
                    .unwrap_or(Some(&1))
                    .unwrap_or(&1)
            );
        }
        return res
    }

    pub fn render(&self, start_row_i: usize, tree_type: TreeType) -> (LazyNodes, usize) {


        match tree_type {
            TreeType::LefToRig => {
                let rendered_blocks = self.lefToRigTree.render(start_row_i+1, true);
                (
                    rsx!{
                        div {
                            style: "
                                display: contents;
                            ",
                            div {
                                style: "
                                    grid-column: 1 / -1;
                                    grid-row: {start_row_i} / span 1;
                                    background-color: #0ff5;
                                ",
                                "{self.str_id}"
                            },
                            rendered_blocks.0,
                        },
                    },
                    rendered_blocks.1+1
                )
            },
            TreeType::RigToLef => {
                let rendered_blocks = self.rigToLefTree.render(start_row_i+1, false);
                (
                    rsx!{
                        div {
                            style: "
                                display: contents;
                            ",
                            div {
                                style: "
                                    grid-column: 1 / -1;
                                    grid-row: {start_row_i} / span 1;
                                    background-color: #0ff5;
                                ",
                                "{self.str_id}"
                            },
                            rendered_blocks.0,
                        },
                    },
                    rendered_blocks.1+1
                )
            },
            TreeType::Hourglass => {
                let rendered_blocks_left: (LazyNodes<'_, '_>, usize) = self.splituptree.render(start_row_i+1, false);
                let rendered_blocks_right: (LazyNodes<'_, '_>, usize) = self.splitdowntree.render(start_row_i+1, true);
                let selected_grid_col_str = format!(
                    "grid-column: {}-mid-0-1 / {}-mid-1-1", 
                    Stage::from_comp_typ(self.typ).short_rep(), 
                    Stage::from_comp_typ(self.typ).short_rep(),
                );
                (
                    rsx!{
                        div {
                            style: "
                                display: contents;
                            ",
                            div {
                                style: "
                                    grid-column: 1 / -1;
                                    grid-row: {start_row_i} / span 1;
                                    background-color: #0ff5;
                                ",
                                "{self.str_id}"
                            },
                            div {
                                style: "display: contents",
                                rendered_blocks_left.0,
                            },
                            div {
                                style: "
                                    {selected_grid_col_str};
                                    grid-row: {start_row_i+1} / {rendered_blocks_left.1.max(rendered_blocks_right.1)};
                                ",
                                Block{
                                    comp_id: self.id,
                                }
                            },
                            div {
                                style: "display: contents",
                                rendered_blocks_right.0,
                            },
                            
                            
                        },
                    },
                    rendered_blocks_left.1.max(rendered_blocks_right.1),
                )
            }
        }
      

    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
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





