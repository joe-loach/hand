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
