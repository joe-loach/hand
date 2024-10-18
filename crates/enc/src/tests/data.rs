use super::*;

struct AddImm;

impl Pattern for AddImm {
    fn pattern(&self) -> &[CIR] {
        use CIR::*;
        static PATTERN: &[CIR] = &[
            Char('A'),
            Char('D'),
            Char('D'),
            Register('d' as u32),
            Register('n' as u32),
            Number(u32::MAX),
        ];
        PATTERN
    }
}

#[rustfmt::skip]
impl_encodable!(
    AddImm,
    [COND, 0, 0, 1, 0, 1, 0, 0, S, R('n'), R('d'), IMM12]
);

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
