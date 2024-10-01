use std::sync::Arc;

use parser::rowan;

use crate::SyntaxKind::*;
use crate::UAL;

pub type SyntaxNode = rowan::SyntaxNode<UAL>;
pub type SyntaxToken = rowan::SyntaxToken<UAL>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

type Parser = parser::Parser<UAL>;

pub fn parse(text: Arc<str>) -> SyntaxNode {
    let tokens = crate::lexer::lex(Arc::clone(&text));
    let mut parser = Parser::new(text, tokens);
    root(&mut parser);
    parser.finish()
}

/// Root(Item(s))
fn root(p: &mut Parser) {
    let m = p.start();
    while p.peek().is_some() {
        item(p);
    }
    m.finish(p, Root);
}

fn item(p: &mut Parser) {
    match p.peek() {
        Some(Whitespace) => unreachable!(),
        Some(Ident) => name(p),
        Some(OpenCurly) => optional(p),
        Some(OpenAngled) => special(p),
        Some(Comma | Hash) => punct(p),
        Some(_) => error(p),
        None => (),
    }
}

/// < Name >
fn special(p: &mut Parser) {
    assert!(p.at(OpenAngled));
    let m = p.start();
    // <
    p.bump(OpenAngled);
    // Name
    while let Some(kind) = p.peek() {
        match kind {
            CloseAngled => break,
            Ident => name(p),
            _ => error(p),
        }
    }
    // >
    if !p.eat(CloseAngled) {
        error(p);
    }
    m.finish(p, Special);
}

/// { Item(s) }
fn optional(p: &mut Parser) {
    assert!(p.at(OpenCurly));
    let m = p.start();
    // {
    p.bump(OpenCurly);
    // Item(s)
    loop {
        match p.peek() {
            Some(CloseCurly) => break,
            Some(_) => item(p),
            None => break,
        }
    }
    // }
    if !p.eat(CloseCurly) {
        error(p);
    }
    m.finish(p, Optional);
}

/// Name(Ident)
fn name(p: &mut Parser) {
    assert!(p.at(Ident));
    let m = p.start();
    // Ident
    p.bump(Ident);
    m.finish(p, Name);
}

/// Puct(, | #)
fn punct(p: &mut Parser) {
    assert!(p.at(Comma) | p.at(Hash));
    let m = p.start();
    // , | #
    p.bump_any();
    m.finish(p, Punct);
}

/// Error(Any)
fn error(p: &mut Parser) {
    let n = p.start();
    // Any
    p.bump_any();
    n.finish(p, Error);
}

#[cfg(test)]
fn print(indent: usize, element: SyntaxElement) {
    let kind = element.kind();
    print!("{:indent$}", "", indent = indent);
    match element {
        rowan::NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        rowan::NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}

#[test]
fn works() {
    let text = Arc::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let root = parse(text);
    print(0, root.into());
}
