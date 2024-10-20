use crate::*;

/// Form PC-relative address adds an immediate value to the PC value to form a PC-relative address,
/// and writes the result to the destination register.
pub struct Adr;

impl Pattern for Adr {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('A'),
            Char('D'),
            Char('R'),
            Condition(cir::Condition::AL),
            Register('d' as u32),
            Label(i32::MAX)
        ];
        PATTERN
    }
}

impl Encodable for Adr {
    fn schema(&self, obj: &[CIR]) -> Schema {
        let (label, negative) = label(6, obj);

        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(25)
            .flag_bit(!negative, 23)
            .flag_bit(negative, 22)
            .set(Variable::Rn, 0b1111, 20, 16)
            .set(Variable::Rd, reg(5, obj), 16, 12)
            .set(Variable::Label, label, 12, 0)
    }
}