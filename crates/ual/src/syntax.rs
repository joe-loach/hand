use parser::rowan;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    /// An identifier
    Ident,
    /// Any sequence of whitespace characters
    Whitespace,

    /// '['
    OpenSquare,
    /// ']'
    CloseSquare,
    /// '<'
    OpenAngled,
    /// '>'
    CloseAngled,
    /// '#'
    Hash,
    /// ','
    Comma,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '!'
    Bang,

    /// Syntax tree root
    Root,
    /// Special items enclosed in '< >'
    Special,
    /// [<Rn> , ...]
    OffsetAddress,
    /// [<Rn>, ...]!
    PreIndexAddress,
    /// [<Rn>], ...
    PostIndexAddress,
    /// An offset for an address base
    Offset,
    /// A register 'r_'
    Register,
    /// Name identifiers
    Name,
    /// Punctuation
    Punct,

    /// An error
    Error,
    /// An unknown character
    Unknown,

    __LAST,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
