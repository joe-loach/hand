mod data;
mod load_store;
mod multi_load_store;

use super::*;
use cir::Convert;
use matcher::Pattern;
use variable::Variable;

fn single_pattern(enc: Box<dyn Encodable>) -> matcher::Matcher<Box<dyn Encodable>> {
    let mut p = matcher::Patterns::new();
    p.push(enc, |enc| enc.pattern());
    p.finish()
}
