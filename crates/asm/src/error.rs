use codespan_reporting::diagnostic::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsmError {
    #[error("hand errors")]
    Hand(Vec<hand::Error>),
    #[error("instruction encoding failed")]
    EncodingError { instruction: String },
}

impl AsmError {
    pub fn report(self) -> Box<dyn Iterator<Item = Diagnostic<()>>> {
        match self {
            AsmError::Hand(errors) => Box::new(errors.into_iter().map(|err| err.report())),
            AsmError::EncodingError { instruction } => Box::new(std::iter::once(
                Diagnostic::error()
                    .with_code("E201")
                    .with_message(format!("Failed to encode instruction '{instruction}'")),
            )),
        }
    }
}
