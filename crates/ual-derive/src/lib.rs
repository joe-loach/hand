extern crate proc_macro;

use std::sync::{Arc, LazyLock};

use miette::IntoDiagnostic;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Expr, ExprLit, Lit};
use ual::{
    lowering::{Fragment, Special},
    UAL,
};

struct FragToken(Fragment);

impl ToTokens for FragToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = &*MODULE;
        let fr_ty = quote! { #ual::lowering::Fragment };
        let fragment = match self.0 {
            Fragment::IdRange(pos) => quote! { #fr_ty::IdRange(#pos) },
            Fragment::Special(sp) => {
                let sp = SpecialToken(sp);
                quote! { #fr_ty::Special(#sp) }
            }
            Fragment::Byte(b) => quote! { #fr_ty::Byte(#b) },
            Fragment::ToggleOptional => quote! { #fr_ty::ToggleOptional },
        };
        tokens.append_all(fragment);
    }
}

struct SpecialToken(Special);

impl ToTokens for SpecialToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = &*MODULE;
        let sp_ty = quote! { #ual::lowering::Special };
        let special = match self.0 {
            Special::Register(rn) => quote! { #sp_ty::Register(#rn) },
            Special::Registers => quote! { #sp_ty::Registers },
            Special::Condition => quote! { #sp_ty::Condition },
            Special::Const => quote! { #sp_ty::Const },
            Special::Shift => quote! { #sp_ty::Shift },
            Special::ShiftAmount => quote! { #sp_ty::ShiftAmount },
            Special::Label => quote! { #sp_ty::Label },
            Special::Immediate => quote! { #sp_ty::Immediate },
        };
        tokens.append_all(special);
    }
}

static MODULE: LazyLock<Module> =
    LazyLock::new(
        || match crate_name("ual").expect("ual should be in `Cargo.toml`") {
            FoundCrate::Itself => Module("ual".to_string()),
            FoundCrate::Name(name) => Module(name),
        },
    );

struct Module(String);

impl quote::ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Ident::new(&self.0, Span::call_site()).to_tokens(tokens)
    }
}

#[proc_macro_error]
#[proc_macro_derive(UAL, attributes(ual))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let defs = input
        .attrs
        .iter()
        .filter_map(|atr| match atr.meta {
            syn::Meta::NameValue(ref pair) if pair.path.is_ident("ual") => Some(pair),
            _ => None,
        })
        .collect::<Vec<_>>();

    let Some(def) = defs.first() else {
        abort!(input, "A single UAL definition should be supplied");
    };

    let Expr::Lit(ExprLit {
        lit: Lit::Str(ual_str),
        ..
    }) = &def.value
    else {
        abort!(
            input,
            "UAL definition should be a key-value pair of \"ual = \"...\"\""
        );
    };

    let name = &input.ident;

    if let Data::Struct(syn::DataStruct { .. }) = &input.data {
        let res = UAL::parse(Arc::from(ual_str.value())).into_diagnostic();

        let ual = match res {
            Ok(ual) => ual,
            Err(errors) => {
                abort!(ual_str, errors)
            }
        };

        let frags = ual.iter().map(|f| FragToken(*f));
        // remember to set the span as the actual UAL text string
        let span = def.value.span();

        let ual = &*MODULE;
        quote_spanned! { span =>
            #[automatically_derived]
            impl #name {
                pub const UAL: #ual::Pattern<'static> = #ual::Pattern::new(::std::borrow::Cow::Borrowed(&[
                    #(#frags),*
                ]));
            }
        }
        .into()
    } else {
        panic!("UAL can only be used with named structs")
    }
}
