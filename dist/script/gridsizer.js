
var document = window.document;


const mutation_observer_config = { attributes: true, childList: true, subtree: true };
const stages = ["lb", "ui", "ly", "pa", "ra", "gx", "pf"];
const pseudos = ["all", "pre", "mid", "post"];
const transition_str = ".sub_stage_back{transition: width 500ms linear;}";

var targets = new Set();


var sizeGrid = function(targ_id) {
    console.log("sizing grid");
    var startTime = performance.now()
    const targ_node = document.getElementById(targ_id);
    grid_sizer_style = targ_node;
    fitting_parent = grid_sizer_style.parentElement;
    restrainer_parent = fitting_parent.parentElement;

    prev_style = grid_sizer_style.innerHTML;
    grid_sizer_style.innerHTML = "";
   
    stage_widths = []; 
    for (stage of stages) {
        let pseudo_wids = [];
        for (pseudo of pseudos) {
            pseudo_backs = fitting_parent.getElementsByClassName("ssb-" + stage + "-" + pseudo);
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

        stage_widths.push(pseudo_wids);
    }

    available_non_scroll_wid = restrainer_parent.scrollWidth;
    minimum_wid = stage_widths.reduce((acc, nex) => acc + nex[0], 0);

    if (available_non_scroll_wid > minimum_wid) {
        let extra_space = available_non_scroll_wid - minimum_wid;
        let remainder = extra_space % stages.length;
        let stage_extra_space = (extra_space - remainder) / stages.length;

        for (stage_pseudo_wids of stage_widths) {
            stage_pseudo_wids[0] += stage_extra_space;
            stage_pseudo_wids[2] += stage_extra_space;
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
};

function createGridSizerCallback(id) {
    return function(mutations) {
        sizeGrid(id);
    }
}

function addObserver(id) {
    var observer = new MutationObserver(createGridSizerCallback(id));
    observer.observe(document.getElementById(id).parentElement.lastChild, mutation_observer_config);
    var resize_observer = new ResizeObserver(createGridSizerCallback(id));
    resize_observer.observe(document.getElementById(id).parentElement.parentElement);
}


function updateGridSizerList() {
    console.log("ye?");
    let grid_sizers = document.getElementsByClassName("grid_sizer_anchor");
    for (grid_sizer_anchor of grid_sizers) {
        if (!targets.has(grid_sizer_anchor.id)) {
            sizeGrid(grid_sizer_anchor.id);
            addObserver(grid_sizer_anchor.id);
            targets.add(grid_sizer_anchor.id);
        }
    } 
}


