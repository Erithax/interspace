
use dioxus::prelude::*;
use super::*;
use sha2::{Sha256, Digest};
use log::info;

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
                    "{CONSTELLATION.get_comp(*comp_id).info.name}\n{debug_info.as_ref().unwrap()}",
                }}
            } else {
                rsx!{div {
                    class: "name",
                    "{CONSTELLATION.get_comp(*comp_id).info.name}",
                }}
            }
        }
    })
}