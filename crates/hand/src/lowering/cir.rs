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
        let mut template = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                Fragment::Instruction(range) | Fragment::Name(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.bytes() {
                        template.push(CIR::ident(c));
                    }
                    continue;
                }
                Fragment::Register(_) => CIR::register(),
                Fragment::RegisterList(_) => CIR::register_list(),
                Fragment::Number(_) => CIR::number(),
                Fragment::Address(kind) => match kind {
                    AddressKind::Offset => CIR::offset_address(),
                    AddressKind::PreIndex => CIR::pre_index_address(),
                    AddressKind::PostIndex => CIR::post_index_address(),
                },
                Fragment::ShiftKind(_) => CIR::shift(),
                Fragment::Bang => CIR::bang(),
                Fragment::Label(_) => continue,
            };

            template.push(part);
        }

        template
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
