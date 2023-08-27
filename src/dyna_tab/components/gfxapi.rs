
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Gfxapi {}

impl Blockify for Gfxapi {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxgfx",
            Info::new(
                "Erithax GFX",
                Owner::Erithax,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $
            }
        ),
        ("Cpu", 
            Info::new(
                "CPU",
                Owner::TODO,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Gfxapi {},
            parsetree2!{$}
        ),
        ("Vulkan", 
            Info::new(
                "Vulkan",
                Owner::Khronos,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::C],
                None,
            ),
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $
            }
        ),
        ("Opengl",
            Info::new(
                "OpenGL",
                Owner::Khronos,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
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
            Info::new(
                "OpenGL ES",
                Owner::Khronos,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
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
            Info::new(
                "WebGL",
                Owner::Khronos,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Gfxapi {
            },
            parsetree2!{
                $ Web,
            }
        ),
        ("Webgpu",
            Info::new(
                "WebGPU",
                Owner::Webstandards,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::NA],
                None,
            ),
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $ Web,
            }
        ),
        ("D3d",
            Info::new(
                "D3d",
                Owner::Microsoft,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Gfxapi {},
            parsetree2!{
                $ Windows,
            }
        ),
        ("Metal", 
            Info::new(
                "Metal",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
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