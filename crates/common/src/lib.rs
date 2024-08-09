pub trait Language: rowan::Language {
    type Error;
}

pub trait Filterable {
    /// Is trivial and can be filtered out.
    fn is_trivia(&self) -> bool;
    /// Represents whitespace.
    fn is_whitespace(&self) -> bool;
}
