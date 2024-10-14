mod node;
mod token;
mod validate;

use crate::grammar::{SyntaxNode, SyntaxToken};
use crate::syntax::SyntaxKind;

pub use node::*;
pub use token::*;
pub use validate::validate;

pub trait AstNode: Sized {
    fn castable(kind: SyntaxKind) -> bool;
    fn cast(node: SyntaxNode) -> Option<Self>;
    fn syntax(&self) -> &SyntaxNode;
}

pub trait AstToken: Sized {
    fn castable(kind: SyntaxKind) -> bool;
    fn cast(node: SyntaxToken) -> Option<Self>;
    fn syntax(&self) -> &SyntaxToken;
    fn text(&self) -> &str {
        self.syntax().text()
    }
}

impl Root {
    pub fn items(&self) -> impl Iterator<Item = Item> {
        self.syntax().children().filter_map(Item::cast)
    }
}

impl Special {
    pub fn name(&self) -> Option<Name> {
        self.syntax().children().find_map(Name::cast)
    }
}

impl Punct {
    pub fn kind(&self) -> PunctKind {
        let token = self.syntax().first_token().unwrap();
        match token.kind() {
            SyntaxKind::Comma => PunctKind::Comma(Comma::cast(token).unwrap()),
            SyntaxKind::Hash => PunctKind::Hash(Hash::cast(token).unwrap()),
            SyntaxKind::Bang => PunctKind::Bang(Bang::cast(token).unwrap()),
            _ => unreachable!(),
        }
    }
}

impl Address {
    pub fn base(&self) -> Special {
        self.syntax().first_child().and_then(Special::cast).unwrap()
    }

    pub fn offset(&self) -> Option<Offset> {
        self.syntax().children().find_map(Offset::cast)
    }
}

impl Offset {
    pub fn amount(&self) -> Special {
        self.syntax().children().find_map(Special::cast).unwrap()
    }

    pub fn shift(&self) -> Option<Special> {
        self.syntax().last_child().and_then(Special::cast)
    }
}

#[allow(dead_code)]
pub enum PunctKind {
    Comma(Comma),
    Hash(Hash),
    Bang(Bang),
}

#[allow(dead_code)]
impl PunctKind {
    /// Returns `true` if the punctuation is a [`Comma`].
    ///
    /// [`Comma`]: PunctKind::Comma
    #[must_use]
    pub fn is_comma(&self) -> bool {
        matches!(self, Self::Comma(..))
    }

    /// Returns `true` if the punctuation is a [`Hash`].
    ///
    /// [`Hash`]: PunctKind::Hash
    #[must_use]
    pub fn is_hash(&self) -> bool {
        matches!(self, Self::Hash(..))
    }
}

impl Name {
    pub fn ident(&self) -> Ident {
        self.syntax().first_token().and_then(Ident::cast).unwrap()
    }
}

#[test]
fn usage() {
    let text = std::sync::Arc::from("ADD<c> <Rn>, #<const>");

    let root = crate::grammar::parse(text);
    let root = Root::cast(root).unwrap();

    fn print_item(it: Item) {
        match it {
            Item::Name(name) => print!("'{}'", name.ident().text()),
            Item::Address(addr) => {
                print!("[");
                print_special(addr.base());
                if let Some(offset) = addr.offset() {
                    print!(", ");
                    print_special(offset.amount());
                    if let Some(shift) = offset.shift() {
                        print!(", ");
                        print_special(shift);
                    }
                }
                print!("]");
            }
            Item::Special(sp) => print_special(sp),
            Item::Punct(pt) => match pt.kind() {
                PunctKind::Comma(_) => print!(","),
                PunctKind::Hash(_) => print!("#"),
                PunctKind::Bang(_) => print!("!"),
            },
            Item::Error(err) => print!("*{}*", err.syntax().text()),
        }
    }

    fn print_special(sp: Special) {
        if let Some(name) = sp.name() {
            print!("<");
            print!("{}", name.ident().text());
            print!(">");
        }
    }

    for it in root.items() {
        print_item(it)
    }
}
