
# All
Name of each directory/file is unique identifier and must characters must all be lowercase ascii alphabetic [a-z] or number [0-9]
e.g.
    * not GTK.toml  but gtk.toml
    * not 2P2.svg   but 2p2.svg
    * not (●'◡'●)



# Owner
[info]
name: `string`     e.g. "Erithax" _first letter capitalized ascii letter, rest lowercase ascii letters / numbers_
website: `string`  e.g. "erithax.com" 
type: `string`     "individual" | "community" | "corporation"

// if type == "corporation"
nonprofit: `bool`  true | false
aggregate: `array<string>` e.g. ["Microsoft", "Google", "Oracle"]  with string a valid owner symbol
country  : `string` e.g. "Iceland"

[style]
light_back: `string` e.g. "radial-gradient(circle at 0 0, #f0f5 0%, #0000 30%), radial-gradient(circle at 100% 100%, #ff03 0%, #0000 30%), #0000;"
dark_back: `string` 

# Lang
[info]
name: `string`
website: `string`
owner: `string`
type: `string` "aot" | "jit" | "interpreted"


# All blocks
[info]
name: `string`     
owner: `string` where `string` is a valid owner identifier
source_openess: "superfree" | "copyleft" | "source-available" | "closed"
use_freeness: "free" | "commercial"
website: `string` 
stage:  `string` | `[<string>, <string>]` where `string` a valid stage identifier
description: `string`
imp_langs: `string` | `array<string>` | `array<[<string>, <float>]>`  where `string` a valid language identifier, and `float` in (0..1)

branches = `string` e.g. "
    $ erithaxraster erithaxgfx
    $
"
  

[style]
light_back: `string`
dark_back: `string`

# Ui blocks
[extra]
declarativity: `string` "none" | "lo" | "mid" | "hi"
is_immediate: `bool`
reactivity: `string`  "none" | "fine-grained" | "elmy" | "reacty" | "swifty"
macrotivity: `string` "none" | "lo" | "mid" | "hi"
language: `string`
        or {custom_lang_name: `string`, logic_langs: `array<string>`}

hot_reload: `bool`
ssr: `bool`
liveview: `bool`


# Styling
[extra]
css: `string` "none" | "subset" | "full" 

# Layout
[extra]
constraint_based: `bool`
css: `bool`
flexbox: `bool`
grid: `bool`

# Paint
[extra]

# Raster
[extra]

# Gfx
[extra]
low_level: `bool`

# Intergfx
[extra]

# Platform
[extra]
 