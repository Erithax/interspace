
use macroni::parsetree2;
use crate::dyna_tab::Blockify;
use crate::dyna_tab::component::*;
use crate::dyna_tab::owner::*;
use crate::dyna_tab::lang::*;
use super::ExtraInfo;




#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Intergfx {}

impl Blockify for Intergfx {
    fn add_all(&self) -> Vec<(ComponentStrId, Info, ExtraInfo, Vec<Vec<ComponentStrId>>)> {
        vec!
        [
        ("Erithaxintergfx",
            Info {
                name: "Erithax InterGFX",
                owner: Owner::Erithax,
                description: "TODO",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                $
            }
        ),
        ("Angle", 
            Info {
                name: "ANGLE",
                owner: Owner::Google,
                description: "Almost Native Graphics Layer Engine: translates Opengl ES 2/3 calls to DirectX 9, 11, Opengl or Vulkan API calls",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                Opengles $ D3d Windows,
                Opengles $ Opengl Linux,
                Opengles $ Vulkan,
            }
        ),
        ("Dawn",
            Info {
                name: "Dawn",
                owner: Owner::Google,
                description: "Dawn is an open-source and cross-platform implementation of the work-in-progress WebGPU standard.",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Cpp],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                Webgpu $ Vulkan,
                Webgpu $ Vulkan Linux,
                Webgpu $ Vulkan Chromeos,
                Webgpu $ Opengles,
                Webgpu $ D3d Windows,
                Webgpu $ Metal *,
            }
        ),
        ("Dxvk", 
            Info {
                name: "DXVK",
                owner: Owner::Doitsujin,
                description: "a Vulkan-based translation layer for D3d 9/10/11 which allows running 3D applications on Linux using Wine.",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                D3d $ Vulkan,
            }
        ),
        ("Glow", 
            Info {
                name: "GLOW",
                owner: Owner::Grovesnl,
                description: "GL on Whatever: a set of bindings to run GL anywhere (Open GL, Opengl ES, and Webgl) and avoid target-specific code.",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                Opengles $ Opengl,
                Opengles $ Webgl,
            }
        ),
        ("Moltenvk", 
            Info {
                name: "MoltenVK",
                owner: Owner::Khronos,
                description: "MoltenVK allows Vulkan applications to run on top of Metal on Apple's macOS, iOS, and tvOS operating systems",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::TODO],
            },
            ExtraInfo::Intergfx {},
            parsetree2!{
                Vulkan $ Metal,
            }
        ),
        ("Wgpu", 
            Info {
                name: "wgpu",
                owner: Owner::Gfxrs,
                description: "wgpu is a cross-platform, safe, pure-rust graphics api. It runs natively on Vulkan, Metal, D3D12, D3D11, and Opengles; and on top of Webgpu on wasm. The api is based on the Webgpu standard. It serves as the core of the Webgpu integration in Firefox, Servo, and Deno.",
                website: "",
                code_openness: SourceOpenness::NA,
                impl_langs: vec![Lang::Rust],
            },
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

