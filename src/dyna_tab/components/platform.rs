
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Platform {}


impl Blockify for Platform {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxplatform",
            Info::new(
                "Erithax Platform",
                Owner::Erithax,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Linux", 
            Info::new(
                "Linux",
                Owner::LinuxFoundation,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::C],
                Repo::opt_with_url("https://git.kernel.org"),
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Windows", 
            Info::new(
                "Windows",
                Owner::Microsoft,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Macos", 
            Info::new(
                "MacOS",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Android", 
            Info::new(
                "Android",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://android.googlesource.com/"),
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Ios", 
            Info::new(
                "Ios",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Chromeos",
            Info::new(
                "ChromeOS",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Web", 
            Info::new(
                "Web",
                Owner::Webstandards,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Svg", 
            Info::new(
                "SVG",
                Owner::Webstandards,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Pdf", 
            Info::new(
                "PDF",
                Owner::Adobe,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ("Png", 
            Info::new(
                "PNG",
                Owner::Webstandards,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Platform {},
            parsetree2!{$}
        ),
        ]
    }
}




