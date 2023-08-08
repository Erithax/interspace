#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_braces)]




/*
## Web
WebDOM, WebSVG, Webcanvas, WebGL, WebGPU
* option to stop when encountering one of these

## Filter
## Sort
## Relation styling
### Persistent
* background color
* background repeating text
* border
### Hover/Focus/Emphasize
* highlight components (border/background)
* scale highlighted components
* fade non-related components

* show owner logo on hover, also on all owner components
* show superproject logo on hover, also on all all superproject components
### 
* Union blocks (OR)
* Intersection blocks (AND)
* way to show fallback (e.g. CPU fallback)
* show shell (Browser, Webview, Electron, winit, glazier)

### Clicking focussed item expands row below with all its info

### Popover component

### Dragging rows

### Comparator component (cfr. current pipeline/stage component)
*/



mod dyna_tab;
mod image;

use crate::image::*;
use crate::dyna_tab::DynaTab;


use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;


use dioxus::html::a;
use dioxus::prelude::*;
use num_integer;
use sha2::{Sha256, Sha512, Digest};
use log::{info, LevelFilter};

use serde::Serialize;
use serde::Deserialize;

/*
Before rewrite: size in bytes:
    Skia fulldowntree : 1384
        fulluptree: 1192
        splitdowntree: 808
        splituptree: 616

*/


pub fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init dioxus logger");
    dioxus_web::launch_cfg(App, dioxus_web::Config::new().rootname("entry"));
}



pub fn indent(s: &str) -> String {
    let res: String = s.replace("\n", "\n\t");
    return res
}

// pub fn variant_eq<T>(a: &T, b: &T) -> bool {
//     return std::mem::discriminant(a) == std::mem::discriminant(b)
// }





fn App(cx: Scope) -> Element {

    let dyna_tab_id = use_state(cx, || 0 as usize);
    let d_count = use_state(cx, || 1);

    cx.render(rsx!{
        Header{},
        main {
            id: "main-content",
            div {
                style: "
                    margin: 10px 0;
                    background: #ff03;
                    padding: 20px;
                    border-radius: 10px;
                ",
                "⚠️Work in progress. Data is incomplete and contains errors.",
            },
            div {
                onclick: move |_| {
                    d_count.set(d_count.get()+1 % 5);
                    dyna_tab_id.set(dyna_tab_id.get() + 1);
                    let create_eval = use_eval(cx);
                    create_eval(r#"
                        updateGridSizerList()
                    "#).unwrap();
                },
                style: "
                    width: 40px; height: 40px; border-radius: 20px; background: #777; display: flex; justify-content: center; align-items: center;
                    margin: 10px 0;
                ",
                span {
                    "+"
                }
            },
            for i in 0..*d_count.get() {
                rsx!{DynaTab{
                    id: *dyna_tab_id.get(),
                }}
            }
        },
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}
impl Theme {
    pub fn set_favicon() {
        let prefers_dark_color_theme = web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)");
        let icon = web_sys::window().unwrap().document().unwrap().get_element_by_id("favicon").unwrap();

        if prefers_dark_color_theme.is_ok() && prefers_dark_color_theme.as_ref().unwrap().is_some() && prefers_dark_color_theme.unwrap().unwrap().matches() {
            icon.set_attribute("href", "/img/Erithax/Erithax_dark.ico");
        } else {
            icon.set_attribute("href", "/img/Erithax/Erithax_light.ico");
        }
    }

    pub fn apply<T>(&self, cx: Scope<T>) {
        let create_eval = use_eval(cx);
        match self {
            Theme::Dark => {
                create_eval(r#"
                    window.document.getElementById("entry").classList.remove("th_light");
                    window.document.getElementById("entry").classList.add("th_dark");
                "#,).unwrap();
            },
            Theme::Light => {
                create_eval(r#"
                    window.document.getElementById("entry").classList.remove("th_dark");
                    window.document.getElementById("entry").classList.add("th_light");
                "#).unwrap();
            }
        };
    }

    pub fn toggle<T>(&self, cx: Scope<'_, T>) -> Self {
        let newth = match *self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };

        let create_eval = use_eval(cx);

        match newth {
            Theme::Dark => {
                create_eval(
                    r#"
                    window.document.getElementById("entry").classList.add("switching_theme");
                    window.document.getElementById("entry").classList.remove("th_light");
                    window.document.getElementById("entry").classList.add("th_dark");
                    setTimeout(() => {window.document.getElementById("entry").classList.remove("switching_theme")}, 200);
                    "#,
                ).unwrap();
            },
            Theme::Light => {
                create_eval(
                    r#"
                    window.document.getElementById("entry").classList.add("switching_theme");
                    window.document.getElementById("entry").classList.remove("th_dark");
                    window.document.getElementById("entry").classList.add("th_light");
                    setTimeout(() => {window.document.getElementById("entry").classList.remove("switching_theme")}, 200);
                    "#
                ).unwrap();
            }
        };
        return newth
    }

    pub fn to_class(&self) -> String {
        match self {
            Theme::Dark => {return "th_dark".to_string()},
            Theme::Light => {return "th_light".to_string()}
        }
    }
}



fn Header(cx: Scope) -> Element {
    Theme::set_favicon();
    let theme = use_state(cx, || Theme::Dark);
    theme.get().apply(cx);

    let show_love_dialog = use_state(cx, || false);

    cx.render(rsx!{
        header {
            div {
                class: "lef",
                a {
                    img {
                        src: "./img/Erithax/Erithax.svg",
                        alt: "erithax logo",
                    }
                }
            },
            div {
                class: "rig",
                a {
                    class: "code",
                    href: "https://github.com/erithax/ui-overview",
                    target: "_blank",
                    img {
                        src: "./img/code.svg",
                        alt: "repository",
                    },
                },
                div {
                    class: "seperator",
                }
                button {
                    onclick: move |_| {
                        show_love_dialog.set(true);
                    },
                    class: "heart",
                    img {
                        src: "./img/heart.svg",
                        alt: "support me",
                    },
                    if *show_love_dialog.get() {
                        rsx!{LoveDialog {show: show_love_dialog.clone()}}
                    }
                },
                // button {
                //     class: "wrench",
                //     img {
                //         src: "./img/wrench.svg",
                //         alt: "contribute",
                //     },
                // },
                // button {
                //     class: "font",
                //     img {
                //         src: "./img/font.svg",
                //         alt: "font",
                //     },
                // },
                button {
                    class: "sunmoon",
                    onclick: move |_| {theme.set(theme.toggle(cx));},
                    if **theme == Theme::Dark {
                        rsx!{img {
                            class: "sun",
                            src: "./img/sun.svg",
                            alt: "enable light mode",
                        }}
                    } else {
                        rsx!{img {
                            class: "moon",
                            src: "./img/moon.svg",
                            alt: "enable dark mode",
                        }}
                    }
                },
            }
        }
    })
}

pub fn inline_erithax<T: Into<f64>>(id: String, col: String, size: T, stroke_wid: T) -> String {
    let mut res = erithax(id, col, size, stroke_wid);
    res.retain(|char| char != '\n');
    return res
}

pub fn erithax<T: Into<f64>>(id: String, col: String, size: T, stroke_wid: T) -> String {
    let size: f64 = size.into();
    let stroke_wid: f64 = stroke_wid.into();

    let octa_side_len = (size-stroke_wid) / (1.0 + 2.0_f64.sqrt()); // top flat
    let octa_side_proj = 2.0_f64.sqrt()/2.0 * octa_side_len;

    let cs0: f64 = 0.0;
    let cs1: f64 = 0.0 + stroke_wid/2.0;
    let cs2: f64 = 0.0 + stroke_wid/2.0 + octa_side_proj;
    let cm: f64 = size/2.0;
    let ce2: f64 = size - cs2;
    let ce1: f64 = size - cs1;
    let ce0: f64 = size;

    let L = octa_side_len / 3.0; // theta and xi stripe length
    let th0 = cm-L/2.0;
    let th1 = cm+L/2.0;
    let xi0 = ce2 + octa_side_proj/2.0 + stroke_wid/2.0 - L/2.0;
    let xi1 = ce2 + octa_side_proj/2.0 + stroke_wid/2.0 + L/2.0;

    let res = format!(r#"
        <svg viewBox="0 0 {size} {size}" preserveAspectRatio="xMidYMid meet">
            <path d="M {cs2},{cs1}  {ce2},{cs1}  {ce2},{ce1}  {cs2},{ce1} Z M {th0},{cm}  {th1},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
            <path d="M {cs2},{cs2}  {cs1},{cs2}  {cs1},{ce2}  {cs2},{ce2} M {cs1},{cm}  {cs2},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
            <path d="M {ce2},{cs2}  {ce0},{cs2} M {ce0},{ce2}  {ce2},{ce2} M {xi0},{cm}  {xi1},{cm}" stroke-width="{stroke_wid}" stroke="{col}" fill="none"/>
       </svg>
    "#);

    return res
}

#[inline_props]
pub fn LoveDialog(cx: Scope, show: UseState<bool>) -> Element {
    render!{
        div {
            onclick: move |_| {
                show.set(false);
            },
            style: "
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background-color: #000a;
                color: white;
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 100;
                font-size: 14px;
                line-height: 2;
            ",
            div {
                onclick: move |e| {e.stop_propagation();},
                style: "padding: 40px; border: 1px solid white; background-color: #000;",
                div {
                    "Hire me! 𝕖𝕣𝕚𝕥𝕙𝕒𝕩@𝕡𝕣𝕠𝕥𝕠𝕟.𝕞𝕖",
                },
                div {
                    "Star on ",
                    a {
                        href: "https://www.github.com/erithax/ui-overview",
                        target: "_blank",
                        "Github",
                    }
                },
                div {
                    a {
                        href: "https://www.github.com/erithax/ui-overview",
                        target: "_blank",
                        "Contribute code/data",
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsableToggle {
    Collapsed,
    Expanded
}

impl CollapsableToggle {

    pub fn toggle_mut(&mut self) {
        *self = self.toggle();
    }

    pub fn toggle(&self) -> Self {
        match self {
            CollapsableToggle::Collapsed => {CollapsableToggle::Expanded},
            CollapsableToggle::Expanded => {CollapsableToggle::Collapsed},
        }
    }

    pub fn is_collapsed(&self) -> bool {
        match self {
            Self::Collapsed => {true},
            Self::Expanded => {false},
        }
    }

    pub fn is_expanded(&self) -> bool {
        match self {
            Self::Collapsed => {false},
            Self::Expanded => {true},
        }
    }
}

impl std::fmt::Display for CollapsableToggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollapsableToggle::Collapsed => { write!(f, "collapsed")},
            CollapsableToggle::Expanded => {write!(f, "expanded")},
        }
    }
}




