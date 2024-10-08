use parser::rowan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    Ident,

    Decimal,
    Hex,
    Octal,
    Binary,

    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Comma,
    Hash,
    Plus,
    Minus,
    Bang,
    Colon,
    Equals,

    Root,
    Statement,
    Instruction,
    Arguments,
    Item,
    Address,
    Offset,
    Shift,
    Register,
    RegisterList,
    Label,
    Number,
    Name,
    Punct,
    Error,

    NewLine,
    Whitespace,
    Comment,
    Unknown,

    __LAST,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
