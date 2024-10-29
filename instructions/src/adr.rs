use crate::*;

/// Form PC-relative address adds an immediate value to the PC value to form a PC-relative address,
/// and writes the result to the destination register.
#[derive(Pattern)]
#[name = "ADR"]
pub struct Adr(Condition, Register<D>, Label);

impl Encodable for Adr {
    fn encode(&self) -> Word {
        let Self(cond, rd, Label(address, _)) = self;
        let imm12 = Number::<12>(*address);
        encode![cond | 0 0 1 0 | 1 0 0 | 0 | 1 1 1 1 | rd | imm12]
    }
}