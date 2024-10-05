use std::sync::Arc;

use crate::{attributes, module::module};
use crate::tokens::FragToken;

use miette::IntoDiagnostic;
use proc_macro_error2::{abort, abort_if_dirty};
use quote::quote_spanned;
use syn::{spanned::Spanned, DeriveInput, Expr, ExprLit, Lit};
use ual::UAL;

pub fn process_struct(input: DeriveInput) -> proc_macro2::TokenStream {
    let attribs = attributes::collect(&input.attrs);
    abort_if_dirty();

    let [def] = attribs.as_slice() else {
        abort!(input, "Structs should only have one ual attribute");
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

    let res = UAL::parse(Arc::from(ual_str.value())).into_diagnostic();

    let ual = match res {
        Ok(ual) => ual,
        Err(errors) => {
            abort!(ual_str, errors)
        }
    };

    let name = input.ident;

    let frags = ual.iter().map(|f| FragToken(*f));
    // remember to set the span as the actual UAL text string
    let span = def.value.span();

    let ual = module();

    quote_spanned! { span =>
        #[automatically_derived]
        impl #ual::UalSyntax for #name {
            const PATTERN: #ual::Pattern<'static> = #ual::Pattern::new(::std::borrow::Cow::Borrowed(&[
                #(#frags),*
            ]));
        }
    }
}