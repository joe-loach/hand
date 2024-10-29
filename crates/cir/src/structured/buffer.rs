use crate::CIR;

use super::Structured;

pub struct Buffer<'a> {
    pos: usize,
    inner: &'a [CIR],
}

impl<'a> Buffer<'a> {
    pub fn new(cir: &'a [CIR]) -> Self {
        Self {
            pos: 0_usize,
            inner: cir,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos == self.inner.len()
    }

    pub fn peek(&self) -> Option<CIR> {
        self.inner.get(self.pos).copied()
    }

    pub fn bump(&mut self) {
        self.pos += 1;
    }

    pub fn parse<T: Structured>(&mut self) -> Option<T> {
        T::parse(self)
    }
}