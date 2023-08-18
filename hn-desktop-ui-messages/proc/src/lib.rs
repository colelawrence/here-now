#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro2::TokenStream;

/// Convenience macro for marking messages going to the UI.
#[proc_macro_attribute]
pub fn to_ui(
    _input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let following = proc_macro2::TokenStream::from(following);

    let doc = format!("📲 Message sent to the slint ui.\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
    });
    output.extend(following.into_iter());
    proc_macro::TokenStream::from(output)
}

/// Convenience macro for marking messages going to the Executor.
#[proc_macro_attribute]
pub fn to_executor(
    _input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let following = proc_macro2::TokenStream::from(following);

    let doc = format!("☎️ Message sent to the executor / backend.\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
    });
    output.extend(following.into_iter());
    proc_macro::TokenStream::from(output)
}

/// Convenience macro for marking messages going to the Executor.
#[proc_macro_attribute]
pub fn shared(
    _input: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let following = proc_macro2::TokenStream::from(following);

    let doc = format!("Used in messages to the executor and to the ui.\n");
    let mut output = TokenStream::from(quote::quote! {
        #[doc = #doc]
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
    });
    output.extend(following.into_iter());
    proc_macro::TokenStream::from(output)
}
