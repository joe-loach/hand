use std::marker::PhantomData;

use byteorder::{ByteOrder, WriteBytesExt, BE, LE};

use crate::Word;

pub struct Encoder<ORDER> {
    buffer: Vec<u8>,
    _order: PhantomData<ORDER>,
}

impl<ORDER> Encoder<ORDER> {
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl Encoder<BE> {
    pub fn new_be() -> Self {
        Self::new()
    }
}

impl Encoder<LE> {
    pub fn new_le() -> Self {
        Self::new()
    }
}

impl<ORDER: ByteOrder> Encoder<ORDER> {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            _order: PhantomData,
        }
    }

    pub fn push(&mut self, word: Word) {
        self.buffer
            .write_u32::<ORDER>(word.get())
            .expect("Buffer can be written to");
    }
}
