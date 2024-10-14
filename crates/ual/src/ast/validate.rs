use crate::ast::{Item, Root, Special};
use crate::error::{ErrorKind, SyntaxError};

use super::AstNode;

/// Validates a syntax tree from its root.
pub fn validate(root: Root, errors: &mut Vec<SyntaxError>) {
    for it in root.items() {
        if let Item::Special(s) = it.clone() {
            is_closed(s, errors)
        }
    }
}

fn is_closed(special: Special, errors: &mut Vec<SyntaxError>) {
    let has_right_brace = special.right_brace().is_some();

    if !has_right_brace {
        let node = special.syntax().clone();
        errors.push(SyntaxError::new(node, ErrorKind::UnClosed));
    }
}
