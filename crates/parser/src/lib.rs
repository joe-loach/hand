use std::iter::Peekable;

use common::{Filterable, Language};
use lexer::TokenInfo;
use rowan::GreenNodeBuilder;

pub struct Parser<'t, L, I>
where
    L: Language,
    I: Iterator<Item = TokenInfo<L>>,
{
    text: &'t str,
    builder: GreenNodeBuilder<'static>,
    tokens: Peekable<I>,
}

impl<'t, L, I> Parser<'t, L, I>
where
    L: Language,
    I: Iterator<Item = TokenInfo<L>>,
{
    pub fn new(text: &'t str, tokens: I) -> Self {
        Self {
            text,
            builder: GreenNodeBuilder::new(),
            tokens: tokens.peekable(),
        }
    }
}

impl<'t, L, I> Parser<'t, L, I>
where
    L: Language,
    I: Iterator<Item = TokenInfo<L>>,
    <L as rowan::Language>::Kind: Filterable,
    <L as rowan::Language>::Kind: Into<rowan::SyntaxKind>,
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
}

pub struct Node {
    finished: bool,
}

impl Node {
    pub fn new() -> Self {
        Node { finished: false }
    }

    pub fn finish<'t, L, I>(mut self, p: &mut Parser<'t, L, I>)
    where
        L: Language,
        I: Iterator<Item = TokenInfo<L>>,
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
