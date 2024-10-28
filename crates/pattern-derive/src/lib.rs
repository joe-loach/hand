mod pattern;
mod tests;

use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_error]
#[proc_macro_derive(Pattern, attributes(name))]
pub fn derive_pattern(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let res = pattern::derive(input);

    match res {
        Ok(it) => it.into(),
        Err((span, msg)) => abort!(span, msg),
    }
}
