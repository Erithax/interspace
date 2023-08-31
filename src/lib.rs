#![allow(non_snake_case)]

pub mod dyna_tab;
pub mod image;
pub mod infoboard;
pub mod utils;

use dioxus::{prelude::*};
use dyna_tab::component::Repo;

use log::info;
use nucleo_matcher::{pattern::{Pattern, CaseMatching}};


pub const STARS_FILE_PATH: &'static str = "./res/state/stars.ron";

pub fn indent(s: &str) -> String {
    let res: String = s.replace("\n", "\n\t");
    return res
}

pub fn strip_website(mut s: &str) -> &str {
    s = s.strip_prefix("https://").unwrap_or(s);
    s = s.strip_prefix("www.").unwrap_or(s);
    s = s.strip_suffix("/").unwrap_or(s);
    return s
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct StarCache {
    // #[serde(with = "chrono::serde::ts_seconds_option")]
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub comp_id: usize,
    pub comp_str_id: String,
    pub repo: Option<Repo>,
}

impl Default for StarCache {
    fn default() -> Self {
        return StarCache{
            update_time: chrono::Utc::now(),
            comp_id: 0,
            comp_str_id: "DEFAULT".to_owned(),
            repo: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsableToggle {
    Collapsed,
    Expanded
}

impl CollapsableToggle {

    pub fn toggle_mut(&mut self) {
        *self = self.toggle();
    }

    pub fn toggle(&self) -> Self {
        match self {
            CollapsableToggle::Collapsed => {CollapsableToggle::Expanded},
            CollapsableToggle::Expanded => {CollapsableToggle::Collapsed},
        }
    }

    pub fn is_collapsed(&self) -> bool {
        match self {
            Self::Collapsed => {true},
            Self::Expanded => {false},
        }
    }

    pub fn is_expanded(&self) -> bool {
        match self {
            Self::Collapsed => {false},
            Self::Expanded => {true},
        }
    }
}

impl std::fmt::Display for CollapsableToggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollapsableToggle::Collapsed => { write!(f, "collapsed")},
            CollapsableToggle::Expanded => {write!(f, "expanded")},
        }
    }
}






