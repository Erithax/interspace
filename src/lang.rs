

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum LangType {
    AOT,
    JIT,
    Interpreted,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LangInfo {
    name: String,
    langtype: LangType,
}

// =======================
// ADD LANGUAGES BELOW
// =======================
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
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
    fn value(&self) -> LangInfo {
        match *self {
            Lang::Rust => LangInfo {
                name: String::from("Rust"),
                langtype: LangType::AOT,
            },
            Lang::Cpp => LangInfo {
                name: String::from("C++"),
                langtype: LangType::AOT,
            },
            _ => panic!("did not implement language data for Lang variant")
        }
    }
}
