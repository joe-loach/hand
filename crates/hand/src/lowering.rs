mod cir;

use std::collections::HashMap;

use parser::rowan::TextRange;

use crate::ast::{self, AstToken};

/// TODO: Use Handles to reduce size?
/// A statement begins with [Label?, Instruction?, ..args]
#[derive(Debug, Clone, Copy)]
pub enum Fragment {
    Label(i32),
    Instruction(TextRange),
    Register(u32),
    RegisterList(u16),
    Name(TextRange),
    Number(u32),
    Address(AddressKind),
    Shift(ShiftKind),
    Bang,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressKind {
    Offset,
    PreIndex,
    PostIndex,
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum ShiftKind {
    LSL,
    LSR,
    ASR,
    ROR,
    RRX,
}

/// TODO: error handling
pub fn lower(root: ast::Root) -> Vec<Fragment> {
    let mut frags = Vec::new();

    // TODO: this base of the address should be changable
    let mut address = 0x0_u32;
    let mut label_addresses = HashMap::new();
    for stmt in root.statements() {
        if let Some(label) = stmt.label() {
            let id = label.name().ident().unwrap();
            let text = id.syntax().text().to_string();
            let old = label_addresses.insert(text, address);
            // TODO: validate the ast against this
            assert!(old.is_none(), "Label defined twice");
        }
        // instructions are 4 bytes
        address += 4;
    }

    let mut address = 0x0_u32;
    for stmt in root.statements() {
        if let Some(body) = stmt.instruction() {
            let name = body.name();
            let id = name.ident().unwrap();

            frags.push(Fragment::Instruction(id.syntax().text_range()));

            for item in body.args().iter() {
                let kind = item.kind();
                match kind {
                    ast::ItemKind::Register(reg) => lower_register(&mut frags, reg),
                    ast::ItemKind::Name(name) => {
                        if let Some(ident) = name.ident() {
                            let text = ident.syntax().text();
                            if let Some(&label) = label_addresses.get(text) {
                                lower_label(&mut frags, label, address);
                                continue;
                            }
                        }

                        lower_name(&mut frags, name)
                    }
                    ast::ItemKind::Number(number) => lower_number(&mut frags, number),
                    ast::ItemKind::Address(address) => lower_address(&mut frags, address),
                    ast::ItemKind::RegList(list) => lower_reg_list(&mut frags, list),
                    // Ignore punctuation
                    ast::ItemKind::Punct(_) => (),
                    // TODO: process errors properly
                    ast::ItemKind::Error(_error) => panic!("Error in ast"),
                }
            }
        } else {
            panic!("Statement has no body");
        }
        address += 4;
    }

    frags
}

/// Address Register Offset?
fn lower_address(frags: &mut Vec<Fragment>, address: ast::Address) {
    let kind = match &address {
        ast::Address::Offset(_) => AddressKind::Offset,
        ast::Address::PreIndex(_) => AddressKind::PreIndex,
        ast::Address::PostIndex(_) => AddressKind::PostIndex,
    };
    frags.push(Fragment::Address(kind));
    let base = address.base();
    lower_register(frags, base);
    if let Some(offset) = address.offset() {
        lower_offset(frags, offset);
    }
}

/// Amount Shift
fn lower_offset(frags: &mut Vec<Fragment>, offset: ast::Offset) {
    lower_amount(frags, Some(offset.amount()));
    lower_shift(frags, offset.shift());
}

/// ShiftKind Amount
fn lower_shift(frags: &mut Vec<Fragment>, shift: Option<ast::Shift>) {
    if let Some(kind) = shift.and_then(|shift| shift.kind()) {
        match kind {
            ast::ShiftKind::LSL { amount } => {
                frags.push(Fragment::Shift(ShiftKind::LSL));
                lower_amount(frags, amount)
            }
            ast::ShiftKind::LSR { amount } => {
                frags.push(Fragment::Shift(ShiftKind::LSR));
                lower_amount(frags, amount);
            }
            ast::ShiftKind::ASR { amount } => {
                frags.push(Fragment::Shift(ShiftKind::ASR));
                lower_amount(frags, amount);
            }
            ast::ShiftKind::ROR { amount } => {
                frags.push(Fragment::Shift(ShiftKind::ROR));
                lower_amount(frags, amount);
            }
            ast::ShiftKind::RRX => {
                frags.push(Fragment::Shift(ShiftKind::RRX));
                lower_amount(frags, None)
            }
        }
    }
}

fn lower_reg_list(frags: &mut Vec<Fragment>, list: ast::RegList) {
    let mut regs = 0b0000_0000_0000_0000_u16;

    let mut set_bit = |n| regs |= 1_u16 << n;

    for item in list.items() {
        match item {
            ast::RegListItem::Group(reg_group) => {
                let (low, high) = reg_group.range().unwrap_or((0, 0));
                for value in low..=high {
                    set_bit(value);
                }
            }
            ast::RegListItem::Single(register) => {
                let value = register.value().unwrap_or(0);
                set_bit(value);
            }
        }
    }

    frags.push(Fragment::RegisterList(regs));
}

/// Number | Register
fn lower_amount(frags: &mut Vec<Fragment>, amount: Option<ast::NumOrReg>) {
    match amount {
        Some(ast::NumOrReg::Num(number)) => lower_number(frags, number),
        Some(ast::NumOrReg::Reg(register)) => lower_register(frags, register),
        None => (),
    }
}

/// Number
fn lower_number(frags: &mut Vec<Fragment>, number: ast::Number) {
    if let Some(val) = number.value() {
        frags.push(Fragment::Number(val));
    }
}

/// Label
fn lower_label(frags: &mut Vec<Fragment>, label: u32, current: u32) {
    let offset = label as i32 + 8 - current as i32;
    frags.push(Fragment::Label(offset));
}

/// Name
fn lower_name(frags: &mut Vec<Fragment>, name: ast::Name) {
    let id = name.ident().unwrap();
    frags.push(Fragment::Name(id.syntax().text_range()));
}

/// Register Bang?
fn lower_register(frags: &mut Vec<Fragment>, reg: ast::Register) {
    let value = reg.value().unwrap_or(u32::MAX);
    frags.push(Fragment::Register(value));
    if reg.bang().is_some() {
        frags.push(Fragment::Bang);
    }
}
