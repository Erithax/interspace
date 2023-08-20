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
use quote::quote;
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

    static ref STAGES: HashMap<String, usize> = HashMap::from([
        ("Langbridge".to_string(), 0),
        ("Ui".to_string(), 1),
        ("Layout".to_string(), 2),
        ("Render".to_string(), 3),
        ("Intergfx".to_string(), 4),
        ("Gfx".to_string(), 4),
        ("Platform".to_string(), 5),
    ]);
}


fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn get_valid_block_idents_from_files() -> (HashMap<String, Vec<String>>, HashSet<String>) {
    use syn::parse_file;

    let valid_enum_names: Vec<String> = FILE_NAMES.iter().map(|n| capitalize(n)).collect();
    let mut enum_with_variants_idents: HashMap<String, Vec<Ident>> = HashMap::new();


    for name in FILE_NAMES.iter() {
        let path = SRC_DIR.to_string() + name + ".rs" ;
        for item in
                parse_file(
                    &read_to_string(&path)
                    .expect(& ("Unable to read file ".to_string().to_owned() + &path))
                ).expect(&("Unable to parse file ".to_string().to_owned() + &path))
                .items
        {
            match item {
                Item::Enum(e) => {
                    let enum_str = e.ident.to_string();
                    if valid_enum_names.contains(&enum_str) {
                        let mut variants: Vec<Ident> = Vec::new();
                        for var in e.variants {
                            variants.push(var.ident);
                        }
                        enum_with_variants_idents.insert(enum_str, variants);
                    }
                },
                _ => {}
            }
        }
    }

    // ERROR IF 'META' is not first variant, remove METAs
    for (en, vars) in enum_with_variants_idents.iter_mut() {
        if vars[0].to_string() != "META" {
            emit_error!(
                vars[0],
                "first variant of block should always be `META`"
            );
        }
        vars.remove(0);
    }



    // ERROR ON DUPLICATES
    for (i, (en, vars)) in enum_with_variants_idents.iter().enumerate() {
        for (j, ident) in vars.iter().enumerate() {
            
            for (k, (en2, vars2)) in enum_with_variants_idents.iter().enumerate() {
                for (l, ident2) in vars2.iter().enumerate() {
                if (i != k || j != l) && ident.to_string() == ident2.to_string() && ident.to_string() != "META" {
                        emit_error!(ident, "found duplicate block variant name {} in enum {} and {}", ident.to_string(), en, en2);
                        emit_error!(ident2, "found duplicate block variant name {} in enum {} and {}", ident.to_string(), en, en2);
                    }
                }
            }

        }
    }

    for (en, vars) in enum_with_variants_idents.iter() {
        for variant in vars {
            if !variant.to_string().is_ascii() {
                emit_error!(
                    variant,
                    "block variant does not consist of just ascii characters (a-z A-Z 0-9)"
                );
            }

            let variant_chars = variant.to_string().chars().collect::<Vec<char>>();

            if !variant_chars[0].is_ascii_uppercase() {
                emit_error!(
                    variant,
                    "block variant must start with capital ascii letter"
                );
            }

            for char in variant_chars.iter().skip(1) {
                if !char.is_ascii_alphanumeric() || char.is_uppercase() {
                    emit_error!(
                        variant.span(),
                        "block variant name must be capital letter followed by lowercase letters or numbers, found {}",
                        variant.to_string()
                    );
                }
            }

            let lower = variant.to_string().to_lowercase();
            if lower == "root" || lower == "leaf" || lower == "meta" || lower == "all" {
                emit_error!(
                    variant,
                    "names 'root', 'leaf', 'meta', and 'all' are reserved for internal use",
                );
            }
        } 
    }

    let mut enum_var_strs = HashMap::new();
    let mut all_variants = HashSet::new();
    for (en, vars) in enum_with_variants_idents {
        let mut temp = vec![];
        for var in vars {
            temp.push(var.to_string());
            all_variants.insert(var.to_string());
        }
        enum_var_strs.insert(en, temp);
    }

    return (enum_var_strs, all_variants)
}




#[proc_macro_error]
#[proc_macro]
pub fn parsetree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // set_dummy(quote!("Vec::<Vec<BlockType>>::new()"));
    let input = proc_macro2::TokenStream::from(input);

    let (enum_with_variants, all_variants) = get_valid_block_idents_from_files();

    let valid_special_tokens = vec!["*".to_string(), "$".to_string(), ",".to_string()];


    // VALIDATE THAT ONLY VALID VARIANT NAMES AND ALLOWED SPECIAL TOKENS ARE USED
    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Group(group) => {
                emit_error!(
                    group, "invalid tokens '{}'", group.to_string()
                )
            },
            TokenTree::Ident(ident) => {
                if !all_variants.contains(&ident.to_string()) {
                    let mut best_matches: Vec<(String, f64)> = vec![("".to_string(), 0.0); 3];
                
                    for var_name in all_variants.iter() {
                        
                        let score =  normalized_damerau_levenshtein(&ident.to_string(), var_name);

                        for i in (0..best_matches.len()).rev() {
                            if score > best_matches[i].1 && (i == 0 || score < best_matches[i-1].1) {
                                best_matches.insert(i, (var_name.clone(), score));
                                best_matches.pop();
                                break
                            }
                        }
                    }
                    emit_error!(
                        ident,
                        "block name '{}' not found", ident.to_string();
                        note = "did you perhaps mean one of these: \n\t-{}\n\t-{}\n\t-{}?", best_matches[0].0, best_matches[1].0, best_matches[2].0
                    );
                }
            },
            TokenTree::Punct(punct) => {
                if !valid_special_tokens.contains(&punct.to_string()) {
                    emit_error!(
                        punct,
                        "invalid token '{}'", punct.to_string()
                    )
                }
            },
            TokenTree::Literal(literal) => {
                emit_error!(
                    literal, "invalid token: found literal {}", literal.to_string();
                    note = "do this and that"
                );
            },
        }
    }
    
    abort_if_dirty();


    // check for $ in every row
    let mut found_dollar = false;
    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Punct(ident) => {
                if ident.to_string() == "$" && !found_dollar {
                    found_dollar = true;
                } else if ident.to_string() == "," && !found_dollar {
                    emit_error!(
                        ident,
                        "unexpected token ',' ";
                        hint = "every line must contain '$'"
                    )
                } else if ident.to_string() == "," {
                    found_dollar = false;
                } else if ident.to_string() == "$" && found_dollar {
                    emit_error!(
                        ident,
                        "token '$' can only appear once on a line";
                        info = "did you perhaps forget a ','"
                    )
                }
            },
            _ => {},
        }
    }

    abort_if_dirty();

    // parse into vec
    let mut identified_res: Vec<Vec<(Ident, Option<Ident>)>> = vec![];
    //                             BlockType Variant, BlockTypeVariant, Variant
    //                            e.g. (Render, Some(Erithaxrender)) or (SELF, None)
    // blocktype variant or blocktypevariant variant gets span of corresponding input ident
    let mut curr_line = vec![];

    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Punct(punct) => {
                if punct.to_string() == "$" {
                    let id = Ident::new("SELF", punct.span());
                    curr_line.push((id, None));
                } else if punct.to_string() == "*" {
                    let id = Ident::new("ALL", punct.span());
                    curr_line.push((id, None));
                } else if punct.to_string() == "," {
                    identified_res.push(vec![]);
                    identified_res.last_mut().unwrap().append(&mut curr_line);
                    curr_line.clear();
                }
            },
            TokenTree::Ident(ident) => {
                let id = ident.to_string();
                let mut found_match = false;
                for (en, vars) in enum_with_variants.iter() {
                    match vars.iter().find(|&var| *var == id) {
                        Some(t) => {
                            curr_line.push((
                                Ident::new(en, ident.span()), 
                                Some(Ident::new( t, ident.span()))
                            ));
                            found_match = true;
                            break
                        },
                        None => {}
                    }
                }
                if !found_match {
                    panic!( "failure in parsetree macro");
                }
            },
            _ => {}
        }
    }


    if curr_line.len() > 0 {
        identified_res.push(curr_line);
    }

    // verify monotonically increasing stages
    for line in identified_res.iter() {
        let curr_stage: usize = 0;
        let mut last_non_self_index;
        let mut curr: &(Ident, Option<Ident>);
        for i in 0..line.len()  {
            curr = &line[i];
            match &curr.1 {
                Some(id) => {
                    last_non_self_index = i;
                    if STAGES.get(&curr.0.to_string()).expect("HERIO") < &curr_stage {
                        emit_error!(
                            id,
                            "a block {} of type {} cannot follow a block of type {}", id.to_string(), curr.0.to_string(), line[last_non_self_index].0.to_string()
                        );
                    }
                },
                None => {
                    // curr is SELF or ALL
                    if curr.0.to_string() == "ALL" && i != line.len()-1 {
                        emit_error!(
                            curr.0,
                            "a block ALL can only occur at the end of a line";
                            info = "did you perhaps forget a ','?"
                        );
                    }
                }
            }
        }
    }

    abort_if_dirty();

    let mut lines = TokenStream::new();
    let mut bt_id = Ident::new("BlockType", Span::call_site());

    for line in identified_res.iter_mut() {
        let mut line_token_stream = TokenStream::new();
        for (ref mut en, var) in line.iter_mut() {
            match var {
                Some(id) => {
                    bt_id.set_span(id.span());
                    en.set_span(id.span());
                    line_token_stream.extend(quote!(#bt_id::#en(#en::#id), ));
                },
                None => {
                    bt_id.set_span(en.span());
                    line_token_stream.extend(quote!(#bt_id::#en, ));
                }
            }
        }
        let line_in = Group::new(Delimiter::Bracket, line_token_stream);
        lines.extend(quote!(vec!#line_in, ));
    }
    let lines_grouped = Group::new(Delimiter::Bracket, lines);
    let out = quote!(vec!#lines_grouped);
    // dbg!(&out.to_string());
    return quote!(#out).into()

}


#[proc_macro_error]
#[proc_macro]
pub fn parsetree2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // set_dummy(quote!("Vec::<Vec<BlockType>>::new()"));
    let input = proc_macro2::TokenStream::from(input);

    // let (enum_with_variants, all_variants) = get_valid_block_idents_from_files();

    let valid_special_tokens = vec!["*".to_string(), "$".to_string(), ",".to_string()];


    // VALIDATE THAT ONLY VALID VARIANT NAMES AND ALLOWED SPECIAL TOKENS ARE USED
    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Group(group) => {
                emit_error!(
                    group, "invalid tokens '{}'", group.to_string()
                )
            },
            TokenTree::Ident(ident) => {
                // check ident format
            },
            TokenTree::Punct(punct) => {
                if !valid_special_tokens.contains(&punct.to_string()) {
                    emit_error!(
                        punct,
                        "invalid token '{}'", punct.to_string()
                    )
                }
            },
            TokenTree::Literal(literal) => {
                emit_error!(
                    literal, "invalid token: found literal {}", literal.to_string();
                    note = "do this and that"
                );
            },
        }
    }
    
    abort_if_dirty();


    // check for $ in every row
    let mut found_dollar = false;
    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Punct(ident) => {
                if ident.to_string() == "$" && !found_dollar {
                    found_dollar = true;
                } else if ident.to_string() == "," && !found_dollar {
                    emit_error!(
                        ident,
                        "unexpected token ',' ";
                        hint = "every line must contain '$'"
                    )
                } else if ident.to_string() == "," {
                    found_dollar = false;
                } else if ident.to_string() == "$" && found_dollar {
                    emit_error!(
                        ident,
                        "token '$' can only appear once on a line";
                        info = "did you perhaps forget a ','"
                    )
                }
            },
            _ => {},
        }
    }

    abort_if_dirty();

    // parse into vec
    let mut literalified_res: Vec<Vec<TokenTree>> = vec![];
    //                             BlockType Variant, BlockTypeVariant, Variant
    //                            e.g. (Render, Some(Erithaxrender)) or (SELF, None)
    // blocktype variant or blocktypevariant variant gets span of corresponding input ident
    let mut curr_line: Vec<TokenTree> = vec![];

    for tokentree in input.clone().into_iter() {
        match tokentree {
            TokenTree::Punct(punct) => {
                if punct.to_string() == "$" {
                    let mut lit = proc_macro2::Literal::string("$");
                    lit.set_span(punct.span());
                    curr_line.push(TokenTree::Literal(lit));
                } else if punct.to_string() == "*" {
                    let mut lit = proc_macro2::Literal::string("*");
                    lit.set_span(punct.span());
                    curr_line.push(TokenTree::Literal(lit));
                } else if punct.to_string() == "," {
                    literalified_res.push(vec![]);
                    literalified_res.last_mut().unwrap().append(&mut curr_line);
                    curr_line.clear();
                }
            },
            TokenTree::Ident(ident) => {
                let id = ident.to_string();
                let mut lit = proc_macro2::Literal::string(ident.to_string().as_str());
                lit.set_span(ident.span());
                curr_line.push(TokenTree::Literal(lit));
            },
            _ => {}
        }
    }


    if curr_line.len() > 0 {
        literalified_res.push(curr_line);
    }

    // verify monotonically increasing stages
    // for line in identified_res.iter() {
    //     let mut curr_stage: usize = 0;
    //     let mut last_non_self_index = 0;
    //     let mut curr = &line[0];
    //     for i in 0..line.len()  {
    //         curr = &line[i];
    //         match &curr.1 {
    //             Some(id) => {
    //                 last_non_self_index = i;
    //                 if STAGES.get(&curr.0.to_string()).expect("HERIO") < &curr_stage {
    //                     emit_error!(
    //                         id,
    //                         "a block {} of type {} cannot follow a block of type {}", id.to_string(), curr.0.to_string(), line[last_non_self_index].0.to_string()
    //                     );
    //                 }
    //             },
    //             None => {
    //                 // curr is SELF or ALL
    //                 if curr.0.to_string() == "ALL" && i != line.len()-1 {
    //                     emit_error!(
    //                         curr.0,
    //                         "a block ALL can only occur at the end of a line";
    //                         info = "did you perhaps forget a ','?"
    //                     );
    //                 }
    //             }
    //         }
    //     }
    // }

    abort_if_dirty();

    let mut lines = TokenStream::new();
    let bt_id = Ident::new("BlockType", Span::call_site());

    for line in literalified_res.iter_mut() {
        let mut line_token_stream = TokenStream::new();
        for lit in line.iter_mut() {
            line_token_stream.extend(quote!(#lit,));
        }
        let line_in = Group::new(Delimiter::Bracket, line_token_stream);
        lines.extend(quote!(vec!#line_in, ));
    }
    let lines_grouped = Group::new(Delimiter::Bracket, lines);
    let out = quote!(vec!#lines_grouped);
    // dbg!(&out.to_string());
    return quote!(#out).into()
    // return quote!().into();
}



#[proc_macro]
pub fn TDS(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
    let mut ids: Vec<TokenTree> = Vec::new();
    for i in input.into_iter() { ids.push(i);}
    if ids.len() > 0 {
        panic!("TDS (TODOSTRING) takes 0 arguments");
    }
    return proc_macro::TokenStream::from_str("\"TODO\".to_string()").unwrap()
}


#[proc_macro_error]
#[proc_macro]
pub fn use_all_block_modules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = syn::parse_macro_input!(input);

    let mut ids: Vec<TokenTree> = Vec::new();

    for i in input.into_iter() {
        ids.push(i);
    }

    if ids.len() > 1 {
        panic!("more than one argument given to use_all_block_modules!()");
    }

    let mut out: TokenStream = TokenStream::new();

    out.extend(quote!(
        use macroni::{parsetree, TDS};
        use crate::common::{Blockify, BlockType, Info};
        use crate::{lang::Lang, owner::Owner};

        use dioxus::prelude::*;
    ));

    for i in FILE_NAMES.to_vec() {
        if i.to_string() != ids[0].to_string() {
            out.extend(TokenStream::from_str(&("use crate::".to_owned() + &i + "::{self, *};\n")));
        }
    }

    return out.into()
}
