use macroni::use_all_block_modules;

// use_all_block_modules!(layout);

use macroni::parsetree2;
use macroni::parsetree;
use crate::swig::Blockify;
use crate::swig::component::*;
use crate::swig::owner::*;
use crate::swig::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, serde::Deserialize, serde::Serialize)]
pub struct Layout {}

impl Blockify for Layout {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {


        vec![
            ("Erithaxlayout",
                Info {
                    name: "Erithax Layout",
                    owner: Owner::Erithax,
                    description: "TODO",
                    website: "erithax.com",
                    source_openess: SourceOpeness::NA,
                    impl_langs: vec![Lang::Rust],
                },
                ExtraInfo::Layout {
                    css: false,
                    flexbox: false,
                    grid: false,
                    constraint_based: false,
                },
                parsetree2!{
                    $
                }
            ),
            (
                "Taffy", 
                Info {
                    name: "Taffy",
                    owner: Owner::Dioxuslabs,
                    description: "TODO",
                    website: "",
                    source_openess: SourceOpeness::NA,
                    impl_langs: vec![Lang::Rust],
                },
                ExtraInfo::Layout {
                    css: true,
                    flexbox: true,
                    grid: false,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Vello *
                }
            
            )
        ]
    }
}