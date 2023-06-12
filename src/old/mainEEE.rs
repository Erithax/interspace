use dioxus::prelude::*;

pub fn main() {
    dioxus_web::launch_cfg(App, dioxus_web::Config::new().rootname("entry"));
}

pub fn App(cx: Scope) -> Element {
    let a = use_state(cx, || true);
    return cx.render(rsx!(
        div {
            style: "display: flex; flex-direction: column;",
            div{
                onclick: move |_| {a.set(!*a.get());},
                "CLICK",
            },
            div {"{a.get()}"},
            Sub{a: *a.get()},
        }
    ))
}

#[inline_props]
pub fn Sub(cx: Scope, a: bool) -> Element {
    let r = use_ref(cx, || a.clone());
    return cx.render(rsx!(
        div {"{r.with(|r| r.clone())}"}
    ))
}