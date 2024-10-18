use super::*;

#[derive(UAL, Clone)]
#[ual = "LDR <Rt>, [<Rn>, #<imm>]!"]
struct LdrImmPreIndex;

impl_encodable!(LdrImmPreIndex, [COND, 0, 1, 0, P, U, 0, W, 1, R('n'), R('t'), IMM12]);

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

#[derive(UAL, Clone)]
#[ual = "LDR <Rt>, [<Rn>, <Rm>, <shift>]!"]
struct LdrRegPreIndex;

impl_encodable!(
    LdrRegPreIndex,
    [COND, 0, 1, 1, P, U, 0, W, 1, R('n'), R('t'), IMM5, STYPE, 0, R('m')]
);

#[test]
fn ldr_reg_preidx() {
    let matcher = single_pattern(Box::new(LdrRegPreIndex));

    let text = "LDR r0, [r1, r2, LSL #1]!".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("pattern exists!");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    assert_eq!(bits, Word(0b1110_0111_1011_0001_0000_0000_1000_0010));
}

// TODO: labels
// #[derive(UAL, Clone)]
// #[ual = "LDR <Rt>, <label>"]
// struct LdrImmLit;

// impl_encodable!(LdrImmLit, [COND, 0, 1, 0, P, U, 0, W, 1, 1, 1, 1, 1, R('t'), LABEL]);

// #[test]
// fn ldr_imm_lit() {
//     let matcher = single_pattern(Box::new(LdrImmLit));

//     let text = "LDR r0, label!".into();
//     let hand = hand::parse(text);
//     let hand_cir = hand.to_cir();
//     let pair = matcher::match_pair(&matcher, &hand_cir).expect("pattern exists!");

//     let bits = encode_instruction(pair.value().as_ref(), pair.matched());

//     eprintln!("{:032b}", bits);

//     assert_eq!(bits, 0b1110_0101_1011_0001_0000_0000_0000_0001);
// }