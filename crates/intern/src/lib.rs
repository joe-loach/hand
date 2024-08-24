use string_interner::{backend::StringBackend, StringInterner};

pub use string_interner::Symbol;

use std::num::NonZeroU16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Handle(NonZeroU16);

impl Symbol for Handle {
    fn try_from_usize(index: usize) -> Option<Self> {
        NonZeroU16::new((index as u16).wrapping_add(1)).map(Handle)
    }

    fn to_usize(self) -> usize {
        self.0.get() as usize - 1
    }
}

pub type Interner = StringInterner<StringBackend<Handle>>;
