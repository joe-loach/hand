mod error;

use std::sync::Arc;

pub use error::AsmError;

use cir::{
    structured::{self, Structured},
    CIR,
};
use enc::{Encodable, Encoder};
use instructions::*;
use matcher::ConstPattern;

type CB = fn(&[CIR]) -> Box<dyn Encodable>;

pub fn assemble(text: Arc<str>, matcher: &matcher::Matcher<CB>) -> Result<Vec<u8>, AsmError> {
    use cir::Convert;
    use matcher::pattern;

    let hand = hand::parse(text).map_err(AsmError::Hand)?;

    let cir = hand.to_cir();

    let instructions = instructions(&cir);

    let mut encoder = Encoder::new_le();

    for (_inst_id, args) in instructions {
        let pattern = pattern::from_cir(args.cir());
        let pair = matcher::match_pair(matcher, &pattern).ok_or_else(|| {
            let instruction_name = args.name();

            AsmError::EncodingError {
                instruction: instruction_name,
            }
        })?;

        let cb = pair.value();
        let instruction = cb(args.cir());
        let bits = instruction.encode();

        encoder.push(bits);
    }

    Ok(encoder.finish())
}

struct Instruction();
struct Args<'a>(&'a [CIR]);

impl<'a> Args<'a> {
    pub fn cir(&self) -> &[CIR] {
        self.0
    }

    pub fn name(&self) -> String {
        self.0
            .iter()
            .take_while(|cir| matches!(cir, CIR::Char(_)))
            .map(|cir| {
                let CIR::Char(c) = cir else { unreachable!() };
                *c
            })
            .collect::<String>()
    }
}

fn instructions(cir: &[CIR]) -> impl Iterator<Item = (Instruction, Args)> {
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
    .map(|inst| {
        inst.split_first()
            .expect("group has an Instruction and Args")
    })
    .map(|(inst, args)| {
        let CIR::Instruction(_id) = *inst else {
            unreachable!()
        };
        let inst = Instruction();
        let args = Args(args);
        (inst, args)
    })
}

pub fn build_matcher() -> matcher::Matcher<CB> {
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
