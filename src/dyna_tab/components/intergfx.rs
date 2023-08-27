
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Intergfx {}

impl Blockify for Intergfx {
    fn add_all(&self) -> Vec<(&'static str, Info, ExtraInfo, Vec<Vec<&'static str>>)> {
        vec!
        [
        ("Erithaxintergfx",
            Info::new(
                "Erithax InterGFX",
                Owner::Erithax,
                "TODO",
                "",
                SourceOpenness::NA,
                vec![Lang::Rust],
                None,
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                $
            }
        ),
        ("Angle", 
            Info::new(
                "ANGLE",
                Owner::Google,
                "Almost Native Graphics Layer Engine: translates Opengl ES 2/3 calls to DirectX 9, 11, Opengl or Vulkan API calls",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/google/angle"),
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                Opengles $ D3d Windows,
                Opengles $ Opengl Linux,
                Opengles $ Vulkan,
            }
        ),
        ("Dawn",
            Info::new(
                "Dawn",
                Owner::Google,
                "Dawn is an open-source and cross-platform implementation of the work-in-progress WebGPU standard.",
                "",
                SourceOpenness::NA,
                vec![Lang::Cpp],
                Repo::opt_with_url("https://github.com/google/angle"),
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                Webgpu $ Vulkan,
                Webgpu $ Vulkan Linux,
                Webgpu $ Vulkan Chromeos,
                Webgpu $ Opengles,
                Webgpu $ D3d Windows,
                Webgpu $ Metal *,
            },
        ),
        ("Dxvk", 
            Info::new(
                "DXVK",
                Owner::Doitsujin,
                "a Vulkan-based translation layer for D3d 9/10/11 which allows running 3D applications on Linux using Wine.",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/doitsujin/dxvk"),
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                D3d $ Vulkan,
            }
        ),
        ("Glow", 
            Info::new(
                "GLOW",
                Owner::Grovesnl,
                "GL on Whatever: a set of bindings to run GL anywhere (Open GL, Opengl ES, and Webgl) and avoid target-specific code.",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/grovesNL/glow")
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                Opengles $ Opengl,
                Opengles $ Webgl,
            }
        ),
        ("Moltenvk", 
            Info::new(
                "MoltenVK",
                Owner::Khronos,
                "MoltenVK allows Vulkan applications to run on top of Metal on Apple's macOS, iOS, and tvOS operating systems",
                "",
                SourceOpenness::NA,
                vec![Lang::TODO],
                Repo::opt_with_url("https://github.com/KhronosGroup/MoltenVK")
            ),
            ExtraInfo::Intergfx {},
            parsetree2!{
                Vulkan $ Metal,
            }
        ),
        ("Wgpu", 
            Info::new(
                "wgpu",
                Owner::Gfxrs,
                "wgpu is a cross-platform, safe, pure-rust graphics api. It runs natively on Vulkan, Metal, D3D12, D3D11, and Opengles; and on top of Webgpu on wasm. The api is based on the Webgpu standard. It serves as the core of the Webgpu integration in Firefox, Servo, and Deno.",
                "https://wgpu.rs",
                SourceOpenness::NA,
                vec![Lang::Rust],
                Repo::opt_with_url("https://github.com/gfx-rs/wgpu"),
            ),
            ExtraInfo::Intergfx {
                
            },
            parsetree2!{
                Webgpu $ Opengles,
                Webgpu $ Opengles Glow *,
                Webgpu $ Vulkan *,
                Webgpu $ D3d Windows,
                Webgpu $ Metal,
            }
        ),
        ]
    }

}

