
use crate::dyna_tab::component::*;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize, strum_macros::EnumIter)]
pub enum Stage {
    Langbridge,
    Ui,
    Layout,
    Paint,
    Raster,
    Gfx,
    Platform,
}

impl Stage {
    pub fn lvl(&self) -> usize {
        match self {
            Self::Langbridge    => {0},
            Self::Ui            => {1},
            Self::Layout        => {2},
            Self::Paint         => {3},
            Self::Raster        => {4},
            Self::Gfx           => {5},
            Self::Platform      => {6},
        }
    }

    pub fn short_rep(&self) -> &'static str {
        match self {
            Self::Langbridge => {"lb"},
            Self::Ui => {"ui"},
            Self::Layout => {"ly"},
            Self::Paint => {"pa"},
            Self::Raster => {"ra"},
            Self::Gfx => {"gx"},
            Self::Platform => {"pf"},
        }
    }

    pub fn iter_reals() -> impl Iterator<Item = Stage> {
        let mut res = Stage::iter()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum PseudoStage {
    All,
    Pre,
    Mid,
    Post,
}

impl PseudoStage {
    pub fn lvl(&self) -> usize {
        match self {
            Self::All => {0},
            Self::Pre => {1},
            Self::Mid => {2},
            Self::Post => {3},
        }
    }

    pub fn short_rep(&self) -> &'static str {
        match self {
            Self::All => {"all"},
            Self::Pre => {"pre"},
            Self::Mid => {"mid"},
            Self::Post => {"post"},
        }
    }
}
impl PartialOrd for PseudoStage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lvl() < other.lvl() {
            return Some(std::cmp::Ordering::Less)
        } else if self.lvl() == other.lvl() {
            return Some(std::cmp::Ordering::Equal)
        } else {
            return Some(std::cmp::Ordering::Greater)
        }
    }
}
impl Ord for PseudoStage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubStage {
    pub stage: Stage,
    pub pseudostage: PseudoStage,
}

impl PartialOrd for SubStage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.stage < other.stage  {
            return Some(std::cmp::Ordering::Less)
        } else if self.stage > other.stage {
            return Some(std::cmp::Ordering::Greater)
        } else {
            if self.pseudostage < other.pseudostage {
                return Some(std::cmp::Ordering::Less)
            } else if self.pseudostage > other.pseudostage {
                return Some(std::cmp::Ordering::Greater)
            }
            return Some(std::cmp::Ordering::Equal) 
        }
    }
}
impl Ord for SubStage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap()
    }
}

