use std::sync::Arc;

use parser::rowan::{self, Checkpoint};

use crate::HAND;

pub type SyntaxNode = rowan::SyntaxNode<HAND>;
pub type SyntaxToken = rowan::SyntaxToken<HAND>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

use crate::syntax::SyntaxKind::*;

type Parser = parser::Parser<HAND>;

pub fn parse(text: Arc<str>) -> SyntaxNode {
    let tokens = crate::lexer::lex(Arc::clone(&text));
    let mut parser = Parser::new(text, tokens);
    root(&mut parser);
    parser.finish()
}

#[test]
fn it_works() {
    let text = "loop: ADD r0, r1, #1\n SUB r0, r0, r1";
    let text = Arc::<str>::from(text);
    dbg!(parse(text));
}

/// statement(s)
fn root(p: &mut Parser) {
    let n = p.start(Root);
    while p.peek().is_some() {
        statement(p);
    }
    n.finish(p);
}

/// label \n
///
/// label? instr \n
fn statement(p: &mut Parser) {
    let n = p.start(Statement);

    if !p.at(Ident) {
        unexpected(p);
        n.finish(p);
        return;
    }

    let start = p.checkpoint();
    let label = label(p, start);

    let cp = label.unwrap_or(start);

    instruction(p, cp);

    if !matches!(p.peek(), Some(NewLine) | None) {
        unexpected(p);
    } else {
        p.bump();
    }
    n.finish(p);
}

/// (name) arguments
fn instruction(p: &mut Parser, name: Checkpoint) {
    let n = p.start_at(name, Instruction);
    arguments(p);
    n.finish(p);
}

/// name | register | number | comma
fn arguments(p: &mut Parser) {
    let n = p.start(Arguments);

    while let Some(kind) = p.peek() {
        match kind {
            NewLine => break,
            Ident => name(p),
            Hash => number(p),
            _ => p.bump(),
        }
    }

    n.finish(p);
}

/// name:
fn label(p: &mut Parser, cp: Checkpoint) -> Option<Checkpoint> {
    assert!(p.at(Ident));
    name(p);
    if !p.at(Colon) {
        Some(cp)
    } else {
        let n = p.start_at(cp, Label);
        p.bump();
        n.finish(p);
        None
    }
}

/// #(Decimal | Hex | Octal | Binary)
fn number(p: &mut Parser) {
    assert!(p.at(Hash));
    let n = p.start(Number);
    p.bump();
    match p.peek() {
        Some(Decimal | Hex | Octal | Binary) => p.bump(),
        _ => unexpected(p),
    }
    n.finish(p);
}

/// ident
fn name(p: &mut Parser) {
    if p.at(Ident) {
        let n = p.start(Name);
        p.bump();
        n.finish(p);
    } else {
        let n = p.start(Error);
        n.finish(p);
    }
}

fn unexpected(p: &mut Parser) {
    let n = p.start(Error);
    // Any
    p.bump();
    n.finish(p);
}
