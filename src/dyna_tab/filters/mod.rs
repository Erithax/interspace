
use dioxus::prelude::*;

use crate::dyna_tab::component::*;

pub mod component_type_filter;
pub mod stage_filter;
pub mod owner_filter;
pub mod component_id_filter;


pub trait CompFilter {
    fn filter(&self, comp: &Componentus) -> bool;
    fn render<'a>(cx: Scope<'a>, self_: UseRef<Box<Self>>) -> Element<'a> where Self: Sized;
}

