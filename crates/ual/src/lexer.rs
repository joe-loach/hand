use lexer::{Lexer, TokenInfo};

use crate::{SyntaxKind, UAL};

pub fn lex(text: &str) -> impl Iterator<Item = TokenInfo<UAL>> + '_ {
    lexer::lex::<UAL>(text, token)
}

fn token(l: &mut Lexer<UAL>, first: char) -> SyntaxKind {
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
    let text = "ADD{S}{<c>} {<Rd>,} <Rn>, #<const>";
    let toks = lex(text);
    for t in toks {
        print!("{}", &text[t.text_range()]);
    }
}
