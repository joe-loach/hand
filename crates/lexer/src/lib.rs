use std::ops::Range;

use common::Language;

mod cursor;

#[derive(Debug)]
pub struct TokenInfo<L: Language> {
    pub tok: L::Kind,
    pub start: u32,
    pub len: u32,
    pub error: Option<L::Error>,
}

impl<L: Language> TokenInfo<L> {
    pub fn text_range(&self) -> Range<usize> {
        self.start as usize..(self.start + self.len) as usize
    }
}

pub fn lex<L: Language>(
    mut text: &str,
    next: impl Fn(&mut Lexer<L>, char) -> L::Kind + 'static,
) -> impl Iterator<Item = TokenInfo<L>> + '_ {
    let mut pos = 0_usize;

    std::iter::from_fn(move || {
        if text.is_empty() {
            return None;
        }

        let mut lexer = Lexer::new(text);
        let first = lexer.eat().expect("text is not empty");

        let tok = next(&mut lexer, first);

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
