#[cfg(test)]
mod tests;

#[cfg(feature = "macros")]
extern crate encode_proc;

mod encoder;
mod word;

use cir::structured;
pub use encoder::Encoder;
pub use word::{Word, WordBuilder};

#[cfg(feature = "macros")]
pub use encode_proc::encode;

pub trait Encodable {
    fn encode(&self) -> Word;

    fn size(&self) -> u8 {
        32
    }
}

impl<T: Encodable> Encodable for &T {
    fn encode(&self) -> Word {
        (*self).encode()
    }

    fn size(&self) -> u8 {
        (*self).size()
    }
}

impl Encodable for bool {
    fn encode(&self) -> Word {
        Word::base(match self {
            true => 1,
            false => 0,
        })
    }

    fn size(&self) -> u8 {
        1
    }
}

impl Encodable for u32 {
    fn encode(&self) -> Word {
        Word::base(*self)
    }

    fn size(&self) -> u8 {
        if *self == 0 {
            1
        } else {
            self.ilog2() as u8 + 1
        }
    }
}

impl Encodable for structured::Label {
    fn encode(&self) -> Word {
        Word::base(self.0)
    }

    fn size(&self) -> u8 {
        12
    }
}

impl Encodable for structured::Condition {
    fn encode(&self) -> Word {
        Word::base(self.0 as u8 as u32)
    }

    fn size(&self) -> u8 {
        4
    }
}

impl<T: structured::RegName> Encodable for structured::Register<T> {
    fn encode(&self) -> Word {
        Word::base(self.0)
    }

    fn size(&self) -> u8 {
        4
    }
}

impl Encodable for structured::RegisterList {
    fn encode(&self) -> Word {
        Word::base(self.0 as u32)
    }

    fn size(&self) -> u8 {
        16
    }
}

impl<const BITS: u8> Encodable for structured::Number<BITS> {
    fn encode(&self) -> Word {
        Word::base(self.0)
    }

    fn size(&self) -> u8 {
        BITS
    }
}

impl Encodable for structured::Shift {
    fn encode(&self) -> Word {
        Word::base(match self.0 {
            cir::Shift::LSL => 0b00,
            cir::Shift::LSR => 0b01,
            cir::Shift::ASR => 0b10,
            cir::Shift::ROR => 0b11,
            cir::Shift::RRX => 0b11,
        })
    }

    fn size(&self) -> u8 {
        2
    }
}

impl Encodable for structured::Address<structured::Offset> {
    fn encode(&self) -> Word {
        Word::empty()
    }

    fn size(&self) -> u8 {
        0
    }
}

impl Encodable for structured::Address<structured::PreIndex> {
    fn encode(&self) -> Word {
        Word::empty()
    }

    fn size(&self) -> u8 {
        0
    }
}

impl Encodable for structured::Address<structured::PostIndex> {
    fn encode(&self) -> Word {
        Word::empty()
    }

    fn size(&self) -> u8 {
        0
    }
}

impl Encodable for structured::Bang {
    fn encode(&self) -> Word {
        Word::empty()
    }

    fn size(&self) -> u8 {
        0
    }
}
