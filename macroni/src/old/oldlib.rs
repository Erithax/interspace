
// #![feature(proc_macro_internals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_braces)]


use std::collections::HashSet;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

use proc_macro2::{TokenStream, TokenTree, Punct, Spacing, Span, Group, Delimiter};
use proc_macro_error::abort_if_dirty;
use proc_macro_error::set_dummy;
use quote::quote_spanned;
use syn::{Item, Token, Ident, parse::{Parse, ParseStream, Result}};
use quote::{quote};
use proc_macro_error::{abort, emit_error, proc_macro_error};
use strsim::normalized_damerau_levenshtein;

lazy_static::lazy_static! {
    static ref SRC_DIR: String = "./src/".to_string();
    static ref COMMON_FILE: String = "common".to_string();
    static ref FILE_NAMES: Vec<String> = vec![
        "langbridge".to_string(),
        "ui".to_string(),
        "layout".to_string(),
        "render".to_string(),
        "intergfx".to_string(),
        "gfx".to_string(),
        "platform".to_string(),
    ];
}

#[derive(Debug)]
struct PunctIdTree {
    nodes: Vec<VecDeque<Ident>>,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_stringified_enum_from_file(path_enum_ids : &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    use syn::parse_file;

    let mut enum_id_to_var_id: HashMap<String, Vec<String>> = HashMap::new();

    for (path, _) in path_enum_ids {
        for item in
                parse_file(
                    &read_to_string(path)
                    .expect(& ("Unable to read file ".to_string().to_owned() + path))
                ).expect(&("Unable to parse file ".to_string().to_owned() + path))
                .items
        {
            match item {
                Item::Enum(e) => {
                    // dbg!("{:?}", e);
                    let id = e.ident.to_string();
                    // dbg!("{:?}", &id);
                    if path_enum_ids.values().any(|x| x == &e.ident.to_string()) {
                        // dbg!("GOOD ID: {:?}", &id);
    
                        let mut vars: Vec<String> = Vec::new();
                        for var in e.variants {
                            let var_id = var.ident.to_string();
                            // dbg!("VAR ID: {:?}", &var_id);
                            vars.push(var_id);
                        }
                        enum_id_to_var_id.insert(id, vars);
                    }
                   
    
                },
                _ => {}
            }
        }
    }
    return enum_id_to_var_id
    
}


impl Parse for PunctIdTree {
    fn parse(content: ParseStream) -> Result<Self> {
        // let content: ParseBuffer;
        // let _brace_token = braced!(content in input);
        
        let mut id_stack: Vec<Ident> = Vec::new();
        let mut token_lvl: usize = 0;

        let mut lines: Vec<VecDeque<Ident>> = Vec::new();
        let mut curr_line: VecDeque<Ident> = VecDeque::new();

        enum LastToken {
            Id,
            Pu,
        }

        if content.is_empty() {
            return Ok(PunctIdTree { nodes: lines });
        }
        if ! content.peek(Token![>]) {
            panic!("all items should be nested at least one `>` deep, so the first char in the tree should always be `>`")
        }

        let mut last_token: LastToken = LastToken::Pu;
        let mut pun: Punct;
        let mut id: Ident;

        while ! content.is_empty() {
            // println!("==============================================");
            // println!("{lines:?}");
            // println!("{id_stack:?}");
            // println!("{curr_line:?}");
            // println!("token_lvl: {token_lvl:?}");
            // println!("id: {}, pu: {}", content.peek(Ident), content.peek(Token![>]));
            if content.peek(Ident) {
                id = content.parse().unwrap();
                token_lvl += 1;
                
                if id_stack.len() > 0 && token_lvl-2 <= id_stack.len()-1 {
                    for _ in token_lvl-2..=id_stack.len()-1 {
                        // println!("{i}");
                        id_stack.pop();
                    }
                }
                id_stack.push(id.clone());
                curr_line.push_back(id);
                assert!(token_lvl-2 == curr_line.len()-1);
                assert!(token_lvl-2 == id_stack.len()-1);
                last_token = LastToken::Id;
            } else if content.peek(Token![>]) {
                pun = content.parse().unwrap();
                token_lvl += 1;

                if pun.as_char() != '>' {panic!("char is not >!!")}

                match last_token {
                    LastToken::Pu => {},
                    LastToken::Id => {
                        lines.push(curr_line.clone());
                        curr_line.clear();
                        token_lvl = 1;
                    }
                }

                if token_lvl > 1 {
                    curr_line.push_back(id_stack[token_lvl-2].clone());
                }

                last_token = LastToken::Pu;

            } else {
                panic!("stream does not consist of `>` and Identifiers")
            }
        }

        if curr_line.len() > 0 {
            lines.push(curr_line.clone())
        }

        // println!("lines: {lines:?}");

        return Ok(PunctIdTree {
            nodes: lines,
        })
        
    }
}



#[proc_macro]
pub fn parsetree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let mut path_n_enums : HashMap<String, String> = HashMap::new();

    for filename in FILE_NAMES.iter() {
        let path :String = SRC_DIR.to_string() + filename + ".rs" ;
        path_n_enums.insert( path, capitalize(filename));
    }

    path_n_enums.insert(SRC_DIR.to_string() + &COMMON_FILE.to_string() + ".rs", "BlockTypes".to_string());

    let enum_vars: HashMap<String, Vec<String>> = get_stringified_enum_from_file(&path_n_enums);

    // dbg!(&enum_vars);

    let punidtree: PunctIdTree = syn::parse2(input.into()).expect("incorrect indent based identifier tree");
    let ids: Vec<VecDeque<Ident>> = punidtree.nodes;

    let mut qualified_ids: TokenStream = TokenStream::new();
    
    let  mut qual_id_line: TokenStream;
    let mut found_match: bool;

    for line in ids {
        qual_id_line = TokenStream::new();
        for id in line {
            found_match = false;

            if id.to_string() == "ALL" {
                qual_id_line.extend(quote!{
                    BlockType::ALL,
                });
                continue
            } else if id.to_string() == "TODO" {
                qual_id_line.extend(quote!(
                    BlockType::TODO,
                ));
                continue
            }

            for (enum_id, var_ids) in enum_vars.clone() {
                for var_id in var_ids {
                    if id.to_string() == var_id {
                        if found_match == true {
                            panic!("Duplicate variant name {} found across blocktypes", var_id)
                        }
                        found_match = true;

                        let enum_id_id = Ident::new(&enum_id, Punct::new('.', Spacing::Alone).span());//<dyn Span>::def_site());
                        let var_id_id = Ident::new(&var_id, Punct::new('.', Spacing::Alone).span());

                        qual_id_line.extend(quote!{
                            BlockType::#enum_id_id(#enum_id_id::#var_id_id),
                        });
                    }
                }
            }
            if ! found_match {
                panic!("Did not find variant name : {} in any enum", id);
            }
        }
    
        qualified_ids.extend(
            quote!{
                vec![#qual_id_line],
            }
        )
    }

    // dbg!(&qualified_ids);
    // dbg!("END OF MACRO");

    let out: proc_macro::TokenStream = proc_macro::TokenStream::from(quote!{
        vec![#qualified_ids]
    });
    // dbg!(&out.to_string());
    return out
}