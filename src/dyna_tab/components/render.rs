use macroni::use_all_block_modules;
// use_all_block_modules!(render);

use macroni::parsetree;
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::*;
use super::ExtraInfo;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Render {}


impl Blockify for Render {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxrender",
            Info {
                name: "Erithax Render",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $
            }
        ),
        ("Gecko", 
            Info {
                name: "Gecko",
                owner: Owner::Mozilla,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Angle,
            }
        ),
        ("Servo", 
            Info {
                name: "Servo",
                owner: Owner::LinuxFoundation,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Angle,
                $ Webgpu Wgpu,
                $ Skia,
                $ Cairo,
            }
        ),
        ("Blink", 
            Info {
                name: "Blink",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Skia *,
                $ Webgpu Dawn *
            }
        ),
        ("Webkit", 
            Info {
                name: "Webkit",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Angle,
            }
        ),
        ("Xwindow", 
            Info {
                name: "XWindow Render",
                owner: Owner::Xorg,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $
            }
        ),
        ("Direct2d", 
            Info {
                name: "Direct2D",
                owner: Owner::Microsoft,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ D3d Windows,
            }
        ),
        ("Coregraphics", 
            Info {
                name: "Quartz",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Metal,
                $ Metal Macos,
                $ Metal Ios,
            }
        ),
        ("Gdk", 
            Info {
                name: "GTK Drawing Kit",
                owner: Owner::Gnome,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Xwindow Linux,
                $ Coregraphics Macos,
                $ Direct2d Windows,
            }
        ),
        ("Cairo", 
            Info {
                name: "Cairo",
                owner: Owner::Cairogfx,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Xwindow *,
                $ Coregraphics,
                $ Opengl,
                $ Opengl Windows,
                $ Opengl Linux,
                $ Opengl Macos,
                $ Pdf,
                $ Svg,
                $ Png,
            }
        ),
        ("Femtovg",
            Info {
                name: "femtovg",
                owner: Owner::Femtovg,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Opengles
            }
        ),
        ("Piet", 
            Info {
                name: "Piet",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Direct2d Windows,
                $ Coregraphics Macos,
                $ Cairo Linux,
            }
        ),
        ("Skia", 
            Info {
                name: "Skia",
                owner: Owner::Google,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Vulkan,
                $ Vulkan Windows,
                $ Vulkan Linux,
                $ Vulkan Android,
                $ Opengles,
                $ Opengles Angle,
                $ Opengles Angle D3d Windows,
                $ Opengles Angle Opengl Linux,
                $ Metal,
                $ Metal Ios,
                $ Metal Macos,
                $ Cpu,
                $ Svg,
                $ Pdf,
            }
        ),
        ("Slintcpu",
            Info {
                name: "Slint CPU",
                owner: Owner::Sixtyfps,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Cpu
            }
        ),
        ("Vello", 
            Info {
                name: "Vello",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                source_openess: SourceOpeness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Render {},
            parsetree2!{
                $ Wgpu *
            }
        ),
        ] 
    }
}
