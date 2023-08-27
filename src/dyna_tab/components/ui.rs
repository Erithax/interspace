
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use super::ExtraInfo;
use super::*;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Ui {}


impl Blockify for Ui {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxui",
            Info::new(
                "Erithax UI",
                Owner::Erithax,
                "TODO",
                "https://www.erithax.com/",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
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
            Info::new(
                "Angular",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::Superopen,
                vec![Lang::Javascript],
                Repo::opt_with_url("https://github.com/angular/angular"),
            ),
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
            Info::new(
                "DOM",
                Owner::Webstandards,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
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
            Info::new(
                "Dioxus",
                Owner::Dioxuslabs,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/dioxuslabs/dioxus"),
            ),
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
            Info::new(
                "Druid",
                Owner::Linebender,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/linebender/druid"),
            ),
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
            Info::new(
                "Egui",
                Owner::Emilk,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/emilk/egui"),
            ),
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
            Info::new(
                "Fltk",
                Owner::TODO,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/fltk/fltk"),
            ),
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
            Info::new(
                "Flutter",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::C],
                Repo::opt_with_url("https://github.com/flutter/flutter"),
            ),
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
            Info::new(
                "GTK",
                Owner::Gnome,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::C],
                Repo::opt_with_url("https://gitlab.gnome.org/GNOME/gtk"),
            ),
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
            Info::new(
                "Iced",
                Owner::Icedrs,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/iced-rs/iced"),
            ),
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
            Info::new(
                "Leptos",
                Owner::Leptosrs,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/leptos-rs/leptos"),
            ),
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
            Info::new(
                "Qt",
                Owner::Qtcompany,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                Repo::opt_with_url("https://code.qt.io/cgit/")
            ),
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
            Info::new(
                "React",
                Owner::Meta,
                "TODO",
                "https://react.dev",
                SourceOpenness::Superopen,
                vec![Lang::Javascript],
                Repo::opt_with_url("https://github.com/facebook/react"),
            ),
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
            Info::new(
                "Scither",
                Owner::Terrainformatica,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                Repo::opt_with_url("https://github.com/sciter-sdk/rust-sciter"),
            ),
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
            Info::new(
                "Slint",
                Owner::Sixtyfps,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/slint-ui/slint"),
            ),
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
            Info::new(
                "Svelte",
                Owner::Svelte,
                "TODO",
                "https://svelte.dev/",
                SourceOpenness::Superopen,
                vec![Lang::Javascript],
                Repo::opt_with_url("https://github.com/sveltejs/svelte"),
            ),
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
            Info::new(
                "SwiftUI",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![],
                None,
            ),
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
            Info::new(
                "Vue",
                Owner::Vue,
                "",
                "https://vuejs.org",
                SourceOpenness::Superopen,
                vec![],
                Repo::opt_with_url("https://github.com/vuejs/vue"),
            ),
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
            Info::new(
                "Xilem",
                Owner::Linebender,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/linebender/xilem"),
            ),
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
            Info::new(
                "Yew",
                Owner::Yew,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/yewstack/yew"),
            ),
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




