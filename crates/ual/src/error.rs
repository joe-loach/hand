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
    #[error("Missing matching brace")]
    Unclosed,
}

#[derive(Debug, thiserror::Error)]
#[error("Syntax error")]
pub struct SyntaxError {
    node: SyntaxNode,
    #[source]
    source: ErrorKind
}

impl SyntaxError {
    pub fn new(node: SyntaxNode, source: ErrorKind) -> Self {
        Self { node, source }
    }
}
