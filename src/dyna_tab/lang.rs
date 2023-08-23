

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum LangType {
    AOT,
    JIT,
    Interpreted,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct LangInfo {
    pub name: &'static str,
    pub langtype: LangType,
}

// =======================
// ADD LANGUAGES BELOW
// =======================
#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Lang {
    NA,
    TODO,
    C,
    Cpp,
    Csharp,
    Dart,
    Html,
    Java,
    Javascript,
    Kotlin,
    Objectivec,
    Qml,
    Python,
    Ruby,
    Rust,
    Slintmarkup,
    Swift,
    Typescript,

    SlintMarkUp,
    QML,
}

impl Lang {
    pub fn value(&self) -> LangInfo {
        match *self {
            Lang::TODO => LangInfo {
                name: "TODO",
                langtype: LangType::AOT,
            },
            Lang::C => LangInfo {
                name: "C",
                langtype: LangType::AOT,
            },
            Lang::Cpp => LangInfo {
                name: "C",
                langtype: LangType::AOT,
            },
            Lang::Javascript => LangInfo {
                name: "Javascript",
                langtype: LangType::JIT,
            },
            Lang::Rust => LangInfo {
                name: "Rust",
                langtype: LangType::AOT,
            },
            _ => panic!("did not implement language data for Lang variant {:?}", self)
        }
    }
}
