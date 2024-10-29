use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn crate_name() -> proc_macro2::TokenStream {
    use proc_macro2::Span;
    use proc_macro_crate::FoundCrate;
    use quote::quote;
    use syn::Ident;

    let found_crate = proc_macro_crate::crate_name("cir").expect("cir is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}

#[proc_macro_error]
#[proc_macro_derive(Structured, attributes(name))]
pub fn derive_structure(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive(input).into()
}

fn derive(input: DeriveInput) -> proc_macro2::TokenStream {
    let module = crate_name();
    let name = input.ident;

    let syn::Data::Struct(data_struct) = input.data else {
        panic!("Only supports structs")
    };

    let fields = data_struct.fields;
    let members = fields.members();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #module::structured::Structured for #name #ty_generics #where_clause {
            fn parse(buffer: &mut #module::structured::Buffer) -> Option<Self> {
                Some(Self { #(#members: buffer.parse()?),* })
            }
        }
    }
}
