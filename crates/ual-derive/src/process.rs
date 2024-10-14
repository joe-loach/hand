use std::sync::Arc;

use crate::tokens::FragToken;
use crate::{attributes, module::module};

use miette::IntoDiagnostic;
use proc_macro_error2::{abort, abort_if_dirty};
use quote::quote_spanned;
use syn::{spanned::Spanned, DeriveInput, Expr, ExprLit, Lit};
use ual::UAL;

pub fn process_struct(input: DeriveInput) -> proc_macro2::TokenStream {
    let attribs = attributes::collect(&input.attrs);
    abort_if_dirty();

    let def = match attribs.as_slice() {
        [] => abort!(input, "Requires a ual attribute '#[ual = \"...\"])"),
        [def] => def,
        [_, ..] => abort!(input, "Structs should only have one ual attribute"),
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

    let text: Arc<str> = Arc::from(ual_str.value());
    let res = UAL::parse(text.clone()).into_diagnostic();

    let frags = match res {
        Ok(ual) => ual,
        Err(errors) => {
            abort!(ual_str, errors)
        }
    };

    let name = input.ident;

    let frags = frags.fragments().iter().map(|frag| FragToken(*frag));

    // remember to set the span as the actual UAL text string
    let span = def.value.span();

    let ual = module();
    let text = text.as_ref();

    quote_spanned! { span =>
        #[automatically_derived]
        impl #ual::UalSyntax for #name {
            type Source = &'static str;

            const PATTERN: #ual::Pattern<'static, &str> = #ual::Pattern::new(#text, ::std::borrow::Cow::Borrowed(&[
                #(#frags),*
            ]));
        }
    }
}
