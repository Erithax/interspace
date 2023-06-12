#[macro_export]
macro_rules! GenerateNamedDivComp {
    ($comp_name:ident, $name:literal) => {
        #[inline_props]
        pub fn $comp_name(cx: Scope, col: String) -> Element {
            cx.render(rsx!{
                div {
                    style: "background-color: {col};",
                    $name
                }
            })
        }
    }
}
fn main() {
    dioxus_web::launch(App);
}
fn App(cx: Scope) -> Element {
    GenerateNamedDivComp!(BonkComp, "bonk");
    cx.render(rsx!{
        BonkComp{col: "red".to_string()}
        BonkComp{col: "blue".to_string()}
    })
}