mod node;
mod validate;
mod token;

pub use node::*;
pub use token::*;

use crate::{
    grammar::{SyntaxNode, SyntaxToken},
    syntax::SyntaxKind,
};

pub trait AstNode: Sized {
    fn castable(kind: SyntaxKind) -> bool;
    fn cast(node: SyntaxNode) -> Option<Self>;
    fn syntax(&self) -> &SyntaxNode;
}

pub trait AstToken: Sized {
    fn castable(kind: SyntaxKind) -> bool;
    fn cast(node: SyntaxToken) -> Option<Self>;
    fn syntax(&self) -> &SyntaxToken;
    fn text(&self) -> &str {
        self.syntax().text()
    }
}
