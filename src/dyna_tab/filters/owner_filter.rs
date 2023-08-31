



use strum::IntoEnumIterator;

use crate::utils::largebag::LargeBackSelectorComp;
use crate::dyna_tab::CONSTELLATION;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::filters::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerFilter {
    pub disallowed: Vec<Owner>,
}

impl OwnerFilter {
    pub fn toggle(&mut self, owner: Owner) {
        if self.disallowed.contains(&owner) {
            self.disallowed.retain(|&i| i != owner);
        } else {
            self.disallowed.push(owner);
        }
    }
}

impl CompFilter for OwnerFilter {
    fn filter(&self, comp: &Componentus) -> bool {
        return !self.disallowed.contains(&comp.info.owner);
    }

    // #[inline_props]
    fn render(cx: Scope, self_: UseRef<Box<OwnerFilter>>) -> Element {
        // let s0 = self_.clone();
        cx.render(rsx!{div{}})
    }
}


#[inline_props]
pub fn OwnerFilterComp(cx: Scope, self_: UseRef<OwnerFilter>) -> Element {

    let selected = use_ref(cx, || self_.read().disallowed.clone());

    cx.render(rsx!{
        div {
            class: "",
            h4 {
                "Owner filter",
            },
            LargeBackSelectorComp{
                items: Owner::iter().map(|o| (o, o.value().name.to_string())).collect(),
                selected: selected.clone(),
                ontoggleitem: move |owner: Owner| {
                    self_.write().toggle(owner);
                    *selected.write() = self_.read().disallowed.clone();
                },
            }
        }
    })
}


