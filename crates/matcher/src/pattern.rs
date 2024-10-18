use cir::CIR;
use trie_rs::map::TrieBuilder;

use crate::Matcher;

pub trait Pattern {
    fn pattern(&self) -> &[CIR];
}

pub struct Patterns<V> {
    inner: TrieBuilder<cir::CIRKind, V>,
}

impl<V> Patterns<V> {
    pub fn new() -> Self {
        Self {
            inner: TrieBuilder::new(),
        }
    }

    pub fn finish(self) -> Matcher<V> {
        Matcher {
            inner: self.inner.build(),
        }
    }

    pub fn push(&mut self, mut pattern: V, cir: impl FnOnce(&mut V) -> &[CIR]) {
        let cir = cir(&mut pattern);
        self.inner
            .push(cir.iter().map(CIR::kind).collect::<Vec<_>>(), pattern);
    }
}

impl<V> Default for Patterns<V> {
    fn default() -> Self {
        Self::new()
    }
}
