
use serde;
use strum_macros::Display;



#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Deserialize, serde::Serialize)]
pub struct OwnerInfo {
    name: String,
    incorporated: bool,
    nonprofit: bool,
}

impl Default for Owner {fn default() -> Self {return Owner::Erithax}}

// =======================
// ADD OWNERS BELOW
// =======================
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Display, strum_macros::EnumIter, strum_macros::EnumString, serde::Deserialize, serde::Serialize)]
pub enum Owner {
    TODO,
    Erithax,

    Adobe,
    Apple,
    Cairogfx,
    Doitsujin,
    Dioxuslabs,
    Emilk,
    Femtovg,
    Gfxrs,
    Gnome,
    Google,
    Grovesnl,
    Icedrs,
    Khronos,
    Leptosrs,
    Linebender,
    LinuxFoundation,
    Microsoft,
    Mozilla,
    Qtcompany,
    Sixtyfps,
    Terrainformatica,
    Webstandards,
    Yew,
    Xorg,

    Gtkrs,
    // Custom(String, bool, bool),
}


impl Owner {
    fn value(&self) -> OwnerInfo {
        match self {
            Owner::Apple => OwnerInfo {
                name: String::from("Apple"),
                incorporated: true,
                nonprofit: false,
            },
            Owner::Doitsujin => OwnerInfo {
                name: String::from("Doitsujin"),
                incorporated: false,
                nonprofit: false,
            },
            Owner::Gfxrs => OwnerInfo {
                name: String::from("gfx-rs"),
                incorporated: false,
                nonprofit: false,
            },
            Owner::Gnome => OwnerInfo {
                name: String::from("The GNOME Project"),
                incorporated: true,
                nonprofit: true,
            },
            Owner::Google => OwnerInfo {
                name: String::from("Google"),
                incorporated: true,
                nonprofit: false,
            },
            Owner::Grovesnl => OwnerInfo {
                name: String::from("GrovesNL"),
                incorporated: false,
                nonprofit: false,
            },
            Owner::Khronos => OwnerInfo {
                name: String::from("Khronos Group"),
                incorporated: true,
                nonprofit: true,
            },
            Owner::Linebender => OwnerInfo {
                name: String::from("Linebender"),
                incorporated: false,
                nonprofit: false,
            },
            Owner::LinuxFoundation => OwnerInfo {
                name: String::from("Linux Foundation"),
                incorporated: true,
                nonprofit: true,
            },
            Owner::Mozilla => OwnerInfo {
                name: String::from("Mozilla"),
                incorporated: true,
                nonprofit: true,
            },
            Owner::Xorg => OwnerInfo {
                name: String::from("Xorg"),
                incorporated: true,
                nonprofit: true,
            },
            // Owner::Custom(name, incorporated, nonprofit) => OwnerInfo {
            //     name: String::from(name),
            //     incorporated: *incorporated,
            //     nonprofit: *nonprofit,
            // },
            _ => panic!("did not implement owner data for Owner variant")
        }
    }
}
