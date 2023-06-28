use macroni::use_all_block_modules;

use_all_block_modules!(langbridge);



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct LangbridgeInfo {

}

impl Default for Langbridge {fn default() -> Self {return Langbridge::META}}


/////////////////////////////////////////////
/// ADD LANGBRIDGES BELOW 
/////////////////////////////////////////////


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, strum_macros::EnumIter, strum_macros::Display, serde::Deserialize, serde::Serialize)]
pub enum Langbridge {
    META,
    Erithaxlangbridge,

    Gtk3rs,
}


impl Blockify<Langbridge, LangbridgeInfo> for Langbridge {
    fn add_all() -> Vec<(Langbridge, Info, LangbridgeInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (
            Langbridge::Erithaxlangbridge, 
                Info {
                    name: "Erithax Langbridge".to_string(),
                    owner: Owner::Erithax,
                    desc: TDS!(),
                    imp_lang: Lang::Rust,
                    bind_langs: vec![Lang::Rust],
                },
                LangbridgeInfo {
                    
                },
                parsetree!{
                    $
                }
        ),
        (
        Langbridge::Gtk3rs, 
            Info {
                name: "Gtk3-rs".to_string(),
                owner: Owner::Gtkrs,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            LangbridgeInfo {
                
            },
            parsetree!{
                $ Gtk
            }
        ),
        ] 
    }
}


