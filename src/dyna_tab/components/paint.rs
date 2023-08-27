
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Paint {}


impl Blockify for Paint {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxpaint",
            Info::new(
                "Erithax Paint",
                Owner::Erithax,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Paint {},
            parsetree2!{
                $
            }
        ),
        ("Blink_pa", 
            Info::new(
                "Blink",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                None,
            ),
            ExtraInfo::Paint {},
            parsetree2!{
                $ Skia *,
                $ Webgpu Dawn *
            }
        ),
        (
            "Flutter_pa",
            Info::new(
                "Flutter paint",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::Superopen,
                vec![Lang::Dart],
                None,
            ),
            ExtraInfo::Paint{},
            parsetree2!{
                $ Skia *,
                $ Impeller *
            }
        ),
        ("Gecko_pa", 
            Info::new(
                "Gecko",
                Owner::Mozilla,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![],
                None,
            ),
            ExtraInfo::Paint {},
            parsetree2!{
                $ Webrender *
            }
        ),
        ("Servo_pa", 
            Info::new(
                "Servo",
                Owner::LinuxFoundation,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Paint {},
            parsetree2!{
                $ Webrender *
            }
        ),
        ("Webkit_pa", 
            Info::new(
                "Webkit",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![],
                None,
            ),
            ExtraInfo::Paint {},
            parsetree2!{
                $ Coregraphics *
            }
        ),
        ] 
    }
}
