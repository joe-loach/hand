use lexer::{Language, Lexer};

#[derive(Debug)]
pub enum Token {
    /// An identifier
    Ident,
    /// Any sequence of whitespace characters
    Whitespace,

    /// '{'
    OpenCurly,
    /// '}'
    CloseCurly,
    /// '<'
    OpenAngled,
    /// '>'
    CloseAngled,
    /// '#'
    Hash,
    /// ','
    Comma,
    /// '+'
    Plus,
    /// '-'
    Minus,

    /// An unknown character
    Unknown,
}

#[derive(Debug)]
pub enum Error {}

#[derive(Debug)]
pub enum UAL {}

impl Language for UAL {
    type Token = crate::Token;
    type Error = crate::Error;

    fn token(l: &mut Lexer<UAL>, first: char) -> Token {
        match first {
            c if is_whitespace(c) => {
                l.eat_while(is_whitespace);
                Token::Whitespace
            }

            c if is_ident(c) => {
                l.eat_while(is_ident_continue);
                Token::Ident
            }

            '{' => Token::OpenCurly,
            '}' => Token::CloseCurly,
            '<' => Token::OpenAngled,
            '>' => Token::CloseAngled,
            '#' => Token::Hash,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,

            _ => Token::Unknown,
        }
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
    let toks = lexer::lex::<UAL>(text);
    for t in toks {
        print!("{}", &text[t.text_range()]);
    }
}