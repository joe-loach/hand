use super::*;

#[derive(UAL, Clone)]
#[ual = "ADD <Rd>, <Rn>, #<const>"]
struct AddImm;

impl_encodable!(AddImm, [COND, 0, 0, 1, 0, 1, 0, 0, S, R('n'), R('d'), IMM12]);

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