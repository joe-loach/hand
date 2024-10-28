use super::*;

#[test]
pub(crate) fn api() {
    use crate::Pattern::*;
    use cir::Convert;

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

    struct AddReg;

    impl ConstPattern for AddReg {
        const PATTERN: &[Pattern] = &[
            Char('A'),
            Char('D'),
            Char('D'),
            Condition,
            Register,
            Register,
            Register,
        ];
    }

    struct LdrImm;

    impl ConstPattern for LdrImm {
        const PATTERN: &[Pattern] = &[
            Char('L'),
            Char('D'),
            Char('R'),
            Condition,
            Register,
            OffsetAddress,
            Register,
            Number,
        ];
    }

    let mut p = Patterns::new();
    p.push(1, AddImm.pattern());
    p.push(2, AddReg.pattern());
    p.push(3, LdrImm.pattern());

    let matcher = p.finish();

    let text = "ADD r0, r1, #10".into();
    let hand = hand::parse(text);
    let pattern = pattern::from_cir(&hand.to_cir());
    let pair = match_pair(&matcher, &pattern).expect("Failed to match pair");

    assert_eq!(*pair.value(), 1);

    let text = "LDR r0, [r1, #1]".into();
    let hand = hand::parse(text);
    let pattern = pattern::from_cir(&hand.to_cir());
    let pair = match_pair(&matcher, &pattern).expect("Failed to match pair");

    assert_eq!(*pair.value(), 3);
}
