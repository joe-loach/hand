mod grammar;
mod lexer;
mod syntax;
mod ast;

use std::sync::Arc;

use parser::rowan;
use syntax::SyntaxKind;
use ual_derive::UAL;

/// loop:
/// ADD r0, r1, #1
/// CMP r0, #100
/// BLT loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HAND {}

impl HAND {
    pub fn parse(text: Arc<str>) {
        todo!()
    }
}

impl rowan::Language for HAND {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        assert!(raw.0 <= SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(UAL)]
#[ual = "ADD{S}{<c>} {<Rd>,} <Rn>, #<const>"]
pub struct ADD;