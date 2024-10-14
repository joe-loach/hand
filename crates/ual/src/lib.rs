mod ast;
mod error;
mod grammar;
mod lexer;
pub mod lowering;
mod syntax;

use std::{borrow::Cow, sync::Arc};

use ast::AstNode as _;
use error::Errors;
use lowering::Fragment;
use parser::rowan;
use syntax::SyntaxKind;

pub use rowan::{TextRange, TextSize};

pub trait UalSyntax {
    type Source: Source;

    const PATTERN: Pattern<'_, Self::Source>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UAL {}

impl UAL {
    pub fn parse(text: Arc<str>) -> Result<Pattern<'static, Arc<str>>, Errors> {
        let tree = crate::grammar::parse(text.clone());
        let root = dbg!(crate::ast::Root::cast(tree).expect("grammar starts at root"));

        let mut errors = Vec::new();
        crate::ast::validate(root.clone(), &mut errors);

        let frags = lowering::lower(root, &mut errors);

        if !errors.is_empty() {
            return Err(Errors {
                src: text.clone(),
                inner: errors,
            });
        }

        Ok(Pattern::new(text, Cow::Owned(frags)))
    }
}

pub trait Source: AsRef<str> {
    fn resolve(&self, range: TextRange) -> &str {
        &self.as_ref()[range]
    }
}

impl Source for Arc<str> {}
impl Source for &str {}

pub type ArcPattern<'a> = Pattern<'a, Arc<str>>;

pub struct Pattern<'a, S: Source> {
    source: S,
    fragments: Cow<'a, [Fragment]>,
}

impl<'a, S: Source + std::fmt::Debug> std::fmt::Debug for Pattern<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "[")?;
            for frag in self.fragments.iter() {
                match frag {
                    Fragment::Address(_) => write!(f, "")?,
                    Fragment::Ident(range) => write!(f, "{}", self.source.resolve(*range))?,
                    Fragment::Special(special) => write!(f, "{:?}", special)?,
                    Fragment::Byte(b) if b.is_ascii_whitespace() => write!(f, " ")?,
                    Fragment::Byte(b) => write!(f, "{}", char::from_u32(*b as u32).unwrap())?,
                }
            }
            write!(f, "]")
        } else {
            f.debug_struct("Pattern")
                .field("fragments", &self.fragments)
                .finish()
        }
    }
}

impl<'a, S: Source> Pattern<'a, S> {
    pub const fn new(source: S, fragments: Cow<'a, [Fragment]>) -> Self {
        Self { source, fragments }
    }

    pub fn into_owned_source(self) -> ArcPattern<'a> {
        Pattern {
            source: Arc::from(self.source.as_ref()),
            fragments: self.fragments,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Fragment> {
        self.fragments.iter()
    }

    pub fn fragments(&self) -> &[Fragment] {
        &self.fragments
    }

    pub fn source(&self) -> &str {
        self.source.as_ref()
    }
}

impl<'a> From<Pattern<'a, &str>> for ArcPattern<'a> {
    fn from(value: Pattern<'a, &str>) -> Self {
        value.into_owned_source()
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

    let text = Arc::from("ADD<c> <Rd>, <Rn>, #<const>");
    // let text = Arc::from("LDR<c> <Rt>, [<Rn>, <Rm>], <shift>");
    // let text = Arc::from("LDR <Rt>, [<Rn>], #<imm>");
    let res = UAL::parse(text).into_diagnostic();

    match res {
        Ok(pat) => println!("{:#?}", pat.fragments()),
        Err(report) => {
            eprintln!("{report}")
        }
    }
}
