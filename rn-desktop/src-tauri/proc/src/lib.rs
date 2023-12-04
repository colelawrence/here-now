#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};

/// Convenience macro for generating tauri commands
///
/// example:
/// ```rs
/// #[ui_command]
/// async fn start_session(app: tauri::AppHandle) -> Result<(), Error> {
///     Ok(())
/// }
/// // Expands to:
/// #[fn_codegen]
/// #[codegen(tauri_command, tauri_plugin = "RightNowTodos", tags = "rn-ui")]
/// #[tauri::command(async)]
/// #[tracing::instrument(skip(app))]
/// async fn start_session(app: tauri::AppHandle) -> Result<(), Error> {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn ui_command(
    _attributes: proc_macro::TokenStream,
    following: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // let input = proc_macro2::TokenStream::from(_attributes);
    let following = proc_macro2::TokenStream::from(following);
    let mut output = TokenStream::from(quote::quote! {
        #[derive_codegen::fn_codegen]
        #[derive_codegen::i_codegen_derive::codegen(tauri_command, tauri_plugin = "RightNowTodos", tags = "rn-ui")]
        #[tauri::command(async)]
        #[tracing::instrument(skip(app))]
    });
    output.extend(following.into_iter());

    proc_macro::TokenStream::from(output)
}
