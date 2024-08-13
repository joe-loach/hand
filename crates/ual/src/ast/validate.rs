use crate::ast::{Error as AstError, Item, Optional, Root, Special};
use crate::error::{ErrorKind, SyntaxError};
use crate::syntax::SyntaxKind;

use super::AstNode;

/// Validates a syntax tree from its root.
pub fn validate(root: Root, errors: &mut Vec<SyntaxError>) {
    for it in root.items() {
        match it.clone() {
            Item::Special(s) => is_closed(Braced::Special(s), errors),
            Item::Optional(o) => {
                is_closed(Braced::Optional(o.clone()), errors);
                not_nested(o, errors);
            }
            _ => (),
        }

        // make sure we catch errors inside of items
        for err in it.syntax().descendants().filter_map(AstError::cast) {
            if let Some(t) = err.syntax().first_token() {
                if t.kind() == SyntaxKind::Unknown {
                    errors.push(SyntaxError::new(
                        err.syntax().clone(),
                        ErrorKind::UnknownCharacter,
                    ));
                }
            }
        }
    }
}

enum Braced {
    Special(Special),
    Optional(Optional),
}

fn is_closed(braced: Braced, errors: &mut Vec<SyntaxError>) {
    let has_right_brace = match &braced {
        Braced::Special(it) => it.right_brace().is_some(),
        Braced::Optional(it) => it.right_brace().is_some(),
    };

    if !has_right_brace {
        let node = match &braced {
            Braced::Special(it) => it.syntax().clone(),
            Braced::Optional(it) => it.syntax().clone(),
        };
        errors.push(SyntaxError::new(node, ErrorKind::Nesting));
    }
}

/// Ensure that an Optional does not have nested children
fn not_nested(optional: Optional, errors: &mut Vec<SyntaxError>) {
    let has_nested_children = optional
        .syntax()
        .children()
        .map(|n| n.kind())
        .any(Optional::castable);

    if has_nested_children {
        errors.push(SyntaxError::new(
            optional.syntax().clone(),
            ErrorKind::Nesting,
        ));
    }
}
