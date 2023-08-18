
use dioxus::prelude::*;

use crate::dyna_tab::component::*;

pub mod component_type_filter;
pub mod stage_filter;

use component_type_filter::*;
use stage_filter::*;


pub trait CompFilter {
    fn filter(&self, comp: &Componentus) -> bool;
    fn render<'a>(cx: Scope<'a>, self_: UseRef<Box<Self>>) -> Element<'a> where Self: Sized;
}

pub struct SelectionFilters {
    pub component_type_filter: ComponentTypeFilter,
}


pub struct InTreeFilters {
    pub comp_type_filter: ComponentTypeFilter,
}


