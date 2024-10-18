mod cir;

use std::{collections::VecDeque, str::FromStr};

use lexer::Token;
use parser::rowan::TextRange;

use crate::{
    ast::{AstNode, AstToken, Item, PunctKind, Root},
    error::{ErrorKind, SyntaxError},
    grammar::{SyntaxElement, SyntaxNode},
};

#[derive(Clone, Copy)]
pub enum Fragment {
    Ident(TextRange),
    Special(Special),
    Byte(u8),
    Address(AddressKind),
}

impl std::fmt::Debug for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(id) => write!(f, "Ident({:?})", id),
            Self::Special(sp) => write!(f, "{:?}", sp),
            Self::Byte(b) => write!(f, "'{}'", std::char::from_u32(*b as u32).unwrap()),
            Self::Address(kind) => write!(f, "{:?}", kind),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Special {
    /// <Rn>
    Register(char),
    /// <registers>
    Registers,
    /// <c>
    Condition,
    /// <const>
    Const,
    /// <shift>
    Shift,
    ///<label>
    Label,
    /// <imm>
    Immediate,
}

impl std::fmt::Debug for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(digit) => f.debug_tuple("Register").field(&digit).finish(),
            Self::Registers => write!(f, "Registers"),
            Self::Condition => write!(f, "Condition"),
            Self::Const => write!(f, "Const"),
            Self::Shift => write!(f, "Shift"),
            Self::Label => write!(f, "Label"),
            Self::Immediate => write!(f, "Immediate"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressKind {
    Offset,
    PreIndex,
    PostIndex,
}

pub fn lower(root: Root, errors: &mut Vec<SyntaxError>) -> Vec<Fragment> {
    let mut t = Traversal::new(errors);

    for el in elements(root.syntax()) {
        t.push(el);
    }

    while let Some(it) = t.pop() {
        t.lower(it);
    }

    t.finish()
}

#[derive(Clone)]
enum Element {
    Item(Item),
    Whitespace,
}

fn elements(node: &SyntaxNode) -> impl Iterator<Item = Element> {
    node.children_with_tokens().filter_map(|n| match n {
        SyntaxElement::Node(n) => Item::cast(n).map(Element::Item),
        SyntaxElement::Token(t) if t.kind().is_whitespace() => Some(Element::Whitespace),
        _ => None,
    })
}

struct Traversal<'a> {
    stack: VecDeque<Element>,
    frags: Vec<Fragment>,
    errors: &'a mut Vec<SyntaxError>,
}

impl<'a> Traversal<'a> {
    fn lower(&mut self, el: Element) {
        match el {
            Element::Item(Item::Name(name)) => {
                let ident = name.ident();
                let ident = ident.syntax();
                let range = ident.text_range();
                self.frag(Fragment::Ident(range));
            }

            Element::Item(Item::Address(addr)) => {
                let kind = match addr {
                    crate::ast::Address::Offset(_) => AddressKind::Offset,
                    crate::ast::Address::PreIndex(_) => AddressKind::PreIndex,
                    crate::ast::Address::PostIndex(_) => AddressKind::PostIndex,
                };
                self.frag(Fragment::Address(kind));
                self.lower_special(addr.base());
                if let Some(offset) = addr.offset() {
                    self.lower_special(offset.amount());
                    if let Some(shift) = offset.shift() {
                        self.lower_special(shift);
                    }
                }
            }

            Element::Item(Item::Special(s)) => self.lower_special(s),

            Element::Item(Item::Punct(p)) => self.frag(Fragment::Byte(match p.kind() {
                PunctKind::Comma(_) => b',',
                PunctKind::Hash(_) => b'#',
                PunctKind::Bang(_) => b'!',
            })),

            Element::Item(Item::Error(err)) => self.error(SyntaxError::new(
                err.syntax().clone(),
                ErrorKind::UnknownItem,
            )),

            Element::Whitespace => self.frag(Fragment::Byte(b' ')),
        }
    }

    fn lower_special(&mut self, s: crate::ast::Special) {
        let Some(name) = s.name() else {
            self.error(SyntaxError::new(s.syntax().clone(), ErrorKind::NoIdent));
            return;
        };

        let Ok(kind) = name.ident().text().parse() else {
            self.error(SyntaxError::new(
                name.syntax().clone(),
                ErrorKind::UnknownSpecial,
            ));
            return;
        };

        self.frag(Fragment::Special(kind));
    }
}

impl<'a> Traversal<'a> {
    fn new(errors: &'a mut Vec<SyntaxError>) -> Self {
        Self {
            stack: VecDeque::new(),
            frags: Vec::new(),
            errors,
        }
    }

    fn finish(self) -> Vec<Fragment> {
        self.frags
    }

    fn frag(&mut self, frag: Fragment) {
        self.frags.push(frag);
    }

    fn error(&mut self, err: SyntaxError) {
        self.errors.push(err);
    }

    fn push(&mut self, it: Element) {
        self.stack.push_back(it);
    }

    fn pop(&mut self) -> Option<Element> {
        self.stack.pop_front()
    }
}

impl FromStr for Special {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        if let Some(rest) = s.strip_prefix('r') {
            if let [c @ 'A'..='Z' | c @ 'a'..='z'] = rest.chars().collect::<Vec<_>>().as_slice() {
                return Ok(Special::Register(*c));
            }
        }

        let kind = match s.as_str() {
            "registers" => Special::Registers,
            "c" => Special::Condition,
            "const" => Special::Const,
            "shift" => Special::Shift,
            "label" => Special::Label,
            "imm" => Special::Immediate,
            _ => return Err(()),
        };

        Ok(kind)
    }
}
