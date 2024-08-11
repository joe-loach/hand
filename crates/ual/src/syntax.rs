use parser::rowan;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    /// An identifier
    Ident,
    /// Any sequence of whitespace characters
    Whitespace,

    /// '{'
    OpenCurly,
    /// '}'
    CloseCurly,
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

    /// Syntax tree root
    Root,
    /// Special items enclosed in '< >'
    Special,
    /// Optional items enclosed in '{ }'
    Optional,
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
