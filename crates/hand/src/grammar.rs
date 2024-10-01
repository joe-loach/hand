use std::sync::Arc;

use parser::{rowan, Marker};

use crate::{syntax::SyntaxKind, HAND};

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
    let text = "loop: ADD r0, r1, #1 \n\
                SUB r0, r0, r1 \n\
                CMP r0, r1, #1 \n\
                ADR r1, loop \n\
                BEQ loop \n\
                LDR r2, [r3, r4, LSL #2]! \n\
                STMDB SP!, {R0-R4, SP}";
    let text = Arc::<str>::from(text);
    dbg!(parse(text));
}

/// statement(s)
fn root(p: &mut Parser) {
    let m = p.start();
    while p.peek().is_some() {
        statement(p);
    }
    m.finish(p, Root);
}

/// label? instr? \n
fn statement(p: &mut Parser) {
    let m = p.start();

    // label?
    let label = label(p);

    let im = match label {
        Ok(()) => p.start(),
        Err(m) => m,
    };

    // instr?
    instruction(p, im);

    // \n
    if !p.eat(NewLine) {
        unexpected(p);
    }

    m.finish(p, Statement);
}

/// (name) arguments
fn instruction(p: &mut Parser, m: Marker) {
    arguments(p);
    m.finish(p, Instruction);
}

/// item(s)
fn arguments(p: &mut Parser) {
    let m = p.start();
    while let Some(kind) = p.peek() {
        if kind == NewLine {
            break;
        }
        item(p);
    }
    m.finish(p, Arguments);
}

fn item(p: &mut Parser) {
    let m = p.start();
    match p.peek() {
        Some(Ident) => name(p),
        Some(Hash) => number(p),
        Some(Comma) => punct(p),
        Some(OpenSquare) => address(p),
        Some(OpenCurly) => register_list(p),
        _ => unexpected(p),
    }
    m.finish(p, Item);
}

/// { (ident | comma) (s) }
fn register_list(p: &mut Parser) {
    assert!(p.at(OpenCurly));
    let m = p.start();
    // {
    p.bump(OpenCurly);
    // (ident | comma) (s)
    while let Some(kind) = p.peek() {
        if kind == CloseCurly {
            break;
        }
        let m = p.start();
        match kind {
            Ident => name(p),
            Comma | Minus => punct(p),
            _ => unexpected(p),
        }
        m.finish(p, Item);
    }
    // }
    expect(p, CloseCurly);
    m.finish(p, RegisterList);
}

/// [ item(s) ](!)?
fn address(p: &mut Parser) {
    assert!(p.at(OpenSquare));
    let m = p.start();
    // [
    p.bump(OpenSquare);
    // item(s)
    while let Some(kind) = p.peek() {
        if kind == CloseSquare {
            break;
        }
        item(p);
    }
    // ]
    expect(p, CloseSquare);
    // (!)?
    if p.at(Bang) {
        punct(p);
    }
    m.finish(p, Address);
}

/// name:
fn label(p: &mut Parser) -> Result<(), Marker> {
    assert!(p.at(Ident));
    let m = p.start();
    name(p);
    if p.eat(Colon) {
        m.finish(p, Label);
        Ok(())
    } else {
        Err(m)
    }
}

/// #(Decimal | Hex | Octal | Binary)
fn number(p: &mut Parser) {
    assert!(p.at(Hash));
    let m = p.start();
    // #
    p.bump(Hash);
    match p.peek() {
        Some(num @ (Decimal | Hex | Octal | Binary)) => p.bump(num),
        _ => unexpected(p),
    }
    m.finish(p, Number);
}

/// Ident
fn name(p: &mut Parser) {
    let m = p.start();
    if p.eat(Ident) {
        m.finish(p, Name);
    } else {
        m.finish(p, Error);
    }
}

/// Comma | Bang | Minus | Plus
fn punct(p: &mut Parser) {
    assert!(matches!(p.peek(), Some(Comma | Bang | Minus | Plus)));
    let m = p.start();
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

/// Any
fn unexpected(p: &mut Parser) {
    if !p.at_end() {
        let m = p.start();
        // Any
        p.bump_any();
        m.finish(p, Error);
    }
}
