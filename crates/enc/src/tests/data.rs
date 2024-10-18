use super::*;

struct AddImm;

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

#[test]
fn add_imm() {
    let matcher = single_pattern(Box::new(AddImm));

    let text = "ADD r0, r0, #0".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("Correct pattern");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_0010_1000_0000_0000_0000_0000_0000));
}
