pub mod pattern;
mod token;

#[cfg(feature = "derive")]
extern crate pattern_derive;

#[cfg(feature = "derive")]
pub use pattern_derive::Pattern;

use trie_rs::map::{Trie, TrieBuilder};

pub use pattern::Pattern;
pub use token::PatternToken;

pub trait ConstPattern {
    const PATTERN: &[Pattern];
}

pub trait HasPattern {
    fn pattern(&self) -> &[Pattern];
}

impl<T: ConstPattern> HasPattern for T {
    fn pattern(&self) -> &[Pattern] {
        T::PATTERN
    }
}

/// Convenience function that wraps matched values together.
pub fn match_pair<'a, 'b, V>(
    matcher: &'a Matcher<V>,
    pattern: &'b [Pattern],
) -> Option<Match<'a, 'b, V>> {
    let value = matcher.find_match(pattern)?;
    Some(Match {
        value,
        matched: pattern,
    })
}

pub struct Matcher<V> {
    inner: Trie<Pattern, V>,
}

impl<V> Matcher<V> {
    pub fn find_match(&self, pattern: &[Pattern]) -> Option<&V> {
        self.inner.exact_match(pattern)
    }
}

pub struct Patterns<V> {
    inner: TrieBuilder<Pattern, V>,
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

    pub fn push(&mut self, value: V, pattern: &[Pattern]) {
        self.inner.push(pattern, value);
    }
}

impl<V> Default for Patterns<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Match<'a, 'b, V> {
    value: &'a V,
    matched: &'b [Pattern],
}

impl<'a, 'b, V> Match<'a, 'b, V> {
    pub fn value(&self) -> &V {
        self.value
    }

    pub fn matched(&self) -> &[Pattern] {
        self.matched
    }
}
