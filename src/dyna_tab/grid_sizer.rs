

// use wasm_timer::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

const DEBUG_SHOW_MINIMIZED_GRID: bool = false;

const ALL_IND: usize = 0;
const PRE_IND: usize = 1;
const MID_IND: usize = 2;
const POST_IND: usize = 3;

const TRANSITION_STYLE_STR: &'static str = ".sub_stage_back{transition: var(--hover-stage-expand);}";


#[wasm_bindgen]
pub fn size_grid(id: usize) {
    let id_str = format!("grid_sizer_{}", id);
    let stages: [&'static str; 7] = ["lb", "ui", "ly", "pa", "ra", "gx", "pf"];
    let pseudos: [&'static str; 4] = ["all", "pre", "mid", "post"];

    // let execution_timer = Instant::now();

    let docu = web_sys::window().unwrap().document().unwrap();

    let grid_sizer_style_node = docu.get_element_by_id(&id_str).expect("invalid target id in grid sizer");

    let fitting_parent = grid_sizer_style_node.parent_element().unwrap();
    let restrainer_parent = fitting_parent.parent_element().unwrap();

    let mutation_observer_ele = fitting_parent.get_elements_by_class_name("mutation_observer_container").item(0).unwrap();

    // prev style

    grid_sizer_style_node.set_inner_html(".sub_stage_back{border:none !important;}");

    if DEBUG_SHOW_MINIMIZED_GRID {
        return
    }

    let mut stage_wids: [[i32; 4]; 7] = [[0; 4]; 7];

    for (stage_i, stage) in stages.into_iter().enumerate() {
        let mut pseudo_wids: [i32; 4] = [0, 0, 0, 0];
        for (pseudo_i, pseudo) in pseudos.iter().enumerate() {
            let mut pseudo_max_wid = 0;
            let sub_backs: web_sys::HtmlCollection = mutation_observer_ele.get_elements_by_class_name(& format!("ssb-{stage}-{pseudo}"));
            for back_i in 0..sub_backs.length() {
                let back = sub_backs.item(back_i).unwrap();
                let back_wid = back.scroll_width();
                // back_wid += web_sys::window().unwrap().get_computed_style(&back).unwrap().unwrap()
                //     .get_property_value("border-left").unwrap()
                //     .split("px").next().unwrap_or("0")
                //     .parse::<i32>().unwrap_or(0);
                // back_wid += web_sys::window().unwrap().get_computed_style(&back).unwrap().unwrap()
                //     .get_property_value("border-right").unwrap()
                //     .split("px").next().unwrap_or("0")
                //     .parse::<i32>().unwrap_or(0);
                pseudo_max_wid = pseudo_max_wid.max(back_wid);
            }
            pseudo_wids[pseudo_i] = pseudo_max_wid; 
        }

        if pseudo_wids[ALL_IND] < pseudo_wids[PRE_IND] + pseudo_wids[MID_IND] + pseudo_wids[POST_IND] {
            pseudo_wids[ALL_IND] = pseudo_wids[PRE_IND] + pseudo_wids[MID_IND] + pseudo_wids[POST_IND];
        } else {
            pseudo_wids[MID_IND] = pseudo_wids[ALL_IND] - pseudo_wids[PRE_IND] - pseudo_wids[POST_IND];
        }

        // if pseudo_wids[ALL_IND] == 0 {
        //     let all_backs = mutation_observer_ele.get_elements_by_class_name(format!("ssb-{}-all", stage));
        //     for all_back_i in 0..all_backs.length() {
        //         all_backs.item(all_back_i).unwrap().add_event_listener_with_callback(type_, listener);
        //         all_backs.item(all_back_i).unwrap().add_event_listener_with_callback_and_bool(type_, listener, options);
        //         all_backs.item(all_back_i).unwrap().add_event_listener_with_callback_and_bool_and_wants_untrusted(type_, listener, options, wants_untrusted)
        //     }
        // }

        /*
        if (pseudo_wids[0] == 0) {
            console.log("adding hover observers for stage " + stage);
            all_backs = mutation_observer_ele.getElementsByClassName("ssb-" + stage + "-all");
            for (all_back of all_backs) {
                all_back.addEventListener("mouseenter", acquireCollapsedSubStageMouseEnterHandler(fitting_parent, stage));
                all_back.addEventListener("mouseleave", acquireCollapsedSubStageMouseLeaveHandler(fitting_parent, stage));
                acquireCollapsedSubStageMouseLeaveHandler(fitting_parent, stage)(undefined);
            }
        } else {
            fitting_parent.classList.remove("ssb-mouse-off-" + stage);
            fitting_parent.classList.remove("ssb-mouse-on-" + stage);
        }
        */

        stage_wids[stage_i] = pseudo_wids;
    }

    let available_non_scroll_wid = restrainer_parent.scroll_width();
    let minimum_wid = stage_wids.iter().fold(0, |acc, nex| acc + nex[ALL_IND]);

    // 32 is hack to determine collapsed stages
    let non_collapsed_stage_count = stage_wids.iter().fold(0, |acc, nex| {if nex[ALL_IND] > 32 {return acc + 1} else {return acc}});

    if non_collapsed_stage_count == 0 as i32 {
        for stage_pseudo_wids in stage_wids.iter_mut() {
            stage_pseudo_wids[ALL_IND] = available_non_scroll_wid / (stages.len() as i32);
            stage_pseudo_wids[MID_IND] = available_non_scroll_wid / (stages.len() as i32);
        }
    } else if available_non_scroll_wid > minimum_wid {
        let flex_space = available_non_scroll_wid - minimum_wid;
        let remainder = flex_space % non_collapsed_stage_count;
        let stage_extra_space = (flex_space - remainder) / non_collapsed_stage_count;

        for stage_pseudo_wids in stage_wids.iter_mut() {
            if stage_pseudo_wids[ALL_IND] != 0 {
                stage_pseudo_wids[ALL_IND] += stage_extra_space;
                stage_pseudo_wids[MID_IND] += stage_extra_space;
            }
        }
    }

    grid_sizer_style_node.set_inner_html(TRANSITION_STYLE_STR);

    let mut new_style = TRANSITION_STYLE_STR.to_owned();
    for i in 0..stages.len() {
        for j in 0..pseudos.len() {
            new_style += &format!(".ssb-{}-{}{{width:{}px;}}", stages[i], pseudos[j], stage_wids[i][j]);
        }
    }
    grid_sizer_style_node.set_inner_html(&new_style);

    // info!("[RUST] size_grid() in {}ms", execution_timer.elapsed().as_millis());
}