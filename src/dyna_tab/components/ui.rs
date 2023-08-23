
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
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
                website: "https://www.erithax.com/",
                code_openness: SourceOpenness::NA,
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
        (
            "Angular",
            Info {
                name: "Angular",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![Lang::Javascript],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::Mid,
                macrotivity: RoughRange::None,
                language: UiLang::Lang(Lang::Javascript),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ("Dom",
            Info {
                name: "DOM",
                owner: Owner::Webstandards,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
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
                $ Gecko_ly *,
                $ Servo_ly *,
                $ Blink_ly *,
                $ Webkit_ly *,
            }
        ),
        ("Dioxus",
            Info {
                name: "Dioxus",
                owner: Owner::Dioxuslabs,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                $ Flutter_ly *
            }
        ),
        ("Gtk",
            Info {
                name: "GTK",
                owner: Owner::Gnome,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
        (
            "React",
            Info {
                name: "React",
                owner: Owner::Meta,
                description: "TODO",
                website: "https://react.dev",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![Lang::Javascript],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::Reacty,
                declarativity: RoughRange::Hi,
                macrotivity: RoughRange::None,
                language: UiLang::Lang(Lang::Javascript),
                hot_reload: true,
                ssr: true,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ("Scither",
            Info {
                name: "Scither",
                owner: Owner::Terrainformatica,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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
        (
            "Svelte",
            Info {
                name: "Svelte",
                owner: Owner::Svelte,
                description: "TODO",
                website: "https://svelte.dev/",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![Lang::Javascript],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::Hi,
                macrotivity: RoughRange::None,
                language: UiLang::Lang(Lang::Javascript),
                hot_reload: true,
                ssr: true,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        (
            "SwiftUI",
            Info {
                name: "SwiftUI",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::Swifty,
                declarativity: RoughRange::TODO,
                macrotivity: RoughRange::TODO,
                language: UiLang::Lang(Lang::Swift),
                hot_reload: false,
                ssr: false,
                liveview: false,
            },
            parsetree2!{
                $ Coregraphics *
            }
        ),
        (
            "Vue",
            Info {
                name: "Vue",
                owner: Owner::Vue,
                description: "",
                website: "https://vuejs.org",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![],
            },
            ExtraInfo::Ui {
                is_immediate: false,
                reactivity: Reactivity::TODO,
                declarativity: RoughRange::Hi,
                macrotivity: RoughRange::None,
                language: UiLang::Lang(Lang::Javascript),
                hot_reload: true,
                ssr: true,
                liveview: false,
            },
            parsetree2!{
                $ Dom *
            }
        ),
        ("Xilem", 
            Info {
                name: "Xilem",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
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
                code_openness: SourceOpenness::NA,
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




