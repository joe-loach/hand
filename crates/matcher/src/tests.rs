use super::*;

#[test]
pub(crate) fn api() {
    use cir::Convert;

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

    struct AddReg;

    impl Pattern for AddReg {
        fn pattern(&self) -> &[CIR] {
            use CIR::*;
            static PATTERN: &[CIR] = &[
                Char('A'),
                Char('D'),
                Char('D'),
                Register('d' as u32),
                Register('n' as u32),
                Register('m' as u32),
            ];
            PATTERN
        }
    }

    struct LdrImm;

    impl Pattern for LdrImm {
        fn pattern(&self) -> &[CIR] {
            use CIR::*;
            static PATTERN: &[CIR] = &[
                Char('L'),
                Char('D'),
                Char('R'),
                Register('t' as u32),
                OffsetAddress,
                Register('n' as u32),
                Number(u32::MAX),
            ];
            PATTERN
        }
    }

    let mut p = Patterns::new();
    p.push(1, |_| AddImm.pattern());
    p.push(2, |_| AddReg.pattern());
    p.push(3, |_| LdrImm.pattern());

    let t = p.finish();

    let text = "ADD r0, r1, #10".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand.to_cir()).expect("pattern exists!");

    assert_eq!(*pattern, 1);

    let text = "LDR r0, [r1, #1]".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand.to_cir()).expect("pattern exists!");

    assert_eq!(*pattern, 3);
}
