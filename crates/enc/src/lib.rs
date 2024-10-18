#[cfg(test)]
mod tests;

mod encoder;
mod schema;
mod word;

pub mod variable;

pub use encoder::Encoder;
use matcher::Pattern;
pub use schema::*;
pub use word::Word;

use cir::CIR;

pub trait Encodable: Pattern {
    fn schema(&self) -> Schema;
}

pub fn encode_instruction(encodable: &dyn Encodable, obj: &[CIR]) -> Word {
    let Schema { base, variables } = encodable.schema();
    let pattern = &encodable.pattern();

    let mut bits = base;

    for var in variables.into_iter().filter_map(|mut var| var.take()) {
        let encoded = if let Some(value) = variable::find(var.name, pattern, obj) {
            var.name.encode_with_ir(value)
        } else if let Some(default) = var.name.has_default() {
            default
        } else {
            panic!(
                "failed to find '{:?}' in obj, perhaps you have defined your schema incorrectly",
                var.name
            );
        };

        bits |= encoded << var.low;
    }

    Word(bits)
}
