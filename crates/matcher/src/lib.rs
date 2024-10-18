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

#[test]
fn api() {
    use cir::Convert;

    struct AddImm;

    impl Pattern for AddImm {
        fn pattern(&self) -> &[CIR] {
            use CIR::*;
            static PATTERN: &[CIR] = &[
                Char('A'),
                Char('D'),
                Char('D'),
                Register('d' as u32),
                Register('n' as u32),
                Number(u32::MAX),
            ];
            PATTERN
        }
    }

    struct AddReg;

    impl Pattern for AddReg {
        fn pattern(&self) -> &[CIR] {
            use CIR::*;
            static PATTERN: &[CIR] = &[
                Char('A'),
                Char('D'),
                Char('D'),
                Register('d' as u32),
                Register('n' as u32),
                Register('m' as u32),
            ];
            PATTERN
        }
    }

    struct LdrImm;

    impl Pattern for LdrImm {
        fn pattern(&self) -> &[CIR] {
            use CIR::*;
            static PATTERN: &[CIR] = &[
                Char('L'),
                Char('D'),
                Char('R'),
                Register('t' as u32),
                OffsetAddress,
                Register('n' as u32),
                Number(u32::MAX),
            ];
            PATTERN
        }
    }

    let mut p = Patterns::new();
    p.push(1, |_| AddImm.pattern());
    p.push(2, |_| AddReg.pattern());
    p.push(3, |_| LdrImm.pattern());

    let t = p.finish();

    let text = "ADD r0, r1, #10".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand.to_cir()).expect("pattern exists!");

    assert_eq!(*pattern, 1);

    let text = "LDR r0, [r1, #1]".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand.to_cir()).expect("pattern exists!");

    assert_eq!(*pattern, 3);
}
