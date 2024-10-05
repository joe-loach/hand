use quote::{quote, ToTokens, TokenStreamExt};
use ual::lowering::{Fragment, Special};

use crate::module;

pub struct FragToken(pub(crate) Fragment);

impl ToTokens for FragToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = module();
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

pub struct SpecialToken(pub(crate) Special);

impl ToTokens for SpecialToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = module();
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