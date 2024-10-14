use super::AstToken;
use crate::grammar::SyntaxToken;
use crate::syntax::SyntaxKind;

macros::token!(pub struct Ident(SyntaxKind::Ident));
macros::token!(pub struct Hash(SyntaxKind::Hash));
macros::token!(pub struct Comma(SyntaxKind::Comma));
macros::token!(pub struct Plus(SyntaxKind::Plus));
macros::token!(pub struct Minus(SyntaxKind::Minus));
macros::token!(pub struct Bang(SyntaxKind::Bang));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AngledBrace(SyntaxToken);

#[allow(dead_code)]
impl AngledBrace {
    #[inline]
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.syntax().kind() == SyntaxKind::OpenAngled
    }

    #[inline]
    #[must_use]
    pub fn is_close(&self) -> bool {
        self.syntax().kind() == SyntaxKind::CloseAngled
    }
}

impl AstToken for AngledBrace {
    fn castable(kind: SyntaxKind) -> bool {
        matches!(kind, SyntaxKind::OpenAngled | SyntaxKind::CloseAngled)
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

mod macros {
    macro_rules! token {
        ($v:vis struct $ast:ident($kind:path)) => {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            $v struct $ast(SyntaxToken);

            impl AstToken for $ast {
                fn castable(kind: SyntaxKind) -> bool{
                    matches!(kind, $kind)
                }

                fn cast(token: SyntaxToken) -> Option<Self> {
                    if token.kind() == $kind {
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
