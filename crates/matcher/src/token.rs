use cir::structured::*;

use crate::Pattern;

pub trait PatternToken {
    const TOKEN: Pattern;
}

/*
Register,
RegisterList,
Condition,
Shift,
Number,
Label,
OffsetAddress,
PreIndexAddress,
PostIndexAddress,
Bang
*/

impl<T> PatternToken for Register<T> {
    const TOKEN: Pattern = Pattern::Register;
}

impl PatternToken for RegisterList {
    const TOKEN: Pattern = Pattern::RegisterList;
}

impl PatternToken for Condition {
    const TOKEN: Pattern = Pattern::Condition;
}

impl PatternToken for Shift {
    const TOKEN: Pattern = Pattern::Shift;
}

impl<const BITS: u8> PatternToken for Number<BITS> {
    const TOKEN: Pattern = Pattern::Number;
}

impl PatternToken for Label {
    const TOKEN: Pattern = Pattern::Label;
}

impl PatternToken for Address<Offset> {
    const TOKEN: Pattern = Pattern::OffsetAddress;
}

impl PatternToken for Address<PreIndex> {
    const TOKEN: Pattern = Pattern::PreIndexAddress;
}

impl PatternToken for Address<PostIndex> {
    const TOKEN: Pattern = Pattern::PostIndexAddress;
}

impl PatternToken for Bang {
    const TOKEN: Pattern = Pattern::Bang;
}
