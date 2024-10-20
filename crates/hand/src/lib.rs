mod ast;
mod grammar;
mod lexer;
mod lowering;
mod syntax;

use std::sync::Arc;

use ast::AstNode as _;
pub use lowering::{AddressKind, Fragment};
use parser::rowan;
use syntax::SyntaxKind;

#[test]
fn it_works() {
    let text = "loop: ADD r0, r1, #1\n\
                SUB r0, r0, r1\n\
                CMP r0, r1, #1\n\
                ADR r1, loop\n\
                BEQ loop\n\
                STMDB SP!, {R0-R4, SP}\n\n\
                SUBEQ r0, r1, #5\n\
                LDR r2, [r3]\n\
                LDR r2, [r3, #1]\n\
                LDR r2, [r3, r4]\n\
                LDR r2, [r3, r4, LSL #2]\n\
                LDR r2, [r3, r4, LSL #2]!\n\
                LDR r2, [r3, r4, LSL r5]!\n\
                LDR r2, [r3], r4\n\
                HLT";
    let text = Arc::<str>::from(text);
    let frags = parse(text);
    dbg!(frags);
}

#[derive(Debug)]
pub struct ParseResult {
    text: Arc<str>,
    fragments: Vec<Fragment>,
    // errors: Vec<Error>?
}

impl ParseResult {
    pub fn source(&self) -> &str {
        &self.text
    }

    pub fn fragments(&self) -> &[Fragment] {
        &self.fragments
    }
}

/// loop:
/// ADD r0, r1, #1
/// CMP r0, #100
/// BLT loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HAND {}

pub fn parse(text: Arc<str>) -> ParseResult {
    let tree = crate::grammar::parse(text.clone());
    let root = crate::ast::Root::cast(tree).expect("grammar starts at root");

    // TODO: error handling
    // let mut errors = Vec::new();
    // crate::ast::validate(root.clone(), &mut errors);

    let fragments = lowering::lower(root /*, &mut errors*/);

    // TODO: error handling
    // if !errors.is_empty() {
    //     return Err(Errors {
    //         src: text.clone(),
    //         inner: errors,
    //     });
    // }

    ParseResult { text, fragments }
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
