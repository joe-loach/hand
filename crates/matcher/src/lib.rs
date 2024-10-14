mod template;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use template::{UALCursor, *};
use trie_rs::map::{Trie, TrieBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Match(u32);

pub struct Patterns {
    used_markers: HashMap<TypeId, Match>,
    next_index: u32,
    inner: TrieBuilder<Template, Match>,
}

impl Patterns {
    pub fn new() -> Self {
        Self {
            used_markers: HashMap::new(),
            next_index: 0_u32,
            inner: TrieBuilder::new(),
        }
    }

    pub fn finish(self) -> Matcher {
        Matcher {
            inner: self.inner.build(),
        }
    }

    pub fn push<T: ual::UalSyntax + Any>(&mut self, _: &T) -> Match {
        // TrieBuilder won't change if you push the same T twice,
        // so we need to ensure that we also give out the correct marker.
        if let Some(marker) = self.used_markers.get(&TypeId::of::<T>()) {
            return *marker;
        }

        let pattern = T::PATTERN;
        let template = UALCursor::new(pattern.source(), pattern.fragments()).process();

        let index = self.next_index();
        self.inner.push(template, index);
        self.used_markers.insert(TypeId::of::<T>(), index);
        index
    }

    fn next_index(&mut self) -> Match {
        let index = Match(self.next_index);
        self.next_index += 1;
        index
    }
}

impl Default for Patterns {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Matcher {
    inner: Trie<Template, Match>,
}

impl Matcher {
    pub fn find_match(&self, hand: &hand::ParseResult) -> Option<Match> {
        let template = HANDCursor::new(hand.source(), hand.fragments()).process();

        self.inner.exact_match(&template).copied()
    }
}

#[test]
fn api() {
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
    let add_imm = p.push(&AddImm);
    let _add_reg = p.push(&AddReg);
    let ldr_imm = p.push(&LdrImm);

    let add_imm_2 = p.push(&AddImm);
    assert_eq!(add_imm, add_imm_2);

    let t = p.finish();

    let text = "ADD r0, r1, #10".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand).expect("pattern exists!");

    assert_eq!(pattern, add_imm);

    let text = "LDR r0, [r1, #1]".into();
    let hand = hand::parse(text);
    let pattern = t.find_match(&hand).expect("pattern exists!");

    assert_eq!(pattern, ldr_imm);
}
