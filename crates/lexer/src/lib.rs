use std::ops::Range;

mod cursor;

pub trait Language: Sized {
    /// Tokens produced by the [`Lexer`].
    type Token;
    /// Errors produced by the [`Lexer`].
    type Error;

    /// Parses the next [`Token`](Language::Token) for the [`Language`].
    fn token(l: &mut Lexer<Self>, first: char) -> Self::Token;
}

#[derive(Debug)]
pub struct TokenInfo<L: Language> {
    pub tok: L::Token,
    pub start: u32,
    pub len: u32,
    pub error: Option<L::Error>,
}

impl<L: Language> TokenInfo<L> {
    pub fn text_range(&self) -> Range<usize> {
        self.start as usize..(self.start + self.len) as usize
    }
}

pub fn lex<L: Language>(mut text: &str) -> impl Iterator<Item = TokenInfo<L>> + '_ {
    let mut pos = 0_usize;

    std::iter::from_fn(move || {
        if text.is_empty() {
            return None;
        }

        let mut lexer = Lexer::new(text);
        let first = lexer.eat().expect("text is not empty");

        let tok = L::token(&mut lexer, first);

        let (consumed, rest) = lexer.cursor.finish();

        let start = pos as u32;
        pos += consumed;
        text = rest;

        Some(TokenInfo {
            tok,
            start,
            len: pos as u32 - start,
            error: lexer.error,
        })
    })
}

/// A streaming [`Lexer`] for a [`Language`].
pub struct Lexer<'t, L: Language> {
    pub(crate) cursor: cursor::Cursor<'t>,
    error: Option<L::Error>,
}

impl<'t, L: Language> Lexer<'t, L> {
    #[inline]
    pub(self) fn new(text: &'t str) -> Self {
        Self {
            cursor: cursor::Cursor::new(text),
            error: None,
        }
    }

    #[inline]
    pub fn error(&mut self, err: L::Error) {
        let _ = self.error.replace(err);
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
