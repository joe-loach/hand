use super::*;

#[derive(UAL, Clone)]
#[ual = "LDM <Rn>, <registers>"]
struct Ldm;

impl_encodable!(Ldm, [COND, 1, 0, 0, 0, 1, 0, W, 1, R('n'), REGISTER_LIST]);

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
