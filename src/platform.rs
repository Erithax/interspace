use macroni::use_all_block_modules;
use_all_block_modules!(platform);




#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PlatformInfo {

}

impl Default for Platform {fn default() -> Self {return Platform::META}}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Platform {
    META,
    Erithaxplatform,

    Linux,
    Windows,
    Macos,
    Android,
    Ios,
    Chromeos,

    Web,

    Svg,
    Pdf,
    Png,
}


impl Blockify<Platform, PlatformInfo> for Platform {
    fn add_all() -> Vec<(Platform, Info, PlatformInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (Platform::Erithaxplatform,
            Info {
                name: "Erithax Platform".to_string(),
                owner: Owner::Erithax,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Linux, 
            Info {
                name: "Linux".to_string(),
                owner: Owner::LinuxFoundation,
                desc: TDS!(),
                imp_lang: Lang::C,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Windows, 
            Info {
                name: "Windows".to_string(),
                owner: Owner::Microsoft,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Macos, 
            Info {
                name: "MacOS".to_string(),
                owner: Owner::Apple,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Android, 
            Info {
                name: "Android".to_string(),
                owner: Owner::Google,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Ios, 
            Info {
                name: "Ios".to_string(),
                owner: Owner::Apple,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Chromeos,
            Info {
                name: "ChromeOS".to_string(),
                owner: Owner::Google,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Web, 
            Info {
                name: "Web".to_string(),
                owner: Owner::Webstandards,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::Javascript],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Svg, 
            Info {
                name: "SVG".to_string(),
                owner: Owner::Webstandards,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Pdf, 
            Info {
                name: "PDF".to_string(),
                owner: Owner::Adobe,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        (Platform::Png, 
            Info {
                name: "PNG".to_string(),
                owner: Owner::Webstandards,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::NA],
            },
            PlatformInfo {},
            parsetree!{$}
        ),
        ]
    }
}




