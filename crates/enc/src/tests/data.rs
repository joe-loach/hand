use matcher::{pattern, ConstPattern, Pattern::{self, *}};

use super::*;

struct AddImm;

impl ConstPattern for AddImm {
    const PATTERN: &[Pattern] = &[
        Char('A'),
        Char('D'),
        Char('D'),
        Condition,
        Register,
        Register,
        Number,
    ];
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
    let encodable = Box::new(AddImm);
    let matcher = single_pattern(encodable.as_ref());

    let text = "ADD r0, r0, #0".into();
    let hand = hand::parse(text);
    let cir = hand.to_cir();
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

    let bits = encode_instruction(*pair.value(), &cir);

    assert_eq!(bits, Word(0b1110_0010_1000_0000_0000_0000_0000_0000));
}
