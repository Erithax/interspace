

use core::fmt::LowerHex;
use std::{collections::BTreeMap, char::MAX};
use log::{info, LevelFilter};
use dioxus::prelude::*;
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
use {constellation::*, component::*, owner::*, lang::*, stage::*, tree::*};

use components::{langbridge::*, ui::*, layout::*, render::*, gfxapi::*, intergfx::*, platform::*};

  
lazy_static!{
    static ref CONSTELLATION: Constellation = Constellation::generate();
}   


pub fn AppV2(cx: Scope) -> Element {
    cx.render(rsx!{
        DynaTab{}
    })
} 



pub trait SelectionFilter {
    fn filter(cb: &Componentus) -> bool;
}

pub struct NameFirstLetterFilter {
}
impl SelectionFilter for NameFirstLetterFilter {
    fn filter(cb: &Componentus) -> bool {
        if cb.str_id.starts_with("A") {
            return true
        } else {
            return false
        }
    }
}

pub struct RatingFilter {
}
impl SelectionFilter for RatingFilter {
    fn filter(cb: &Componentus) -> bool {
        todo!{}
    }
}


pub struct DynaTabSettings {
    selected_comp_typ: ComponentType,
    tree_typ: TreeType,
    selected_cells: SelectedCells,
    // hovering: Option<Stage>,
}



#[inline_props]
pub fn DynaTab(cx: Scope) -> Element {

    let constellation = CONSTELLATION.clone();

    // CALCULATE UNFILTERED SET OF COMPONENTS
    let mut grid_template_rows_string = String::from("grid-template-rows: ");

    let settings = use_ref(cx, || DynaTabSettings{
        selected_comp_typ: ComponentType::Ui,
        tree_typ: TreeType::Hourglass,
        selected_cells: SelectedCells::Combined,
    });

    


    for (comp_id, comp) in constellation.comps.iter().enumerate() {
        grid_template_rows_string += &("[".to_owned() + comp.str_id);
        grid_template_rows_string += "-s] ";
        // TODO: row_count = max(all trees of comp) ??
        let row_count = comp.get_leaf_count(settings.with(|s| s.tree_typ));
        grid_template_rows_string += &("repeat(".to_owned() + row_count.to_string().as_str());
        grid_template_rows_string += ", auto) ";
        grid_template_rows_string += &("[".to_owned() + comp.str_id);
        grid_template_rows_string += "-e] ";
        grid_template_rows_string += "auto ";
    } 


    let mut stage_max_subdivision_lvls: BTreeMap<Stage, usize> = BTreeMap::new();
    for comp in CONSTELLATION.comps.iter() {
        let comp_max_sub_div_lvls = comp.get_max_subchain_len();
        for (stage, comp_max_sub_div_lvl) in comp_max_sub_div_lvls {
            if !stage_max_subdivision_lvls.contains_key(&stage) ||
                *stage_max_subdivision_lvls.get(&stage).unwrap() < comp_max_sub_div_lvl 
            {
                stage_max_subdivision_lvls.insert(stage, comp_max_sub_div_lvl);
            }
        }
    }

    let epsilon = 0.01;

    let mut lines: Vec<Vec<Vec<String>>> = Vec::new();
    
    for (stage, stage_max_sub_div_lvl) in stage_max_subdivision_lvls {
        let mut stage_lines: Vec<(f64, Vec<String>)> = Vec::new();
        for i in 1..=stage_max_sub_div_lvl {
            for j in 0..=i {
                let line_str = format!("{}-{}", j, i);
                let frac = j as f64 / i as f64;
                let mut line_already_present = false;
                for (other_frac, other_lines) in stage_lines.iter_mut() {
                    if (*other_frac-frac).abs() < epsilon {
                        other_lines.push(line_str.clone());
                        line_already_present = true;
                        break
                    } 
                }
                if !line_already_present {
                    stage_lines.push((frac, vec![line_str]))
                }
            }
        }
        stage_lines.sort_unstable_by(|a: &(f64, Vec<String>), b| a.0.partial_cmp(&b.0).unwrap());

        lines.push(vec![
            stage_lines.iter().map(
                |line| 
                    line.1.iter()
                        .fold("[".to_owned(), |acc, next| acc + " " + stage.short_rep() + "-pre-" + next)
                    + "] auto "
            ).collect(),
            stage_lines.iter().map(
                |line| 
                    line.1.iter()
                        .fold("[".to_owned(), |acc, next| acc + " " + stage.short_rep() + "-mid-" + next)
                    + "] auto "
            ).collect(),
            stage_lines.iter().map(
                |line| 
                    line.1.iter()
                        .fold("[".to_owned(), |acc, next| acc + " " + stage.short_rep() + "-post-" + next)
                    + "] auto "
            ).collect(),
        ]);
    }

    let grid_template_cols_string = "grid-template-columns: ".to_owned() + lines.iter()
        .fold("".to_owned(), |acc, next: &Vec<Vec<String>>| acc + 
            next.iter().fold("".to_owned(), |acc: String, next: &Vec<String>| acc + 
                next.iter().fold("".to_owned(), |acc: String, next: &String| acc + next).as_str()
            ).as_str()
    ).as_str();

    info!("{grid_template_rows_string}");
    info!("{grid_template_cols_string}");

    // APPLY SELECTION FILTERS
    // APPLY SELECTION SORTS
    // TRAVERSE & RENDER SELECTION LIST PRIMARIES
        // TRAVERSE & RENDER TREE
            // APPLY IN-TREE FILTER
            // APPLY GROUPCLUSTER AND DELETECLUSTER

    let mut curr_row_i = 1;
    let mut row_lazy_nodes = Vec::new();

    for comp in CONSTELLATION.comps.iter() {
        let temp = comp.render(curr_row_i, TreeType::Hourglass);
        row_lazy_nodes.push(temp.0);
        curr_row_i = temp.1;
    }

    cx.render(rsx!{
        div {
            class: "garfonkel",
            style: "
                display: grid;
                background-color: #f002;
                width: 80%;
                margin-left: 10%;
                {grid_template_cols_string};
                {grid_template_rows_string};
                justify-content: stretch;
                align-content: start;
                align-items: stretch;
                justify-items: stretch;
            ",
            for stage in Stage::iter_reals() {
                div {
                    class: "back {stage.short_rep()}",
                    style: "
                        grid-column: {stage.short_rep()}-pre-0-1 / {stage.short_rep()}-post-1-1;
                        grid-row: 1 / -1;
                        border-right: 1px solid red;
                    ",
                }
            },
            // for comp in CONSTELLATION.comps.iter() {
            //     div {
            //         class: "prim",
            //         style: "
            //             grid-column: 1 / -1;
            //             grid-row: {comp.str_id}-s / {comp.str_id}-e;
            //             border-top: 3px solid purple;
            //             border-bottom: 3px solid violet;
            //         ",
            //     }
            // },
            rsx!{row_lazy_nodes.into_iter()},
        }
    })
}

pub trait Blockify {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, crate::swig::components::ExtraInfo, Vec<Vec<ComponentStrId>>)>;
}