use lexer::Lexer;

use crate::{SyntaxKind, UAL};

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
    matches!(c, 'a'..='z' | 'A'..='Z')
}

fn is_ident_continue(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

#[test]
fn lexing() {
    use std::rc::Rc;

    let text = Rc::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let toks = lexer::lex::<UAL>(Rc::clone(&text));

    for t in toks {
        print!("{}", &text[t.text_range()]);
    }
}
