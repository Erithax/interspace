
use dioxus::prelude::*;

use crate::bowl::component::*;

pub mod component_type_filter;


pub trait CompFilter {
    fn filter(&self, comp: &Componentus) -> bool;
    fn render<'a>(cx: Scope<'a>, self_: UseRef<Box<Self>>) -> Element<'a> where Self: Sized;
}
