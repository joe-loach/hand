mod ast;
mod error;
mod grammar;
mod lexer;
mod syntax;
mod lowering;

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

    let text = std::rc::Rc::<str>::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let tree = crate::grammar::parse(text.clone());
    let root = crate::ast::Root::cast(tree).unwrap();

    let mut errors = Vec::new();
    crate::ast::validate(root.clone(), &mut errors);
    let frags = lowering::lower(root, None, &mut errors);

    println!("{frags:?}");
    println!("{errors:?}");
}