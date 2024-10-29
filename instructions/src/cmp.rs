use crate::*;

/// Compare (immediate) subtracts an immediate value from a register value.
/// It updates the condition flags based on the result, and discards the result.
#[derive(Pattern, Structured)]
#[name = "CMP"]
pub struct CmpImm(Condition, Register<N>, Number<12>);

impl Encodable for CmpImm {
    fn encode(&self) -> Word {
        let Self(cond, rn, imm12) = self;
        encode![cond | 0 0 1 1 0 | 1 0 | 1 | rn | 0 0 0 0 | imm12]
    }
}
