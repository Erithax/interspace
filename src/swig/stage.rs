
use crate::swig::component::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize, strum_macros::EnumIter)]
pub enum Stage {
    ROOT,
    Langbridge,
    Ui,
    Layout,
    Paint,
    Raster,
    Gfx,
    Platform,
    LEAF,
}

impl Stage {
    pub fn lvl(&self) -> usize {
        match self {
            Self::ROOT          => {0},
            Self::Langbridge    => {1},
            Self::Ui            => {2},
            Self::Layout        => {3},
            Self::Paint         => {4},
            Self::Raster        => {5},
            Self::Gfx           => {6},
            Self::Platform      => {7},
            Self::LEAF          => {8},
        }
    }

    pub fn short_rep(&self) -> &'static str {
        match self {
            Self::ROOT => {"RT"},
            Self::Langbridge => {"lb"},
            Self::Ui => {"ui"},
            Self::Layout => {"ly"},
            Self::Paint => {"pa"},
            Self::Raster => {"ra"},
            Self::Gfx => {"gx"},
            Self::Platform => {"pf"},
            Self::LEAF => {"LF"},
        }
    }

    pub fn iter_reals() -> impl Iterator<Item = Stage> {
        let mut res = Stage::iter()
            .filter(|bt|
                match bt {
                    Stage::ROOT => {false},
                    Stage::LEAF => {false},
                    _ => {true}
                }
            )
            .collect::<Vec<Stage>>();
        
        res.sort_unstable();
        return res.into_iter()
    }

    pub fn from_comp_typ(comp_typ: ComponentType) -> Stage {
        match comp_typ {
            ComponentType::Langbridge => Stage::Langbridge,
            ComponentType::Ui => Stage::Ui,
            ComponentType::Layout => Stage::Layout,
            ComponentType::Paint => Stage::Paint,
            ComponentType::Raster => Stage::Raster,
            ComponentType::Gfxapi => Stage::Gfx,
            ComponentType::Intergfx => Stage::Gfx,
            ComponentType::Platform => Stage::Platform,
        }
    }
}

impl PartialOrd for Stage {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lvl() == other.lvl() {
            return Some(std::cmp::Ordering::Equal)
        } else if self.lvl() < other.lvl() {
            return Some(std::cmp::Ordering::Less)
        } else {
            return Some(std::cmp::Ordering::Greater)
        }
    }
}
impl Ord for Stage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.lvl() < other.lvl() {
            return std::cmp::Ordering::Less
        } else if self.lvl() == other.lvl() {
            return std::cmp::Ordering::Equal
        } else {
            return std::cmp::Ordering::Greater
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoStage {
    All,
    Pre,
    Mid,
    Post,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubStage {
    pub stage: Stage,
    pub pseudostage: PseudoStage,
}



