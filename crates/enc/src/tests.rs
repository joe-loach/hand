mod data;
mod load_store;
mod multi_load_store;

use super::*;
use cir::{Convert, CIR};
use matcher::Pattern;

type BuildEncoder = fn(&[CIR]) -> Box<dyn Encodable>;

fn single_pattern(pattern: &[Pattern], v: BuildEncoder) -> matcher::Matcher<BuildEncoder> {
    let mut p = matcher::Patterns::new();
    p.push(v, pattern);
    p.finish()
}

mod macros {
    macro_rules! test_encoding {
        ($name:ident of $ty:ty; $hand:expr => $expected:expr) => {
            #[test]
            fn $name() {
                use matcher::pattern;
                use matcher::ConstPattern as _;
                let matcher = single_pattern(<$ty>::PATTERN, |cir| {
                    Box::new(cir::structured::parse_from_args::<$ty>(&cir).unwrap())
                });
                let text = $hand.into();
                let hand = hand::parse(text).unwrap();
                let cir = hand.to_cir();
                let cir = &cir[1..];
                let pattern = pattern::from_cir(cir);
                let pair = matcher::match_pair(&matcher, &pattern).expect("Correct pattern");
                let bits = (pair.value())(cir).encode();
                assert_eq!(bits, Word($expected));
            }
        };
    }

    pub(crate) use test_encoding;
}
