use cir::CIR;

use crate::{
    lowering::{AddressKind, Fragment, Special}, Pattern, TextRange
};

pub(crate) struct UALCursor<'a> {
    pos: usize,
    source: &'a str,
    frags: &'a [Fragment],
}

impl<S: crate::Source> cir::Convert for Pattern<'_, S> {
    fn to_cir(&self) -> Vec<CIR> {
        UALCursor::new(self.source(), self.fragments()).process()
    }
}

impl<'a> UALCursor<'a> {
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
                Fragment::Ident(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.bytes() {
                        template.push(CIR::ident(c));
                    }
                    continue;
                }
                Fragment::Special(special) => match special {
                    Special::Register(_) => CIR::register(),
                    Special::Registers => CIR::register_list(),
                    Special::Condition => CIR::condition(),
                    Special::Const | Special::Immediate => self.number(),
                    Special::Shift => CIR::shift(),
                    Special::Label => continue,
                },
                Fragment::Address(kind) => match kind {
                    AddressKind::Offset => CIR::offset_address(),
                    AddressKind::PreIndex => CIR::pre_index_address(),
                    AddressKind::PostIndex => CIR::post_index_address(),
                },
                Fragment::Byte(b'!') => CIR::bang(),
                Fragment::Byte(b'#') => self.number(),
                Fragment::Byte(_) => continue,
            };

            template.push(part);
        }

        template
    }

    fn number(&mut self) -> CIR {
        assert!(matches!(
            self.bump(),
            Some(Fragment::Special(Special::Const | Special::Immediate))
        ));
        CIR::number()
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
