use std::rc::Rc;

use parser::rowan;

use crate::SyntaxKind::*;
use crate::UAL;

pub type SyntaxNode = rowan::SyntaxNode<UAL>;
pub type SyntaxToken = rowan::SyntaxToken<UAL>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

type Parser = parser::Parser<UAL>;

pub fn parse(text: Rc<str>) -> SyntaxNode {
    let tokens = crate::lexer::lex(Rc::clone(&text));
    let mut parser = Parser::new(text, tokens);
    root(&mut parser);
    parser.finish()
}

/// Root(Item(s))
fn root(p: &mut Parser) {
    let n = p.start(Root);
    while p.peek().is_some() {
        item(p);
    }
    n.finish(p);
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

/// < Item(s) >
fn special(p: &mut Parser) {
    assert!(p.at(OpenAngled));
    let n = p.start(Special);
    // <
    p.bump();
    // Item(s)
    loop {
        match p.peek() {
            Some(CloseAngled) => break,
            Some(_) => item(p),
            None => break,
        }
    }
    // >
    if !p.at(CloseAngled) {
        error(p);
    } else {
        p.bump();
    }
    n.finish(p);
}

/// { Item(s) }
fn optional(p: &mut Parser) {
    assert!(p.at(OpenCurly));
    let n = p.start(Optional);
    // {
    p.bump();
    // Item(s)
    loop {
        match p.peek() {
            Some(CloseCurly) => break,
            Some(_) => item(p),
            None => break,
        }
    }
    // }
    if !p.at(CloseCurly) {
        error(p);
    } else {
        p.bump();
    }
    n.finish(p);
}

/// Name(Ident)
fn name(p: &mut Parser) {
    assert!(p.at(Ident));
    let n = p.start(Name);
    // Ident
    p.bump();
    n.finish(p);
}

/// Puct(, | #)
fn punct(p: &mut Parser) {
    assert!(p.at(Comma) | p.at(Hash));
    let n = p.start(Punct);
    // , | #
    p.bump();
    n.finish(p);
}

/// Error(Any)
fn error(p: &mut Parser) {
    let n = p.start(Error);
    // Any
    p.bump();
    n.finish(p);
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
    let text = Rc::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");
    let root = parse(text);
    print(0, root.into());
}
