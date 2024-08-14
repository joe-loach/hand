use miette::SourceSpan;

use crate::grammar::SyntaxNode;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Unknown special item")]
    UnknownSpecial,
    #[error("Unknown Item")]
    UnknownItem,
    // #[error("Unknown Character")]
    // UnknownCharacter,
    #[error("Identifier missing")]
    NoIdent,
    #[error("Optional items cannot be nested")]
    Nesting,
    #[error("Missing matching brace")]
    UnClosed,
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Syntax error")]
#[diagnostic()]
pub struct SyntaxError {
    #[label]
    span: SourceSpan,
    #[source]
    source: ErrorKind,
}

impl SyntaxError {
    pub fn new(node: SyntaxNode, source: ErrorKind) -> Self {
        let range = node.text_range();
        let start: usize = range.start().into();
        let len: usize = range.len().into();
        let span = (start, len).into();
        Self { span, source }
    }
}
