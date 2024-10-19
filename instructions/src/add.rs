use cir::CIR;
use enc::*;
use matcher::Pattern;

pub struct AddImm;

impl Pattern for AddImm {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('A'),
            Char('D'),
            Char('D'),
            Condition(cir::Condition::AL),
            Register('d' as u32),
            Register('n' as u32),
            Number(u32::MAX),
        ];
        PATTERN
    }
}

impl Encodable for AddImm {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(25)
            .one(23)
            .bit(Variable::Signed, obj[4] == CIR::Char('S'), 20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rd, reg(5, obj), 16, 12)
            .set(Variable::Imm12, imm12(7, obj), 12, 0)
    }
}