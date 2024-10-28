#[cfg(test)]
mod tests;

mod encoder;
mod schema;
mod word;

mod variable;

pub use encoder::Encoder;
use matcher::HasPattern;
pub use schema::*;
pub use variable::Variable;
pub use word::Word;

use cir::CIR;

pub trait Encodable: HasPattern {
    fn schema(&self, obj: &[CIR]) -> Schema;
}

pub fn encode_instruction(encodable: &dyn Encodable, obj: &[CIR]) -> Word {
    let schema = encodable.schema(obj);

    let mut bits = 0_u32;

    for VariableDef {
        value, high, low, ..
    } in schema.variables()
    {
        bits |= value << low & range_mask(high, low);
    }

    Word(bits)
}

const fn range_mask(high: u8, low: u8) -> u32 {
    let top = if high == 32 { u32::MAX } else { 1 << high };
    let bottom = if high == 32 && low == 0 { 0 } else { 1 << low };

    top - bottom
}
