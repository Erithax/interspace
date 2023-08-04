use macroni::use_all_block_modules;

// use_all_block_modules!(layout);

use macroni::parsetree2;
use macroni::parsetree;
use crate::bowl::Blockify;
use crate::bowl::component::*;
use crate::bowl::owner::*;
use crate::bowl::*;
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