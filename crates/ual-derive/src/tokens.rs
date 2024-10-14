use quote::{quote, ToTokens, TokenStreamExt};
use ual::{lowering::AddressKind, TextRange};
use ual::lowering::{Fragment, Special};

use crate::module;

pub struct FragToken(pub(crate) Fragment);

impl ToTokens for FragToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = module();
        let fr_ty = quote! { #ual::lowering::Fragment };
        let fragment = match self.0 {
            Fragment::Ident(range) => {
                let range = RangeToken(range);
                quote! { #fr_ty::Ident(#range) }
            },
            Fragment::Special(sp) => {
                let sp = SpecialToken(sp);
                quote! { #fr_ty::Special(#sp) }
            }
            Fragment::Byte(b) => quote! { #fr_ty::Byte(#b) },
            Fragment::Address(kind) => {
                let kind = AddressKindToken(kind);
                quote! { #fr_ty::Address(#kind) }
            },
        };
        tokens.append_all(fragment);
    }
}

struct RangeToken(TextRange);

impl ToTokens for RangeToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = module();
        let start: u32 = self.0.start().into();
        let end: u32 = self.0.end().into();

        tokens.append_all(quote! { #ual::TextRange::new(#ual::TextSize::new(#start), #ual::TextSize::new(#end)) });
    }
}

struct SpecialToken(Special);

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
            Special::Label => quote! { #sp_ty::Label },
            Special::Immediate => quote! { #sp_ty::Immediate },
        };
        tokens.append_all(special);
    }
}

struct AddressKindToken(AddressKind);

impl ToTokens for AddressKindToken {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ual = module();
        let adr_ty = quote! { #ual::lowering::AddressKind };
        let address_kind = match self.0 {
            AddressKind::Offset => quote! { #adr_ty::Offset },
            AddressKind::PreIndex => quote! { #adr_ty::PreIndex },
            AddressKind::PostIndex => quote! { #adr_ty::PostIndex },
        };
        tokens.append_all(address_kind);
    }
}
