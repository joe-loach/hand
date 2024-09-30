#[cfg(test)]
mod tests;

use crate::{syntax::SyntaxKind, HAND};
use lexer::{Lexable, TokenStream};
use std::sync::Arc;

pub fn lex(text: Arc<str>) -> TokenStream<HAND> {
    lexer::lex(text)
}

impl Lexable for HAND {
    type Token = SyntaxKind;

    fn next(lexer: &mut lexer::Lexer, first: char) -> Self::Token {
        use SyntaxKind::*;

        match first {
            '\n' => NewLine,

            c if is_whitespace(c) => {
                lexer.eat_while(is_whitespace_cons);
                Whitespace
            }

            c if is_ident(c) => {
                lexer.eat_while(is_ident_cons);
                Ident
            }

            ';' => {
                lexer.eat_while(|c| c != '\n');
                Comment
            }
            '/' => match lexer.peek() {
                Some('/') => {
                    lexer.eat();
                    lexer.eat_while(|c| c != '\n');
                    Comment
                }
                Some('*') => {
                    lexer.eat();
                    let mut indent = 1_usize;

                    while let Some(c) = lexer.eat() {
                        match (c, lexer.peek()) {
                            _ if indent == 0 => break,
                            ('/', Some('*')) => indent += 1,
                            ('*', Some('/')) => indent -= 1,
                            _ => (),
                        }
                    }

                    Comment
                }
                _ => Unknown,
            },

            c if is_number_start(c) => match (c, lexer.peek()) {
                ('0', Some('x' | 'X')) => {
                    lexer.eat();
                    lexer.eat_while(|c| matches!(c, '0'..='9' | 'A'..='F' | 'a'..='f' | '_'));
                    Hex
                }
                ('0', Some('o' | 'O')) => {
                    lexer.eat();
                    lexer.eat_while(|c| matches!(c, '0'..='7' | '_'));
                    Octal
                }
                ('0', Some('b' | 'B')) => {
                    lexer.eat();
                    lexer.eat_while(|c| matches!(c, '0'..='1' | '_'));
                    Binary
                }
                _ => {
                    lexer.eat_while(|c| matches!(c, '0'..='9' | '_'));
                    Decimal
                }
            },

            '{' => OpenCurly,
            '}' => CloseCurly,
            '[' => OpenSquare,
            ']' => CloseSquare,
            ',' => Comma,
            '#' => Hash,
            '+' => Plus,
            '-' => Minus,
            '!' => Bang,
            ':' => Colon,
            '=' => Equals,

            _ => Unknown,
        }
    }
}

impl lexer::Token for SyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, SyntaxKind::Whitespace | SyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, SyntaxKind::Whitespace)
    }
}

fn is_number_start(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn is_whitespace_cons(c: char) -> bool {
    // everything BUT newline
    matches!(c, '\t' | '\x0C' | '\r' | ' ')
}

fn is_ident(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_ident_cons(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}
