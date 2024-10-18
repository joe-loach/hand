use cir::CIR;

use crate::{
    lowering::{AddressKind, Fragment, Special},
    Pattern, TextRange,
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
        let mut cir = Vec::new();

        while let Some(frag) = self.bump() {
            let part = match frag {
                Fragment::Ident(range) => {
                    let text = self.resolve(range);
                    assert!(text.is_ascii());
                    for c in text.chars() {
                        cir.push(CIR::Char(c));
                    }
                    continue;
                }
                Fragment::Special(special) => match special {
                    Special::Register(value) => CIR::Register(value as u32),
                    Special::Registers => CIR::RegisterList(0x0),
                    Special::Condition => CIR::Condition(Default::default()),
                    Special::Const | Special::Immediate => self.number(),
                    // FIXME: don't assume this is always a number, it could be a register too
                    Special::Shift => {
                        cir.push(CIR::Shift(Default::default()));
                        cir.push(CIR::Number(0x0));
                        continue;
                    },
                    Special::Label => continue,
                },
                Fragment::Address(kind) => match kind {
                    AddressKind::Offset => CIR::OffsetAddress,
                    AddressKind::PreIndex => CIR::PreIndexAddress,
                    AddressKind::PostIndex => CIR::PostIndexAddress,
                },
                Fragment::Byte(b'!') => CIR::Bang,
                Fragment::Byte(b'#') => self.number(),
                Fragment::Byte(_) => continue,
            };

            cir.push(part);
        }

        cir
    }

    fn number(&mut self) -> CIR {
        assert!(matches!(
            self.bump(),
            Some(Fragment::Special(Special::Const | Special::Immediate))
        ));
        CIR::Number(0x0)
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
