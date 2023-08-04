
var document = window.document;


// Select the node that will be observed for mutations
var targetNodes = document.getElementsByClassName('dynatab');

// Options for the observer (which mutations to observe)
var config = { attributes: true, childList: true, subtree: true };

var sizeGrid = function(mutationsList) {
    console.log("sizing grid");
    var startTime = performance.now()
    grid_sizer_style = targetNodes[0].firstChild;
    grid_sizer_style.innerHTML = "";
   
    col_str = "";
    for (stage of ["lb", "ui", "ly", "pa", "ra", "gx", "pf"]) {
        all_min_wid = 0;
        all_backs = document.getElementsByClassName("sub_stage_back-" + stage + "-all");

        for (back of all_backs) {
            let back_wid = back.scrollWidth;
            back_wid += parseFloat(window.getComputedStyle(back).borderLeftWidth.slice(0, -2));
            back_wid += parseFloat(window.getComputedStyle(back).borderRightWidth.slice(0, -2));
            all_min_wid = Math.max(all_min_wid, back_wid);
        }
        all_min_wid = Math.ceil(all_min_wid);

        pseudo_min_wids = [];
        for (pseudo of ["pre", "mid", "post"]) {
            pseudo_backs = document.getElementsByClassName("sub_stage_back-" + stage + "-" + pseudo);
            let pseudo_min_wid = 0;
            for (back of pseudo_backs) {
                let back_wid = back.scrollWidth;
                back_wid += parseFloat(window.getComputedStyle(back).borderLeftWidth.slice(0, -2));
                back_wid += parseFloat(window.getComputedStyle(back).borderRightWidth.slice(0, -2));
                pseudo_min_wid = Math.max(pseudo_min_wid, back_wid);
            }
            pseudo_min_wids.push(pseudo_min_wid);
        }
        pseudo_min_wids.map((a) => Math.ceil(a));

        
        // all_min_wid += 20;
        // pseudo_min_wids.map((a) => a + 20);

        if (all_min_wid < pseudo_min_wids.reduce((a, b) => a + b, 0)) {
            all_min_wid = pseudo_min_wids.reduce((a, b) => a + b, 0);
        } else {
            pseudo_min_wids[1] = all_min_wid - pseudo_min_wids[0] - pseudo_min_wids[2]
        }

        col_str += ".sub_stage_back-" + stage + "-all{min-width:" + all_min_wid + "px;} "
        col_str += ".sub_stage_back-" + stage + "-pre{min-width:" + pseudo_min_wids[0] + "px;} "
        col_str += ".sub_stage_back-" + stage + "-mid{min-width:" + pseudo_min_wids[1] + "px;} "
        col_str += ".sub_stage_back-" + stage + "-post{min-width:" + pseudo_min_wids[2] + "px;} "

    }
    grid_sizer_style.innerHTML = col_str;
    
    var endTime = performance.now()
    console.log(`gridsizing took ${endTime - startTime} milliseconds`)
};

var attempts = 0;
var max_attempts = 20;

function waitAndCreateMutationObserver() {       
  setTimeout(function() { 
    if (targetNodes.length > 0)  {
        console.log("creating MutationObserver");
        var observer = new MutationObserver(sizeGrid);
        observer.observe(targetNodes.item(0).lastChild, config);
        sizeGrid();
    } else if (attempts < max_attempts) {
        console.log("waiting..."); 
        attempts += 1;
        waitAndCreateMutationObserver();
    }                                
  }, 100)
}
waitAndCreateMutationObserver() 

