use std::collections::VecDeque;
use std::str::FromStr;

use proc_macro2::{TokenStream, Group, Literal};
use quote::quote;
use quote::format_ident;

use syn::{parse::{Parse, ParseStream, Result, ParseBuffer}, braced, parse_macro_input};
use syn::spanned::Spanned;


use proc_macro2::Punct;
use syn::{ItemStruct, Token, Ident};
use quote::TokenStreamExt;

/*

Block \
    Block \
        Block
        Block
        Block /
    Block /
Block

 */

/*
    [
        enum::var(
            
        )
    ]

 */

fn macro1() {
    let s = "{
        Wopper >
            Yeya
            Woosh <
        Waaa
    }";

    let tokens = TokenStream::from_str(s).unwrap();


    let mut input: IndentIDTree = syn::parse2(tokens).expect("incorrect indent based identifier tree");

    assert!(input.nodes.len() > input.puncts.len(), "nodes.len() was {} and puncts.len() was {}", input.nodes.len(), input.puncts.len() );
    
    let mut serialized: Vec<Ident> = Vec::new();


    /*while input.nodes.len() > 0 {
        serialized.push(input.nodes.pop_front().unwrap());
        let pun = input.puncts.pop_front().unwrap();

        assert!(pun.as_char() == '>' || pun.as_char() == '<');

        if pun.as_char() == '>' {
            serialized.push(format_ident!("[("))
        } else if pun.as_char() == '<' {
            serialized.push(format_ident!("]),\n"))
        }
        serialized.push(format_ident!(",\n"))
    }*/

    let mut out_tokens = TokenStream::new();
    //out_tokens.append(Token![{]);

    

    while input.nodes.len() > 0 {

        // in array
        out_tokens.append(input.nodes.pop_front().unwrap());
        let pun = input.puncts.pop_front().unwrap();

        assert!(pun.as_char() == '>' || pun.as_char() == '<');

        if pun.as_char() == '>' {
            // out_tokens.append(Literal::from("[("));
        } else if pun.as_char() == '<' {
            // out_tokens.append(format!("]),\n"))
        }
    }
    
    println!("seri: {:?}", out_tokens);

    let out_group = Group::new(proc_macro2::Delimiter::Brace, out_tokens);


    println!("outgroup: {}", out_group.stream());


    // let output: TokenStream = quote!{
    //     {
    //         #(#serialized)*
    //     }
    // };


    // println!("output: {:?}", output);


    println!("end reached :)");
    //println!("{}", tokens);
    // return out_group.stream()
}

#[derive(Debug)]
struct IndentIDTree {
    nodes: VecDeque<Ident>,
    puncts: VecDeque<Punct>
}

impl Parse for IndentIDTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let content: ParseBuffer;
        let _brace_token = braced!(content in input);
        
        let mut ids: VecDeque<Ident> = VecDeque::new();
        let mut puncts: VecDeque<Punct> = VecDeque::new();
        
        loop {
            let id: Result<Ident> = content.parse();

            let pun: Result<Punct> = content.parse();

            match (id, pun) {
                (Err(_), Err(_)) => {break},
                (Ok(i), Err(_)) => {ids.push_back(i)},
                (Err(_), Ok(j)) => {puncts.push_back(j)},
                (Ok(i), Ok(j)) => {
                    ids.push_back(i);
                    puncts.push_back(j);
                },
            }


        }

        println!("ids: {:?}", ids);
        println!("puncts: {:?}", puncts);
        println!("content: {:?}", content);

        return Ok(IndentIDTree {
            nodes: ids,
            puncts: puncts

        })
        
    }
}


fn mainOTHER() {
    // struct sample
    let s = "struct Point { x : u16 , y : u16 }";

    // create a new token stream from our string
    let tokens = TokenStream::from_str(s).unwrap();

    // build the AST: note the syn::parse2() method rather than the syn::parse() one
    // which is meant for "real" procedural macros
    let ast: ItemStruct = syn::parse2(tokens).unwrap();

    // save our struct type for future use
    let struct_type = ast.ident.to_string();
    assert_eq!(struct_type, "Point");

    // we have 2 fields
    assert_eq!(ast.fields.len(), 2);

    // syn::Fields is implementing the Iterator trait, so we can iterate through the fields
    let mut iter = ast.fields.iter();

    // this is x
    let x_field = iter.next().unwrap();
    assert_eq!(x_field.ident.as_ref().unwrap(), "x");

    // this is y
    let y_field = iter.next().unwrap();
    assert_eq!(y_field.ident.as_ref().unwrap(), "y");

    // now the most tricky part: use the quote!() macro to generate code, aka a new
    // TokenStream

    // first, build our function name: point_summation
    let function_name = format_ident!("{}_summation", struct_type.to_lowercase());

    // and our argument type. If we don't use the format ident macro, the function prototype
    // will be: pub fn point_summation (pt : "Point")
    let argument_type = format_ident!("{}", struct_type);

    // same for x and y
    let x = format_ident!("{}", x_field.ident.as_ref().unwrap());
    let y = format_ident!("{}", y_field.ident.as_ref().unwrap());

    // the quote!() macro is returning a new TokenStream. This TokenStream is returned to
    // the compiler in a "real" procedural macro
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            pt.#x + pt.#y
        }
    };

    // output our function as Rust code
    println!("{}", summation_fn);
}






