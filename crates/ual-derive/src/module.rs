use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use std::{borrow::Cow, sync::LazyLock};

pub fn module() -> Module<'static> {
    (*MODULE).borrow()
}

static MODULE: LazyLock<Module> =
    LazyLock::new(
        || match crate_name("ual").expect("ual should be in `Cargo.toml`") {
            FoundCrate::Itself => Module(Cow::Borrowed("ual")),
            FoundCrate::Name(name) => Module(Cow::Owned(name)),
        },
    );

pub struct Module<'a>(Cow<'a, str>);

impl<'a> quote::ToTokens for Module<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Ident::new(&self.0, Span::call_site()).to_tokens(tokens)
    }
}

impl<'a> Module<'a> {
    fn borrow<'b>(&'b self) -> Module<'b>
    where
        'b: 'a,
    {
        Self(Cow::Borrowed(self.0.as_ref()))
    }
}
