
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy, serde::Deserialize, serde::Serialize)]
pub struct Layout {}

impl Blockify for Layout {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {


        vec![
            ("Erithaxlayout",
                Info::new(
                    "Erithax Layout",
                    Owner::Erithax,
                    "TODO",
                    "erithax.com",
                    SourceOpenness::NA,
                    vec![Lang::Rust],
                    None,
                ),
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
                Info::new(
                    "Blink",
                    Owner::Google,
                    "TODO",
                    "",
                    SourceOpenness::NA,
                    vec![],
                    None,
                ),
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
                Info::new(
                    "Flutter layout",
                    Owner::Google,
                    "TODO",
                    "https://api.flutter.dev/flutter/rendering/rendering-library.html",
                    SourceOpenness::Superopen,
                    vec![Lang::Dart],
                    None,
                ),
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
                Info::new(
                    "Gecko",
                    Owner::Mozilla,
                    "TODO",
                    "",
                    SourceOpenness::NA,
                    vec![],
                    None,
                ),
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
                Info::new(
                    "Servo",
                    Owner::LinuxFoundation,
                    "TODO",
                    "",
                    SourceOpenness::NA,
                    vec![Lang::Rust],
                    None,
                ),
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
                Info::new(
                    "Webkit",
                    Owner::Apple,
                    "TODO",
                    "",
                    SourceOpenness::NA,
                    vec![],
                    None,
                ),
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
                Info::new(
                    "Taffy",
                    Owner::Dioxuslabs,
                    "TODO",
                    "",
                    SourceOpenness::NA,
                    vec![Lang::Rust],
                    Repo::opt_with_url("https://github.com/DioxusLabs/taffy"),
                ),
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