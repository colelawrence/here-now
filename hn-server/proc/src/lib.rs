#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};

/// Convenience macro for generating codegen attributes
///
/// example:
/// ```rs
/// #[protocol_type("agent")]
/// struct LocalKey(String);
/// // Expands to:
/// #[derive(serde::Serialize, serde::Deserialize)]
/// #[codegen(tags = "protocol-agent")]
/// struct LocalKey(String);
/// ```
/// Additional parameters are passed into the codegen attribute
/// ```rs
/// #[protocol_type("agent", import_from = "./global_id.ts")]
/// struct GlobalID(String, String);
/// // Expands to:
/// #[derive(serde::Serialize, serde::Deserialize)]
/// #[codegen(tags = "protocol-agent", import_from = "./global_id.ts")]
/// struct GlobalID(String, String);
/// ```
#[proc_macro_attribute]
pub fn protocol_type(
    input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let following = proc_macro2::TokenStream::from(following);
    let mut input_it = input.into_iter();
    let protocol_name = input_it
        .next()
        .expect("protocol attribute must have a value");
    let name = match protocol_name {
        TokenTree::Literal(lit) => {
            let value = lit.to_string();
            value[1..value.len() - 1].to_string()
        }
        other => {
            panic!("Unexpected token in protocol attribute: {:?}", other);
        }
    };

    let tag = format!("protocol-{name}");
    let attrs = input_it
        .map(proc_macro2::TokenTree::from)
        .collect::<TokenStream>();
    let codegen_attr = quote::quote! {#[codegen(tags = #tag #attrs)]};
    let mut output = TokenStream::from(quote::quote! {
        #[derive(Codegen, serde::Serialize, serde::Deserialize)] #codegen_attr
    });
    output.extend(following.into_iter());

    proc_macro::TokenStream::from(output)
}

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
