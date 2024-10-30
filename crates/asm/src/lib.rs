use std::sync::Arc;

use cir::{
    structured::{self, Structured},
    CIR,
};
use enc::{Encodable, Encoder};
use instructions::*;
use matcher::ConstPattern;

type CB = fn(&[CIR]) -> Box<dyn Encodable>;

pub fn assemble(text: Arc<str>) -> Vec<u8> {
    use cir::Convert;
    use matcher::pattern;

    let matcher = build_matcher();

    let hand = hand::parse(text);
    let cir = hand.to_cir();

    let instructions = instructions(&cir);

    let mut encoder = Encoder::new_le();

    for (_inst, args) in instructions {
        println!("{:?}", args);
        let pattern = pattern::from_cir(args);
        let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

        let bits = (pair.value())(args).encode();

        encoder.push(bits);
    }

    encoder.finish()
}

fn instructions(cir: &[CIR]) -> impl Iterator<Item = (&CIR, &[CIR])> {
    let mut curr = 0;
    cir.chunk_by({
        move |_, b| {
            if let CIR::Instruction(it) = b {
                if *it > curr {
                    curr = *it;
                    return false;
                }
            }
            true
        }
    })
    .filter_map(|inst| inst.split_first())
}

fn build_matcher() -> matcher::Matcher<CB> {
    fn add_pattern<T: ConstPattern + Encodable + Structured + 'static>(
        p: &mut matcher::Patterns<CB>,
    ) {
        p.push(
            |cir| {
                Box::new(structured::parse_from_args::<T>(cir).expect("CIR matches this pattern"))
            },
            T::PATTERN,
        );
    }

    let mut p = matcher::Patterns::<CB>::new();

    add_pattern::<AddImm>(&mut p);
    add_pattern::<B>(&mut p);

    p.finish()
}
