use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Expr, ExprLit, Field, Ident, Lit, LitStr};

pub(crate) fn derive(item: DeriveInput) -> Result<TokenStream, (Span, String)> {
    let syn::Data::Struct(ref data) = item.data else {
        return Err((item.span(), "Only structs can derive `Pattern`".to_string()));
    };

    if !item.generics.params.is_empty() {
        return Err((item.generics.span(), "No generics support".to_string()));
    }

    let name = item.ident;

    let name_attrs = item
        .attrs
        .iter()
        .filter_map(|atr| atr.meta.require_name_value().ok())
        .filter(|meta| meta.path.is_ident("name"))
        .collect::<Vec<_>>();

    let name_attr = match name_attrs.as_slice() {
        [name] => Some(name.value.clone()),
        [] => None,
        [_, ..] => {
            return Err((
                Span::call_site(),
                "Must only have one #[name = \"...\"] attribute".to_string(),
            ))
        }
    };

    let name_attr = name_attr.and_then(|expr| {
        if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str @ LitStr { .. }),
            ..
        }) = expr
        {
            Some(lit_str.value())
        } else {
            None
        }
    });

    impl_pattern(name, name_attr, &mut data.fields.iter())
}

fn impl_pattern(
    name: Ident,
    name_attr: Option<String>,
    fields: &mut dyn Iterator<Item = &Field>,
) -> Result<TokenStream, (Span, String)> {
    let mut tokens = fields.map(pattern_token).collect::<Vec<_>>();

    if let Some(name_attr) = name_attr {
        for c in name_attr.chars().rev() {
            tokens.insert(0, quote! { ::matcher::Pattern::Char(#c) });
        }
    }

    Ok(quote! {
        impl ::matcher::ConstPattern for #name {
            const PATTERN: &[::matcher::Pattern] = &[ #(#tokens),* ];
        }
    })
}

fn pattern_token(field: &Field) -> TokenStream {
    let ty = &field.ty;
    quote! { <#ty as ::matcher::PatternToken>::TOKEN }
}
