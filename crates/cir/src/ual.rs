use ual::{lowering::AddressKind, TextRange};

use super::CIR;

use ual::lowering::Fragment as UAL;
use ual::lowering::Special;

pub(crate) struct UALCursor<'a> {
    pos: usize,
    source: &'a str,
    frags: &'a [ual::lowering::Fragment],
}

impl<'a> UALCursor<'a> {
    pub(crate) fn new(source: &'a str, frags: &'a [ual::lowering::Fragment]) -> Self {
        Self {
            pos: 0_usize,
            source,
            frags,
        }
    }

    pub(crate) fn process(&mut self) -> Vec<CIR> {
        let mut template = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                UAL::Ident(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.bytes() {
                        template.push(CIR::ident(c));
                    }
                    continue;
                }
                UAL::Special(special) => match special {
                    Special::Register(_) => CIR::register(),
                    Special::Registers => CIR::register_list(),
                    Special::Condition => CIR::condition(),
                    Special::Const | Special::Immediate => self.number(),
                    Special::Shift => CIR::shift(),
                    Special::Label => continue,
                },
                UAL::Address(kind) => match kind {
                    AddressKind::Offset => CIR::offset_address(),
                    AddressKind::PreIndex => CIR::pre_index_address(),
                    AddressKind::PostIndex => CIR::post_index_address(),
                },
                UAL::Byte(b'!') => CIR::bang(),
                UAL::Byte(b'#') => self.number(),
                UAL::Byte(_) => continue,
            };

            template.push(part);
        }

        template
    }

    fn number(&mut self) -> CIR {
        assert!(matches!(
            self.bump(),
            Some(UAL::Special(Special::Const | Special::Immediate))
        ));
        CIR::number()
    }

    fn resolve(&self, range: TextRange) -> &str {
        &self.source[range]
    }

    fn bump(&mut self) -> Option<ual::lowering::Fragment> {
        let frag = self.peek();
        if frag.is_some() {
            self.pos += 1;
        }
        frag
    }

    fn peek(&mut self) -> Option<ual::lowering::Fragment> {
        self.frags.get(self.pos).copied()
    }
}
