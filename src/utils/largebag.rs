
use dioxus::prelude::*;
use nucleo_matcher::pattern::*;

#[inline_props] 
pub fn LargeBackSelectorComp<'a, ID> (
    cx: Scope<'a>, 
    items: Vec<(ID, String)>, 
    selected: UseRef<Vec<ID>>,
    ontoggleitem: EventHandler<'a, ID>,
) -> Element<'a> where ID: PartialEq + Clone + core::fmt::Debug + 'static {
    assert!(items.len() > 0);

    let item_order: &UseRef<Vec<usize>> = use_ref(cx, || (0..items.len()).collect());

    let explicitely_expanded_sack = use_state(cx, || false);
    let search_focussed = use_state(cx, || false);

    let search_input = use_state(cx, || "".to_string());

    let order_items = |item_order: &mut Vec<usize>, needle: &str| {
        if needle == "" {
            item_order.sort_unstable_by(|&a, &b|
                items[a].1.cmp(&items[b].1)
            );
        } else {
            let mut matcher = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT);
            let matches: Vec<(String, u32)> = Pattern::parse(needle, CaseMatching::Ignore)
                .match_list(items.iter().map(|(_id, s)| s.clone()).collect::<Vec<String>>(), &mut matcher);
            item_order.sort_unstable_by_key(
                |&i|
                    {
                        let (_id, item_s) = &items[i];
                        match matches.iter().find(|(match_s, score)| item_s == match_s) {
                            Some((_, score)) => {
                                u32::MAX - (score + 1)
                            },
                            None => {u32::MAX}
                        }
                    }
                
            );
        }
    };

    render!{
        div {
            class: "large_bag_selector",
            style: "
                display: flex;
                flex-direction: column;
                justify-content: stretch;
            ",
            div {
                style: "
                    display: grid;
                    grid-template-columns: auto min-content;
                    grid-template-rows: auto;
                    grid-column-gap: 6px;
                    margin-bottom: 6px;
                ",
                div {
                    class: "input_bar",
                    style: "
                        position: relative;
                    ",
                    input {
                        style: "
                            width: 100%;
                            font-size: 18px;
                            border-radius: 12px;
                        ",
                        value: "{search_input.get()}",
                        oninput: move |e| {
                            search_input.set(e.value.clone());
                            order_items(&mut item_order.write(), &e.value.clone());
                        },
                        onfocusin: move |_| {
                            search_focussed.set(true);
                        },
                        onfocusout: move |_| {
                            search_focussed.set(false);
                            search_input.set("".to_string());
                        }
                    },
                    div {
                        class: "search_icon",
                        style: "
                            position: absolute;
                            left: 0;
                            top: 0;
                            bottom: 0;
                            aspect-ratio: 1 / 1;
                        ",
                        img {
                            class: "svg_black",
                            src: "./img/magglass.svg",
                            alt: "search icon",
                            style: "
                                padding: 4px;
                                height: 100%;
                                width: 100%;
                            ",
                        }
                    }
                },
                div {
                    onclick: move |_| {
                        explicitely_expanded_sack.set(!explicitely_expanded_sack.get());
                    },
                    style: "
                        height: 28px;
                        aspect-ratio: 1 / 1;
                    ",
                    class: "collapse-arrow",
                    img {
                        class: "svg_black",
                        style: "
                            padding: 4px;
                            height: 100%;
                            width: 100%;
                            transform: rotate({(*explicitely_expanded_sack.get() as i32) * 180}deg);
                            transition: transform 100ms linear;
                        ",
                        src: "./img/downarrow.svg",
                        alt: "expand/collapse",
                    }
                }
            },
            div {
                class: "items part_collapsable collapsed-{!(*explicitely_expanded_sack.get() || *search_focussed.get())}",
                style: "--mh: 100px;",
                div {
                    for (id, s) in item_order.read().iter().map(|&i| &items[i]) {
                        div {
                            class: "selected-{selected.read().contains(id)}",
                            onclick: move |_| {
                                ontoggleitem.call(id.clone());
                            },
                            "{s}"
                        }
                    }
                }
            }
        }
    }
}