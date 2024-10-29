use proc_macro::TokenStream;
use proc_macro_error2::{abort, proc_macro_error};
use quote::quote;
use syn::{ext::IdentExt, parse::Parse, parse_macro_input, Ident, LitInt, Token};

struct EncodeStream {
    items: Vec<proc_macro2::TokenStream>,
}

impl Parse for EncodeStream {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = EncodeStream {
            items: Vec::new(),
        };
        for _ in 0..32 {
            if input.is_empty() {
                break;
            }
            // could be either ident, '|', or int literal
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident::peek_any) {
                let id: Ident = input.parse()?;
                this.items.push(quote! { encode(#id) });

            } else if lookahead.peek(Token![|]) {
                let _: Token![|] = input.parse()?;
                // ignore this!
            } else if lookahead.peek(LitInt) {
                let lit: LitInt = input.parse()?;
                let value = lit.base10_parse::<u32>()?;
    
                if value > 1 {
                    abort!(lit, "Int literals must be a binary value, 0 or 1");
                }

                this.items.push(quote! { encode(#value) });
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(this)
    }
}

pub(crate) fn crate_name() -> proc_macro2::TokenStream {
    use proc_macro2::Span;
    use proc_macro_crate::FoundCrate;
    use quote::quote;
    use syn::Ident;

    let found_crate =
        proc_macro_crate::crate_name("enc").expect("enc is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}

/// Defines the encoding schema for an instruction in an ARM-Spec-like format.
#[proc_macro_error]
#[proc_macro]
pub fn encode(stream: TokenStream) -> TokenStream {
    let EncodeStream { items, .. } = parse_macro_input!(stream as EncodeStream);
    let module = crate_name();

    quote! {
        {
            #module::WordBuilder::new()
                #(. #items)*
                .finish()
        }
    }
    .into()
}
