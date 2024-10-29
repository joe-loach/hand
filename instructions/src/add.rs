use crate::*;

pub enum Add {
    Imm(AddImm),
    Reg(AddReg),
    RegShiftReg(AddRegShiftReg),
}

/// `Add (immediate)` adds an immediate value to a register value,
/// and writes the result to the destination register.
#[derive(Pattern)]
#[name = "ADD"]
pub struct AddImm(Condition, Register<D>, Register<N>, Number<12>);

/// `Add (register)`` adds a register value and an optionally-shifted register value,
/// and writes the result to the destination register.
#[derive(Pattern)]
#[name = "ADD"]
pub struct AddReg(
    Condition,
    Register<D>,
    Register<N>,
    Register<M>,
    Shift,
    Number<5>,
);

/// `Add (register-shifted register)` adds a register value and a register-shifted register value.
/// It writes the result to the destination register,
/// and can optionally update the condition flags based on the result.
#[derive(Pattern)]
#[name = "ADD"]
pub struct AddRegShiftReg(
    Condition,
    Register<D>,
    Register<N>,
    Register<M>,
    Shift,
    Register<S>,
);

impl Encodable for AddImm {
    fn encode(&self) -> Word {
        let AddImm(cond, rd, rn, imm12) = self;
        let s = 1;
        encode![cond | 0 0 1 0 | 1 0 0 | s | rn | rd | imm12]
    }
}

impl Encodable for AddReg {
    fn encode(&self) -> Word {
        let AddReg(cond, rd, rn, rm, stype, imm5) = self;
        let s = 1;
        encode![cond | 0 0 0 0 | 1 0 0 | s | rn | rd | imm5 | stype | 0 | rm]
    }
}

impl Encodable for AddRegShiftReg {
    fn encode(&self) -> Word {
        let AddRegShiftReg(cond, rd, rn, rm, stype, rs) = self;
        let s = 1;
        encode![cond | 0 0 0 0 | 1 0 0 | s | rn | rd | rs | 0 | stype | 1 | rm ]
    }
}
