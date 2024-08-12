use crate::ast::{Error as AstError, Item, Optional, Root, Special};
use crate::error::{ErrorKind, ParseError};
use crate::syntax::SyntaxKind;

use super::{AstNode, AstToken};

/// Validates a syntax tree from its root.
pub fn validate(root: Root, errors: &mut Vec<ParseError>) {
    for it in root.items() {
        match it.clone() {
            Item::Special(s) => special_name(s, errors),
            Item::Optional(o) => nesting(o, errors),
            Item::Error(err) => errors.push(ParseError::new(
                err.syntax().clone(),
                ErrorKind::UnknownItem,
            )),
            _ => (),
        }

        // make sure we catch errors inside of items
        for err in it.syntax().descendants().filter_map(AstError::cast) {
            if let Some(t) = err.syntax().first_token() {
                if t.kind() == SyntaxKind::Unknown {
                    errors.push(ParseError::new(
                        err.syntax().clone(),
                        ErrorKind::UnknownCharacter,
                    ));
                }
            }
        }
    }
}

/// Ensure the name inside a Special marker is recognised
fn special_name(special: Special, errors: &mut Vec<ParseError>) {
    let Some(name) = special.name() else {
        errors.push(ParseError::new(
            special.syntax().clone(),
            ErrorKind::NoIdent,
        ));
        return;
    };

    let id = name.ident();
    let text = id.text();

    if text.strip_prefix("R").is_some() {
        // Register <Rd>, <Rn>, ...etc
        return;
    }

    if ["c", "const", "shift", "amount"].iter().any(|&s| s == text) {
        // Other special words
        return;
    }

    errors.push(ParseError::new(
        special.syntax().clone(),
        ErrorKind::UnknownSpecial,
    ));
}

/// Ensure that an Optional does not have nested children
fn nesting(optional: Optional, errors: &mut Vec<ParseError>) {
    let has_nested_children = optional
        .syntax()
        .children()
        .map(|n| n.kind())
        .any(Optional::castable);

    if has_nested_children {
        errors.push(ParseError::new(
            optional.syntax().clone(),
            ErrorKind::Nesting,
        ));
    }
}
