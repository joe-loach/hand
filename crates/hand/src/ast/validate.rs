use crate::{ast::Root, Error};

pub struct Valid(pub Root);

pub fn validate(root: Root, errors: &mut Vec<Error>) -> Valid {
    Valid(root)
}
