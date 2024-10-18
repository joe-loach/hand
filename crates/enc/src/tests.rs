mod data;
mod load_store;
mod multi_load_store;

use super::*;
use cir::Convert;
use matcher::Pattern;

macro_rules! impl_encodable {
    ($target:ident, [$($t:expr),*]) => {
        impl Encodable for $target {
            fn schema(&self) -> Schema {
                const { schema([ $($t),* ]) }
            }
        }
    };
}

pub(crate) use impl_encodable;

fn single_pattern(enc: Box<dyn Encodable>) -> matcher::Matcher<Box<dyn Encodable>> {
    let mut p = matcher::Patterns::new();
    p.push(enc, |enc| enc.pattern());
    p.finish()
}
