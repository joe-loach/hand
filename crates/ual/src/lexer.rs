use std::sync::Arc;

use lexer::{Lexer, TokenStream};

use crate::{SyntaxKind, UAL};

pub fn lex(text: Arc<str>) -> TokenStream<UAL> {
    lexer::lex::<UAL>(text)
}

impl lexer::Lexable for UAL {
    type Token = SyntaxKind;

    fn next(l: &mut Lexer, first: char) -> Self::Token {
        match first {
            c if is_whitespace(c) => {
                l.eat_while(is_whitespace);
                SyntaxKind::Whitespace
            }

            c if is_ident(c) => {
                l.eat_while(is_ident_continue);
                SyntaxKind::Ident
            }

            '{' => SyntaxKind::OpenCurly,
            '}' => SyntaxKind::CloseCurly,
            '<' => SyntaxKind::OpenAngled,
            '>' => SyntaxKind::CloseAngled,
            '#' => SyntaxKind::Hash,
            ',' => SyntaxKind::Comma,
            '+' => SyntaxKind::Plus,
            '-' => SyntaxKind::Minus,

            _ => SyntaxKind::Unknown,
        }
    }
}

impl lexer::Token for SyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, SyntaxKind::Whitespace)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, SyntaxKind::Whitespace)
    }
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_ident_continue(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

#[test]
fn lexing() {
    use std::sync::Arc;

    let text = Arc::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let toks = lexer::lex::<UAL>(Arc::clone(&text));

    for t in toks {
        print!("{}", &text[t.text_range()]);
    }
}
