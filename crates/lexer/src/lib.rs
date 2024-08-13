use std::{marker::PhantomData, ops::Range, sync::Arc};

mod cursor;

#[derive(Debug)]
pub struct TokenInfo<T> {
    pub tok: T,
    pub start: u32,
    pub len: u32,
}

impl<L> TokenInfo<L> {
    pub fn text_range(&self) -> Range<usize> {
        self.start as usize..(self.start + self.len) as usize
    }
}

pub trait Lexable {
    type Token;

    fn next(lexer: &mut Lexer, first: char) -> Self::Token;
}

pub trait Token {
    /// Is trivial and can be filtered out.
    fn is_trivia(&self) -> bool;
    /// Represents whitespace.
    fn is_whitespace(&self) -> bool;
}

pub struct TokenStream<T> {
    text: Arc<str>,
    pos: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Lexable> Iterator for TokenStream<T> {
    type Item = TokenInfo<T::Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = &self.text[self.pos..];

        if text.is_empty() {
            return None;
        }

        let mut lexer = Lexer::new(text);
        let first = lexer.eat().expect("text is not empty");

        let tok = T::next(&mut lexer, first);

        let consumed = lexer.cursor.finish();

        let start = self.pos as u32;
        self.pos += consumed;

        Some(TokenInfo {
            tok,
            start,
            len: self.pos as u32 - start,
        })
    }
}

pub fn lex<L: Lexable>(text: Arc<str>) -> TokenStream<L> {
    TokenStream {
        text,
        pos: 0,
        _marker: PhantomData,
    }
}

/// A streaming [`Lexer`] for a [`Language`].
pub struct Lexer<'t> {
    pub(crate) cursor: cursor::Cursor<'t>,
}

impl<'t> Lexer<'t> {
    #[inline]
    pub(self) fn new(text: &'t str) -> Self {
        Self {
            cursor: cursor::Cursor::new(text),
        }
    }

    #[inline]
    pub fn eat(&mut self) -> Option<char> {
        self.cursor.eat()
    }

    #[inline]
    pub fn eat_while(&mut self, pred: impl Fn(char) -> bool) {
        self.cursor.eat_while(pred)
    }
}
