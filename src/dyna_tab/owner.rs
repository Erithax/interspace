
use serde;
use strum_macros::Display;
use dioxus::prelude::*;

use crate::{CollapsableToggle, strip_website};

#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub enum OwnerType {
    Individual,
    Community,
    Corporation {
        nonprofit: bool,
        sub_corps: Vec<Owner>,
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct OwnerInfo {
    pub name: &'static str,
    pub website: &'static str,
    pub ownertype: OwnerType,
    pub light_back: Option<&'static str>,
    pub dark_back: Option<&'static str>,
}

impl Default for Owner {fn default() -> Self {return Owner::Erithax}}

// =======================
// ADD OWNERS BELOW
// =======================
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display, strum_macros::EnumIter, strum_macros::EnumString, serde::Deserialize, serde::Serialize)]
pub enum Owner {
    TODO,
    Erithax,

    Adobe,
    Apple,
    Cairogfx,
    Doitsujin,
    Dioxuslabs,
    Emilk,
    Femtovg,
    Gfxrs,
    Gnome,
    Google,
    Grovesnl,
    Icedrs,
    Khronos,
    Leptosrs,
    Linebender,
    LinuxFoundation,
    Meta,
    Microsoft,
    Mozilla,
    Qtcompany,
    Sixtyfps,
    Svelte,
    Terrainformatica,
    Vue,
    Webstandards,
    Yew,
    Xorg,

    Gtkrs,
    // Custom(String, bool, bool),
}


impl Owner {
    pub fn value(&self) -> OwnerInfo {
        match self {
            Owner::TODO => OwnerInfo {
                name: "TODO",
                website: "https://www.erithax.com/",
                ownertype: OwnerType::Individual,
                light_back: Some("#ccc"),
                dark_back: Some("#444"),
            },
            Owner::Erithax => OwnerInfo {
                name: "Erithax",
                website: "https://www.erithax.com/",
                ownertype: OwnerType::Individual,
                light_back: Some("
                    linear-gradient(to top left,  black 0%, #0000 30%, #0000 70%, black 100%),
                    linear-gradient(to top right, blueviolet 0%, #0000 70%, #0000 70%, black 100%)
                "),
                dark_back: Some("white"),
            },
            Owner::Apple => OwnerInfo {
                name: "Apple",
                website: "https://www.apple.com/",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![] },
                light_back: Some("linear-gradient(to right, black 0%, #0000 30%, #0000 70%, #555f 100%)"),
                dark_back: Some("linear-gradient(to right, white 0%, #0000 30%, #0000 70%, #aaaf 100%)"),
            },
            Owner::Adobe => OwnerInfo {
                name: "Adobe",
                website: "https://www.adobe.com/",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![] },
                light_back: Some("linear-gradient(to right, #ed2224 0%, #0000 30%, #0000 70%, #555f 100%)"),
                dark_back: Some("linear-gradient(to right, #ed2224 0%, #0000 30%, #0000 70%, black 100%)"),
            },
            Owner::Cairogfx => OwnerInfo { 
                name: "Cairo Graphics", 
                website: "https://www.cairographics.org/", 
                ownertype: OwnerType::Community, 
                light_back: Some("linear-gradient(to right, #162284 0%, #0000 30%, #0000 70%, #f39914 100%)"), 
                dark_back: Some("linear-gradient(to right, #162284 0%, #0000 30%, #0000 70%, #f39914 100%)"), 
            },
            Owner::Dioxuslabs => OwnerInfo {
                name: "Dioxuslabs",
                website: "https://www.dioxuslabs.com/",
                ownertype: OwnerType::Individual,
                light_back: Some("
                    linear-gradient(to top right,    #00a8d6 0%, #0000 50%, #0000 50%, #00a8d6 100%), 
                    linear-gradient(to top left,     #e96020 0%, #0000 50%, #0000 50%, #e96020 100%)"
                ),
                dark_back: Some("
                    linear-gradient(to top right,    #00a8d6 0%, #0000 50%, #0000 50%, #00a8d6 100%), 
                    linear-gradient(to top left,     #e96020 0%, #0000 50%, #0000 50%, #e96020 100%)"
                ),
            },
            Owner::Doitsujin => OwnerInfo {
                name: "Doitsujin",
                website: "https://github.com/doitsujin",
                ownertype: OwnerType::Individual,
                light_back: None,
                dark_back: None,
            },
            Owner::Emilk => OwnerInfo {
                name: "Emilk",
                website: "https://github.com/emilk",
                ownertype: OwnerType::Individual,
                light_back: None,
                dark_back: None,
            },
            Owner::Femtovg => OwnerInfo {
                name: "femtovg",
                website: "https://github.com/femtovg",
                ownertype: OwnerType::Individual,
                light_back: None,
                dark_back: None,
            },
            Owner::Gfxrs => OwnerInfo {
                name: "gfx-rs",
                website: "gfx-rs.github.io",
                ownertype: OwnerType::Community,
                light_back: None,
                dark_back: None,
            },
            Owner::Gtkrs => OwnerInfo {
                name: "gtk-rs",
                website: "gtk-rs.org",
                ownertype: OwnerType::Individual,
                light_back: Some("linear-gradient(to top, #246cc9 0%, #0000 20%, #0000 80%, #c881ce 100%"),
                dark_back: Some("linear-gradient(to top, #246cc9 0%, #0000 20%, #0000 80%, #c881ce 100%"),
            },
            Owner::Gnome => OwnerInfo {
                name: "The GNOME Project",
                website: "https://foundation.gnome.org/",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![] },
                light_back: Some("linear-gradient(to right, #4a86cf 0%, #0000 10%)"),
                dark_back: Some("linear-gradient(to right, #4a86cf 0%, #0000 10%)"),
            },
            Owner::Google => OwnerInfo {
                name: "Google",
                website: "abc.xyz",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![] },
                light_back: Some("
                    linear-gradient(to top left,  #f4b400 0%, #0000 50%, #4285f4 100%), 
                    linear-gradient(to top right, #db4437 0%, #0000 50%, #0f9d58 100%), 
                    #0000;"),
                dark_back: Some("
                    linear-gradient(to top left,  #f4b400 0%, #0000 50%, #4285f4 100%), 
                    linear-gradient(to top right, #db4437 0%, #0000 50%, #0f9d58 100%), 
                    #0000;")
            },
            Owner::Grovesnl => OwnerInfo {
                name: "GrovesNL",
                website: "github.com/grovesNL",
                ownertype: OwnerType::Individual,
                light_back: None,
                dark_back: None,
            },
            Owner::Icedrs => OwnerInfo {
                name: "Iced-rs",
                website: "https://iced.rs",
                ownertype: OwnerType::Community,
                light_back: Some("
                    linear-gradient(to top left,  #2e10ff, #0000 50%, #2e10ff 100%),
                    linear-gradient(to top right, #2e10ff, #0000 50%, #2e10ff 100%)
                "),
                dark_back: Some("
                    linear-gradient(to top left,  #2e10ff, #0000 50%, #2e10ff 100%),
                    linear-gradient(to top right, #2e10ff, #0000 50%, #2e10ff 100%)
                "),
            },
            Owner::Khronos => OwnerInfo {
                name: "Khronos Group",
                website: "https://www.khronos.org/",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![]},
                light_back: Some("
                    linear-gradient(to top right, red 0%, #0000 50%),
                    linear-gradient(to top left , red 0%, #0000 50%)
                "),
                dark_back: Some("
                    linear-gradient(to top right, red 0%, #0000 50%),
                    linear-gradient(to top left,  red 0%, #0000 50%)
                "),
            },
            Owner::Leptosrs => OwnerInfo {
                name: "Leptos",
                website: "https://leptos.dev",
                ownertype: OwnerType::Individual,
                light_back: Some("
                    linear-gradient(to top right, #1b0e38 0%, #0000 60%, #e8363b 100%),
                    linear-gradient(to top left, #e8363b 0%, #fff0 40%, #1b0e38 100%)
                "),
                dark_back: Some("linear-gradient(to top left, #e8363b 0%, #0000 50%, #1b0e38 100%)"),
            },
            Owner::Linebender => OwnerInfo {
                name: "Linebender",
                website: "https://linebender.org",
                ownertype: OwnerType::Community,
                light_back: None,
                dark_back: None,
            },
            Owner::LinuxFoundation => OwnerInfo {
                name: "Linux Foundation",
                website: "https://www.linuxfoundation.org/",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![] },
                light_back: Some("linear-gradient(to top right, #0094ff 0%, #0000 30%, #0000 60%, #003778 100%)"),
                dark_back: Some("linear-gradient(to top right, #0094ff 0%, #0000 30%, #0000 60%, #003778 100%)"),
            },
            Owner::Meta => OwnerInfo {
                name: "Meta",
                website: "https://meta.com",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![]},
                light_back: Some("
                    linear-gradient(to top right, #0094ff 0%, #0000 50%, #0094ff 100%),
                    linear-gradient(to top left,  #003778 0%, #0000 50%, #003778 100%)
                "),
                dark_back: Some("
                    linear-gradient(to top right, #0094ff 0%, #0000 50%, #0094ff 100%),
                    linear-gradient(to top left,  #003778 0%, #0000 50%, #003778 100%)
                "),
            },
            Owner::Microsoft => OwnerInfo {
                name: "Microsoft",
                website: "https://microsoft.com",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![]},
                light_back: Some("
                    linear-gradient(to top left,  #ee4a1b 0%, #0000 50%, #81b900 100%), 
                    linear-gradient(to top right, #fbb900 0%, #0000 50%, #00a6e8 100%), 
                    #0000;"),
                dark_back: Some("
                    linear-gradient(to top left,  #ee4a1b 0%, #0000 50%, #81b900 100%), 
                    linear-gradient(to top right, #fbb900 0%, #0000 50%, #00a6e8 100%), 
                    #0000;")
            },
            Owner::Mozilla => OwnerInfo {
                name: "Mozilla Foundation",
                website: "https://www.mozilla.org/",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![] },
                light_back: Some("linear-gradient(to bottom right, #331E54 0%, #0000 30%, #0000 70%, #E66000 100%)"),
                dark_back: Some("linear-gradient(to bottom right, #331E54 0%, #0000 30%, #0000 70%, #E66000 100%)"),
            },
            Owner::Qtcompany => OwnerInfo {
                name: "Qt Company",
                website: "https://qt.io",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![]},
                light_back: Some("linear-gradient(to right, #41cd52 0%, #0000 30%, #0000 70%, #41cd52 100%)"),
                dark_back: Some("linear-gradient(to right, #41cd52 0%, #0000 30%, #0000 70%, #41cd52 100%)"),
            },
            Owner::Sixtyfps => OwnerInfo {
                name: "SixtyFPS GmbH",
                website: "https://slint.dev",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![] },
                light_back: Some("#2379f4"),
                dark_back: Some("#2379f4"),
            },
            Owner::Svelte => OwnerInfo {
                name: "Svelte Team",
                website: "https://github.com/sveltejs",
                ownertype: OwnerType::Community,
                light_back: Some("#ff3d01"),
                dark_back: Some("#ff3d01"),
            },
            Owner::Terrainformatica => OwnerInfo {
                name: "Terra Informatica",
                website: "https://terrainformatica.com",
                ownertype: OwnerType::Corporation { nonprofit: false, sub_corps: vec![] },
                light_back: Some("
                    linear-gradient(to top left,  #d93131 0%, #0000 50%, #d93131 100%),
                    linear-gradient(to top right, #d93131 0%, #0000 50%, #d93131 100%)
                "),
                dark_back: Some("#d93131"),
            },
            Owner::Vue => OwnerInfo {
                name: "Vue Team",
                website: "w3.org",
                ownertype: OwnerType::Community,
                light_back: Some("
                    linear-gradient(to top left,  #3fb984 0%, #0000 50%, #31475e 100%),
                    linear-gradient(to top right, #3fb984 0%, #0000 50%, #31475e 100%)
                "),
                dark_back: Some("
                    linear-gradient(to top left: #3fb984 0%, #0000 50%, #31475e 100%),
                    linear-gradient(to top right: #3fb984 0%, #0000 50%, #31475e 100%)
                "),
            },
            Owner::Webstandards => OwnerInfo {
                name: "WebStandards",
                website: "w3.org",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![]},
                light_back: Some("
                    linear-gradient(to top left,     #005a9c 0%, #0000 50%, #005a9c 100%),
                    linear-gradient(to top right,    #005a9c 0%, #0000 50%, #005a9c 100%)
                "),
                dark_back: Some("#005a9c"),
            },
            Owner::Yew => OwnerInfo {
                name: "Yew",
                website: "https://yew.rs/",
                ownertype: OwnerType::Community,
                light_back: Some("
                    linear-gradient(to top left,     #009a5b 0%, #0000 50%, #009a5b 100%),
                    linear-gradient(to top right,    #009a5b 0%, #0000 50%, #009a5b 100%)
                "),
                dark_back: Some("#009a5b"),
            },
            Owner::Xorg => OwnerInfo {
                name: "X.org Foundation",
                website: "https://x.org/",
                ownertype: OwnerType::Corporation { nonprofit: true, sub_corps: vec![]},
                light_back: Some("linear-gradient(to right, #ed4f1a 0%, #0000 30%, #0000 70%, #fdc253 100%"),
                dark_back: Some("linear-gradient(to right, #ed4f1a 0%, #0000 30%, #0000 70%, #fdc253 100%"),
            },
            // Owner::Custom(name, incorporated, nonprofit) => OwnerInfo {
            //     name: name),
            //     incorporated: *incorporated,
            //     nonprofit: *nonprofit,
            // },
        }
    }

    
}


#[inline_props]
pub fn OwnerComp(cx: Scope, self_: Owner) -> Element {

    let collapsed = use_state(cx, || CollapsableToggle::Collapsed);
    let disp_website = strip_website(self_.value().website.clone());
    
    render!{
        div {
            onclick: move |_| {collapsed.set(collapsed.get().toggle());},
            class: "owner",
            div {
                class: "label",
                "{self_.value().name}"
            },
            div{class:"collapsable {collapsed}", div {class: "ownerinfo",
                a {
                    class: "website",
                    href: "{disp_website}",
                    target: "_blank",
                    "{disp_website}"
                },
                match self_.value().ownertype {
                    OwnerType::Individual => {
                        rsx!{
                            div {
                                "individual"
                            }
                        }
                    },
                    OwnerType::Community => {
                        rsx!{
                            div {
                                "community"
                            }
                        }
                    },
                    OwnerType::Corporation{nonprofit, sub_corps} => {
                        let nonprofit_str = if nonprofit {"non-profit"} else {"for-profit"};
                        rsx!{
                            div {
                                "{nonprofit_str} corporation"
                            },
                            if sub_corps.len() > 0 {
                                rsx!{div {
                                    "sub corporations:",
                                    for sub_corp in sub_corps {
                                        div {
                                            "{sub_corp.value().name}"
                                        }
                                    }
                                }}
                            }
                            
                        }
                    }
                }
            }}
        }
    }
}