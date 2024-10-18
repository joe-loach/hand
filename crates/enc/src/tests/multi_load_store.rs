use super::*;

struct Ldm;

impl Pattern for Ldm {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('L'),
            Char('D'),
            Char('M'),
            Register('n' as u32),
            RegisterList(u16::MAX),
        ];
        PATTERN
    }
}

impl Encodable for Ldm {
    fn schema(&self, obj: &[CIR]) -> Schema {
        Schema::new()
            .cond()
            .one(27)
            .one(23)
            .bit(Variable::W, false, 21)
            .one(20)
            .set(Variable::Rn, reg(4, obj), 20, 16)
            .set(Variable::RegisterList, register_list(5, obj), 16, 0)
    }
}

#[test]
fn ldm() {
    let matcher = single_pattern(Box::new(Ldm));

    let text = "LDM r0, {r1}".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("Correct pattern");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_1000_1001_0000_0000_0000_0000_0010));
}
