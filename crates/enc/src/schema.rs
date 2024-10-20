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

    pub fn one(self, bit: u8) -> Self {
        self.set(Variable::One, 1, bit + 1, bit)
    }

    pub fn flag_bit(self, value: bool, bit: u8) -> Self {
        if value {
            self.one(bit)
        } else {
            self
        }
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

pub fn cond(pos: usize, obj: &[CIR]) -> u32 {
    let CIR::Condition(value) = obj[pos - 1] else {
        panic!("No conditional at {}", pos)
    };
    match value {
        Condition::EQ => 0b0000,
        Condition::NE => 0b0001,
        Condition::CS => 0b0010,
        Condition::CC => 0b0011,
        Condition::MI => 0b0100,
        Condition::PL => 0b0101,
        Condition::VS => 0b0110,
        Condition::VC => 0b0111,
        Condition::HI => 0b1000,
        Condition::LS => 0b1001,
        Condition::GE => 0b1010,
        Condition::LT => 0b1011,
        Condition::GT => 0b1100,
        Condition::LE => 0b1101,
        Condition::AL => 0b1110,
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
