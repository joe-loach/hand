use cir::{Condition, CIR};

pub fn find(name: Variable, pattern: &[CIR], obj: &[CIR]) -> Option<CIR> {
    let pos = pattern.iter().position(|ir| name == *ir)?;
    obj.get(pos).copied()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Variable {
    /// Rn
    Rn,
    /// Rm
    Rm,
    /// Rt
    Rt,
    /// Rd
    Rd,
    /// registers
    RegisterList,
    /// S
    Signed,
    /// cond
    Condition,
    /// stype
    Stype,
    /// imm5
    Imm5,
    /// imm12
    Imm12,
    /// imm24
    Imm24,
    /// P
    IndexFlag,
    /// U
    UnsignedFlag,
    /// W
    WriteBackFlag,
}

impl Variable {
    /// `value` comes from the obj, not the patterns
    ///
    /// # Panics
    ///
    /// If the pair are not encodable
    pub fn encode_with_ir(&self, value: CIR) -> u32 {
        match (&self, value) {
            (Variable::Rn, CIR::Register(x)) => x,
            (Variable::Rm, CIR::Register(x)) => x,
            (Variable::Rt, CIR::Register(x)) => x,
            (Variable::Rd, CIR::Register(x)) => x,
            (Variable::RegisterList, CIR::RegisterList(x)) => x as u32,
            (Variable::Signed, CIR::Char('S')) => true as u32,
            (Variable::Condition, CIR::Condition(cond)) => cond as u32,
            (Variable::Stype, CIR::Shift(shift)) => {
                match shift {
                    cir::Shift::LSL => 0b00,
                    cir::Shift::LSR => 0b01,
                    cir::Shift::ASR => 0b10,
                    cir::Shift::ROR => 0b11,
                    cir::Shift::RRX => 0b11,
                }
            },
            (Variable::Imm5, CIR::Number(x)) => x,
            (Variable::Imm12, CIR::Number(x)) => x,
            (Variable::Imm24, CIR::Number(x)) => x,
            (Variable::IndexFlag, CIR::OffsetAddress | CIR::PreIndexAddress) => true as u32,
            // TODO: signs? U = 0 when negative
            // for now all numbers are positive so U === 1
            (Variable::UnsignedFlag, CIR::Number(_)) => true as u32,
            (Variable::WriteBackFlag, CIR::PreIndexAddress) => true as u32,
            _ => panic!("Pair not encodable"),
        }
    }

    pub fn has_default(&self) -> Option<u32> {
        match self {
            Variable::Signed => Some(false as u32),
            Variable::Condition => Some(self.encode_with_ir(CIR::Condition(Condition::AL))),
            Variable::IndexFlag => Some(false as u32),
            Variable::UnsignedFlag => Some(false as u32),
            Variable::WriteBackFlag => Some(false as u32),
            _ => None,
        }
    }
}

impl PartialEq<cir::CIR> for Variable {
    fn eq(&self, other: &cir::CIR) -> bool {
        const N: u32 = 'n' as u32;
        const M: u32 = 'm' as u32;
        const T: u32 = 't' as u32;
        const D: u32 = 'd' as u32;

        matches!(
            (self, other),
            (Variable::Rn, CIR::Register(N))
                | (Variable::Rm, CIR::Register(M))
                | (Variable::Rt, CIR::Register(T))
                | (Variable::Rd, CIR::Register(D))
                | (Variable::RegisterList, CIR::RegisterList(_))
                | (Variable::Signed, CIR::Char('S'))
                | (Variable::Condition, CIR::Condition(_))
                | (Variable::Stype, CIR::Shift(_))
                | (Variable::Imm5, CIR::Number(_))
                | (Variable::Imm12, CIR::Number(_))
                | (Variable::Imm24, CIR::Number(_))
                | (Variable::IndexFlag, CIR::OffsetAddress | CIR::PreIndexAddress)
                | (Variable::UnsignedFlag, CIR::Number(_))
                | (Variable::WriteBackFlag, CIR::PreIndexAddress)
        )
    }
}

pub struct VariableDef {
    pub(crate) name: Variable,
    pub(crate) high: u8,
    pub(crate) low: u8,
}
