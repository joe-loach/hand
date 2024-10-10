use super::{AstNode, AstToken, Bang, Ident, PunctKind};
use crate::{
    grammar::{SyntaxElement, SyntaxNode},
    syntax::SyntaxKind,
};

macros::node!(pub struct Root(SyntaxKind::Root));
macros::node!(pub struct Stmt(SyntaxKind::Statement));
macros::node!(pub struct Instr(SyntaxKind::Instruction));
macros::node!(pub struct Args(SyntaxKind::Arguments));
macros::node!(pub struct Item(SyntaxKind::Item));
macros::node!(pub struct OffsetAddress(SyntaxKind::OffsetAddress));
macros::node!(pub struct PreIndexAddress(SyntaxKind::PreIndexAddress));
macros::node!(pub struct PostIndexAddress(SyntaxKind::PostIndexAddress));
macros::node!(pub struct Offset(SyntaxKind::Offset));
macros::node!(pub struct Shift(SyntaxKind::Shift));
macros::node!(pub struct Register(SyntaxKind::Register));
macros::node!(pub struct RegList(SyntaxKind::RegisterList));
macros::node!(pub struct RegRange(SyntaxKind::RegisterRange));
macros::node!(pub struct Label(SyntaxKind::Label));
macros::node!(pub struct Number(SyntaxKind::Number));
macros::node!(pub struct Name(SyntaxKind::Name));
macros::node!(pub struct Punct(SyntaxKind::Punct));
macros::node!(pub struct Error(SyntaxKind::Error));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Register(Register),
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
        matches!(
            kind,
            Register | Name | Number | Punct | RegisterList | Error
        ) || Address::castable(kind)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(address) = Address::cast(node.clone()) {
            return Some(Self::Address(address));
        }

        let res = match node.kind() {
            SyntaxKind::Register => Self::Register(Register(node)),
            SyntaxKind::Name => Self::Name(Name(node)),
            SyntaxKind::Number => Self::Number(Number(node)),
            SyntaxKind::Punct => Self::Punct(Punct(node)),
            SyntaxKind::RegisterList => Self::RegList(RegList(node)),
            SyntaxKind::Error => Self::Error(Error(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            ItemKind::Register(n) => n.syntax(),
            ItemKind::Name(n) => n.syntax(),
            ItemKind::Number(n) => n.syntax(),
            ItemKind::Punct(n) => n.syntax(),
            ItemKind::Address(n) => n.syntax(),
            ItemKind::RegList(n) => n.syntax(),
            ItemKind::Error(n) => n.syntax(),
        }
    }
}

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

pub enum NumOrReg {
    Num(Number),
    Reg(Register),
}

impl AstNode for NumOrReg {
    fn castable(kind: SyntaxKind) -> bool {
        use SyntaxKind::*;
        matches!(kind, Number | Register)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::Number => Self::Num(Number(node)),
            SyntaxKind::Register => Self::Reg(Register(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Num(n) => n.syntax(),
            Self::Reg(n) => n.syntax(),
        }
    }
}

pub enum RegListItem {
    Group(RegRange),
    Single(Register),
}

impl AstNode for RegListItem {
    fn castable(kind: SyntaxKind) -> bool {
        use SyntaxKind::*;
        matches!(kind, RegisterRange | Register)
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            SyntaxKind::RegisterRange => Self::Group(RegRange(node)),
            SyntaxKind::Register => Self::Single(Register(node)),
            _ => return None,
        };

        Some(res)
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Group(n) => n.syntax(),
            Self::Single(n) => n.syntax(),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum ShiftKind {
    LSL { amount: Option<NumOrReg> },
    LSR { amount: Option<NumOrReg> },
    ASR { amount: Option<NumOrReg> },
    ROR { amount: Option<NumOrReg> },
    RRX,
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
        self.syntax().first_child().and_then(Name::cast).unwrap()
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

impl Address {
    pub fn base(&self) -> Register {
        self.syntax()
            .first_child()
            .and_then(Register::cast)
            .unwrap()
    }

    pub fn offset(&self) -> Offset {
        self.syntax().children().find_map(Offset::cast).unwrap()
    }
}

impl Offset {
    pub fn amount(&self) -> NumOrReg {
        self.syntax()
            .first_child()
            .and_then(NumOrReg::cast)
            .unwrap()
    }

    pub fn number(&self) -> Option<Number> {
        self.syntax().first_child().and_then(Number::cast)
    }

    pub fn register(&self) -> Option<Register> {
        self.syntax().first_child().and_then(Register::cast)
    }

    pub fn shift(&self) -> Option<Shift> {
        self.syntax().children().find_map(Shift::cast)
    }
}

impl Shift {
    pub fn kind(&self) -> Option<ShiftKind> {
        let name = self.name().ident()?;
        let amount = self.amount();

        let kind = match name.text() {
            "LSL" => ShiftKind::LSL { amount },
            "LSR" => ShiftKind::LSR { amount },
            "ASR" => ShiftKind::ASR { amount },
            "ROR" => ShiftKind::ROR { amount },
            "RRX" => ShiftKind::RRX,
            _ => unreachable!(),
        };

        Some(kind)
    }

    pub fn name(&self) -> Name {
        self.syntax().first_child().and_then(Name::cast).unwrap()
    }

    pub fn amount(&self) -> Option<NumOrReg> {
        self.syntax().first_child().and_then(NumOrReg::cast)
    }

    pub fn number(&self) -> Option<Number> {
        self.syntax().children().find_map(Number::cast)
    }

    pub fn register(&self) -> Option<Register> {
        self.syntax().children().find_map(Register::cast)
    }
}

impl Label {
    pub fn name(&self) -> Name {
        self.syntax().children().find_map(Name::cast).unwrap()
    }
}

impl RegList {
    pub fn items(&self) -> impl Iterator<Item = RegListItem> {
        self.syntax().children().filter_map(RegListItem::cast)
    }
}

impl RegRange {
    pub fn range(&self) -> Option<(u32, u32)> {
        let low = self.lower();
        let high = self.higher();

        let low = low.value()?;
        let high = high.value()?;

        Some((low, high))
    }

    pub fn lower(&self) -> Register {
        self.syntax()
            .first_child()
            .and_then(Register::cast)
            .unwrap()
    }

    pub fn higher(&self) -> Register {
        self.syntax().last_child().and_then(Register::cast).unwrap()
    }
}

impl Register {
    pub fn value(&self) -> Option<u32> {
        let id = self.syntax().first_token().and_then(Ident::cast)?;
        let text = id.text().to_lowercase();

        if let Some(rest) = text.strip_prefix('r') {
            // numbered register
            rest.parse::<u32>().ok()
        } else {
            // named registers
            match text.as_str() {
                "sp" => Some(13),
                "lr" => Some(14),
                "pc" => Some(15),
                _ => unreachable!(),
            }
        }
    }

    pub fn bang(&self) -> Option<Bang> {
        self.syntax()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(Bang::cast)
    }
}

impl Name {
    pub fn ident(&self) -> Option<Ident> {
        self.syntax()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(Ident::cast)
    }
}

impl Punct {
    pub fn kind(&self) -> PunctKind {
        self.syntax()
            .first_token()
            .and_then(PunctKind::cast)
            .unwrap()
    }
}

impl Number {
    pub fn value(&self) -> Option<u32> {
        let number_token = self.syntax().last_token()?;
        let number_text = number_token.text();
        let radix = match number_token.kind() {
            SyntaxKind::Decimal => 10,
            SyntaxKind::Hex => 16,
            SyntaxKind::Octal => 8,
            SyntaxKind::Binary => 2,
            _ => unreachable!(),
        };
        u32::from_str_radix(number_text, radix).ok()
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
