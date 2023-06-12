use std::collections::VecDeque;
use std::str::FromStr;

use proc_macro2::{TokenStream, TokenTree, Spacing, Group, Literal, Delimiter};
use quote::quote;
use quote::format_ident;

use strum_macros::{Display, EnumIter};
use syn::{parse::{Parse, ParseStream, Result, ParseBuffer}, braced, parse_macro_input};
use syn::spanned::Spanned;


use proc_macro2::Punct;
use syn::{ItemStruct, Token, Ident, token};
use quote::TokenStreamExt;
use strum::IntoEnumIterator;

/*
EnumQualifier

Dioxus => Ui(Dioxus)
Taffy => Layout(Taffy)
*/

/*
Mainstuff

> Dioxus
> > Dom
> > CustomTree
> > > Taffy
> > > > Blitz
> > > > > Opengl
> > > > > Vulkan
> > > > BONK
> > > > > ALL

TO

Ui(Dioxus)
Ui(Dioxus), Ui(Dom)
Ui(Dioxus), Ui(CustomTree)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy), Render(Blitz)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy), Render(Blitz), Gfx(Opengl)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy), Render(Blitz), Gfx(Vulkan)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy), Render(BONK)
Ui(Dioxus), Ui(CustomTree), Layout(Taffy), Render(BONK), ALL
*/

#[derive(EnumIter, Display, Debug, PartialEq)]
pub enum Woosh {
    Plonk,
    Bam,
    Boof,
}

#[derive(Debug)]
struct PunctIdTree {
    nodes: Vec<VecDeque<Ident>>,
}

impl Parse for PunctIdTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let content: ParseBuffer;

        let _brace_token = braced!(content in input);
        
        let mut ids: VecDeque<Ident> = VecDeque::new();
        let mut puncts: VecDeque<Punct> = VecDeque::new();
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
            println!("==============================================");
            println!("{lines:?}");
            println!("{id_stack:?}");
            println!("{curr_line:?}");
            println!("token_lvl: {token_lvl:?}");
            println!("id: {}, pu: {}", content.peek(Ident), content.peek(Token![>]));
        

            if content.peek(Ident) {
                id = content.parse().unwrap();
                token_lvl += 1;
                
                if id_stack.len() > 0 && token_lvl-2 <= id_stack.len()-1 {
                    for i in token_lvl-2..=id_stack.len()-1 {
                        println!("{i}");
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



        /*
            lines = [
                [id1, id2, id3],
                [id1, id2, id4],
                [id1, id2, id5],
            ]
            e.g.
            lines = [
                [Wgpu, Opgengles, Glow, OpenGL, Windows],
                [Wgpu, Vulkan, Ash, Vulkan, Windows],
                [Wgpu, Vulkan, Ash, Vulkan, Android]
                [Wgpu, Vulkan, Ash, Vulkan, MoltenVK, Metal, Ios]
            ]
        */

        println!("lines: {lines:?}");


        return Ok(PunctIdTree {
            nodes: lines,
        })
        
    }
}


pub fn macro2() -> TokenStream {
    let s = "{
        > Plonk
        > > Bam
        > > Plonk Boof Woops
        > > > > Yes
        > Boof
    }";

    let tokens = TokenStream::from_str(s).unwrap();


    let mut punidtree: PunctIdTree = syn::parse2(tokens).expect("incorrect indent based identifier tree");

    let mut ids: Vec<VecDeque<Ident>> = punidtree.nodes;


    let mut qualified_ids: TokenStream = TokenStream::new();
    
    let  mut qual_id_line: TokenStream;

    for line in ids {
        qual_id_line = TokenStream::new();
        for id in line {
            for i in PolygonEnum::iter() {
                println!("{}", i.to_string());
                println!("{:?}", qualified_ids);

                let enumid: TokenStream = TokenStream::from_str("PolygonEnum").unwrap();
                let varid =  TokenStream::from_str(&i.to_string()).unwrap() ;
                let delim: Delimiter = proc_macro2::Delimiter::Parenthesis.clone();

                let mut s: TokenStream = TokenStream::new();
                s.extend(enumid);
                s.extend(Group::new(delim, varid).stream());

                qual_id_line.extend(s)
            }
            for i in WavyEnum::iter() {
                println!("{}", i.to_string());
                println!("{:?}", qualified_ids);

                let enumid: TokenStream = TokenStream::from_str("WaveEnum").unwrap();
                let varid =  TokenStream::from_str(&i.to_string()).unwrap() ;
                let delim: Delimiter = proc_macro2::Delimiter::Parenthesis.clone();

                let mut s: TokenStream = TokenStream::new();
                s.extend(enumid);
                s.extend(Group::new(delim, varid).stream());

                qual_id_line.extend(s)
            }
        }
        qualified_ids.extend(
            TokenStream::from_iter(
                vec![
                    TokenStream::from_str("vec").unwrap(),
                    TokenStream::from_str("!").unwrap(),
                    Group::new(Delimiter::Bracket, qual_id_line).stream()
                ]
            )
        )
    }

    

    return qualified_ids








    // let mut out_tokens = TokenStream::new();
    // //out_tokens.append(Token![{]);

    

    // while input.nodes.len() > 0 {

    //     // in array
    //     out_tokens.append(input.nodes.pop_front().unwrap());
    //     let pun = input.puncts.pop_front().unwrap();

    //     assert!(pun.as_char() == '>' || pun.as_char() == '<');

    //     if pun.as_char() == '>' {
    //         // out_tokens.append(Literal::from("[("));
    //     } else if pun.as_char() == '<' {
    //         // out_tokens.append(format!("]),\n"))
    //     }
    // }
    
    // println!("seri: {:?}", out_tokens);

    // let out_group = Group::new(proc_macro2::Delimiter::Brace, out_tokens);


    // println!("outgroup: {}", out_group.stream());


    // // let output: TokenStream = quote!{
    // //     {
    // //         #(#serialized)*
    // //     }
    // // };


    // // println!("output: {:?}", output);


    // println!("end reached :)");
    //println!("{}", tokens);
    // return out_group.stream()
}
