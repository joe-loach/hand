mod load_store;
mod data;
mod multi_load_store;

use super::*;
use cir::Convert;
use std::sync::LazyLock;
use ual::UalSyntax;
use ual_derive::UAL;

macro_rules! impl_encodable {
    ($target:ident, [$($t:expr),*]) => {
        impl Encodable for $target {
            fn cir(&self) -> &[CIR] {
                static CIR: LazyLock<Vec<CIR>> = LazyLock::new(|| $target::PATTERN.to_cir());
                &CIR
            }
        
            fn schema(&self) -> Schema {
                const { schema([ $($t),* ]) }
            }
        }
    };
}

pub(crate) use impl_encodable;

fn single_pattern(enc: Box<dyn Encodable>) -> matcher::Matcher<Box<dyn Encodable>> {
    let mut p = matcher::Patterns::new();
    p.push(enc, |enc| enc.cir());
    p.finish()
}
