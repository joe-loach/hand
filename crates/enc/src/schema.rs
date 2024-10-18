use std::collections::HashSet;

use cir::{Condition, CIR};

use crate::variable::Variable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct VariableDef {
    pub name: Variable,
    pub value: u32,
    pub high: u8,
    pub low: u8,
}

#[derive(Default)]
pub struct Schema {
    map: HashSet<VariableDef>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cond(self) -> Self {
        // FIXME: #3
        // self.set(Variable::Condition, Value::Ref(2), 32, 28)
        self.set(Variable::Condition, Condition::AL as u32, 32, 28)
    }

    pub fn one(self, bit: u8) -> Self {
        self.set(Variable::One, 1, bit + 1, bit)
    }

    pub fn bit(mut self, name: Variable, value: bool, bit: u8) -> Self {
        if value {
            self.map.insert(VariableDef {
                name,
                value: 1,
                high: bit + 1,
                low: bit,
            });
        }
        self
    }

    pub fn set(mut self, name: Variable, value: u32, high: u8, low: u8) -> Self {
        self.map.insert(VariableDef {
            name,
            value,
            high,
            low,
        });
        self
    }
}

impl Schema {
    pub(crate) fn variables(self) -> impl Iterator<Item = VariableDef> {
        self.map.into_iter()
    }
}

pub fn reg(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::Register(value) = obj[pos - 1] else {
        panic!("No register at {}", pos)
    };
    value
}

pub fn imm12(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::Number(value) = obj[pos - 1] else {
        panic!("No number at {}", pos)
    };
    value & 0xFFF
}

pub fn imm5(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::Number(value) = obj[pos - 1] else {
        panic!("No number at {}", pos)
    };
    value & 0x1F
}

pub fn stype(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::Shift(shift) = obj[pos - 1] else {
        panic!("No stype at {}", pos)
    };

    match shift {
        cir::Shift::LSL => 0b00,
        cir::Shift::LSR => 0b01,
        cir::Shift::ASR => 0b10,
        cir::Shift::ROR => 0b11,
        cir::Shift::RRX => 0b11,
    }
}

pub fn label(pos: usize, obj: &[CIR]) -> (u32, bool) {
    let CIR::Label(value) = obj[pos - 1] else {
        panic!("no label at {}", pos)
    };
    let signed = value.is_negative();
    // value
    if signed {
        (value.wrapping_neg() as u32, signed)
    } else {
        (value as u32, signed)
    }
}

pub fn register_list(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::RegisterList(value) = obj[pos - 1] else {
        panic!("No register list at {}", pos)
    };
    value as u32
}
