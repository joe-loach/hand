use super::*;

struct LdrImmPreIndex;

impl Pattern for LdrImmPreIndex {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('L'),
            Char('D'),
            Char('R'),
            Register('t' as u32),
            PreIndexAddress,
            Register('n' as u32),
            Number(u32::MAX),
        ];
        PATTERN
    }
}

impl Encodable for LdrImmPreIndex {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .cond()
            .one(26)
            .bit(Variable::P, true, 24)
            // FIXME: check if imm12 has a '-' sign
            .bit(Variable::U, true, 23)
            .bit(Variable::W, true, 21)
            .one(20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rt, reg(4, obj), 16, 12)
            .set(Variable::Imm12, imm12(7, obj), 12, 0)
    }
}

#[test]
fn ldr_imm_preidx() {
    let matcher = single_pattern(Box::new(LdrImmPreIndex));

    let text = "LDR r0, [r1, #1]!".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("Correct pattern");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_0101_1011_0001_0000_0000_0000_0001));
}

struct LdrRegPreIndex;

impl Pattern for LdrRegPreIndex {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('L'),
            Char('D'),
            Char('R'),
            Register('t' as u32),
            PreIndexAddress,
            Register('n' as u32),
            Register('m' as u32),
            Shift(cir::Shift::LSL),
            Number(u32::MAX),
        ];
        PATTERN
    }
}

impl Encodable for LdrRegPreIndex {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .cond()
            .one(26)
            .one(25)
            .bit(Variable::P, true, 24)
            // FIXME: check if imm12 has a '-' sign
            .bit(Variable::U, true, 23)
            .bit(Variable::W, true, 21)
            .one(20)
            .set(Variable::Rn, reg(6, obj), 20, 16)
            .set(Variable::Rt, reg(4, obj), 16, 12)
            .set(Variable::Imm5, imm5(9, obj), 12, 7)
            .set(Variable::Stype, stype(8, obj), 7, 5)
            .set(Variable::Rm, reg(7, obj), 4, 0)
    }
}

#[test]
fn ldr_reg_preidx() {
    let matcher = single_pattern(Box::new(LdrRegPreIndex));

    let text = "LDR r0, [r1, r2, LSL #1]!".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("Correct pattern");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_0111_1011_0001_0000_0000_1000_0010));
}

struct LdrImmLit;

impl Pattern for LdrImmLit {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('L'),
            Char('D'),
            Char('R'),
            Register('t' as u32),
            Label(i32::MAX),
        ];
        PATTERN
    }
}

impl Encodable for LdrImmLit {
    fn schema(&self, obj: &[CIR]) -> Schema {
        let (label, u) = label(5, obj);

        Schema::new()
            .cond()
            .one(26)
            .bit(Variable::P, true, 24)
            .bit(Variable::U, u, 23)
            .bit(Variable::W, false, 21)
            .one(20)
            .set(Variable::Rn, 0b1111, 20, 16)
            .set(Variable::Rt, reg(4, obj), 16, 12)
            .set(Variable::Label, label, 12, 0)
    }
}

#[test]
fn ldr_imm_lit() {
    let matcher = single_pattern(Box::new(LdrImmLit));

    let text = "label: LDR r0, label".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("pattern exists!");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_0101_0001_1111_0000_0000_0000_1000));
}
