use hand::AddressKind;
use ual::TextRange;

use super::CIR;

pub(crate) struct HANDCursor<'a> {
    pos: usize,
    source: &'a str,
    frags: &'a [hand::Fragment],
}

impl<'a> HANDCursor<'a> {
    pub(crate) fn new(source: &'a str, frags: &'a [hand::Fragment]) -> Self {
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
                hand::Fragment::Instruction(range) | hand::Fragment::Name(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.bytes() {
                        template.push(CIR::ident(c));
                    }
                    continue;
                }
                hand::Fragment::Register(_) => CIR::register(),
                hand::Fragment::RegisterList(_) => CIR::register_list(),
                hand::Fragment::Number(_) => CIR::number(),
                hand::Fragment::Address(kind) => match kind {
                    AddressKind::Offset => CIR::offset_address(),
                    AddressKind::PreIndex => CIR::pre_index_address(),
                    AddressKind::PostIndex => CIR::post_index_address(),
                },
                hand::Fragment::ShiftKind(_) => CIR::shift(),
                hand::Fragment::Bang => CIR::bang(),
                hand::Fragment::Label(_) => continue,
            };

            template.push(part);
        }

        template
    }

    fn resolve(&self, range: TextRange) -> &str {
        &self.source[range]
    }

    fn bump(&mut self) -> Option<hand::Fragment> {
        let frag = self.peek();
        if frag.is_some() {
            self.pos += 1;
        }
        frag
    }

    fn peek(&mut self) -> Option<hand::Fragment> {
        self.frags.get(self.pos).copied()
    }
}
