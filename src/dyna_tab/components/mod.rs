
use std::fmt::Display;

use crate::dyna_tab::{owner::*, lang::*};

pub mod langbridge;
pub mod ui;
pub mod layout;
pub mod paint;
pub mod raster;
pub mod gfxapi;
pub mod intergfx;
pub mod platform;

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum RoughRange {
    TODO,
    None,
    Lo,
    Mid,
    Hi
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum Reactivity {
    TODO,
    None,
    FineGrained,
    Elmy,
    Reacty,
    Swifty,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum UiLang {
    TODO,
    Lang(Lang),
    Custom{
        name: String,
        logic_langs: Vec<Lang>,
    }
}




// Componenttype-specific info
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ExtraInfo {
    Langbridge{
        bind_langs: Vec<Lang>,
    },
    Ui {
        is_immediate: bool,
        reactivity: Reactivity,
        declarativity: RoughRange,
        macrotivity: RoughRange,
        language: UiLang,
        hot_reload: bool,
        ssr: bool,
        liveview: bool,
    },
    Layout {
        constraint_based: bool,
        css: bool,
        flexbox: bool,
        grid: bool,
    },
    Paint {

    },
    Raster {

    },
    Gfxapi {

    },
    Intergfx {

    },
    Platform {

    }
}

impl Display for ExtraInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Langbridge { bind_langs } => {
                write!(f, "Langbridge::{{\n{bind_langs:?}}}")
            },
            Self::Ui { is_immediate, reactivity, declarativity, macrotivity, language, hot_reload, ssr, liveview } => {
                write!(f, "Ui::{{\n{is_immediate}\n{reactivity:?}\n{declarativity:?}\n{macrotivity:?}\n{language:?}\n{hot_reload}\n{ssr}\n{liveview}\n}}")
            },
            Self::Layout { constraint_based, css, flexbox, grid } => {
                write!(f, "Layout::{{\n constraint_based: {constraint_based}\n css: {css}\n flexbox: {flexbox}\n grid: {grid}}}")
            },
            _ => {
                write!(f, "{self:?}")
            }
        }
    }
}