#[cfg(test)]
mod tests;

mod pattern;

use cir::CIR;
use trie_rs::map::Trie;

pub use pattern::{Pattern, Patterns};

#[derive(Debug)]
pub struct Match<'a, 'b, V> {
    value: &'a V,
    matched: &'b [CIR],
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

pub struct Matcher<V> {
    inner: Trie<cir::CIRKind, V>,
}

impl<V> Matcher<V> {
    pub fn find_match(&self, cir: &[CIR]) -> Option<&V> {
        self.inner
            .exact_match(cir.iter().map(CIR::kind).collect::<Vec<_>>())
    }
}

impl<'a, 'b, V> Match<'a, 'b, V> {
    pub fn value(&self) -> &V {
        self.value
    }

    pub fn matched(&self) -> &[CIR] {
        self.matched
    }
}
