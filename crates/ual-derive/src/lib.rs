extern crate proc_macro;

mod attributes;
mod module;
mod process;
mod tokens;

use crate::module::module;
use crate::process::*;

use proc_macro::TokenStream;
use proc_macro_error2::*;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_error]
#[proc_macro_derive(UAL, attributes(ual))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let stream = match &input.data {
        Data::Struct(_) => process_struct(input),
        Data::Enum(_) => abort! {
            input, "Enums are unsupported";
            help =
            "Consider implementing UAL on a zero-sized struct for each variant,
            then holding them in an enum"
        },
        Data::Union(_) => abort!(input, "Unions are unsupported"),
    };

    stream.into()
}
