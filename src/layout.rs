use macroni::use_all_block_modules;
use_all_block_modules!(layout);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct LayoutInfo {

}

impl Default for Layout {fn default() -> Self {return Layout::META}}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Layout {
    META,
    Erithaxlayout,

    Taffy,
}

impl Blockify<Layout, LayoutInfo> for Layout {
    fn add_all() -> Vec<(Layout, Info, LayoutInfo, Vec<Vec<BlockType>>)> {


        vec![
            (Layout::Erithaxlayout,
                Info {
                    name: "Erithax Layout".to_string(),
                    owner: Owner::Erithax,
                    desc: TDS!(),
                    imp_lang: Lang::Rust,
                    bind_langs: vec![],
                },
                LayoutInfo {},
                parsetree!{
                    $
                }
            ),
            (
                Layout::Taffy, 
                Info {
                    name: "Taffy".to_string(),
                    owner: Owner::Dioxuslabs,
                    desc: TDS!(),
                    imp_lang: Lang::Rust,
                    bind_langs: vec![Lang::Rust],
                },
                LayoutInfo {
                    
                },
                parsetree!{
                    $ Vello *
                }
            
            )
        ]
    }
}