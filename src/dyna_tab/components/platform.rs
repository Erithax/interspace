
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Platform {}


impl Blockify for Platform {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxplatform",
            Info {
                name: "Erithax Platform",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Linux", 
            Info {
                name: "Linux",
                owner: Owner::LinuxFoundation,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::C],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Windows", 
            Info {
                name: "Windows",
                owner: Owner::Microsoft,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Macos", 
            Info {
                name: "MacOS",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Android", 
            Info {
                name: "Android",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Ios", 
            Info {
                name: "Ios",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Chromeos",
            Info {
                name: "ChromeOS",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Web", 
            Info {
                name: "Web",
                owner: Owner::Webstandards,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Svg", 
            Info {
                name: "SVG",
                owner: Owner::Webstandards,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Pdf", 
            Info {
                name: "PDF",
                owner: Owner::Adobe,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Png", 
            Info {
                name: "PNG",
                owner: Owner::Webstandards,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
            },
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ]
    }
}




