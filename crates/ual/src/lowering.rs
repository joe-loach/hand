mod intern;

use std::str::FromStr;

pub use intern::Interner;

use crate::{
    ast::{AstNode, AstToken, Item, PunctKind, Root},
    error::{ErrorKind, SyntaxError},
};

#[derive(Debug)]
pub enum Fragment {
    Name(intern::Handle),
    Special(Special),
    Byte(u8),
    BeginOptional,
    EndOptional,
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

    let mut stream = Vec::new();

    for it in root.items() {
        if let Err(err) = lower_item(it, &mut stream, interner) {
            errors.push(err);
        }
    }

    stream
}

fn lower_item(
    it: Item,
    stream: &mut Vec<Fragment>,
    interner: &mut intern::Interner,
) -> Result<(), SyntaxError> {
    match it {
        Item::Name(name) => {
            let name = name.ident();
            let sym = interner.get_or_intern(name.text());
            stream.push(Fragment::Name(sym));
        }
        Item::Optional(opt) => {
            stream.push(Fragment::BeginOptional);
            for it in opt.items() {
                // TODO: collect all errors from inner items
                // must differentiate inner items to top level ones
                // on a grammar level
                lower_item(it, stream, interner)?;
            }
            stream.push(Fragment::EndOptional);
        }
        Item::Special(s) => {
            let Some(name) = s.name() else {
                return Err(SyntaxError::new(s.syntax().clone(), ErrorKind::NoIdent));
            };

            let Ok(kind) = name.ident().text().parse() else {
                return Err(SyntaxError::new(
                    name.syntax().clone(),
                    ErrorKind::UnknownSpecial,
                ));
            };

            stream.push(Fragment::Special(kind));
        }
        Item::Punct(p) => stream.push(Fragment::Byte(match p.kind() {
            PunctKind::Comma(_) => b',',
            PunctKind::Hash(_) => b'#',
        })),

        Item::Error(err) => {
            return Err(SyntaxError::new(
                err.syntax().clone(),
                ErrorKind::UnknownItem,
            ))
        }
    }

    Ok(())
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
