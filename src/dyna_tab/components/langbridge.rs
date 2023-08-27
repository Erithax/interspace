
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize)]

pub struct Langbridge {}

impl Blockify for Langbridge {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        (
            "Erithaxlangbridge",
                Info::new(
                    "Erithax Langbridge",
                    Owner::Erithax,
                    "TODO",
                    "https://erithax.com",
                    SourceOpenness::Copyleft,
                    vec![Lang::Rust],
                    None,
                ),
                ExtraInfo::Langbridge {
                    bind_langs: vec![Lang::C],
                },
                parsetree2!{
                    $
                }
        ),
        (
        "Gtk3rs", 
            Info::new(
                "Gtk3-rs",
                Owner::Gtkrs,
                "TODO",
                "https://gtk-rs.org",
                SourceOpenness::Copyleft,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/gtk-rs/gtk3-rs"),
            ),
            ExtraInfo::Langbridge {
                bind_langs: vec![Lang::Rust],
            },
            vec![vec!["Gtk3rs", "Gtk"]],
        ),
        ] 
    }
}


