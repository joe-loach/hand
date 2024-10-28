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
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(25)
            .one(23)
            .bit(Variable::Signed, false, 20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rd, reg(5, obj), 16, 12)
            .set(Variable::Imm12, imm12(7, obj), 12, 0)
    }
}

impl Encodable for AddReg {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(23)
            .bit(Variable::Signed, false, 20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rd, reg(5, obj), 16, 12)
            .set(Variable::Imm5, imm5(9, obj), 12, 7)
            .set(Variable::Stype, stype(8, obj), 7, 5)
            .set(Variable::Rm, reg(7, obj), 4, 0)
    }
}

impl Encodable for AddRegShiftReg {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(23)
            .bit(Variable::Signed, false, 20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rd, reg(5, obj), 16, 12)
            .set(Variable::Rs, reg(9, obj), 12, 8)
            .set(Variable::Stype, stype(8, obj), 7, 5)
            .one(4)
            .set(Variable::Rm, reg(7, obj), 4, 0)
    }
}
