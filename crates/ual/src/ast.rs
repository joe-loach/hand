mod node;
mod token;

use crate::grammar::{SyntaxNode, SyntaxToken};
use crate::syntax::SyntaxKind;

pub use node::*;
pub use token::*;

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

impl Optional {
    pub fn items(&self) -> impl Iterator<Item = Item> {
        self.syntax().children().filter_map(Item::cast)
    }
}

impl Special {
    pub fn items(&self) -> impl Iterator<Item = Item> {
        self.syntax().children().filter_map(Item::cast)
    }
}

impl Punct {
    pub fn kind(&self) -> PunctKind {
        let token = self.syntax().first_token().unwrap();
        match token.kind() {
            SyntaxKind::Comma => PunctKind::Comma(Comma::cast(token).unwrap()),
            SyntaxKind::Hash => PunctKind::Hash(Hash::cast(token).unwrap()),
            _ => unreachable!(),
        }
    }
}

pub enum PunctKind {
    Comma(Comma),
    Hash(Hash),
}

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
    let text = std::rc::Rc::from("ADD{S}{<c>} {<Rd>,} <Rn>, #<const>");

    let root = crate::grammar::parse(text);
    let root = Root::cast(root).unwrap();

    fn print_item(it: Item) {
        match it {
            Item::Name(name) => print!("'{}'", name.ident().text()),
            Item::Optional(opt) => {
                print!("{{");
                for it in opt.items() {
                    print_item(it);
                }
                print!("}}");
            }
            Item::Special(sp) => {
                print!("<");
                for it in sp.items() {
                    print_item(it);
                }
                print!(">");
            }
            Item::Punct(pt) => match pt.kind() {
                PunctKind::Comma(_) => print!(","),
                PunctKind::Hash(_) => print!("#"),
            },
            Item::Error(_err) => print!("!"),
        }
    }

    for it in root.items() {
        print_item(it)
    }
}
