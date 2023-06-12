use macroni::use_all_block_modules;
use_all_block_modules!(intergfx);




#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct IntergfxInfo {

}

impl Default for Intergfx {fn default() -> Self {return Intergfx::META}}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum_macros::Display, strum_macros::EnumIter, serde::Deserialize, serde::Serialize)]
pub enum Intergfx{
    META,
    Erithaxintergfx,

    Angle,
    Dawn,
    Dxvk,
    Glow,
    Moltenvk,
    Wgpu,
}

impl Blockify<Intergfx, IntergfxInfo> for Intergfx {
    fn add_all() -> Vec<(Intergfx, Info, IntergfxInfo, Vec<Vec<BlockType>>)> {
        vec!
        [
        (Intergfx::Erithaxintergfx,
            Info {
                name: "Erithax InterGFX".to_string(),
                owner: Owner::Erithax,
                desc: TDS!(),
                imp_lang: Lang::Rust,
                bind_langs: vec![],
            },
            IntergfxInfo {},
            parsetree!{
                $
            }
        ),
        (Intergfx::Angle, 
            Info {
                name: "ANGLE".to_string(),
                owner: Owner::Google,
                desc: "Almost Native Graphics Layer Engine: translates Opengl ES 2/3 calls to DirectX 9, 11, Opengl or Vulkan API calls".to_string(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            IntergfxInfo {},
            parsetree!{
                Opengles $ D3d Windows,
                Opengles $ Opengl Linux,
                Opengles $ Vulkan,
            }
        ),
        (Intergfx::Dawn,
            Info {
                name: "Dawn".to_string(),
                owner: Owner::Google,
                desc: "Dawn is an open-source and cross-platform implementation of the work-in-progress WebGPU standard.".to_string(),
                imp_lang: Lang::Cpp,
                bind_langs: vec![Lang::Cpp],
            },
            IntergfxInfo {},
            parsetree!{
                Webgpu $ Vulkan,
                Webgpu $ Vulkan Linux,
                Webgpu $ Vulkan Chromeos,
                Webgpu $ Opengles,
                Webgpu $ D3d Windows,
                Webgpu $ Metal *,
            }
        ),
        (Intergfx::Dxvk, 
            Info {
                name: "DXVK".to_string(),
                owner: Owner::Doitsujin,
                desc: "a Vulkan-based translation layer for D3d 9/10/11 which allows running 3D applications on Linux using Wine.".to_string(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            IntergfxInfo {},
            parsetree!{
                D3d $ Vulkan,
            }
        ),
        (Intergfx::Glow, 
            Info {
                name: "GLOW".to_string(),
                owner: Owner::Grovesnl,
                desc: "GL on Whatever: a set of bindings to run GL anywhere (Open GL, Opengl ES, and Webgl) and avoid target-specific code.".to_string(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            IntergfxInfo {},
            parsetree!{
                Opengles $ Opengl,
                Opengles $ Webgl,
            }
        ),
        (Intergfx::Moltenvk, 
            Info {
                name: "MoltenVK".to_string(),
                owner: Owner::Khronos,
                desc: "MoltenVK allows Vulkan applications to run on top of Metal on Apple's macOS, iOS, and tvOS operating systems".to_string(),
                imp_lang: Lang::TODO,
                bind_langs: vec![Lang::TODO],
            },
            IntergfxInfo {},
            parsetree!{
                Vulkan $ Metal,
            }
        ),
        (Intergfx::Wgpu, 
            Info {
                name: "wgpu".to_string(),
                owner: Owner::Gfxrs,
                desc: "wgpu is a cross-platform, safe, pure-rust graphics api. It runs natively on Vulkan, Metal, D3D12, D3D11, and Opengles; and on top of Webgpu on wasm. The api is based on the Webgpu standard. It serves as the core of the Webgpu integration in Firefox, Servo, and Deno.".to_string(),
                imp_lang: Lang::Rust,
                bind_langs: vec![Lang::Rust],
            },
            IntergfxInfo {
                
            },
            parsetree!{
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

