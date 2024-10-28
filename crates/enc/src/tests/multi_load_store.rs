use matcher::{
    pattern, ConstPattern, Pattern::{self, *}
};

use super::*;

struct Ldm;

impl ConstPattern for Ldm {
    const PATTERN: &[Pattern] = &[
        Char('L'),
        Char('D'),
        Char('M'),
        Condition,
        Register,
        RegisterList,
    ];
}

impl Encodable for Ldm {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .set(Variable::Condition, cond(4, obj), 32, 28)
            .one(27)
            .one(23)
            .bit(Variable::W, false, 21)
            .one(20)
            .set(Variable::Rn, reg(5, obj), 20, 16)
            .set(Variable::RegisterList, register_list(6, obj), 16, 0)
    }
}

#[test]
fn ldm() {
    let encodable = Box::new(Ldm);
    let matcher = single_pattern(encodable.as_ref());

    let text = "LDM r0, {r1}".into();
    let hand = hand::parse(text);
    let cir = hand.to_cir();
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

    let bits = encode_instruction(*pair.value(), &cir);

    assert_eq!(bits, Word(0b1110_1000_1001_0000_0000_0000_0000_0010));
}
