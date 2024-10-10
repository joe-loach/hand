mod ast;
mod grammar;
mod lexer;
mod lowering;
mod syntax;

use std::sync::Arc;

use ast::AstNode as _;
pub use lowering::Fragment;
use parser::rowan;
use syntax::SyntaxKind;


/// loop:
/// ADD r0, r1, #1
/// CMP r0, #100
/// BLT loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HAND {}

impl HAND {
    pub fn parse(text: Arc<str>) -> Result<Vec<Fragment>, ()> {
        let tree = crate::grammar::parse(text.clone());
        let root = crate::ast::Root::cast(tree).expect("grammar starts at root");

        // TODO: error handling
        // let mut errors = Vec::new();
        // crate::ast::validate(root.clone(), &mut errors);

        let frags = lowering::lower(root /*, &mut errors*/);

        // TODO: error handling
        // if !errors.is_empty() {
        //     return Err(Errors {
        //         src: text.clone(),
        //         inner: errors,
        //     });
        // }

        Ok(frags)
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
