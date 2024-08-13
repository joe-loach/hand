mod intern;

use std::{collections::VecDeque, str::FromStr};

pub use intern::Interner;

use crate::{
    ast::{AstNode, AstToken, Item, PunctKind, Root},
    error::{ErrorKind, SyntaxError},
    grammar::SyntaxNode,
    syntax::SyntaxKind,
};

#[derive(Debug)]
pub enum Fragment {
    Name(intern::Handle),
    Special(Special),
    Byte(u8),
    ToggleOptional,
    Whitespace,
}

#[derive(Debug)]
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

pub fn lower(
    root: Root,
    interner: Option<&mut Interner>,
    errors: &mut Vec<SyntaxError>,
) -> Vec<Fragment> {
    let interner = if let Some(it) = interner {
        it
    } else {
        &mut Interner::new()
    };

    let mut t = Traversal::new(interner, errors);

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
    use parser::rowan::NodeOrToken;

    node.children_with_tokens().filter_map(|n| match n {
        NodeOrToken::Node(n) => Item::cast(n).map(Element::Item),
        NodeOrToken::Token(t) if t.kind() == SyntaxKind::Whitespace => Some(Element::Whitespace),
        _ => None,
    })
}

struct Traversal<'a, 'b> {
    stack: VecDeque<Element>,
    frags: Vec<Fragment>,
    interner: &'a mut Interner,
    errors: &'b mut Vec<SyntaxError>,
}

impl<'a, 'b> Traversal<'a, 'b> {
    fn lower(&mut self, el: Element) {
        match el {
            Element::Item(Item::Name(name)) => {
                let name = name.ident();
                let sym = self.interner.get_or_intern(name.text());
                self.frag(Fragment::Name(sym));
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

            Element::Whitespace => self.frag(Fragment::Whitespace),
        }
    }
}

impl<'a, 'b> Traversal<'a, 'b> {
    fn new(interner: &'a mut intern::Interner, errors: &'b mut Vec<SyntaxError>) -> Self {
        Self {
            stack: VecDeque::new(),
            frags: Vec::new(),
            interner,
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
