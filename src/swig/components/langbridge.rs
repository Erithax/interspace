use macroni::use_all_block_modules;

// use_all_block_modules!(langbridge);

use macroni::parsetree;
use macroni::parsetree2;
use crate::swig::Blockify;
use crate::swig::component::*;
use crate::swig::owner::*;
use crate::swig::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, serde::Deserialize, serde::Serialize)]

pub struct Langbridge {}

impl Blockify for Langbridge {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        (
            "Erithaxlangbridge",
                Info {
                    name: "Erithax Langbridge",
                    owner: Owner::Erithax,
                    description: "TODO",
                    source_openess: SourceOpeness::Copyleft,
                    impl_langs: vec![Lang::Rust],
                    website: "https://erithax.com",
                },
                ExtraInfo::Langbridge {
                    bind_langs: vec![Lang::C],
                },
                parsetree2!{
                    $
                }
                // vec![vec!["Erithaxlangbridge"]],
        ),
        (
        "Gtk3rs", 
            Info {
                name: "Gtk3-rs",
                owner: Owner::Gtkrs,
                description: "TODO",
                source_openess: SourceOpeness::Copyleft,
                impl_langs: vec![Lang::Rust],
                website: "https://gtk-rs.org",
            },
            ExtraInfo::Langbridge {
                bind_langs: vec![Lang::Rust],
            },
            vec![vec!["Gtk3rs", "Gtk"]],
        ),
        ] 
    }
}


