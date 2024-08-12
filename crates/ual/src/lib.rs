mod ast;
mod error;
mod grammar;
mod lexer;
mod syntax;

use parser::rowan;
use syntax::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UAL {}

impl rowan::Language for UAL {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        assert!(raw.0 <= SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[test]
fn usage() {
    use ast::AstNode as _;

    let mut errors = Vec::new();

    let text = std::rc::Rc::<str>::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const> <");
    let tree = crate::grammar::parse(text.clone());
    let root = crate::ast::Root::cast(tree).unwrap();

    crate::ast::validate(root, &mut errors);

    println!("{errors:?}");
}