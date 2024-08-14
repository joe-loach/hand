use super::{AstNode, AstToken as _};
use crate::grammar::SyntaxNode;
use crate::syntax::SyntaxKind;

macros::node!(pub struct Root(SyntaxKind::Root));
macros::node!(pub struct Special(SyntaxKind::Special));
macros::node!(pub struct Optional(SyntaxKind::Optional));
macros::node!(pub struct Name(SyntaxKind::Name));
macros::node!(pub struct Punct(SyntaxKind::Punct));
macros::node!(pub struct Error(SyntaxKind::Error));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Name(Name),
    Optional(Optional),
    Special(Special),
    Punct(Punct),
    Error(Error),
}

impl AstNode for Item {
    fn castable(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::Name
                | SyntaxKind::Optional
                | SyntaxKind::Special
                | SyntaxKind::Punct
                | SyntaxKind::Error
        )
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::Name => Item::Name(Name(node)),
            SyntaxKind::Optional => Item::Optional(Optional(node)),
            SyntaxKind::Special => Item::Special(Special(node)),
            SyntaxKind::Punct => Item::Punct(Punct(node)),
            SyntaxKind::Error => Item::Error(Error(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Name(n) => n.syntax(),
            Item::Optional(n) => n.syntax(),
            Item::Special(n) => n.syntax(),
            Item::Punct(n) => n.syntax(),
            Item::Error(n) => n.syntax(),
        }
    }
}

#[allow(dead_code)]
impl Special {
    pub fn left_brace(&self) -> Option<super::AngledBrace> {
        self.syntax()
            .first_token()
            .and_then(super::AngledBrace::cast)
    }

    pub fn right_brace(&self) -> Option<super::AngledBrace> {
        self.syntax()
            .last_token()
            .and_then(super::AngledBrace::cast)
    }
}

#[allow(dead_code)]
impl Optional {
    pub fn left_brace(&self) -> Option<super::CurlyBrace> {
        self.syntax()
            .first_token()
            .and_then(super::CurlyBrace::cast)
    }

    pub fn right_brace(&self) -> Option<super::CurlyBrace> {
        self.syntax().last_token().and_then(super::CurlyBrace::cast)
    }
}

mod macros {
    macro_rules! node {
        ($v:vis struct $ast:ident($kind:path)) => {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            $v struct $ast(SyntaxNode);

            impl AstNode for $ast {
                fn castable(kind: SyntaxKind) -> bool{
                    matches!(kind, $kind)
                }

                fn cast(node: SyntaxNode) -> Option<Self> {
                    if node.kind() == $kind {
                        Some(Self(node))
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxNode {
                    &self.0
                }
            }
        };
    }

    pub(crate) use node;
}
