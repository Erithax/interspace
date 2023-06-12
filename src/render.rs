use macroni::use_all_block_modules;
use_all_block_modules!(render);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct RenderInfo {

}

impl Default for Render {fn default() -> Self {return Render::META}}

// =======================
// ADD LANGUAGES BELOW
// =======================

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Render {
    META,
    Erithaxrender,

    Gecko,
    Servo,
    Blink,
    Webkit,

    Xwindow,
    Direct2d,
    Coregraphics,

    Gdk,
    Cairo,
    Femtovg,
    Skia,
    Piet,
    Slintcpu,
    Vello,
    // Custom(String, Vec<RenderTarget>)
}


impl Blockify<Render, RenderInfo> for Render {
    fn add_all() -> Vec<(Render, Info, RenderInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (Render::Erithaxrender,
            Info {
                name: "Erithax Render".to_string(),
                owner: Owner::Erithax,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![],
            },
            RenderInfo {},
            parsetree!{
                $
            }
        ),
        (Render::Gecko, 
            Info {
                name: "Gecko".to_string(),
                owner: Owner::Mozilla,
                desc: TDS!(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Cpp, Lang::Rust],
            },
            RenderInfo {},
            parsetree!{
                $ Angle,
            }
        ),
        (Render::Servo, 
            Info {
                name: "Servo".to_string(),
                owner: Owner::LinuxFoundation,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            RenderInfo {},
            parsetree!{
                $ Angle,
                $ Webgpu Wgpu,
                $ Skia,
                $ Cairo,
            }
        ),
        (Render::Blink, 
            Info {
                name: "Blink".to_string(),
                owner: Owner::Google,
                desc: TDS!(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Cpp],
            },
            RenderInfo {},
            parsetree!{
                $ Angle,
                $ Skia,
                $ Webgpu Dawn *
            }
        ),
        (Render::Webkit, 
            Info {
                name: "Webkit".to_string(),
                owner: Owner::Apple,
                desc: TDS!(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Cpp],
            },
            RenderInfo {},
            parsetree!{
                $ Angle,
            }
        ),
        (Render::Xwindow, 
            Info {
                name: "XWindow Render".to_string(),
                owner: Owner::Xorg,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
                $
            }
        ),
        (Render::Direct2d, 
            Info {
                name: "Direct2D".to_string(),
                owner: Owner::Microsoft,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
                $ D3d Windows,
            }
        ),
        (Render::Coregraphics, 
            Info {
                name: "Quartz".to_string(),
                owner: Owner::Apple,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
                $ Metal,
                $ Metal Macos,
                $ Metal Ios,
            }
        ),
        (Render::Gdk, 
            Info {
                name: "GTK Drawing Kit".to_string(),
                owner: Owner::Gnome,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
                $ Xwindow Linux,
                $ Coregraphics Macos,
                $ Direct2d Windows,
            }
        ),
        (Render::Cairo, 
            Info {
                name: "Cairo".to_string(),
                owner: Owner::Cairogfx,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
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
        (Render::Femtovg,
            Info {
                name: "femtovg".to_string(),
                owner: Owner::Femtovg,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            RenderInfo {},
            parsetree!{
                $ Opengles
            }
        ),
        (Render::Piet, 
            Info {
                name: "Piet".to_string(),
                owner: Owner::Linebender,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            RenderInfo {},
            parsetree!{
                $ Direct2d Windows,
                $ Coregraphics Macos,
                $ Cairo Linux,
            }
        ),
        (Render::Skia, 
            Info {
                name: "Skia".to_string(),
                owner: Owner::Google,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
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
        (Render::Slintcpu,
            Info {
                name: "Slint CPU".to_string(),
                owner: Owner::Sixtyfps,
                desc: TDS!(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            RenderInfo {},
            parsetree!{
                $ Cpu
            }
        ),
        (Render::Vello, 
            Info {
                name: "Vello".to_string(),
                owner: Owner::Linebender,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            RenderInfo {},
            parsetree!{
                $ Wgpu *
            }
        ),
        ] 
    }
}
