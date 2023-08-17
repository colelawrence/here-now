#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};

/// Convenience macro for marking ecs saved types
///
/// example:
/// ```rs
/// #[ecs]
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
        idents.push(match item {
            TokenTree::Ident(ide) => ide,
            // TokenTree::Literal(lit) => {
                //     let value = lit.to_string();
                //     value[1..value.len() - 1].to_string()
                // }
                other => {
                    panic!("Unexpected token in ecs_component attribute: {other:?}\necs_bundle attribute can have a value like `ecs_bundle(CredTag)` to indicate what the entity kinds it's associated with.");
        }});
    }

    let ident = idents
        .iter()
        .map(|i| format!("[{i}]"))
        .collect::<Vec<_>>()
        .join(", ");
    let doc = format!("💠💠💠 Bundle saved to disk\nSee [{ident}]\n");
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
    let tag_name = input_it
        .next()
        .expect("ecs_component attribute must have a value like `ecs_component(\"...\")` to indicate what kinds of bundle it's associated with.");
    let name = match tag_name {
        TokenTree::Literal(lit) => {
            let value = lit.to_string();
            value[1..value.len() - 1].to_string()
        }
        other => {
            panic!("Unexpected token in ecs_component attribute: {:?}", other);
        }
    };
    let doc = format!("💠 Component used for {name}\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(shipyard::Component)]
        #[track(All)]
    });
    output.extend(following.into_iter());

    proc_macro::TokenStream::from(output)
}
