use super::AstToken;
use crate::grammar::SyntaxToken;
use crate::syntax::SyntaxKind;

macros::token!(pub struct Ident(SyntaxKind::Ident));
macros::token!(pub struct Hash(SyntaxKind::Hash));
macros::token!(pub struct Comma(SyntaxKind::Comma));
macros::token!(pub struct Plus(SyntaxKind::Plus));
macros::token!(pub struct Minus(SyntaxKind::Minus));
macros::token!(pub struct Bang(SyntaxKind::Bang));
macros::token!(pub struct Colon(SyntaxKind::Colon));
macros::token!(pub struct Equals(SyntaxKind::Equals));
macros::token!(pub struct OpenCurly(SyntaxKind::OpenCurly));
macros::token!(pub struct CloseCurly(SyntaxKind::CloseCurly));
macros::token!(pub struct OpenSquare(SyntaxKind::OpenSquare));
macros::token!(pub struct CloseSquare(SyntaxKind::CloseSquare));
macros::token!(pub struct CurlyBrace(SyntaxKind::OpenCurly | SyntaxKind::CloseCurly));
macros::token!(pub struct SquareBrace(SyntaxKind::OpenSquare | SyntaxKind::CloseSquare));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PunctKind {
    Comma(Comma),
    Bang(Bang),
    Minus(Minus),
    Plus(Plus),
}

impl AstToken for PunctKind {
    fn castable(kind: SyntaxKind) -> bool {
        use SyntaxKind::*;
        matches!(kind, Comma | Bang | Minus | Plus)
    }

    fn cast(node: SyntaxToken) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::Comma => Self::Comma(Comma(node)),
            SyntaxKind::Bang => Self::Bang(Bang(node)),
            SyntaxKind::Minus => Self::Minus(Minus(node)),
            SyntaxKind::Plus => Self::Plus(Plus(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxToken {
        match self {
            PunctKind::Comma(n) => n.syntax(),
            PunctKind::Bang(n) => n.syntax(),
            PunctKind::Minus(n) => n.syntax(),
            PunctKind::Plus(n) => n.syntax(),
        }
    }
}

mod macros {
    macro_rules! token {
        ($v:vis struct $ast:ident($kind:pat)) => {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            $v struct $ast(SyntaxToken);

            impl AstToken for $ast {
                fn castable(kind: SyntaxKind) -> bool{
                    matches!(kind, $kind)
                }

                fn cast(token: SyntaxToken) -> Option<Self> {
                    if Self::castable(token.kind()) {
                        Some(Self(token))
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxToken {
                    &self.0
                }
            }
        };
    }

    pub(crate) use token;
}
