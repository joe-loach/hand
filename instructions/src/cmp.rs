use crate::*;

/// Compare (immediate) subtracts an immediate value from a register value.
/// It updates the condition flags based on the result, and discards the result.
pub struct CmpImm;

impl Pattern for CmpImm {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('C'),
            Char('M'),
            Char('P'),
            Condition(cir::Condition::AL),
            Register('n' as u32),
            Number(u32::MAX),
        ];
        PATTERN
    }
}

impl Encodable for CmpImm {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(25)
            .one(24)
            .one(22)
            .one(20)
            .set(Variable::Rn, reg(5, obj), 20, 16)
            .set(Variable::Imm12, imm12(6, obj), 12, 0)
    }
}