

use crate::dyna_tab::stage::*;
use crate::dyna_tab::filters::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageFilter {
    pub allowed: Vec<Stage>,    
}

impl StageFilter {
    pub fn toggle(&mut self, stage: Stage) {
        if self.allowed.contains(&stage) {
            self.allowed.retain(|&s| s != stage);
        } else {
            self.allowed.push(stage);
        }
    }
}

impl CompFilter for StageFilter {
    fn filter(&self, comp: &Componentus) -> bool {
        return self.allowed.contains(&Stage::from_comp_typ(comp.typ));
    }

    // #[inline_props]
    fn render(cx: Scope, self_: UseRef<Box<StageFilter>>) -> Element {
        // let s0 = self_.clone();
        cx.render(rsx!{div{}})
    }
}


#[inline_props]
pub fn StageFilterComp(cx: Scope, dynatab_id: usize, self_: UseRef<StageFilter>) -> Element {

    cx.render(rsx!{
        div {
            class: "selector",
            div {
                "Stage Filter",
            },
            div {
                for stage in Stage::iter_reals() {{
                    let class_s = if (*self_.read()).allowed.contains(&stage) {"inactive"} else {"active"};
                    rsx!{
                        div {
                            onclick: move |_| {
                                self_.write().toggle(stage);
                                // size_grid(*dynatab_id);
                            },
                            class: "{class_s} clickabletrue",
                            "{stage:?}",
                        }
                    }
                }}
            }
        }
    })
}


