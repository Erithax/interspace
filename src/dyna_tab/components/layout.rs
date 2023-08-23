
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
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
                    code_openness: SourceOpenness::NA,
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
                "Blink_ly", 
                Info {
                    name: "Blink",
                    owner: Owner::Google,
                    description: "TODO",
                    website: "",
                    code_openness: SourceOpenness::NA,
                    impl_langs: vec![],
                },
                ExtraInfo::Layout {
                    css: true,
                    flexbox: true,
                    grid: true,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Blink_pa *
                }
            ),
            (
                "Flutter_ly",
                Info {
                    name: "Flutter layout",
                    owner: Owner::Google,
                    description: "TODO",
                    website: "https://api.flutter.dev/flutter/rendering/rendering-library.html",
                    code_openness: SourceOpenness::Superopen,
                    impl_langs: vec![Lang::Dart],
                },
                ExtraInfo::Layout {
                    css: false,
                    flexbox: true,
                    grid: true,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Flutter_pa *
                }
            ),
            (
                "Gecko_ly", 
                Info {
                    name: "Gecko",
                    owner: Owner::Mozilla,
                    description: "TODO",
                    website: "",
                    code_openness: SourceOpenness::NA,
                    impl_langs: vec![],
                },
                ExtraInfo::Layout {
                    css: true,
                    flexbox: true,
                    grid: true,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Gecko_pa *,
                }
            ),
            (
                "Servo_ly", 
                Info {
                    name: "Servo",
                    owner: Owner::LinuxFoundation,
                    description: "TODO",
                    website: "",
                    code_openness: SourceOpenness::NA,
                    impl_langs: vec![Lang::Rust],
                },
                ExtraInfo::Layout {
                    css: true,
                    flexbox: true,
                    grid: true,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Servo_pa *
                }
            ),
            (
                "Webkit_ly", 
                Info {
                    name: "Webkit",
                    owner: Owner::Apple,
                    description: "TODO",
                    website: "",
                    code_openness: SourceOpenness::NA,
                    impl_langs: vec![],
                },
                ExtraInfo::Layout {
                    css: true,
                    flexbox: true,
                    grid: true,
                    constraint_based: false,
                },
                parsetree2!{
                    $ Webkit_pa *
                }
            ),
            (
                "Taffy", 
                Info {
                    name: "Taffy",
                    owner: Owner::Dioxuslabs,
                    description: "TODO",
                    website: "",
                    code_openness: SourceOpenness::NA,
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
            ),

        ]
    }
}