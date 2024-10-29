use structured::*;

use super::*;

#[derive(Pattern)]
#[name = "LDM"]
struct Ldm(Condition, Register<N>, RegisterList);

impl structured::Parse for Ldm {
    fn parse(buffer: &mut structured::Buffer) -> Option<Self> {
        Some(Self(buffer.parse()?, buffer.parse()?, buffer.parse()?))
    }
}

impl Encodable for Ldm {
    fn encode(&self) -> Word {
        let Self(cond, rn, register_list) = self;
        let w = 0;
        encode![cond | 1 0 0 | 0 | 1 | 0 | w | 1 | rn | register_list]
    }
}

macros::test_encoding!(ldm of Ldm; "LDM r0, {r1}" => 0b1110_1000_1001_0000_0000_0000_0000_0010);
