extern crate proc_macro;
use proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};
use std::fmt::Error;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    let expanded = "fn answer() -> u32 { 42 }".parse().unwrap();
    expanded
}

#[proc_macro_derive(Jsonizer)]
pub fn derive_json(stream: TokenStream) -> TokenStream {
    let tokens = stream.into_iter().peekable();
    let mut new_tokens = String::from("impl Json for ");

    for token in tokens {
        match token {
            TokenTree::Ident(ident) => {
                if !(ident.to_string() == "pub" || ident.to_string() == "struct") {
                    new_tokens.push_str(ident.to_string().as_str());
                    new_tokens.push_str("{\n");
                    new_tokens.push_str("fn json(&self)-> Result<String, std::fmt::Error>{");
                    new_tokens.push_str("Ok(\"hello\".to_string())");
                    new_tokens.push_str("}\n");
                }
            }
            TokenTree::Group(group) => {
                // dbg!(group);
                let struct_body = handle_group(group.stream());
                new_tokens.push_str(struct_body.as_str());
            }
            _ => {
                println!("========================================");
                dbg!(token);
                println!("========================================");
            }
        }
    }
    new_tokens.push_str("}\n");
    dbg!(&new_tokens);
    new_tokens.as_str().parse().unwrap()
}

fn handle_struct_field(t: Ident) -> String {
    dbg!(t);
    "".to_string() 
}

fn handle_group(fields: TokenStream) -> String {
    let mut group_repr = String::from("");
    for field in fields {
        let mut my_struct = String::from("");
        match field.clone() {
            TokenTree::Punct(punct) => {
                my_struct = "".to_string();
            },
            TokenTree::Ident(ident) => {
                if ident.to_string() != "String" {
                    my_struct.push('"');
                    my_struct.push_str(ident.to_string().as_str());
                    my_struct.push('"');
                    my_struct.push(':');
                }
            },
            TokenTree::Group(_group) => {
            },
            _ => {
            }
        }
        if let TokenTree::Ident(field) = field {
                let field_repr = handle_struct_field(field);
                group_repr.push_str(field_repr.as_str());
        }
    }
    group_repr
}
