use macroni::use_all_block_modules;

// use_all_block_modules!(ui);

use macroni::parsetree;
use macroni::parsetree2;
use crate::bowl::Blockify;
use crate::bowl::component::*;
use crate::bowl::owner::*;
use crate::bowl::*;
use super::ExtraInfo;
use super::*;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Ui {}


impl Blockify for Ui {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxui",
            Info {
                name: "Erithax UI",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $
            }
        ),
        ("Dom",
            Info {
                name: "DOM",
                owner: Owner::Webstandards,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Gecko *,
                $ Servo *,
                $ Blink *,
            }
        ),
        ("Dioxus",
            Info {
                name: "Dioxus",
                owner: Owner::Dioxuslabs,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ("Druid",
            Info {
                name: "Druid",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Piet
            }
        ),
        ("Egui",
            Info {
                name: "Egui",
                owner: Owner::Emilk,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: true,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $
            }
        ),
        ("Fltk",
            Info {
                name: "Fltk",
                owner: Owner::TODO,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Xwindow,
                $ Direct2d,
                $ Coregraphics,
            },
        ),
        ("Flutter",
            Info {
                name: "Flutter",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::C],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Skia *
            }
        ),
        ("Gtk",
            Info {
                name: "GTK",
                owner: Owner::Gnome,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::C],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Gdk *,
                $ Cairo
            }
        ),
        ("Iced",
            Info {
                name: "Iced",
                owner: Owner::Icedrs,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $
            }
        ),
        ("Leptos",
            Info {
                name: "Leptos",
                owner: Owner::Leptosrs,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ("Qt",
            Info {
                name: "Qt",
                owner: Owner::Qtcompany,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Xwindow
            }
        ),
        ("Scither",
            Info {
                name: "Scither",
                owner: Owner::Terrainformatica,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Cairo,
                $ Direct2d,
                $ Coregraphics
            }
        ),
        ("Slint",
            Info {
                name: "Slint",
                owner: Owner::Sixtyfps,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Skia,
                $ Slintcpu,
                $ Femtovg,
            }
        ),
        ("Xilem", 
            Info {
                name: "Xilem",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Taffy Vello *
            }
        ),
        ("Yew",
            Info {
                name: "Yew",
                owner: Owner::Yew,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::TODO),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ] 
    }
}




