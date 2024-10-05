use proc_macro_error2::emit_error;
use syn::{Attribute, MetaNameValue};

pub fn collect(attrs: &[Attribute]) -> Vec<syn::MetaNameValue> {
    attrs
        .iter()
        .filter_map(|attr| match parse(attr) {
            Some(Ok(it)) => Some(it),
            Some(Err(err)) => {
                emit_error!(attr, "Invalid attribute: {}", err);
                None
            }
            // Ignore other attributes
            None => None,
        })
        .collect::<Vec<_>>()
}

pub fn parse(attr: &Attribute) -> Option<syn::Result<MetaNameValue>> {
    if !attr.path().is_ident("ual") {
        return None;
    }

    Some(attr.meta.require_name_value().cloned())
}
