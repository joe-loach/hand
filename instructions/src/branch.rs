use crate::*;

/// Branch causes a branch to a target address.
#[derive(Pattern, Structured)]
#[name = "B"]
pub struct B(Condition, Label);

impl Encodable for B {
    fn encode(&self) -> Word {
        let Self(cond, Label(address, negative)) = self;
        let address = address / 4;
        let address = if *negative {
            address.wrapping_neg()
        } else {
            address
        };
        let imm24 = Number::<24>(address);
        encode![cond | 1 0 1 | 0 | imm24]
    }
}