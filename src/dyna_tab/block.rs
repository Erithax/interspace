
use dioxus::prelude::*;
use super::*;
use sha2::{Sha256, Digest};
use super::owner::*;

pub enum BlockBoxerEvent {
    Snip,
    Skip,
}

#[inline_props]
pub fn Block<'a>(cx: Scope<'a>, comp_id: ComponentId, is_focussed: bool, on_bonk: EventHandler<'a, BlockBoxerEvent>, debug_info: Option<String>) -> Element<'a> {

    let show_debug_info = false;
    let comp = CONSTELLATION.get_comp(*comp_id);


    // BACKGROUND COLOR
    let color_str = {
        match comp.info.owner.value().light_back {
            Some(c) => {c.to_string()},
            None => { // gen random color seeded with component owner name
                let mut hasher = Sha256::new();
                hasher.update(comp.info.owner.to_string());
                let result = hasher.finalize();
                let color_nums = &result[1..=3].to_vec();
                format!("rgba({}, {}, {}, 0.2)", color_nums[0], color_nums[1], color_nums[2])
            }
        }
    };


    cx.render(rsx!{
        div {
            class: "block_box normal focussed-{is_focussed}",
            style: "
                background: {color_str};
            ",
            div {
                class: "block_button snip",
                onclick: move |_| {on_bonk.call(BlockBoxerEvent::Snip)},
            },
            div {
                class: "block_button skip",
                onclick: move |_| {on_bonk.call(BlockBoxerEvent::Skip);},
            },
            if show_debug_info && debug_info.is_some() {
                rsx!{div {
                    class: "name",
                    "{comp.info.name}\n{debug_info.as_ref().unwrap()}",
                }}
            } else {
                rsx!{div {
                    class: "name",
                    "{comp.info.name}",
                }}
            },
            if *is_focussed {
                let disp_website = strip_website(comp.info.website);
                let disp_impl_langs = if comp.info.impl_langs.len() == 0 {
                    "".to_owned()
                } else {
                    comp.info.impl_langs[0].value().name.to_owned() + &comp.info.impl_langs.iter().skip(1).fold("".to_owned(), |acc: String, nex| acc + ", " + nex.value().name)
                };
                
                rsx!{div {
                    class: "info",
                    OwnerComp {
                        self_: comp.info.owner,
                    },
                    if disp_website != "" {
                        render!{a {
                            class: "website",
                            href: "comp.info.website",
                            target: "_blank",
                            "{disp_website}",
                        }}
                    },
                    div {
                        class: "openness",
                        "{comp.info.code_openness:?}",
                    },
                    div {
                        class: "impl_langs",
                        "{disp_impl_langs}",
                    },
                    div {
                        "{comp.info.description}",
                    }
                }}
            } else {
                rsx!{pre {hidden: "", style: "display: none;"}}
            }
        }
    })
}