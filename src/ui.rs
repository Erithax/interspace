use macroni::use_all_block_modules;

use_all_block_modules!(ui);



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum StateManaPara {
    TODO,
    React,
    Elm,
    Lenses,
    Signals,
    Swifty,
    Dom,
    Immediate,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum UiMode {
    Immediate,
    Retained
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct UiInfo {
    pub ui_lang: Lang,
    pub ui_mode: UiMode,
    pub state_mana_para: StateManaPara,
}

impl Default for Ui {fn default() -> Self {return Ui::META}}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Ui {
    META,
    Erithaxui,

    Dom,
    Dioxus,
    Druid,
    Egui,
    Fltk,
    Flutter,
    Gtk,
    Iced,
    Leptos,
    Qt,
    Scither,
    Slint,
    Xilem,
    Yew,
}


impl Blockify<Ui, UiInfo> for Ui {
    fn add_all() -> Vec<(Ui, Info, UiInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (Ui::Erithaxui,
            Info {
                name: "Erithax UI".to_string(),
                owner: Owner::Erithax,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![],
            },
            UiInfo {
                ui_lang: Lang::NA,
                ui_mode: UiMode::Retained,
                state_mana_para: StateManaPara::TODO,
            },
            parsetree!{
                $
            }
        ),
        (Ui::Dom,
            Info {
                name: "DOM".to_string(),
                owner: Owner::Webstandards,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::Javascript],
            },
            UiInfo {
                ui_lang: Lang::Html,
                ui_mode: UiMode::Retained,
                state_mana_para: StateManaPara::Dom,
            },
            parsetree!{
                $ Gecko *,
                $ Servo *,
                $ Blink *,
            }
        ),
        (Ui::Dioxus,
            Info {
                name: "Dioxus".to_string(),
                owner: Owner::Dioxuslabs,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                ui_mode: UiMode::Retained,
                state_mana_para: StateManaPara::TODO,
            },
            parsetree!{
                $ Dom *
            }
        ),
        (Ui::Druid,
            Info {
                name: "Druid".to_string(),
                owner: Owner::Linebender,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Lenses,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Piet
            }
        ),
        (Ui::Egui,
            Info {
                name: "Egui".to_string(),
                owner: Owner::Emilk,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Immediate,
                ui_mode: UiMode::Immediate,
            },
            parsetree!{
                $
            }
        ),
        (Ui::Fltk,
            Info {
                name: "Fltk".to_string(),
                owner: Owner::TODO,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            UiInfo {
                ui_lang: Lang::TODO,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Xwindow,
                $ Direct2d,
                $ Coregraphics,
            },
        ),
        (Ui::Flutter,
            Info {
                name: "Flutter".to_string(),
                owner: Owner::Google,
                desc: TDS!(),
                imp_lang: Lang::C,
                bind_langs: vec![Lang::Dart],
            },
            UiInfo {
                ui_lang: Lang::Dart,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Immediate,
            },
            parsetree!{
                $ Skia *
            }
        ),
        (Ui::Gtk,
            Info {
                name: "GTK".to_string(),
                owner: Owner::Gnome,
                desc: TDS!(),
                imp_lang: Lang::C,
                bind_langs: vec![Lang::C, Lang::Cpp, Lang::Javascript, Lang::Python]
            },
            UiInfo {
                ui_lang: Lang::NA,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Gdk *,
                $ Cairo
            }
        ),
        (Ui::Iced,
            Info {
                name: "Iced".to_string(),
                owner: Owner::Icedrs,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Elm,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $
            }
        ),
        (Ui::Leptos,
            Info {
                name: "Leptos".to_string(),
                owner: Owner::Leptosrs,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Signals,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Dom *
            }
        ),
        (Ui::Qt,
            Info {
                name: "Qt".to_string(),
                owner: Owner::Qtcompany,
                desc: TDS!(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Cpp, Lang::Javascript, Lang::Python, Lang::Csharp, Lang::Rust]
            },
            UiInfo {
                ui_lang: Lang::Qml,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Xwindow
            }
        ),
        (Ui::Scither,
            Info {
                name: "Scither".to_string(),
                owner: Owner::Terrainformatica,
                desc: TDS!(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Javascript],
            },
            UiInfo {
                ui_lang: Lang::Html,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Cairo,
                $ Direct2d,
                $ Coregraphics
            }
        ),
        (Ui::Slint,
            Info {
                name: "Slint".to_string(),
                owner: Owner::Sixtyfps,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust, Lang::Cpp, Lang::Javascript],
            },
            UiInfo {
                ui_lang: Lang::Slintmarkup,
                state_mana_para: StateManaPara::TODO,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Skia,
                $ Slintcpu,
                $ Femtovg,
            }
        ),
        (Ui::Xilem, 
            Info {
                name: "Xilem".to_string(),
                owner: Owner::Linebender,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Swifty,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Taffy Vello *
            }
        ),
        (Ui::Yew,
            Info {
                name: "Yew".to_string(),
                owner: Owner::Yew,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            UiInfo {
                ui_lang: Lang::Rust,
                state_mana_para: StateManaPara::Elm,
                ui_mode: UiMode::Retained,
            },
            parsetree!{
                $ Dom *
            }
        ),
        ] 
    }
}




