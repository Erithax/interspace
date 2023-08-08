
use crate::dyna_tab::component::*;
use crate::dyna_tab::filters::*;


pub struct ComponentTypeFilter {
    pub allowed: Vec<ComponentType>,    
}

impl ComponentTypeFilter {
    pub fn toggle(&mut self, comp_type: ComponentType) {
        if self.allowed.contains(&comp_type) {
            self.allowed.retain(|&ct| ct != comp_type);
        } else {
            self.allowed.push(comp_type);
        }
    }
}

impl CompFilter for ComponentTypeFilter {
    fn filter(&self, comp: &Componentus) -> bool {
        return self.allowed.contains(&comp.typ);
    }

    // #[inline_props]
    fn render(cx: Scope, self_: UseRef<Box<ComponentTypeFilter>>) -> Element {
        // let s0 = self_.clone();
        cx.render(rsx!{div{}})
    }
}


#[inline_props]
pub fn ComponentTypeFilterComp(cx: Scope, self_: UseRef<Box<ComponentTypeFilter>>) -> Element {
    cx.render(rsx!{
        div {
            class: "selector",
            div {
                "Component Type Filter",
            },
            div {
                for comp_type in ComponentType::iterator() {{
                    let class_s = if (*self_.read()).allowed.contains(&comp_type) {"active"} else {"inactive"};
                    rsx!{
                        div {
                            onclick: move |_| {
                                self_.write().toggle(comp_type);
                            },
                            class: "{class_s} clickabletrue",
                            "{comp_type:?}",
                        }
                    }
                }}
            }
        }
    })
}


// pub trait IntFilterComp {
//     fn new(cx: Scope) -> UseRef<Box<dyn IntFilterComp>>;
//     fn filter_i(self_: UseRef<Box<dyn IntFilterComp>>, i: i32) -> bool;
//     fn render(self_: UseRef<Box<dyn IntFilterComp>>, cx: Scope) -> Element;
// }

// #[derive(Debug, Copy, Clone)]
// pub enum PosNegFilter {
//     StrictNeg,
//     Neg,
//     Pos,
//     StrictPos,
// }

// impl IntFilterComp for PosNegFilter {
//     fn new(cx: Scope) -> UseRef<Box<dyn IntFilterComp>> where Self: Sized {
//         let temp: Box<dyn IntFilterComp> = Box::new(PosNegFilter::Pos);
//         return use_ref(cx, || 
//             temp
//         ).clone()
//     }

//     fn filter_i(self_: UseRef<Box<Self>>, i: i32) -> bool {
//         match **self_.read() {
//             Self::Pos => {return i >= 0},
//             Self::StrictPos => {return i > 0},
//             Self::Neg => {return i <= 0},
//             Self::StrictNeg => {return i < 0},
//         }
//     }

//     fn render(self_: UseRef<Box<Self>>, cx: Scope) -> Element {
//         let s1 = self_.clone();
//         let s2 = self_.clone();
//         let s3 = self_.clone();
//         cx.render(rsx!{
//             div {
//                 onclick: move |_| {s1.with_mut(|s| *s = Box::new(Self::StrictNeg));},
//                 "strictneg",
//             },
//             div {
//                 onclick: move |_| {s2.clone().with_mut(|s| *s = Box::new(Self::Neg));},
//                 "neg",
//             },
//             div {
//                 onclick: move |_| {s3.clone().with_mut(|s| *s = Box::new(Self::Pos));},
//                 "pos",
//             },
//             div {
//                 onclick: move |_| {self_.clone().with_mut(|s| *s = Box::new(Self::StrictPos));},
//                 "strictpos",
//             },
//         })
//     }
// }





// pub struct ModulusFilter {
//     pub moduli: Vec<i32>,
// }

// impl IntFilterComp for ModulusFilter {
//     fn new(cx: Scope) -> UseRef<Box<dyn IntFilterComp>> {
//         let temp: Box<dyn IntFilterComp> = Box::new(
//             ModulusFilter {
//                 moduli: vec![5, 6],
//             }
//         );
//         return use_ref(cx, || temp).clone()
//     }

//     fn filter_i(self_: UseRef<Box<Self>>, i: i32) -> bool {
//         return self_.read().moduli.iter().any(|m| i % m == 0)
//     }

//     fn render(self_: UseRef<Box<Self>>, cx: Scope) -> Element {
//         cx.render(rsx!{
//             for pot_m in 0..12 {
//                 if self_.read().moduli.contains(&pot_m) {
//                     rsx!{div {
//                         style: "background: blue; color: white;",
//                         "% {pot_m}",
//                     }}
//                 } else {
//                     rsx!{div {
//                         style: "background: none; color: black;"
//                     }}
//                 }
//             }
//         })
//     }
// }

// pub fn get_random() -> i32 {
//     let nanos = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .subsec_nanos();
//     return nanos as i32 - 500
// }

// pub fn Interino(cx: Scope) -> Element {

//     let dataset = (0..100).map(|i| get_random()).collect();

//     let filters: Vec<UseRef<Box<dyn IntFilterComp>>> = vec![
//         PosNegFilter::new(cx),
//         ModulusFilter::new(cx),
//     ];


//     cx.render(rsx!{
//         for filter in filters {
//             IntFilterComp::render(filter, &cx)
//         }
//     })
// }