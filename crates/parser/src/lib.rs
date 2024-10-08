use std::{iter::Peekable, sync::Arc};

use lexer::{Lexable, Token as _, TokenInfo, TokenStream};
use rowan::{Checkpoint, GreenNodeBuilder, SyntaxNode};

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
    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<L::Kind> {
        self.peek_token().map(|t| t.tok)
    }

    pub fn bump(&mut self, kind: L::Kind) {
        debug_assert!(self.at(kind));
        self.bump_inner();
    }

    pub fn bump_any(&mut self) {
        self.bump_inner();
    }

    pub fn at(&mut self, kind: L::Kind) -> bool {
        self.peek().is_some_and(|peek| peek == kind)
    }

    pub fn eat(&mut self, kind: L::Kind) -> bool {
        if self.at(kind) {
            self.bump(kind);
            true
        } else {
            false
        }
    }

    pub fn text<'a, 'b>(&'a mut self) -> Option<&'b str>
    where
        'a: 'b,
    {
        let token_range = self.peek_token().map(TokenInfo::text_range)?;
        Some(&self.text[token_range])
    }

    pub fn start(&mut self) -> Marker {
        Marker::new(self.checkpoint())
    }

    pub fn emit(&mut self, kind: L::Kind) {
        Marker::new(self.checkpoint()).finish(self, kind);
    }

    pub fn finish(self) -> SyntaxNode<L> {
        SyntaxNode::new_root(self.builder.finish())
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn peek_token(&mut self) -> Option<&TokenInfo<L::Kind>> {
        self.skip_trivia();
        self.tokens.peek()
    }

    fn skip_trivia(&mut self) {
        while self.tokens.peek().is_some_and(|t| t.tok.is_trivia()) {
            self.bump_inner();
        }
    }

    fn bump_inner(&mut self) {
        if let Some(info) = self.tokens.next() {
            self.builder
                .token(info.tok.into(), &self.text[info.text_range()]);
        }
    }
}

pub struct Marker {
    checkpoint: Checkpoint,
    finished: bool,
}

impl Marker {
    fn new(checkpoint: Checkpoint) -> Self {
        Self {
            checkpoint,
            finished: false,
        }
    }

    pub fn finish<L>(mut self, p: &mut Parser<L>, kind: L::Kind)
    where
        L: rowan::Language + lexer::Lexable,
        L::Kind: Into<rowan::SyntaxKind>,
    {
        self.finished = true;
        p.builder.start_node_at(self.checkpoint, kind.into());
        p.builder.finish_node()
    }

    pub fn abandon(mut self) {
        self.finished = true;
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        if !self.finished {
            panic!("Marker not finished!")
        }
    }
}
