use std::sync::Arc;

use parser::rowan;

use crate::UAL;
use crate::{syntax::SyntaxKind, SyntaxKind::*};

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
        Some(OpenAngled) => special(p),
        Some(OpenSquare) => address(p),
        Some(Comma | Hash | Bang) => punct(p),
        Some(_) => error(p),
        None => (),
    }
}

/// [Register (,<imm> | <rm>)?](! | ,<imm> | <rm>)?
fn address(p: &mut Parser) {
    assert!(p.at(OpenSquare));
    let mut address_kind = OffsetAddress;

    let m = p.start();
    // [
    p.bump(OpenSquare);
    // register
    special(p);
    // (, offset)?
    let has_offset = if p.at(Comma) {
        punct(p);
        offset(p);
        true
    } else {
        false
    };
    // ]
    expect(p, CloseSquare);
    // (!)?
    if p.at(Bang) {
        punct(p);
        address_kind = PreIndexAddress;
    } else if p.at(Comma) && !has_offset {
        punct(p);
        offset(p);
        address_kind = PostIndexAddress;
    }
    m.finish(p, address_kind);
}

/// <Register | Number> {, <shift>}
fn offset(p: &mut Parser) {
    let m = p.start();

    // #<imm> | <rm>
    let needs_shift = if p.at(Hash) {
        punct(p);
        false
    } else {
        true
    };
    special(p);
    // , <shift>
    if needs_shift && p.at(Comma) {
        punct(p);
        special(p);
    } else if needs_shift && !p.at(Comma) {
        error(p);
    }

    m.finish(p, Offset);
}

/// < Name >
fn special(p: &mut Parser) {
    assert!(p.at(OpenAngled));
    let m = p.start();
    // <
    p.bump(OpenAngled);
    // Name
    if p.at(Ident) {
        name(p);
    } else {
        error(p);
    }
    // >
    if !p.eat(CloseAngled) {
        error(p);
    }
    m.finish(p, Special);
}

/// Name(Ident)
fn name(p: &mut Parser) {
    assert!(p.at(Ident));
    let m = p.start();
    // Ident
    p.bump(Ident);
    m.finish(p, Name);
}

/// Puct(, | # | !)
fn punct(p: &mut Parser) {
    assert!(matches!(p.peek(), Some(Comma | Hash | Bang)));
    let m = p.start();
    // , | # | !
    p.bump_any();
    m.finish(p, Punct);
}

/// Expect a `kind`, emit an Error otherwise
fn expect(p: &mut Parser, kind: SyntaxKind) -> bool {
    if !p.eat(kind) {
        p.emit(Error);
        false
    } else {
        true
    }
}

/// Error(Any)
fn error(p: &mut Parser) {
    if !p.at_end() {
        let n = p.start();
        // Any
        p.bump_any();
        n.finish(p, Error);
    }
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
    let text = Arc::from("ADD<c> <Rd>, <Rn>, #<const>");
    let root = parse(text);
    print(0, root.into());
}
