#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};

/// Convenience macro for marking ecs saved types
///
/// example:
/// ```rs
/// #[ecs_bundle(ItemTag)]
/// struct ECSKey(String);
/// // Expands to:
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct ECSKey(String);
/// ```
#[proc_macro_attribute]
pub fn ecs_bundle(
    input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let following = proc_macro2::TokenStream::from(following);
    let mut idents = Vec::new();

    for item in input.into_iter() {
        match item {
            TokenTree::Ident(ide) => idents.push(ide),
            TokenTree::Punct(punct) => {
                if punct.as_char() == ',' {
                    continue;
                }
            }
            other => {
                panic!("Unexpected token in ecs_component attribute: {other:?}\necs_bundle attribute can have a value like `ecs_bundle(CredTag)` to indicate what the entity kinds it's associated with.");
            }
        };
    }

    let ident_list_print = idents
        .iter()
        .map(|i| format!("[{i}]"))
        .collect::<Vec<_>>()
        .join(", ");

    let doc = format!("💠💠💠 Bundle saved to disk\nSee {ident_list_print}\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
    });
    output.extend(following.into_iter());
    // // check that the reference links
    // output.extend(quote::quote! {
    //     type _ = #ident;
    // });

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ecs_unique(
    input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let following = proc_macro2::TokenStream::from(following);
    if !input.is_empty() {
        panic!("Expecting no args passed to ecs_unique");
    }
    let doc = format!("💠 Unique component\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(shipyard::Component)]
        #[track(All)]
    });
    output.extend(following.into_iter());

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ecs_component(
    input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let following = proc_macro2::TokenStream::from(following);
    let mut input_it = input.into_iter();
    let mut idents = Vec::new();
    let tag_name = input_it
        .next()
        .expect("ecs_component attribute must have a value like `ecs_component(\"...\")` to indicate what kinds of bundle it's associated with.");
    let mut name = String::new();
    match tag_name {
        TokenTree::Literal(lit) => {
            let value = lit.to_string();
            name.push_str(&value[1..value.len() - 1]);
        }
        TokenTree::Ident(ide) => idents.push(ide),
        other => {
            panic!("Unexpected token in ecs_component attribute: {:?}", other);
        }
    };
    for tag_name in input_it {
        match tag_name {
            TokenTree::Punct(punct) => {
                if punct.as_char() == ',' {
                    continue;
                }
                panic!("Unexpected token in ecs_component attribute: {:?}", punct);
            }
            TokenTree::Ident(ide) => idents.push(ide),
            TokenTree::Literal(lit) => {
                let value = lit.to_string();
                if !name.is_empty() {
                    name.push_str(", ");
                }
                name.push_str(&value[1..value.len() - 1]);
            }
            other => {
                panic!("Unexpected token in ecs_component attribute: {:?}", other);
            }
        }
    }
    let ident = idents
        .iter()
        .map(|i| format!("[{i}]"))
        .collect::<Vec<_>>()
        .join(", ");
    if !ident.is_empty() {
        if !name.is_empty() {
            name.push_str(", ");
        }
        name.push_str(&ident);
    }
    let doc = format!("💠 Component used for {name}\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(shipyard::Component)]
        #[track(All)]
    });
    output.extend(following.into_iter());

    proc_macro::TokenStream::from(output)
}
