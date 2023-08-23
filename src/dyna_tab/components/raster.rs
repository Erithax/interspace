
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Raster {}


impl Blockify for Raster {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxraster",
            Info {
                name: "Erithax Raster",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Angle,
            }
        ),
        ("Webrender", 
            Info {
                name: "WebRender",
                owner: Owner::LinuxFoundation,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Angle,
                $ Webgpu Wgpu,
                $ Skia,
                $ Cairo,
            }
        ),
        ("Webkit", 
            Info {
                name: "Webkit",
                owner: Owner::Apple,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Opengles
            }
        ),
        (
            "Impeller",
            Info {
                name: "Impeller",
                owner: Owner::Google,
                description: "TODO",
                website: "https://docs.flutter.dev/perf/impeller",
                code_openness: SourceOpenness::Superopen,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Metal Ios,
                $ Metal Macos,
                $ Vulkan Android,
            }
        ),
        ("Piet", 
            Info {
                name: "Piet",
                owner: Owner::Linebender,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Raster {},
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
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Raster {},
            parsetree2!{
                $ Wgpu *
            }
        ),
        ] 
    }
}
