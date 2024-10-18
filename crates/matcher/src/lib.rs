use cir::CIR;
use trie_rs::map::{Trie, TrieBuilder};

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

    pub fn push(&mut self, pattern: V, cir: &[CIR]) {
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

#[test]
fn api() {
    use cir::Convert;
    use ual::UalSyntax;
    use ual_derive::UAL;

    #[derive(UAL)]
    #[ual = "ADD <Rd>, <Rn>, #<const>"]
    struct AddImm;

    #[derive(UAL)]
    #[ual = "ADD <Rd>, <Rn>, <Rm>"]
    struct AddReg;

    #[derive(UAL)]
    #[ual = "LDR <Rt>, [<Rn>, #<imm>]"]
    struct LdrImm;

    let mut p = Patterns::new();
    p.push(1, &AddImm::PATTERN.to_cir());
    p.push(2, &AddReg::PATTERN.to_cir());
    p.push(3, &LdrImm::PATTERN.to_cir());

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
