use cir::structured::*;

use super::*;

#[derive(Pattern)]
#[name = "ADD"]
pub struct AddImm(Condition, Register<D>, Register<N>, Number<12>);

impl structured::Parse for AddImm {
    fn parse(buffer: &mut structured::Buffer) -> Option<Self> {
        Some(Self(
            buffer.parse()?,
            buffer.parse()?,
            buffer.parse()?,
            buffer.parse()?,
        ))
    }
}

impl Encodable for AddImm {
    fn encode(&self) -> Word {
        let AddImm(cond, rd, rn, imm12) = self;
        let s = 0;
        encode![cond | 0 0 1 0 | 1 0 0 | s | rn | rd | imm12]
    }
}

macros::test_encoding!(add_imm of AddImm; "ADD r0, r0, #0" => 0b1110_0010_1000_0000_0000_0000_0000_0000);
