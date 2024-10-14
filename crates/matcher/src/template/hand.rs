use hand::AddressKind;
use ual::TextRange;

use super::Template;

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

    pub(crate) fn process(&mut self) -> Vec<Template> {
        let mut template = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                hand::Fragment::Instruction(range) | hand::Fragment::Name(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.bytes() {
                        template.push(Template::ident(c));
                    }
                    continue;
                }
                hand::Fragment::Register(_) => Template::register(),
                hand::Fragment::RegisterList(_) => Template::register_list(),
                hand::Fragment::Number(_) => Template::number(),
                hand::Fragment::Address(kind) => match kind {
                    AddressKind::Offset => Template::offset_address(),
                    AddressKind::PreIndex => Template::pre_index_address(),
                    AddressKind::PostIndex => Template::post_index_address(),
                },
                hand::Fragment::ShiftKind(_) => Template::shift(),
                hand::Fragment::Bang => Template::bang(),
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
