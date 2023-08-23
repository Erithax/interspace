
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Paint {},
            parsetree2!{
                $ Skia *,
                $ Webgpu Dawn *
            }
        ),
        (
            "Flutter_pa",
            Info {
                name: "Flutter paint",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![Lang::Dart],
            },
            ExtraInfo::Paint{},
            parsetree2!{
                $ Skia *,
                $ Impeller *
            }
        ),
        ("Gecko_pa", 
            Info {
                name: "Gecko",
                owner: Owner::Mozilla,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![],
            },
            ExtraInfo::Paint {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Paint {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![],
            },
            ExtraInfo::Paint {},
            parsetree2!{
                $ Coregraphics *
            }
        ),
        ] 
    }
}
