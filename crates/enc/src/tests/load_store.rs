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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
impl_encodable!(
    LdrImmLit,
    [COND, 0, 1, 0, P, U, 0, W, 1, 1, 1, 1, 1, R('t'), LABEL]
);

#[test]
fn ldr_imm_lit() {
    let matcher = single_pattern(Box::new(LdrImmLit));

    let text = "label: LDR r0, label".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pair = matcher::match_pair(&matcher, &hand_cir).expect("pattern exists!");

    let bits = encode_instruction(pair.value().as_ref(), pair.matched());

    eprintln!("{:032?}", bits);

    assert_eq!(bits, Word(0b1110_0101_0001_1111_0000_0000_0000_1000));
}
