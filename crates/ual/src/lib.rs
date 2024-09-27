mod ast;
mod error;
mod grammar;
mod lexer;
pub mod lowering;
mod syntax;

use std::borrow::Cow;
use std::sync::Arc;

use ast::AstNode as _;
use error::Errors;
use lowering::Fragment;
use parser::rowan;
use syntax::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UAL {}

impl UAL {
    pub fn parse(text: Arc<str>) -> Result<Pattern<'static>, Errors> {
        let tree = crate::grammar::parse(text.clone());
        let root = crate::ast::Root::cast(tree).expect("grammar starts at root");

        let mut errors = Vec::new();
        crate::ast::validate(root.clone(), &mut errors);

        let frags = lowering::lower(root, &mut errors);

        if !errors.is_empty() {
            return Err(Errors {
                src: text.clone(),
                inner: errors,
            });
        }

        Ok(Pattern::new(Cow::Owned(frags)))
    }
}

#[derive(Debug)]
pub struct Pattern<'a> {
    fragments: Cow<'a, [Fragment]>,
}

impl<'a> Pattern<'a> {
    pub const fn new(fragments: Cow<'a, [Fragment]>) -> Self {
        Self { fragments }
    }

    pub fn iter(&self) -> std::slice::Iter<Fragment> {
        self.fragments.iter()
    }

    pub fn fragments(&self) -> &[Fragment] {
        &self.fragments
    }
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
    use miette::IntoDiagnostic;

    let src = String::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let text = std::sync::Arc::<str>::from(src.as_str());

    let res = UAL::parse(text).into_diagnostic();

    match res {
        Ok(pat) => println!("{:?}", pat.fragments()),
        Err(report) => {
            eprintln!("{report}")
        }
    }
}
