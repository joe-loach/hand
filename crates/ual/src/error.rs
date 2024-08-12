use crate::grammar::SyntaxNode;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Unknown special item")]
    UnknownSpecial,
    #[error("Unknown Item")]
    UnknownItem,
    #[error("Unknown Character")]
    UnknownCharacter,
    #[error("Identifier missing")]
    NoIdent,
    #[error("Optional items cannot be nested")]
    Nesting,
}

#[derive(Debug, thiserror::Error)]
#[error("Parse error")]
pub struct ParseError {
    node: SyntaxNode,
    #[source]
    source: ErrorKind
}

impl ParseError {
    pub fn new(node: SyntaxNode, source: ErrorKind) -> Self {
        Self { node, source }
    }
}
