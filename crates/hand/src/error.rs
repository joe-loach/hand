use codespan_reporting::diagnostic::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("body has no statement")]
    NoBodyStatement,

    #[error("found multiple occurrences of label")]
    MultipleSameLabel,
}

impl Error {
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            Error::NoBodyStatement => Diagnostic::error()
                .with_code("E101")
                .with_message("body has no statement"),
            Error::MultipleSameLabel => Diagnostic::error()
                .with_code("E102")
                .with_message("found multiple occurrences of label"),
        }
    }
}
