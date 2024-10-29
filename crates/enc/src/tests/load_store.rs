use structured::*;

use super::*;

#[derive(Pattern, Structured)]
#[name = "LDR"]
struct LdrImmPreIndex(
    Condition,
    Register<T>,
    Address<PreIndex>,
    Register<N>,
    Number<12>,
);

impl Encodable for LdrImmPreIndex {
    fn encode(&self) -> Word {
        let Self(cond, rt, _, rn, imm12) = self;
        let p = 1;
        let w = 1;
        let u = true;
        encode![cond | 0 1 0 | p | u | 0 | w | 1 | rn | rt | imm12]
    }
}

macros::test_encoding!(ldr_imm_preidx of LdrImmPreIndex; "LDR r0, [r1, #1]!" => 0b1110_0101_1011_0001_0000_0000_0000_0001);

#[derive(Pattern, Structured)]
#[name = "LDR"]
struct LdrRegPreIndex(
    Condition,
    Register<T>,
    Address<PreIndex>,
    Register<N>,
    Register<M>,
    Shift,
    Number<5>,
);

impl Encodable for LdrRegPreIndex {
    fn encode(&self) -> Word {
        let Self(cond, rt, _, rn, rm, stype, imm5) = self;
        let p = 1;
        let w = 1;
        let u = 1;
        encode![cond | 0 1 1 | p | u | 0 | w | 1 | rn | rt | imm5 | stype | 0 | rm]
    }
}

macros::test_encoding!(ldr_reg_preidx of LdrRegPreIndex; "LDR r0, [r1, r2, LSL #1]!" => 0b1110_0111_1011_0001_0000_0000_1000_0010);

#[derive(Pattern, Structured)]
#[name = "LDR"]
struct LdrImmLit(Condition, Register<T>, Label);

impl Encodable for LdrImmLit {
    fn encode(&self) -> Word {
        let Self(cond, rt, Label(address, u)) = self;
        let p = 1;
        let w = 0;
        let imm12 = Number::<12>(*address);
        encode![cond | 0 1 0 | p | u | 0 | w | 1 | 1 1 1 1 | rt | imm12]
    }
}

macros::test_encoding!(ldr_imm_lit of LdrImmLit; "label: LDR r0, label" => 0b1110_0101_0001_1111_0000_0000_0000_1000);
