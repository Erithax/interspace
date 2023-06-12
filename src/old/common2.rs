use std::collections::HashSet;
use std::collections::HashMap;


use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use crate::owners::Owner;
use crate::langs::Lang;

#[derive(Debug)]
pub struct BaseInfo {
    name: String,
    owner: Owner,
    lang: Lang,
    prevs: HashSet<Box<Block>>,
    nexts: HashSet<Box<Block>>,
}

#[derive(Debug)]
pub struct UiInfo {
    goodness: String,
}

#[derive(Debug)]
pub struct RenderInfo {
    is_blazing: bool,
}

#[derive(Debug)]
pub struct Block {
    id: BlockType,
    specinfo: SpecInfo,
    baseinfo: BaseInfo,
    prevs: Vec<BlockType>,
    nexts: Vec<BlockType>,
    // nexts: Vec<(BlockType, Vec<BlockType>)>,  // [one level downstream node, [subset of paths to leaf from that node]]
}

#[derive(Eq, Debug, strum_macros::Display, Hash, PartialEq)]
pub enum BlockType {
    Bridge,
    Ui(Ui),
    Render(Render)
}

fn new_ui(id: BlockType, baseinfo: BaseInfo, uiinfo: UiInfo, prevs: Vec<BlockType>, nexts: Vec<BlockType>) -> Block {

    for bt in &prevs {
        match bt {
            BlockType::Ui(_) => (),
            BlockType::Bridge => (),
            _ => panic!("Incorrect prev of Ui")
        }
    }
    for bt in &nexts {
        match bt {
            BlockType::Ui(_) => (),
            BlockType::Render(_) => (),
            _ => panic!("Incorrect next of Ui")
        }
    }

    return Block {
        id: id,
        specinfo: SpecInfo::Ui(uiinfo),
        baseinfo: baseinfo,
        prevs: prevs,
        nexts: nexts,

    }
}

#[derive(Debug)]
pub enum SpecInfo {
    Ui(UiInfo),
    Render(RenderInfo)
}

#[derive(Eq, Debug, PartialEq, EnumIter, strum_macros::Display, Clone, Copy, Hash)]
pub enum Ui {
    Dom,
    Dioxus,
}

#[derive(Eq, Debug, PartialEq, EnumIter, strum_macros::Display, Clone, Copy, Hash)]
pub enum Render {
    Skia,
    Vello
}

pub fn add_all( blocks: &mut HashMap<BlockType, Block>) {
    for ui in Ui::iter() {
        let block = ui.value();
        assert!(block.id == BlockType::Ui(ui));
        assert!(! blocks.contains_key(&block.id));
        blocks.insert(BlockType::Ui(ui), ui.clone().value());
    }
}

// ========================
// ADD UIs BELOW
// ========================

impl Ui {
pub fn value(&self) -> Block {
match *self {
    Ui::Dom => new_ui(
        BlockType::Ui(Ui::Dom),
        BaseInfo {
            name: "Flutter".to_string(),
            owner: Owner::Google,
            lang: Lang::Dart,
            nexts: HashSet::new(),
            prevs: HashSet::new(),
        },
        UiInfo {
            goodness: "very".to_string()
        },
        vec![BlockType::Ui(Ui::Dioxus)],
        vec![],
    ),
    Ui::Dioxus => new_ui (
        BlockType::Ui(Ui::Dioxus),
        BaseInfo {
            name: "Dioxus".to_string(),
            owner: Owner::Dioxuslabs,
            lang: Lang::Rust,
            nexts: HashSet::new(),
            prevs: HashSet::new(),
        },
        UiInfo {
            goodness: "great".to_string()
        },
        vec![],
        vec![BlockType::Render(Render::Skia)],
    )
}
}
}

