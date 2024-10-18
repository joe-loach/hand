mod encoder;
mod schema;
pub mod variable;

pub use schema::*;

use cir::CIR;

pub trait Encodable {
    fn cir(&self) -> &[CIR];
    fn schema(&self) -> Schema;
}

pub fn encode_instruction(encodable: &dyn Encodable, obj: &[CIR]) -> u32 {
    let Schema { base, variables } = encodable.schema();
    let pattern = &encodable.cir();

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

    bits
}

#[test]
fn api() {
    use cir::Convert;
    use std::sync::LazyLock;
    use ual::UalSyntax;
    use ual_derive::UAL;

    #[derive(UAL, Clone)]
    #[ual = "ADD <Rd>, <Rn>, #<const>"]
    struct AddImm;

    impl Encodable for AddImm {
        fn cir(&self) -> &[CIR] {
            static CIR: LazyLock<Vec<CIR>> = LazyLock::new(|| AddImm::PATTERN.to_cir());
            &CIR
        }

        fn schema(&self) -> Schema {
            const { schema([COND, 0, 0, 1, 0, 1, 0, 0, S, R('n'), R('d'), IMM12]) }
        }
    }

    let mut p = matcher::Patterns::new();
    p.push(Box::new(AddImm) as Box<dyn Encodable>, AddImm.cir());
    let matcher = p.finish();

    // TODO: ensure that they are a matched pair
    // by having a function that does so
    // Matched(pattern: &[CIR], obj: &[CIR])
    let text = "ADD r1, r1, #1".into();
    let hand = hand::parse(text);
    let hand_cir = hand.to_cir();
    let pattern = matcher.find_match(&hand_cir).expect("pattern exists!");

    let bits = encode_instruction(pattern.as_ref(), &hand_cir);

    let mut enc = encoder::Encoder::new_be();
    enc.push(bits);

    eprintln!("{:#x?}", enc.buffer());
}
