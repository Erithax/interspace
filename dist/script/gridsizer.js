

// import {size_grid} from '../assets/dioxus/ui_overview.js';

// import init from "/./assets/dioxus/ui_overview.js";


// init("/./assets/dioxus/ui_overview_bg.wasm").then(wasm => {
//   if (wasm.__wbindgen_start == undefined) {
//     // wasm.main();
//     console.log("YAAAAAAAAAAAAAAAAAAAAAAA");
//     size_grid(8);
//     console.log("YOOOOOOOOOOOOOOOOOOOOOOOOO");
//   }
// });


// size_grid(8)



var document = window.document;

const DEBUG_SHOW_COLLAPSED = true;

var is_resizing = false;
var needs_update = false;

const mutation_observer_config = { attributes: true, childList: true, subtree: true };
const stages = ["lb", "ui", "ly", "pa", "ra", "gx", "pf"];
const pseudos = ["all", "pre", "mid", "post"];
const transition_str = ".sub_stage_back{transition: var(--hover-stage-expand);}";

var targets = new Set();

function acquireCollapsedSubStageMouseEnterHandler(to_style_ele, stage_name) {
    return function(event) {
        // console.log(event);
        to_style_ele.classList.add("ssb-mouse-on-" + stage_name);
        to_style_ele.classList.remove("ssb-mouse-off-" + stage_name)
    }
}

function acquireCollapsedSubStageMouseLeaveHandler(to_style_ele, stage_name) {
    return function(event) {
        // console.log(event);
        to_style_ele.classList.add("ssb-mouse-off-" + stage_name);
        to_style_ele.classList.remove("ssb-mouse-on-" + stage_name)
    }
}


var sizeGrid = function(targ_id) {
    if (is_resizing) {needs_update = true; return};
    is_resizing = true;
    needs_update = false;

    console.log("sizing grid");
    var startTime = performance.now()
    const targ_node = document.getElementById(targ_id);
    grid_sizer_style = targ_node;
    fitting_parent = grid_sizer_style.parentElement;
    restrainer_parent = fitting_parent.parentElement;
    mutation_observer_ele = fitting_parent.getElementsByClassName("mutation_observer_container")[0];

    prev_style = grid_sizer_style.innerHTML;
    grid_sizer_style.innerHTML = ".sub_stage_back{border:none !important;min-width:0px !important;}";

    if (DEBUG_SHOW_COLLAPSED) {return}
   
    stage_widths = []; 
    for (stage of stages) {
        let pseudo_wids = [];
        for (pseudo of pseudos) {
            pseudo_backs = mutation_observer_ele.getElementsByClassName("ssb-" + stage + "-" + pseudo);
            let pseudo_max_wid = 0;
            for (back of pseudo_backs) {
                let back_wid = back.scrollWidth;
                back_wid += parseFloat(window.getComputedStyle(back).borderLeftWidth.slice(0, -2));
                back_wid += parseFloat(window.getComputedStyle(back).borderRightWidth.slice(0, -2));
                pseudo_max_wid = Math.max(pseudo_max_wid, back_wid);
            }
            pseudo_wids.push(Math.ceil(pseudo_max_wid));
        }

        if (pseudo_wids[0] < pseudo_wids[1] + pseudo_wids[2] + pseudo_wids[3]) {
            pseudo_wids[0] = pseudo_wids[1] + pseudo_wids[2] + pseudo_wids[3];
        } else {
            pseudo_wids[2] = pseudo_wids[0] - pseudo_wids[1] - pseudo_wids[3];
        }

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

        stage_widths.push(pseudo_wids);
    }

    available_non_scroll_wid = restrainer_parent.scrollWidth;
    minimum_wid = stage_widths.reduce((acc, nex) => acc + nex[0], 0);

    let non_collapsed_stage_count = stage_widths.reduce((acc, nex) => {if (nex[0] != 0) {return acc + 1} else {return acc}}, 0);
    // console.log(non_collapsed_stage_count);

    // console.log(stage_widths[2][0]);
    // console.log(stage_widths[2][1]);
    // console.log(stage_widths[2][2]);
    // console.log(stage_widths[2][3]);

    if (available_non_scroll_wid > minimum_wid) {
        let flex_space = available_non_scroll_wid - minimum_wid;
        let remainder = flex_space % non_collapsed_stage_count;
        let stage_extra_space = (flex_space - remainder) / non_collapsed_stage_count;

        for (stage_pseudo_wids of stage_widths) {
            if (stage_pseudo_wids[0] != 0) {
                stage_pseudo_wids[0] += stage_extra_space;
                stage_pseudo_wids[2] += stage_extra_space;
            }
        }
    }
    
    grid_sizer_style.innerHTML = transition_str;
    let i = 0;
    while (i < stages.length) {
        let j = 0;
        while (j < pseudos.length) {
            grid_sizer_style.innerHTML += ".ssb-" + stages[i] + "-" + pseudos[j] + "{width:" + stage_widths[i][j] + "px;}";
            j += 1;
        }
        i += 1;
    }

    var endTime = performance.now()
    console.log(`gridsizing took ${endTime - startTime} milliseconds`)

    is_resizing = false;
    if (needs_update) {sizeGrid(targ_id);}
};

function acquireGridSizerCallback(id) {
    return function(mutations) {
        sizeGrid(id);
    }
}

function addObservers(id) {
    var observer = new MutationObserver(acquireGridSizerCallback(id));
    observer.observe(document.getElementById(id).parentElement.lastChild, mutation_observer_config);
    var resize_observer = new ResizeObserver(acquireGridSizerCallback(id));
    resize_observer.observe(document.getElementById(id).parentElement.parentElement);
}


// function updateGridSizerList() {
//     console.log("ye?");
//     let grid_sizers = document.getElementsByClassName("grid_sizer_anchor");
//     for (grid_sizer_anchor of grid_sizers) {
//         if (!targets.has(grid_sizer_anchor.id)) {
//             sizeGrid(grid_sizer_anchor.id);
//             addObservers(grid_sizer_anchor.id);
//             targets.add(grid_sizer_anchor.id);
//         }
//     } 
// }

function acquireMutationObserverCallback(id) {
    return function(mutations) {
        document.getElementById("resize_observer_ele_" + id).click();
    }
}

function addMutationObserver(id) {
    var mut_observer = new MutationObserver(acquireMutationObserverCallback(id));
    mut_observer.observe(document.getElementById("mutation_observer_container_" + id));
}

window.onresize = function() {
    for (var ele of document.getElementsByClassName("resize_observer_ele") ) {
        ele.click();
    }
}

