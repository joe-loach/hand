use std::{collections::VecDeque, str::FromStr};

use lexer::Token;

use crate::{
    ast::{AstNode, AstToken, Item, PunctKind, Root},
    error::{ErrorKind, SyntaxError},
    grammar::{SyntaxElement, SyntaxNode},
};

#[derive(Debug, Clone, Copy)]
pub enum Fragment {
    IdRange(u32),
    Special(Special),
    Byte(u8),
    ToggleOptional,
}

#[derive(Debug, Clone, Copy)]
pub enum Special {
    /// <Rn>
    Register(u8),
    /// <registers>
    Registers,
    /// <c>
    Condition,
    /// <const>
    Const,
    /// <shift>
    Shift,
    /// <amount>
    ShiftAmount,
    ///<label>
    Label,
    /// <imm>
    Immediate,
}

pub fn lower(root: Root, errors: &mut Vec<SyntaxError>) -> Vec<Fragment> {
    let mut t = Traversal::new(errors);

    for el in elements(root.syntax()) {
        if let Element::Item(Item::Optional(ref o)) = el {
            t.push(el.clone());
            for el in elements(o.syntax()) {
                t.push(el);
            }
            t.push(el.clone());
        } else {
            t.push(el)
        }
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
                let range = name.ident().syntax().text_range();
                // start of ident
                self.frag(Fragment::IdRange(range.start().into()));
                // end of ident
                self.frag(Fragment::IdRange(range.end().into()));
            }

            // No need to differentiate begin/end optionals,
            // as they should never nest
            Element::Item(Item::Optional(_)) => self.frag(Fragment::ToggleOptional),

            Element::Item(Item::Special(s)) => {
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

            Element::Item(Item::Punct(p)) => self.frag(Fragment::Byte(match p.kind() {
                PunctKind::Comma(_) => b',',
                PunctKind::Hash(_) => b'#',
            })),

            Element::Item(Item::Error(err)) => self.error(SyntaxError::new(
                err.syntax().clone(),
                ErrorKind::UnknownItem,
            )),

            Element::Whitespace => self.frag(Fragment::Byte(b'\0')),
        }
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
        if let Some(rest) = s.strip_prefix('R') {
            if let [c @ b'A'..=b'Z' | c @ b'a'..=b'z'] = rest.as_bytes() {
                return Ok(Special::Register(*c));
            }
        }

        let kind = match s {
            "registers" => Special::Registers,
            "c" => Special::Condition,
            "const" => Special::Const,
            "shift" => Special::Shift,
            "amount" => Special::ShiftAmount,
            "label" => Special::Label,
            "imm" => Special::Immediate,
            _ => return Err(()),
        };

        Ok(kind)
    }
}
