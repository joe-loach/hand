#[cfg(test)]
mod tests;

use cir::CIR;
use trie_rs::map::{Trie, TrieBuilder};

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

pub struct Matcher<V> {
    inner: Trie<cir::CIRKind, V>,
}

impl<V> Matcher<V> {
    pub fn find_match(&self, cir: &[CIR]) -> Option<&V> {
        self.inner
            .exact_match(cir.iter().map(CIR::kind).collect::<Vec<_>>())
    }
}

/// Convenience function that wraps matched values together.
pub fn match_pair<'a, 'b, V>(
    matcher: &'a Matcher<V>,
    object: &'b [CIR],
) -> Option<Match<'a, 'b, V>> {
    let value = matcher.find_match(object)?;
    Some(Match {
        value,
        matched: object,
    })
}

#[derive(Debug)]
pub struct Match<'a, 'b, V> {
    value: &'a V,
    matched: &'b [CIR],
}

impl<'a, 'b, V> Match<'a, 'b, V> {
    pub fn value(&self) -> &V {
        self.value
    }

    pub fn matched(&self) -> &[CIR] {
        self.matched
    }
}
