use cir::CIR;
use parser::rowan::TextRange;

use crate::{AddressKind, Fragment, ParseResult};

impl cir::Convert for ParseResult {
    fn to_cir(&self) -> Vec<CIR> {
        HANDCursor::new(self.source(), self.fragments()).process()
    }
}

pub(crate) struct HANDCursor<'a> {
    pos: usize,
    source: &'a str,
    frags: &'a [Fragment],
}

impl<'a> HANDCursor<'a> {
    pub(crate) fn new(source: &'a str, frags: &'a [Fragment]) -> Self {
        Self {
            pos: 0_usize,
            source,
            frags,
        }
    }

    pub(crate) fn process(&mut self) -> Vec<CIR> {
        let mut instruction_count = 0;
        let mut cir = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                Fragment::Instruction(range) => {
                    cir.push(CIR::Instruction(instruction_count));
                    instruction_count += 1;
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.chars() {
                        cir.push(CIR::Char(c));
                    }
                    continue;
                }
                Fragment::Name(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.chars() {
                        cir.push(CIR::Char(c));
                    }
                    continue;
                }
                Fragment::Condition(cond) => CIR::Condition(match cond {
                    super::Condition::EQ => cir::Condition::EQ,
                    super::Condition::NE => cir::Condition::NE,
                    super::Condition::CS => cir::Condition::CS,
                    super::Condition::CC => cir::Condition::CC,
                    super::Condition::MI => cir::Condition::MI,
                    super::Condition::PL => cir::Condition::PL,
                    super::Condition::VS => cir::Condition::VS,
                    super::Condition::VC => cir::Condition::VC,
                    super::Condition::HI => cir::Condition::HI,
                    super::Condition::LS => cir::Condition::LS,
                    super::Condition::GE => cir::Condition::GE,
                    super::Condition::LT => cir::Condition::LT,
                    super::Condition::GT => cir::Condition::GT,
                    super::Condition::LE => cir::Condition::LE,
                    super::Condition::AL => cir::Condition::AL,
                }),
                Fragment::Register(r) => CIR::Register(r),
                Fragment::RegisterList(rl) => CIR::RegisterList(rl),
                Fragment::Number(num) => CIR::Number(num),
                Fragment::Address(kind) => match kind {
                    AddressKind::Offset => CIR::OffsetAddress,
                    AddressKind::PreIndex => CIR::PreIndexAddress,
                    AddressKind::PostIndex => CIR::PostIndexAddress,
                },
                Fragment::Shift(kind) => CIR::Shift(match kind {
                    super::ShiftKind::LSL => cir::Shift::LSL,
                    super::ShiftKind::LSR => cir::Shift::LSR,
                    super::ShiftKind::ASR => cir::Shift::ASR,
                    super::ShiftKind::ROR => cir::Shift::ROR,
                    super::ShiftKind::RRX => cir::Shift::RRX,
                }),
                Fragment::Bang => CIR::Bang,
                Fragment::Label(adr) => CIR::Label(adr),
            };

            cir.push(part);
        }

        cir
    }

    fn resolve(&self, range: TextRange) -> &str {
        &self.source[range]
    }

    fn bump(&mut self) -> Option<Fragment> {
        let frag = self.peek();
        if frag.is_some() {
            self.pos += 1;
        }
        frag
    }

    fn peek(&mut self) -> Option<Fragment> {
        self.frags.get(self.pos).copied()
    }
}
