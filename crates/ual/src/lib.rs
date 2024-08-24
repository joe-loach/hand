mod ast;
mod error;
mod grammar;
mod lexer;
mod lowering;
mod syntax;

use std::sync::Arc;

use ast::AstNode as _;
use error::SyntaxError;
use intern::Interner;
use lowering::Fragment;
use parser::rowan;
use syntax::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UAL {}

impl UAL {
    pub fn parse(text: Arc<str>, interner: Option<&mut Interner>) -> Result<Pattern, Errors> {
        let tree = crate::grammar::parse(text.clone());
        let root = crate::ast::Root::cast(tree).expect("grammar starts at root");

        let mut errors = Vec::new();
        crate::ast::validate(root.clone(), &mut errors);

        let frags = lowering::lower(root, interner, &mut errors);

        if !errors.is_empty() {
            return Err(Errors { inner: errors });
        }

        Ok(Pattern(frags))
    }
}

pub struct Pattern(Vec<Fragment>);

impl Pattern {
    pub fn fragment(&self, idx: usize) -> Option<Fragment> {
        self.0.get(idx).copied()
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("UAL Errors")]
pub struct Errors {
    #[related]
    inner: Vec<SyntaxError>,
}

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

    let src = String::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let text = std::sync::Arc::<str>::from(src.as_str());

    let tree = crate::grammar::parse(text.clone());
    let root = crate::ast::Root::cast(tree).unwrap();

    let mut errors = Vec::new();
    crate::ast::validate(root.clone(), &mut errors);
    let frags = lowering::lower(root, None, &mut errors);

    println!("{frags:?}");
    for err in errors {
        let err: miette::Error = err.into();
        println!("{}", err.with_source_code(text.clone()));
    }
}
