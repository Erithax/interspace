



use crate::utils::largebag::LargeBackSelectorComp;
use crate::dyna_tab::CONSTELLATION;
use crate::dyna_tab::stage::*;
use crate::dyna_tab::filters::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentIdFilter {
    pub disallowed: Vec<ComponentId>,
}

impl ComponentIdFilter {
    pub fn toggle(&mut self, id: ComponentId) {
        if self.disallowed.contains(&id) {
            self.disallowed.retain(|&i| i != id);
        } else {
            self.disallowed.push(id);
        }
    }
}

impl CompFilter for ComponentIdFilter {
    fn filter(&self, comp: &Componentus) -> bool {
        return !self.disallowed.contains(&comp.id);
    }

    // #[inline_props]
    fn render(cx: Scope, self_: UseRef<Box<ComponentIdFilter>>) -> Element {
        // let s0 = self_.clone();
        cx.render(rsx!{div{}})
    }
}


#[inline_props]
pub fn ComponentIdFilterComp(cx: Scope, self_: UseRef<ComponentIdFilter>) -> Element {

    let selected = use_ref(cx, || self_.read().disallowed.clone());

    if *selected.read() != self_.read().disallowed {
        *selected.write() = self_.read().disallowed.clone();
    }

    cx.render(rsx!{
        div {
            class: "",
            h4 {
                "Manual filter",
            },
            LargeBackSelectorComp{
                items: CONSTELLATION.comps.iter().map(|comp| (comp.id, comp.info.name.clone())).collect(),
                selected: self_.read().disallowed.clone(),
                ontoggleitem: move |id: usize| {
                    self_.write().toggle(id);
                },
            }
        }
    })
}


