
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Gfxapi {}

impl Blockify for Gfxapi {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxgfx",
            Info {
                name: "Erithax GFX",
                owner: Owner::Erithax,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
                website: "",
            },
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $
            }
        ),
        ("Cpu", 
            Info {
                name: "CPU",
                owner: Owner::TODO,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
                website: "",
            },
            ExtraInfo::Gfxapi {},
            parsetree2!{$}
        ),
        ("Vulkan", 
            Info {
                name: "Vulkan",
                owner: Owner::Khronos,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::C],
                website: "",
            },
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $
            }
        ),
        ("Opengl",
            Info {
                name: "OpenGL",
                owner: Owner::Khronos,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
                website: "",
            },
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $ Windows,
                $ Macos,
                $ Android,
                $ Linux,
                $ Ios,
            }
        ),
        ("Opengles",
            Info {
                name: "OpenGL ES",
                owner: Owner::Khronos,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
                website: "",
            },
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $ Windows,
                $ Linux,
                $ Macos,
                $ Android,
                $ Ios,
            }
        ),
        ("Webgl",
            Info {
                name: "WebGL",
                owner: Owner::Khronos,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
                website: "",
            },
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $ Web,
            }
        ),
        ("Webgpu",
            Info {
                name: "WebGPU",
                owner: Owner::Webstandards,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::NA],
                website: "",
            },
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $ Web,
            }
        ),
        ("D3d",
            Info {
                name: "D3d",
                owner: Owner::Microsoft,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
                website: "",
            },
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $ Windows,
            }
        ),
        ("Metal", 
            Info {
                name: "Metal",
                owner: Owner::Apple,
                description: "TODO",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
                website: "",
            },
            ExtraInfo::Gfxapi {
                
            },
            parsetree2!{
                $ Macos,
                $ Ios,
            }
        ),
        ]
    }
}