
use dioxus::prelude::*;
use super::*;
use sha2::{Sha256, Digest};
use log::info;

#[inline_props]
pub fn Block(cx: Scope, comp_id: ComponentId, debug_info: Option<String>) -> Element {

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
            style: "
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100%;
                width: 100%;
                background: {color_str};
                font-size: 10px;
            ",
            div {
                "{CONSTELLATION.get_comp(*comp_id).info.name}",
            }
        }
    })
}