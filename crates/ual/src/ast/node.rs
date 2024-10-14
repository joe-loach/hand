use super::{AstNode, AstToken as _};
use crate::grammar::SyntaxNode;
use crate::syntax::SyntaxKind;

macros::node!(pub struct Root(SyntaxKind::Root));
macros::node!(pub struct OffsetAddress(SyntaxKind::OffsetAddress));
macros::node!(pub struct PreIndexAddress(SyntaxKind::PreIndexAddress));
macros::node!(pub struct PostIndexAddress(SyntaxKind::PostIndexAddress));
macros::node!(pub struct Offset(SyntaxKind::Offset));
macros::node!(pub struct Special(SyntaxKind::Special));
macros::node!(pub struct Name(SyntaxKind::Name));
macros::node!(pub struct Punct(SyntaxKind::Punct));
macros::node!(pub struct Error(SyntaxKind::Error));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    Offset(OffsetAddress),
    PreIndex(PreIndexAddress),
    PostIndex(PostIndexAddress),
}

impl AstNode for Address {
    fn castable(kind: SyntaxKind) -> bool {
        use SyntaxKind::*;
        matches!(kind, OffsetAddress | PreIndexAddress | PostIndexAddress)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::OffsetAddress => Self::Offset(OffsetAddress(node)),
            SyntaxKind::PreIndexAddress => Self::PreIndex(PreIndexAddress(node)),
            SyntaxKind::PostIndexAddress => Self::PostIndex(PostIndexAddress(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Offset(n) => n.syntax(),
            Self::PreIndex(n) => n.syntax(),
            Self::PostIndex(n) => n.syntax(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Name(Name),
    Address(Address),
    Special(Special),
    Punct(Punct),
    Error(Error),
}

impl AstNode for Item {
    fn castable(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::Name
                | SyntaxKind::Special
                | SyntaxKind::Punct
                | SyntaxKind::Error
        ) || Address::castable(kind)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(address) = Address::cast(node.clone()) {
            return Some(Self::Address(address));
        }

        let res = match node.kind() {
            SyntaxKind::Name => Item::Name(Name(node)),
            SyntaxKind::Special => Item::Special(Special(node)),
            SyntaxKind::Punct => Item::Punct(Punct(node)),
            SyntaxKind::Error => Item::Error(Error(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Address(n) => n.syntax(),
            Item::Name(n) => n.syntax(),
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
