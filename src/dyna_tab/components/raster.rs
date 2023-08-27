
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Raster {}


impl Blockify for Raster {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxraster",
            Info::new(
                "Erithax Raster",
                Owner::Erithax,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $
            }
        ),
        ("Gecko", 
            Info::new(
                "Gecko",
                Owner::Mozilla,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                Repo::opt_with_url("https://hg.mozilla.org/"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Angle,
            }
        ),
        ("Webrender", 
            Info::new(
                "WebRender",
                Owner::LinuxFoundation,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/servo/webrender"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Angle,
                $ Webgpu Wgpu,
                $ Skia,
                $ Cairo,
            }
        ),
        ("Webkit", 
            Info::new(
                "Webkit",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                Repo::opt_with_url("https://github.com/WebKit/WebKit"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Angle,
            }
        ),
        ("Xwindow", 
            Info::new(
                "XWindow Render",
                Owner::Xorg,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://gitlab.freedesktop.org/xorg"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $
            }
        ),
        ("Direct2d", 
            Info::new(
                "Direct2D",
                Owner::Microsoft,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ D3d Windows,
            }
        ),
        ("Coregraphics", 
            Info::new(
                "Quartz",
                Owner::Apple,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Metal,
                $ Metal Macos,
                $ Metal Ios,
            }
        ),
        ("Gdk", 
            Info::new(
                "GTK Drawing Kit",
                Owner::Gnome,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://gitlab.gnome.org/GNOME/gtk/-/tree/main/gdk"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Xwindow Linux,
                $ Coregraphics Macos,
                $ Direct2d Windows,
            }
        ),
        ("Cairo", 
            Info::new(
                "Cairo",
                Owner::Cairogfx,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
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
            Info::new(
                "femtovg",
                Owner::Femtovg,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/femtovg/femtovg"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Opengles
            }
        ),
        (
            "Impeller",
            Info::new(
                "Impeller",
                Owner::Google,
                "TODO",
                "https://docs.flutter.dev/perf/impeller",
                SourceOpenness::Superopen,
                vec![Lang::Cpp],
                None,
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Metal Ios,
                $ Metal Macos,
                $ Vulkan Android,
            }
        ),
        ("Piet", 
            Info::new(
                "Piet",
                Owner::Linebender,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/linebender/piet"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Direct2d Windows,
                $ Coregraphics Macos,
                $ Cairo Linux,
            }
        ),
        ("Skia", 
            Info::new(
                "Skia",
                Owner::Google,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/google/skia"),
            ),
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
            Info::new(
                "Slint CPU",
                Owner::Sixtyfps,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                None,
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Cpu
            }
        ),
        ("Vello", 
            Info::new(
                "Vello",
                Owner::Linebender,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/linebender/vello"),
            ),
            ExtraInfo::Raster {},
            parsetree2!{
                $ Wgpu *
            }
        ),
        ] 
    }
}
