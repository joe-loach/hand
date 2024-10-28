mod data;
mod load_store;
mod multi_load_store;

use super::*;
use cir::Convert;
use variable::Variable;

fn single_pattern(enc: &dyn Encodable) -> matcher::Matcher<&dyn Encodable> {
    let mut p = matcher::Patterns::<&dyn Encodable>::new();
    let pat = enc.pattern();
    p.push(enc, pat);
    p.finish()
}
