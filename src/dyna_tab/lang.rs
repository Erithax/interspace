

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum LangType {
    NA,
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
}

impl Lang {
    pub fn value(&self) -> LangInfo {
        use LangType::*;

        match *self {
            Lang::NA => LangInfo {
                name: "N/A",
                langtype: NA,
            },
            Lang::TODO => LangInfo {
                name: "TODO",
                langtype: NA,
            },
            Lang::C => LangInfo {
                name: "C",
                langtype: AOT,
            },
            Lang::Cpp => LangInfo {
                name: "C++",
                langtype: AOT,
            },
            Lang::Csharp => LangInfo {
                name: "C#",
                langtype: JIT,
            },
            Lang::Dart => LangInfo {
                name: "Dart",
                langtype: JIT,
            },
            Lang::Html => LangInfo {
                name: "HTML",
                langtype: NA,
            },
            Lang::Java => LangInfo {
                name: "Java",
                langtype: JIT,
            },
            Lang::Javascript => LangInfo {
                name: "Javascript",
                langtype: JIT,
            },
            Lang::Kotlin => LangInfo {
                name: "Kotlin",
                langtype: JIT,
            },
            Lang::Objectivec => LangInfo {
                name: "Objective C",
                langtype: AOT,
            },
            Lang::Python => LangInfo {
                name: "Python",
                langtype: NA,
            },
            Lang::Qml => LangInfo {
                name: "QML",
                langtype: NA,
            },
            Lang::Ruby => LangInfo {
                name: "Ruby",
                langtype: JIT,
            },
            Lang::Rust => LangInfo {
                name: "Rust",
                langtype: AOT,
            },
            Lang::Slintmarkup => LangInfo {
                name: "Slint Markup",
                langtype: NA,
            },
            Lang::Swift => LangInfo {
                name: "Swift",
                langtype: AOT,
            },
            Lang::Typescript => LangInfo {
                name: "Typescript",
                langtype: JIT,
            }
        }
    }
}
