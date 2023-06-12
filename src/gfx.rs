use macroni::use_all_block_modules;
use_all_block_modules!(gfx);



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct GfxInfo {
}

impl Default for Gfx {fn default() -> Self {return Gfx::META}}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Gfx{
    META,
    Erithaxgfx,

    Cpu,

    Vulkan, 
    Opengl, 
    Opengles,
    Webgl,

    Webgpu,

    D3d,
    Metal,
}

impl Blockify<Gfx, GfxInfo> for Gfx {
    fn add_all() -> Vec<(Gfx, Info, GfxInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (Gfx::Erithaxgfx,
            Info {
                name: "Erithax GFX".to_string(),
                owner: Owner::Erithax,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![],
            },
            GfxInfo {},
            parsetree!{
                $
            }
        ),
        (Gfx::Cpu, 
            Info {
                name: "CPU".to_string(),
                owner: Owner::TODO,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::NA],
            },
            GfxInfo {},
            parsetree!{$}
        ),
        (Gfx::Vulkan, 
            Info {
                name: "Vulkan".to_string(),
                owner: Owner::Khronos,
                desc: TDS!(),
                imp_lang: Lang::C,
                bind_langs: vec![Lang::C],
            },
            GfxInfo {
            },
            parsetree!{
                $
            }
        ),
        (Gfx::Opengl,
            Info {
                name: "OpenGL".to_string(),
                owner: Owner::Khronos,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            GfxInfo {
            },
            parsetree!{
                $ Windows,
                $ Macos,
                $ Android,
                $ Linux,
                $ Ios,
            }
        ),
        (Gfx::Opengles,
            Info {
                name: "OpenGL ES".to_string(),
                owner: Owner::Khronos,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            GfxInfo {
            },
            parsetree!{
                $ Windows,
                $ Linux,
                $ Macos,
                $ Android,
                $ Ios,
            }
        ),
        (Gfx::Webgl,
            Info {
                name: "WebGL".to_string(),
                owner: Owner::Khronos,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            GfxInfo {
            },
            parsetree!{
                $ Web,
            }
        ),
        (Gfx::Webgpu,
            Info {
                name: "WebGPU".to_string(),
                owner: Owner::Webstandards,
                desc: TDS!(),
                imp_lang: Lang::NA,
                bind_langs: vec![Lang::NA],
            },
            GfxInfo {},
            parsetree!{
                $ Web,
            }
        ),
        (Gfx::D3d,
            Info {
                name: "D3d".to_string(),
                owner: Owner::Microsoft,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            GfxInfo {},
            parsetree!{
                $ Windows,
            }
        ),
        (Gfx::Metal, 
            Info {
                name: "Metal".to_string(),
                owner: Owner::Apple,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            GfxInfo {
                
            },
            parsetree!{
                $ Macos,
                $ Ios,
            }
        ),
        ]
    }
}