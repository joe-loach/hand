use super::{AstNode, AstToken, CloseCurly, Ident};
use crate::{
    grammar::{SyntaxElement, SyntaxNode},
    syntax::SyntaxKind,
};

macros::node!(pub struct Root(SyntaxKind::Root));
macros::node!(pub struct Stmt(SyntaxKind::Statement));
macros::node!(pub struct Instr(SyntaxKind::Instruction));
macros::node!(pub struct Args(SyntaxKind::Arguments));
macros::node!(pub struct Item(SyntaxKind::Item));
macros::node!(pub struct Address(SyntaxKind::Address));
macros::node!(pub struct RegList(SyntaxKind::RegisterList));
macros::node!(pub struct Label(SyntaxKind::Label));
macros::node!(pub struct Number(SyntaxKind::Number));
macros::node!(pub struct Name(SyntaxKind::Name));
macros::node!(pub struct Punct(SyntaxKind::Punct));
macros::node!(pub struct Error(SyntaxKind::Error));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Name(Name),
    Number(Number),
    Punct(Punct),
    Address(Address),
    RegList(RegList),
    Error(Error),
}

impl AstNode for ItemKind {
    fn castable(kind: SyntaxKind) -> bool {
        use SyntaxKind::*;
        matches!(kind, Name | Number | Punct | Address | RegisterList | Error)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::Name => Self::Name(Name(node)),
            SyntaxKind::Number => Self::Number(Number(node)),
            SyntaxKind::Punct => Self::Punct(Punct(node)),
            SyntaxKind::Address => Self::Address(Address(node)),
            SyntaxKind::RegisterList => Self::RegList(RegList(node)),
            SyntaxKind::Error => Self::Error(Error(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            ItemKind::Name(n) => n.syntax(),
            ItemKind::Number(n) => n.syntax(),
            ItemKind::Punct(n) => n.syntax(),
            ItemKind::Address(n) => n.syntax(),
            ItemKind::RegList(n) => n.syntax(),
            ItemKind::Error(n) => n.syntax(),
        }
    }
}

impl Root {
    pub fn statements(&self) -> impl Iterator<Item = Stmt> {
        self.syntax().children().filter_map(Stmt::cast)
    }
}

impl Stmt {
    pub fn label(&self) -> Option<Label> {
        self.syntax().children().find_map(Label::cast)
    }

    pub fn instruction(&self) -> Option<Instr> {
        self.syntax().children().find_map(Instr::cast)
    }
}

impl Instr {
    pub fn name(&self) -> Name {
        self.syntax().children().find_map(Name::cast).unwrap()
    }

    pub fn args(&self) -> Args {
        self.syntax().children().find_map(Args::cast).unwrap()
    }
}

impl Args {
    pub fn iter(&self) -> impl Iterator<Item = Item> {
        self.syntax().children().filter_map(Item::cast)
    }
}

impl Item {
    pub fn kind(&self) -> ItemKind {
        self.syntax()
            .first_child()
            .and_then(ItemKind::cast)
            .unwrap()
    }
}

impl Label {
    pub fn name(&self) -> Name {
        self.syntax().children().find_map(Name::cast).unwrap()
    }
}

impl Name {
    pub fn ident(&self) -> Option<Ident> {
        self.syntax().first_token().and_then(Ident::cast)
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
