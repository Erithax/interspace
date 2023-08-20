
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Paint {}


impl Blockify for Paint {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxpaint",
            Info {
                name: "Erithax Paint",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Paint {},
            parsetree2!{
                $
            }
        ),
        ("Blink_pa", 
            Info {
                name: "Blink",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Skia *,
                $ Webgpu Dawn *
            }
        ),
        ("Gecko_pa", 
            Info {
                name: "Gecko",
                owner: Owner::Mozilla,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Webrender *
            }
        ),
        ("Servo_pa", 
            Info {
                name: "Servo",
                owner: Owner::LinuxFoundation,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Webrender *
            }
        ),
        ("Webkit_pa", 
            Info {
                name: "Webkit",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Coregraphics *
            }
        ),
        ] 
    }
}
