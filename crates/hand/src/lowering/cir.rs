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
        let mut cir = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                Fragment::Instruction(range) | Fragment::Name(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.chars() {
                        cir.push(CIR::Char(c));
                    }
                    continue;
                }
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
                Fragment::Label(_) => continue,
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
