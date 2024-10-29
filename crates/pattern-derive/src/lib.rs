mod pattern;
mod tests;

use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn crate_name() -> proc_macro2::TokenStream {
    use proc_macro2::Span;
    use proc_macro_crate::FoundCrate;
    use quote::quote;
    use syn::Ident;

    let found_crate =
        proc_macro_crate::crate_name("matcher").expect("matcher is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}

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
