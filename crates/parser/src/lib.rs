use std::{iter::Peekable, sync::Arc};

use lexer::{Lexable, Token as _, TokenStream};
use rowan::{GreenNodeBuilder, SyntaxNode};

pub use rowan;

pub struct Parser<L>
where
    L: rowan::Language + Lexable,
{
    text: Arc<str>,
    builder: GreenNodeBuilder<'static>,
    tokens: Peekable<TokenStream<L>>,
}

impl<L> Parser<L>
where
    L: rowan::Language + Lexable,
{
    pub fn new(text: Arc<str>, tokens: TokenStream<L>) -> Self {
        Self {
            text,
            builder: GreenNodeBuilder::new(),
            tokens: tokens.peekable(),
        }
    }
}

impl<L> Parser<L>
where
    L: rowan::Language,
    L: Lexable<Token = L::Kind>,
    L::Token: lexer::Token + Into<rowan::SyntaxKind>,
{
    pub fn peek(&mut self) -> Option<L::Kind> {
        // consume all trivia tokens
        while self.tokens.peek().is_some_and(|t| t.tok.is_trivia()) {
            self.bump();
        }

        self.tokens.peek().map(|t| t.tok)
    }

    pub fn bump(&mut self) {
        if let Some(info) = self.tokens.next() {
            self.builder
                .token(info.tok.into(), &self.text[info.text_range()]);
        }
    }

    pub fn at(&mut self, expected: L::Kind) -> bool {
        self.peek().is_some_and(|kind| kind == expected)
    }

    pub fn start(&mut self, kind: L::Kind) -> Node {
        self.builder.start_node(kind.into());
        Node::new()
    }

    pub fn finish(self) -> SyntaxNode<L> {
        SyntaxNode::new_root(self.builder.finish())
    }
}

pub struct Node {
    finished: bool,
}

impl Node {
    pub fn new() -> Self {
        Node { finished: false }
    }

    pub fn finish<L>(mut self, p: &mut Parser<L>)
    where
        L: rowan::Language + lexer::Lexable,
    {
        self.finished = true;
        p.builder.finish_node()
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if !self.finished {
            panic!("Marker not finished!")
        }
    }
}
