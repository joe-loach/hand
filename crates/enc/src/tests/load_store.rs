use matcher::{pattern, ConstPattern, Pattern::{self, *}};

use super::*;

struct LdrImmPreIndex;

impl ConstPattern for LdrImmPreIndex {
    const PATTERN: &[Pattern] = &[
        Char('L'),
        Char('D'),
        Char('R'),
        Condition,
        Register,
        PreIndexAddress,
        Register,
        Number,
    ];
}

impl Encodable for LdrImmPreIndex {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(26)
            .bit(Variable::P, true, 24)
            // FIXME: check if imm12 has a '-' sign
            .bit(Variable::U, true, 23)
            .bit(Variable::W, true, 21)
            .one(20)
            .set(Variable::Rn, reg(7, obj), 20, 16)
            .set(Variable::Rt, reg(5, obj), 16, 12)
            .set(Variable::Imm12, imm12(8, obj), 12, 0)
    }
}

#[test]
fn ldr_imm_preidx() {
    let encodable = Box::new(LdrImmPreIndex);
    let matcher = single_pattern(encodable.as_ref());

    let text = "LDR r0, [r1, #1]!".into();
    let hand = hand::parse(text);
    let cir = hand.to_cir();
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

    let bits = encode_instruction(*pair.value(), &cir);

    assert_eq!(bits, Word(0b1110_0101_1011_0001_0000_0000_0000_0001));
}

struct LdrRegPreIndex;

impl ConstPattern for LdrRegPreIndex {
    const PATTERN: &[Pattern] = &[
        Char('L'),
        Char('D'),
        Char('R'),
        Condition,
        Register,
        PreIndexAddress,
        Register,
        Register,
        Shift,
        Number,
    ];
}

impl Encodable for LdrRegPreIndex {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(26)
            .one(25)
            .bit(Variable::P, true, 24)
            // FIXME: check if imm12 has a '-' sign
            .bit(Variable::U, true, 23)
            .bit(Variable::W, true, 21)
            .one(20)
            .set(Variable::Rn, reg(7, obj), 20, 16)
            .set(Variable::Rt, reg(5, obj), 16, 12)
            .set(Variable::Imm5, imm5(10, obj), 12, 7)
            .set(Variable::Stype, stype(9, obj), 7, 5)
            .set(Variable::Rm, reg(8, obj), 4, 0)
    }
}

#[test]
fn ldr_reg_preidx() {
    let encodable = Box::new(LdrRegPreIndex);
    let matcher = single_pattern(encodable.as_ref());

    let text = "LDR r0, [r1, r2, LSL #1]!".into();
    let hand = hand::parse(text);
    let cir = hand.to_cir();
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

    let bits = encode_instruction(*pair.value(), &cir);

    assert_eq!(bits, Word(0b1110_0111_1011_0001_0000_0000_1000_0010));
}

struct LdrImmLit;

impl ConstPattern for LdrImmLit {
    const PATTERN: &[Pattern] = &[
        Char('L'),
        Char('D'),
        Char('R'),
        Condition,
        Register,
        Label,
    ];
}

impl Encodable for LdrImmLit {
    fn schema(&self, obj: &[CIR]) -> Schema {
        let (label, u) = label(6, obj);

        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(26)
            .bit(Variable::P, true, 24)
            .bit(Variable::U, u, 23)
            .bit(Variable::W, false, 21)
            .one(20)
            .set(Variable::Rn, 0b1111, 20, 16)
            .set(Variable::Rt, reg(5, obj), 16, 12)
            .set(Variable::Label, label, 12, 0)
    }
}

#[test]
fn ldr_imm_lit() {
    let encodable = Box::new(LdrImmLit);
    let matcher = single_pattern(encodable.as_ref());

    let text = "label: LDR r0, label".into();
    let hand = hand::parse(text);
    let cir = hand.to_cir();
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("pattern exists!");

    let bits = encode_instruction(*pair.value(), &cir);

    assert_eq!(bits, Word(0b1110_0101_0001_1111_0000_0000_0000_1000));
}
