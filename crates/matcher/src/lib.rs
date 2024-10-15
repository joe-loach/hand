use cir::CIR;
use trie_rs::map::{Trie, TrieBuilder};

pub struct Patterns<V> {
    inner: TrieBuilder<CIR, V>,
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
        self.inner.push(cir, pattern);
    }
}

impl<V> Default for Patterns<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Matcher<V> {
    inner: Trie<CIR, V>,
}

impl<V> Matcher<V>
{
    pub fn find_match(&self, cir: &[CIR]) -> Option<&V> {
        self.inner.exact_match(cir)
    }
}

#[test]
fn api() {
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
    p.push(1, &CIR::from_ual(&AddImm::PATTERN));
    p.push(2, &CIR::from_ual(&AddReg::PATTERN));
    p.push(3, &CIR::from_ual(&LdrImm::PATTERN));

    let t = p.finish();

    let text = "ADD r0, r1, #10".into();
    let hand = hand::parse(text);
    let pattern = t
        .find_match(&CIR::from_hand(&hand))
        .expect("pattern exists!");

    assert_eq!(*pattern, 1);

    let text = "LDR r0, [r1, #1]".into();
    let hand = hand::parse(text);
    let pattern = t
        .find_match(&CIR::from_hand(&hand))
        .expect("pattern exists!");

    assert_eq!(*pattern, 3);
}
