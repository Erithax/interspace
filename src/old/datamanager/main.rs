#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_braces)]
/*
The datamanager supports several actions:
* validate data
* validate and generate data
* watch and validate data
* watch and validata and generate data

*/

use async_std::fs::DirEntry;
use ui_overview::dynatree;
use std::env;
use std::cell::OnceCell;
use std::fmt::Display;
use colored::*;

use clap::{Args as ClapArgs, Parser, Subcommand};



#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Args {

    #[command(subcommand)]
    command: Commands,

    /// Add to watch files continuously and perform action (validate/build) on save
    #[arg(short, long)]
    watch: bool,
}


#[derive(Debug, Subcommand)]
enum Commands {
    /// Validates data files and reports errors
    Validate,
    /// Validate & build data files
    Build,
}


const DATA_ROOT_PATH: &'static str = "./data/";

pub fn main() {
    let args = Args::parse();

    println!("{args:?}");

    validate();
    
    if !args.watch {
        println!("validating...");
        match args.command {
            Commands::Validate => {},
            Commands::Build => {build()},
        }
    } else {
        println!("starting watcher");
        match args.command {
            Commands::Validate => {},
            Commands::Build => {build()},
        }
        println!("TODO");
    }

    let args: Vec<_> = env::args().collect();
}

pub fn validate() {

    let valid_owner_lvl_extensions = vec![".toml", ".svg", ".png", ".jpg"];

    for data_item in std::fs::read_dir(DATA_ROOT_PATH)
        .expect(&format!("Data directory `{}` could not be opened", DATA_ROOT_PATH)) 
    {
        let data_item = data_item.unwrap().path();
        let data_item_name = data_item.file_name().unwrap().to_str().unwrap().to_string();
        let data_item_stem = data_item.file_stem().unwrap().to_str().unwrap().to_string();

        assert!(data_item.is_dir() || data_item_name.to_ascii_lowercase() == "readme.md", 
            "all items in data directory {} must be directories, found file {}", DATA_ROOT_PATH, data_item.display());

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
                    has_owner_file = true;
                } else {
                    println!("TODO: IMAGES");
                    //...
                }
            } else if owner_item.is_dir() && owner_item_name == "archive" {
                continue
            } else if owner_item.is_dir() {
                validate_block_dir(owner_item);
            } else {
                // ... error
            }
        }

        assert!(has_owner_file, "every owner xyz must have an owner file xyz.toml in it, did not find in {:?}", &data_item.to_str());



    }
}


pub fn validate_block_dir(path: std::path::PathBuf) {
    let valid_block_dir_item_extensions = vec![".toml", ".svg", ".png", ".jpg"];

    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let stem = path.file_stem().unwrap().to_str().unwrap().to_string();

    for block_dir_item in std::fs::read_dir(path).expect("todo") {

        let block_dir_item = block_dir_item.unwrap().path();
        let block_dir_item_name = block_dir_item.file_name().unwrap().to_str().unwrap().to_string();
        let block_dir_item_stem = block_dir_item.file_stem().unwrap().to_str().unwrap().to_string();

        if block_dir_item.is_dir() && block_dir_item_name == "archive" {
            return
        }
    
        if block_dir_item_stem == stem {
            assert!(block_dir_item.is_file() && valid_block_dir_item_extensions.iter().any(|ext| block_dir_item_name.ends_with(ext)));
            if block_dir_item_name.ends_with(".toml") {
                //...
            } else {
                println!("TODO: IMAGES");
                //...
            }
        } else {
            assert!(block_dir_item_name.is_ascii(), "{}", block_dir_item.display());
            let first_char = block_dir_item_name.chars().collect::<Vec<char>>()[0];
            assert!(first_char.is_uppercase() && first_char.is_ascii_alphabetic(), "{}", block_dir_item.display());
            assert!(block_dir_item_stem.chars().skip(1).all(|c| c.is_numeric() || c.is_lowercase()), "{}", block_dir_item.display());
            // .... recurse into dir
        }
    }

    
}



pub fn build() {

}





pub fn printo() {
    println!("WOOSH!");
}

pub fn wasm_unsafe_fun() {
    use std::time::Instant;
    let n = Instant::now();
    println!("{n:?}");
}


