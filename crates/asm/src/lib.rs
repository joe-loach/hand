use std::sync::Arc;

use cir::{
    structured::{parse_from_args, Structured},
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
    let pattern = pattern::from_cir(&cir);
    let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");

    let mut encoder = Encoder::new_le();

    let bits = (pair.value())(&cir).encode();

    encoder.push(bits);

    encoder.finish()
}

fn build_matcher() -> matcher::Matcher<CB> {
    fn include_ty<T: ConstPattern + Encodable + Structured + 'static>(
        p: &mut matcher::Patterns<CB>,
    ) {
        p.push(
            |cir| Box::new(parse_from_args::<T>(cir).expect("Match failed")),
            T::PATTERN,
        );
    }

    let mut p = matcher::Patterns::<CB>::new();

    include_ty::<AddImm>(&mut p);

    p.finish()
}
