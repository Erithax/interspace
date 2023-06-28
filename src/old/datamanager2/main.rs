
use pest;
use pest::*;
use pest::Parser;
use pest_derive::Parser;
use sha2::digest::block_buffer::Block;


const DATA_SRC_PATH: &'static str = "data/";

mod OwnerP {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "src/bin/datamanager2/base.pest"]
    #[grammar = "src/bin/datamanager2/owner.pest"]
    pub struct Parser;
}

mod LangP {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "src/bin/datamanager2/base.pest"]
    #[grammar = "src/bin/datamanager2/lang.pest"]
    pub struct Parser;
}


mod BlockParser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "src/bin/datamanager2/base.pest"]
    #[grammar = "src/bin/datamanager2/block_base.pest"]
    #[grammar = "src/bin/datamanager2/langbridge.pest"]
    #[grammar = "src/bin/datamanager2/ui.pest"]
    #[grammar = "src/bin/datamanager2/style.pest"]
    #[grammar = "src/bin/datamanager2/layout.pest"]
    #[grammar = "src/bin/datamanager2/paint.pest"]
    #[grammar = "src/bin/datamanager2/raster.pest"]
    #[grammar = "src/bin/datamanager2/gfx.pest"]
    #[grammar = "src/bin/datamanager2/intergfx.pest"]
    #[grammar = "src/bin/datamanager2/shell.pest"]
    #[grammar = "src/bin/datamanager2/platform.pest"]
    pub struct Parser;
}

pub enum RoughRange {
    Lo,
    Mid,
    Hi,
}

pub enum Reactivity {
    None,
    FineGrained,
    Elmy,
    Reacty,
    Swifty,
}

pub enum UiLang {
    Lang,
    Custom{name: String},
} 

pub struct UiInfo {
    declarativity: RoughRange,
    is_immediate: bool,
    reactivity: Reactivity,
    macrotivity: RoughRange,
    language: UiLang,
    hot_reload: bool,
    ssr: bool,
    liveview: bool,
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
   ip: String,
   port: Option<u16>,
   stuff: Vec<String>,
   keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
   github: String,
   travis: Option<String>,
}



pub fn main() {
    println!("main.rs");

    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'
        stuff = 'bonk'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();


    use toml::Table;
    let value = "foo = 'bar'".parse::<Table>().unwrap();
    // assert_eq!(value["foo"].as_str(), Some("bar"));
    println!("{value}");
    println!("{value:?}");

    let valid_owner_lvl_extensions = vec![".toml", ".svg", ".png", ".jpg"];

    for data_item in std::fs::read_dir(DATA_SRC_PATH)
        .expect(&format!("Data directory `{}` could not be opened", DATA_SRC_PATH)) 
    {
        let data_item = data_item.unwrap().path();
        let data_item_name = data_item.file_name().unwrap().to_str().unwrap().to_string();
        let data_item_stem = data_item.file_stem().unwrap().to_str().unwrap().to_string();

        assert!(data_item.is_dir() || data_item_name.to_ascii_lowercase() == "readme.md", 
            "all items in data directory {} must be directories, found file {}", DATA_SRC_PATH, data_item.display());

        if data_item_name == "readme.md" {
            continue
        }
 
        assert!(data_item_name.is_ascii(), 
            "all items in data directory must have ascii names starting with a capital letter, found: `{}`", 
            data_item.display());

        let first_char = data_item_name.chars().collect::<Vec<char>>()[0];

        assert!(first_char.is_ascii_alphabetic() && first_char.is_uppercase(), 
            "all items in data directory must have ascii names starting with a capital letter, found: `{}`", 
            data_item.display());

        assert!(data_item_stem.chars().all(|c| c.is_ascii_alphanumeric()));
        assert!(data_item_stem.chars().skip(1).all(|c| c.is_lowercase()));
        
        let mut has_owner_file = false;
        for owner_item in std::fs::read_dir(data_item.clone())
            .expect(&format!("could not open owner dir {:?}", data_item.display())) 
        {
            let owner_item = owner_item.unwrap().path();
            let owner_item_name = owner_item.file_name().unwrap().to_str().unwrap().to_string();
            let owner_item_stem = owner_item.file_stem().unwrap().to_str().unwrap().to_string();

            if owner_item_stem == *data_item_stem {
                assert!(owner_item.is_file() && valid_owner_lvl_extensions.iter().any(|ext| owner_item_name.ends_with(ext)));
                if owner_item_name.ends_with(".toml") {
                    //...
                    let parsed = OwnerP::Parser::parse(OwnerP::Rule::owner, std::fs::read_to_string(owner_item).unwrap().as_str());
                    has_owner_file = true;
                } else {
                    println!("TODO: IMAGES");
                    //...
                }
            } else if owner_item.is_dir() && owner_item_name == "archive" {
                continue
            } else if owner_item.is_file() {
                parse_block_file(owner_item);
            } else {
                // ... error
            }
        }
        assert!(has_owner_file, "every owner xyz must have an owner file xyz.toml in it, did not find in {:?}", &data_item.to_str());
    }





    // let file = std::fs::read_to_string(DATA_SRC_PATH.to_owned() + "Erithax/" + "ErithaxUi.toml").unwrap();
    // let parsed = BlockParser::Parser::parse(BlockParser::Rule::block_base, &file);

    // let parsed = match parsed {
    //     Err(e) => {
    //         println!("{:?}", e);
    //         panic!("{}", e);
    //     },
    //     Ok(o) => {
    //         o
    //     }
    // };

    // for p in parsed {
    //     for p1 in p.into_inner() {
    //         println!("p1 : {p1:?}");
    //     }
    // }
    
    // let valid_strings: Vec<&str> = vec!["\"Good Stuff\"", "\"good\"bdfdf", "\"I like to woppa woppa.\"", "\"\"", "\"Frog vs \\\" bulldog?\""];
    // let invalid_strings: Vec<&str> = vec!["No good.", "b\"bad\"", "\" Absolutely fwomped", "So delicious\""];

    // for s in valid_strings {
    //     println!("valid:   {} -> {:?}", UiParser::parse(Rule::string, s).is_ok(), s);
    // }
    // for s in invalid_strings {
    //     println!("invalid: {} -> {} {:?}", UiParser::parse(Rule::string, s).is_err(), s, UiParser::parse(Rule::string, s));
    // }

}

pub fn parse_block_file(path: std::path::PathBuf) {

    assert!(path.to_string_lossy().ends_with(".toml"));
    let file = std::fs::read_to_string(path).unwrap();
    let block_base_section = BlockParser::Parser::parse(BlockParser::Rule::block_base_section_start, file.as_str());
    assert!(block_base_section.is_ok());

    let mut curr_i = block_base_section.unwrap().next().unwrap().as_span().end();
    println!("{curr_i}");
    
    let block_base = BlockParser::Parser::parse(BlockParser::Rule::block_base, &file[curr_i..]);


    println!("{:?}", block_base);

}