#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Pattern {
    Char(char),
    Register,
    RegisterList,
    Condition,
    Shift,
    Number,
    Label,
    OffsetAddress,
    PreIndexAddress,
    PostIndexAddress,
    Bang,
}

use cir::CIR;

pub fn from_cir(cir: &[CIR]) -> Vec<Pattern> {
    let mut res = Vec::new();

    for frag in cir {
        let pattern = match frag {
            CIR::Char(c) => Pattern::Char(*c),
            CIR::Register(_) => Pattern::Register,
            CIR::RegisterList(_) => Pattern::RegisterList,
            CIR::Condition(_) => Pattern::Condition,
            CIR::Shift(_) => Pattern::Shift,
            CIR::Number(_) => Pattern::Number,
            CIR::Label(_) => Pattern::Label,
            CIR::OffsetAddress => Pattern::OffsetAddress,
            CIR::PreIndexAddress => Pattern::PreIndexAddress,
            CIR::PostIndexAddress => Pattern::PostIndexAddress,
            CIR::Bang => Pattern::Bang,
        };

        res.push(pattern);
    }

    res
}
