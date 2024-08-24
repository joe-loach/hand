#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SyntaxKind {
    Ident,

    Number,
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

    Register,

    Whitespace,
    Comment,
    Unknown,
}