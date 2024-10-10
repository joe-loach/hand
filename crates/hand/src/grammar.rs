use std::{mem, sync::Arc};

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
                STMDB SP!, {R0-R4, SP} \n\n\
                SUBEQ r0, r1, #5\n\
                LDR r2, [r3, #1] \n\
                LDR r2, [r3, r4] \n\
                LDR r2, [r3, r4, LSL #2] \n\
                LDR r2, [r3, r4, LSL #2]! \n\
                LDR r2, [r3, r4, LSL r5]! \n\
                LDR r2, [r3], r4 \n\
                HLT";
    let text = Arc::<str>::from(text);
    dbg!(parse(text));
}

/// statement(s)
fn root(p: &mut Parser) {
    let m = p.start();
    while p.peek().is_some() {
        statement(p);

        // clean up empty lines
        while let Some(NewLine) = p.peek() {
            p.bump(NewLine);
        }
    }
    m.finish(p, Root);
}

/// label? instr? \n
fn statement(p: &mut Parser) {
    let m = p.start();

    // label?
    let label = label(p);

    // instr?
    instruction(p, label.err());

    // \n
    if !p.eat(NewLine) {
        unexpected(p);
    }

    m.finish(p, Statement);
}

/// name arguments
fn instruction(p: &mut Parser, name: Option<Marker>) {
    let m = name.unwrap_or_else(|| {
        // has a label, get the name
        let m = p.start();
        self::name(p);
        m
    });
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
        Some(Ident) if is_register(p) => assert!(register(p)),
        Some(Ident) => name(p),
        Some(Hash) => number(p),
        Some(Comma) => punct(p),
        Some(OpenSquare) => address(p),
        Some(OpenCurly) => register_list(p),
        _ => unexpected(p),
    }
    m.finish(p, Item);
}

/// { (register | comma) (s) }
fn register_list(p: &mut Parser) {
    enum PartialGroup {
        /// No state
        None,
        /// Register
        Reg(Marker),
        /// Register And
        RegAnd(Marker),
    }

    assert!(p.at(OpenCurly));
    let m = p.start();
    // {
    p.bump(OpenCurly);
    // (register | comma) (s)
    // attempt to group together RegisterGroup = [Reg Minus Reg]
    let mut group = PartialGroup::None;
    while let Some(kind) = p.peek() {
        match kind {
            CloseCurly => {
                // make sure to abandon any markers weren't finished
                match group {
                    PartialGroup::None => (),
                    PartialGroup::Reg(m) | PartialGroup::RegAnd(m) => m.abandon(),
                }
                break;
            }
            Ident if is_register(p) => match mem::replace(&mut group, PartialGroup::None) {
                PartialGroup::None => {
                    group = PartialGroup::Reg(p.start());
                    assert!(register(p));
                }
                PartialGroup::Reg(m) => {
                    // so far it is [Reg Reg], this isn't a group
                    // make sure to abandon the marker
                    m.abandon();
                    group = PartialGroup::Reg(p.start());
                    assert!(register(p));
                }
                PartialGroup::RegAnd(m) => {
                    assert!(register(p));
                    m.finish(p, RegisterRange);
                }
            },
            Minus => match mem::replace(&mut group, PartialGroup::None) {
                PartialGroup::None => {
                    punct(p);
                }
                PartialGroup::Reg(m) => {
                    punct(p);
                    group = PartialGroup::RegAnd(m)
                }
                PartialGroup::RegAnd(m) => {
                    // so far is [Reg Minus Minus], not a group!
                    // make sure to abandon the marker
                    m.abandon();
                    punct(p);
                }
            },
            Comma => {
                group = PartialGroup::None;
                punct(p);
            }
            _ => {
                group = PartialGroup::None;
                unexpected(p)
            }
        }
    }
    // }
    expect(p, CloseCurly);
    m.finish(p, RegisterList);
}

/// [register(, offset)?](! | (, offset))?
fn address(p: &mut Parser) {
    assert!(p.at(OpenSquare));
    let mut address_kind = OffsetAddress;
    let m = p.start();
    // [
    p.bump(OpenSquare);
    // register
    if !register(p) {
        unexpected(p);
    }
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

/// number | register (, shift)
fn offset(p: &mut Parser) {
    let m = p.start();

    match p.peek() {
        Some(Hash) => number(p),
        Some(Ident) if is_register(p) => {
            assert!(register(p));
            // we dont have to worry about accidentally consuming more arguments here
            // addresses are always the last argument to an instruction
            if p.at(Comma) {
                punct(p);
                shift(p);
            }
        }
        _ => unexpected(p),
    }

    m.finish(p, Offset);
}

/// (LSL | LSR | ASR | ROR) (number | register)
/// RRX
fn shift(p: &mut Parser) {
    let m = p.start();

    if !p.at(Ident) {
        unexpected(p);
    } else {
        let text = p.text().unwrap();
        match text {
            "LSL" | "LSR" | "ASR" | "ROR" => {
                name(p);
                match p.peek() {
                    Some(Hash) => number(p),
                    Some(Ident) if is_register(p) => assert!(register(p)),
                    _ => unexpected(p),
                }
            }
            "RRX" => {
                name(p);
            }
            _ => unexpected(p),
        }
    }

    m.finish(p, Shift);
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

/// RN | SP | LR | PC !?
fn register(p: &mut Parser) -> bool {
    assert!(p.at(Ident));
    let m = p.start();
    if is_register(p) {
        p.bump(Ident);
        // !
        p.eat(Bang);
        m.finish(p, Register);
        true
    } else {
        false
    }
}

fn is_register(p: &mut Parser) -> bool {
    let Some(txt) = p.text() else {
        return false;
    };

    let txt = txt.to_lowercase();
    if let Some(rest) = txt.strip_prefix('r') {
        // numbered register
        rest.parse::<u32>().is_ok()
    } else {
        // named registers
        matches!(txt.as_str(), "sp" | "lr" | "pc")
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
    p.eat(Ident);
    m.finish(p, Name);
}

/// Comma | Bang | Minus | Plus
fn punct(p: &mut Parser) {
    let m = p.start();
    if matches!(p.peek(), Some(Comma | Bang | Minus | Plus)) {
        p.bump_any();
    }
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
