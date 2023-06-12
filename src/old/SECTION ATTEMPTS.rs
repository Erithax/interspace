pub struct StageSettings {
    /*
        When a stage is focussed, each Block (possibly filtered) of this stage will get it's own area with all it's connections.
        Call one such block 'Bricky'
        Every dependency path containing Bricky gets rendered.
            This includes brickies own paths
                e.g. Bricky -> A -> (B, C)...
                            -> D -> E...
                            -> F -> (G)...
            And paths starting at upstream blocks
                e.g. Z -> Bricky -> A -> B
                     Y -> X -> Bricky -> D -> E
    */
    pub focus_stage: Option<Stage>,
    /*
        When a stage is pinched, the tree is a dependency tree showing what upstream block (indirect) dependencies, and downstream dependents.
        I.e. 
            The dependencies tree of the upstream blocks is no longer visible.
    */
    pub pinch: Option<Stage>,
    /*
        Take situation
            A0 -> B0 -> C0
            A1 -> B0 -> C0
            A0 -> B1 -> C0
            A3 -> B2 -> C1
        With tree representation
            |(A0, A1) -> B0 | -> C0
            | A0      -> B1 | ^
            A3        -> B2   -> C1
        If B is disregarded, the situation would simplify:
            (A0, A1) -> C0
            A3       -> C1
        The direction of this simplification goes from root to leaf, or when pinch is active, from pinched stage out.
        B
    */
    pub disregarded: Vec<Stage>,
    
    /*
        Unlike disregarded this doesn't change the tree structure but collapsed the stage visually
    */
    pub collapsed: Vec<Stage>,
}

pub fn NewestSection(cx: Scope) -> Element {
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding = con.read();

    let curr_stage = Stage::Render;
    let num_col_lines = Stage::last().lvl() + 3;

    cx.render(rsx!(
        section {
            class: "dynatable",
            div {
                class: "content",
                style: "
                    display: grid; 
                    grid-template-columns: repeat({num_col_lines}, auto); 
                    grid-template-rows: repeat({con.read().get_all_of_stage(curr_stage).len()}, auto);
                    border: 1px solid red;
                ",
                for i in (Stage::first().lvl()..=Stage::last().lvl()+2) {
                    div {
                        class: "stage_back",
                        style: "
                            grid-column: {i+1} / {i+2};
                            grid-row: 1 / {con.read().get_all_of_stage(curr_stage).len()+1};
                            border-left: 1px dashed #444;
                        ",
                    }
                },
                for (i, primary_bt) in con.read().get_all_of_stage(curr_stage).into_iter().enumerate() {
                    PrimaryLine{orig_block: *primary_bt, prim_start_grid_line: (i+1) as i32}
                }
            }
        }
    ))
}

#[inline_props]
pub fn PrimaryLine(
    cx: Scope,
    orig_block: BlockType,
    prim_start_grid_line: i32,
) -> Element {
    // let curr_line_dep = use_state(cx, || 0);
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding: std::cell::Ref<Constellation> = con.read();
    let orig_node = _binding.get_node(orig_block);
    
    let up_lines = orig_node.get_up_wids();
    let up_line_count = orig_node.get_up_wid();
    let down_lines = orig_node.get_down_wids();
    let down_line_count = orig_node.get_down_wid();

    let mid_depths = (
        orig_node.splituptree.max_depth_from_id_for_which(orig_node.splituptree.root, &|sb: SingleBlock| sb.data.stage() == orig_block.stage()),
        orig_node.splitdowntree.max_depth_from_id_for_which(orig_node.splitdowntree.root, &|sb: SingleBlock| sb.data.stage() == orig_block.stage()),
    );

    pub fn get_all_of_stage(st: Stage, tree: &SingleBlockTree) -> Vec<SingleBlockId> {
        let mut res = vec![];
        for (id, ch) in tree.items.iter() {
            if ch.data.stage() == st {
                res.push(*id);
            }
        }
        return res
    }

    let up_item_col_span = |stage: Stage| {
        format!("{} / {}", stage.lvl()+1, stage.lvl()+2)
    };
    let down_item_col_span = |stage: Stage| {
        format!("{} / {}", stage.lvl()+1-orig_block.stage().lvl(), stage.lvl()+2-orig_block.stage().lvl())
    };

    let up_item_row_span = format!("repeat({}, auto)", up_line_count);
    let down_item_row_span = format!("repeat({}, auto)", down_line_count);


    cx.render(rsx!(
        div {
            class: "primary-line",
            style: "
                grid-row: {prim_start_grid_line} / span 1;
                grid-column: 1 / -1;

                display: grid; 
                grid-template-columns: subgrid;
                grid-template-rows: auto;
                align-items: center;

                border-bottom: 1px solid purple;
            ",
            div {
                class: "up",
                style: "
                    display: grid;
                    grid-template-columns: subgrid;
                    grid-template-rows: {up_item_row_span};
                    grid-column: 1 / {orig_block.stage().lvl()+2};
                    grid-row: 1 / span 1;
                ",
                for stage in (Stage::first().lvl()..=orig_block.stage().lvl()).map(|n| Stage::from_i(n)) {
                    div {
                        style: "
                            display: grid;
                            grid-row: 1 / -1;
                            grid-column: {up_item_col_span(stage)};
                            grid-template-columns: auto;
                            grid-template-rows: subgrid;
                        ",
                        for id in get_all_of_stage(stage, &orig_node.splituptree).into_iter().filter(|id| *id != orig_node.splituptree.root) {
                            BranchWithinStage{
                                stage: stage,
                                orig_block: *orig_block,
                                curr_id: id,
                                dir: BlockDirection::Up,
                                row_range: up_lines.get(&id).unwrap().to_owned(),
                            }
                        }
                    }
                }
            },
            div {
                style: "
                    display: flex;
                    grid-row: 1 / span 1;
                    grid-column: {orig_block.stage().lvl()+2} / span 1;
                    align-items: center;
                    justify-content: center;
                ",
                RepName{
                    bt: *orig_block,
                    name: orig_node.info.name.clone(),
                    owner: orig_node.info.owner.to_string(),
                },
            },
            div {
                class: "down",
                style: "
                    display: grid;
                    grid-template-columns: subgrid;
                    grid-template-rows: {down_item_row_span};
                    grid-column: {orig_block.stage().lvl()+3} / -1;
                    grid-row: 1 / span 1;
                ",
                for stage in (orig_block.stage().lvl()..=Stage::last().lvl()).map(|n| Stage::from_i(n)) {
                    div {
                        style: "
                            display: grid;
                            grid-column: {down_item_col_span(stage)};
                            grid-row: 1 / -1;
                            grid-template-columns: auto;
                            grid-template-rows: subgrid;
                        ",
                        for id in get_all_of_stage(stage, &orig_node.splitdowntree).into_iter().filter(|id| *id != orig_node.splitdowntree.root) {
                            BranchWithinStage{
                                stage: stage,
                                orig_block: *orig_block,
                                curr_id: id,
                                dir: BlockDirection::Down,
                                row_range: down_lines.get(&id).unwrap().to_owned(),
                            }
                        }
                    }
                }
            }
        }
    ))
}


#[inline_props]
pub fn BranchWithinStage(
    cx: Scope,
    stage: Stage,
    orig_block: BlockType,
    curr_id: SingleBlockId,
    dir: BlockDirection,
    row_range: (usize, usize),
) -> Element {

    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding: std::cell::Ref<Constellation> = con.read();
    let orig_node = con.read().get_node(orig_block);

    let tree;
    if *dir == BlockDirection::Up {
        tree = &_binding.get_node(orig_block).splituptree;
    } else {
        tree = &_binding.get_node(orig_block).splitdowntree;
    }

    let row_span = format!("{} / {}", row_range.0+1, row_range.1+1);
    let col_span = {
        let mut curr_id = *curr_id;
        let mut depth = 0;
        while tree.get(curr_id).data.stage() == *stage && curr_id != tree.root {
            depth += 1;
            curr_id = tree.get(curr_id).parent;
        }
        if *dir == BlockDirection::Up {
            format!("{} / {}", -depth, -(depth+1))
        } else {
            format!("{} / {}", depth, depth + 1)
        }
    };
    
    cx.render(rsx!(
        div {
            class: "boxio",
            style: "
                grid-column: {col_span};
                grid-row: {row_span};
                display: flex;
                align-items: center;
                justify-content: center;
            ",
            RepName{
                bt: tree.get(*curr_id).data,
                name: con.read().get_node(&tree.get(*curr_id).data).info.name.clone(),
                owner: con.read().get_node(&tree.get(*curr_id).data).info.owner.to_string(),
            }
        }
    ))
}


pub fn NewSection(cx: Scope) -> Element {
    
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding = con.read();
    let uis = _binding.get_all_of_bt(&BlockType::Ui(Ui::META));
    let collapsed = use_state(cx, || false);


    let shown_stages = use_state(cx, || HashMap::<Stage, bool>::new());
    let curr_stage = use_state(cx, || Stage::Render); 

    let select_stage = move |_, st: Stage| {
        if st != *curr_stage.get() {
            curr_stage.set(st);
        }
    };

    let settings_coll = use_state(cx, || CollapsableToggle::Collapsed);
    let content_coll = use_state(cx, || CollapsableToggle::Expanded);

    cx.render(rsx!{
        section {
            class: "dynatable",
            div {
                class: "sectionheader",
                onclick: move |e| {
                    if *collapsed.get() {
                        content_coll.set(CollapsableToggle::Expanded);
                    } else {
                        content_coll.set(CollapsableToggle::Collapsed);
                        settings_coll.set(CollapsableToggle::Collapsed)
                    }
                    collapsed.set(!collapsed.get());
                },
                div {
                    class: "title",
                    "Section Title"
                },
                button {
                    class: "settings",
                    onclick: move |e| {
                        settings_coll.set(settings_coll.toggle());
                        e.stop_propagation();
                    },
                    img {
                        src: "/img/gear.svg",
                        alt: "section settings",
                    }
                }
            },
            div{class:"collapsable {settings_coll}", div {
                class: "sectionsettings",
                div {
                    onclick: move |e| {
                        // shown_stages.with_mut(|s| s.set(Stage::Langbridge, !s.get(Stage::Langbridge)));
                    },
                    "StageLB"
                },
                div {
                    class: "stageselector",
                    div {
                        "Select Stage"
                    },
                    div {
                        for i in 0..=Stage::last().lvl() {
                            div {
                                onclick: move |e| {
                                    select_stage(e, Stage::from_i(i));
                                },
                                class: "pinched{curr_stage.lvl() == i}",
                                Stage::from_i(i).to_string()
                            }
                        }
                    }
                }
            }},
            div{class:"collapsable {content_coll}", div {
                class: "sectioncontent", //TODO TODO TODO remove "flipped"
                div {
                    class: "background",
                    for i in 0..curr_stage.get().lvl() {div {} }
                    div {class: "pinched pre"},
                    div {class: "pinched mid"},
                    div {class: "pinched pos"},
                    for i in curr_stage.get().lvl()+1..=Stage::last().lvl() {div {}}
                },
                div {
                    class: "headers",
                    for i in 0..curr_stage.get().lvl() {
                        div {Stage::from_i(i).to_string()}
                    }
                    div {
                        class: "pinched",
                        curr_stage.get().to_string()
                    },
                    for i in curr_stage.get().lvl()+1..=Stage::last().lvl() {
                        div {Stage::from_i(i).to_string()}
                    }
                },
                div {
                    class: "rows",
                    for primary_bt in con.read().get_all_of_stage(*curr_stage.get()) {
                        div {
                            class: "row",
                            div {
                                class: "up",
                                for (id, bts) in con.read().get_node(primary_bt).get_up_nei_stage_blocks() {
                                    div {
                                        class: "subbranch",
                                        div {
                                            class: "stageblock",
                                            for bt in bts.iter() {
                                                RepName{
                                                    bt: *bt,
                                                    name: con.read().get_node(&bt).info.name.clone(),
                                                    owner: con.read().get_node(&bt).info.owner.to_string(),
                                                }
                                            }
                                        },
                                        div {
                                            class: "others",
                                            for up_id in con.read().get_node(primary_bt).get_up_nei_sb_ids_of(id) {
                                                SubBranch{
                                                    orig_bt: *primary_bt,
                                                    dir: "back".to_string(),
                                                    stage_block_id: *up_id,
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            div {
                                class: "mid",
                                RepName{
                                    bt: *primary_bt,
                                    name: con.read().get_node(&primary_bt).info.name.clone(),
                                    owner: con.read().get_node(&primary_bt).info.owner.to_string(),
                                }
                            },
                            div {
                                class: "down",
                                for (id, bts) in con.read().get_node(primary_bt).get_down_nei_stage_blocks() {
                                    div {
                                        class: "subbranch",
                                        div {
                                            class: "stageblock",
                                            for bt in bts.iter() {
                                                RepName{
                                                    bt: *bt,
                                                    name: con.read().get_node(&bt).info.name.clone(),
                                                    owner: con.read().get_node(&bt).info.owner.to_string(),
                                                }
                                            }
                                        },
                                        div {
                                            class: "others",
                                            for up_id in con.read().get_node(primary_bt).get_down_nei_sb_ids_of(id) {
                                                SubBranch{
                                                    orig_bt: *primary_bt,
                                                    dir: "forw".to_string(),
                                                    stage_block_id: *up_id,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }}
        }
    })
}

#[inline_props]
pub fn SubBranch(
    cx: Scope,
    orig_bt: BlockType,
    dir: String,
    stage_block_id: StageBlockId,
) -> Element {
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding: std::cell::Ref<Constellation> = con.read();
    let node = _binding.get_node(&orig_bt);
    cx.render(rsx!{
        div {
            class: "subbranch",
            div {
                class: "stageblock",
                if dir == "back" {
                    rsx!(for bt in node.blockuptree.get_owned_by_id(stage_block_id).unwrap().data {
                        RepName{
                            bt: bt,
                            name: con.read().get_node(&bt).info.name.clone(),
                            owner: con.read().get_node(&bt).info.owner.to_string(),
                        }
                    })
                } else {
                    rsx!(for bt in node.blockdowntree.get_owned_by_id(stage_block_id).expect(&format!("could not find {stage_block_id} in node {node:?}")).data {
                        RepName{
                            bt: bt,
                            name: con.read().get_node(&bt).info.name.clone(),
                            owner: con.read().get_node(&bt).info.owner.to_string(),
                        }
                    })
                }
                
            },
            div {
                class: "others",
                rsx!(
                    if dir == "back" {
                        rsx!(for id in node.get_up_nei_sb_ids_of(*stage_block_id) {
                            SubBranch{
                                orig_bt: *orig_bt,
                                dir: dir.clone(),
                                stage_block_id: *id,
                            }
                        })
                    } else if dir =="forw" {
                        rsx!(for id in node.get_down_nei_sb_ids_of(*stage_block_id) {
                            SubBranch{
                                orig_bt: *orig_bt,
                                dir: dir.clone(),
                                stage_block_id: *id,
                            }
                        })
                    }
                )
            }
           
        }
    })
}



pub fn NewerSection(cx: Scope) -> Element {
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding = con.read();
    let uis = _binding.get_all_of_bt(&BlockType::Ui(Ui::META));
    let collapsed = use_state(cx, || false);


    let shown_stages = use_state(cx, || HashMap::<Stage, bool>::new());
    let curr_stage = use_state(cx, || Stage::Render); 

    let select_stage = move |_, st: Stage| {
        if st != *curr_stage.get() {
            curr_stage.set(st);
        }
    };

    let settings_coll = use_state(cx, || CollapsableToggle::Collapsed);
    let content_coll = use_state(cx, || CollapsableToggle::Expanded);

    let mut grid_temp_cols = String::new();
    // for i in 0..Stage::last().lvl() {
    //     grid_temp_cols += &format!(" auto [{}]", Stage::from_i(i).to_string().as_str());
    // }
    for i in 0..Stage::last().lvl()+1 {
        grid_temp_cols += &format!(" minmax(100px, auto)");
    }


    cx.render(rsx!{
        section {
            class: "dynatable",
            div {
                class: "sectionheader",
                onclick: move |e| {
                    if *collapsed.get() {
                        content_coll.set(CollapsableToggle::Expanded);
                    } else {
                        content_coll.set(CollapsableToggle::Collapsed);
                        settings_coll.set(CollapsableToggle::Collapsed)
                    }
                    collapsed.set(!collapsed.get());
                },
                div {
                    class: "title",
                    "Section Title"
                },
                button {
                    class: "settings",
                    onclick: move |e| {
                        settings_coll.set(settings_coll.toggle());
                        e.stop_propagation();
                    },
                    img {
                        src: "/img/gear.svg",
                        alt: "section settings",
                    }
                }
            },
            div{class:"collapsable {settings_coll}", div {
                class: "sectionsettings",
                div {
                    onclick: move |e| {
                        // shown_stages.with_mut(|s| s.set(Stage::Langbridge, !s.get(Stage::Langbridge)));
                    },
                    "StageLB"
                },
                div {
                    class: "stageselector",
                    div {
                        "Select Stage"
                    },
                    div {
                        for i in 0..=Stage::last().lvl() {
                            div {
                                onclick: move |e| {
                                    select_stage(e, Stage::from_i(i));
                                },
                                class: "pinched{curr_stage.lvl() == i}",
                                Stage::from_i(i).to_string()
                            }
                        }
                    }
                }
            }},
            div {class: "collapsable {content_coll}", div {
                class: "griddy",
                style: "display: grid; grid-template-columns: {grid_temp_cols}; grid-template-rows: auto;",
                for (i, col) in (0..=Stage::last().lvl()).map(|i| (format!("{} / {}",i+1, i+2), format!("rgb({}, 0, {})", i*10, 200-i*10))) {
                    div {
                        class: "stage",
                        style: "place-self: stretch; grid-column: {i}; border: 1px solid yellow; grid-row: 1 / -1; background-color: {col};",
                    }
                }
                div {
                    class: "primary-items",
                    style: "display: grid; grid-column: 1 / -1; grid-row: 1 / -1; grid-template-columns: subgrid; grid-template-rows: repeat({con.read().get_all_of_stage(*curr_stage.get()).len()}, auto); row-gap: 10px;",
                    for (primary_i, primary_bt) in con.read().get_all_of_stage(*curr_stage.get()).iter().enumerate() {
                        div {
                            class: "primary",
                            style: "display: grid; grid-template-columns: subgrid; grid-template-rows: auto; grid-column: 1 / -1; grid-row: {primary_i+1} / span 1; border-bottom: 2px solid purple; gap: 0px;",
                            NewerSubBranch{
                                orig_bt: **primary_bt, 
                                dir: BlockDirection::Up, 
                                parent_id: con.read().get_node(primary_bt).splituptree.get_root(),
                                parent_stage: primary_bt.stage(),
                            },
                            // for i in (0..1).map(|_| primary_bt.stage().lvl() + 1) {
                            div {
                                class: "mid",
                                style: "display: inline; grid-column: {primary_bt.stage().lvl()+1} / span 1; grid-row: 1 / span 1;",
                                RepName{
                                    bt: **primary_bt,
                                    name: con.read().get_node(primary_bt).info.name.clone(),
                                    owner: con.read().get_node(primary_bt).info.owner.to_string(),
                                },
                            },
                            NewerSubBranch{
                                orig_bt: **primary_bt, 
                                dir: BlockDirection::Down, 
                                parent_id: con.read().get_node(primary_bt).splitdowntree.get_root(),
                                parent_stage: primary_bt.stage(),
                            },
                        }
                    }
                }
            }}
        }
    })
}


#[inline_props]
pub fn NewerSubBranch(
    cx: Scope,
    orig_bt: BlockType,
    dir: BlockDirection,
    parent_id: SingleBlockId,
    parent_stage: Stage,
) -> Element {
    let con = use_shared_state::<Constellation>(cx).unwrap();
    let _binding: std::cell::Ref<Constellation> = con.read();
    let orig_node = _binding.get_node(&orig_bt);
    
    let grid_line_closure = |mut bt_stage: Stage, parent_stage: Stage| {
        if bt_stage == BlockType::TODO.stage() {
            bt_stage = if *dir == BlockDirection::Up {parent_stage.prev()} else {parent_stage.next()};
        }
        match dir {
            BlockDirection::Up => {
                return format!("grid-column: 1 / {};", bt_stage.lvl()+2)
            },
            BlockDirection::Down => {
                return format!("grid-row: {} / {}", bt_stage.lvl()+1, Stage::last().lvl()+1)
            }
        }
    };
    let limit_grid_line_closure = || {
        match dir {
            BlockDirection::Up => {
                return format!("grid-column: 1 / {};", parent_stage.lvl()+2)
            },
            BlockDirection::Down => {
                return format!("grid-column: {} / -1", parent_stage.lvl()+2)
            }
        }
    };

    let decide_stage = |bt: BlockType| {
        match dir {
            BlockDirection::Up => {
                return format!("grid-column: {} / span 1", bt.stage().lvl()+1)
            },
            BlockDirection::Down => {
                return format!("grid-column: {} / span 1", bt.stage().lvl())
            }
        }
    };

    let (
        tree, 
        blockerinos,
    ) = match dir {
        BlockDirection::Up => {
            (
                &orig_node.splituptree, 
                orig_node.get_up_nei_single_blocks_of(*parent_id),
            )
        },
        BlockDirection::Down => {
            (
                &orig_node.splitdowntree, 
                orig_node.get_down_nei_single_blocks_of(*parent_id),
            )
        }
    };

    let align_block = || {
        // will change when vertical-horizontal table direction flipping is implemented
        match dir {
            BlockDirection::Up => {
                return "justify-self: end; align-self: stretch;"
            },
            BlockDirection::Down => {
                return "justify-self: start; align-self: stretch;"
            }
        }
    };

    let align_subbranches = || {
        match dir {
            BlockDirection::Up => {
                return "justify-self: start; align-self: stretch;"
            },
            BlockDirection::Down => {
                return "justify-self: end; align-self: stretch;"
            }
        }
    };

    

    cx.render(rsx!{
        div {
            class: "grid_span",
            style: "
                display: grid;
                grid-template-columns: subgrid; 
                grid-template-rows: auto;
                border: 1px solid cyan; 
                grid-row: 1 / span 1; 
                {limit_grid_line_closure()}
                {align_subbranches()}
            ",
            for (id, bt) in blockerinos.iter() {
                div {
                    class: "subbie",
                    style: "display: grid; grid-template-columns: subgrid; {grid_line_closure(bt.stage(), *parent_stage)};",
                    div {
                        style: "
                            {decide_stage(*bt)}; 
                            {align_block()};
                            grid-row: 1 / span 1;
                        ",
                        RepName{
                            bt: *bt,
                            name: con.read().get_node(&bt).info.name.clone(),
                            owner: con.read().get_node(&bt).info.owner.to_string(),
                        }
                    },
                    NewerSubBranch{
                        orig_bt: *orig_bt,
                        dir: dir.clone(),
                        parent_id: *id,
                        parent_stage: bt.stage(),
                    }
                }
            }
        }
    })
}

