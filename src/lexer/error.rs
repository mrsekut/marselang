use crate::lexer::token::{Annot, Loc};

#[derive(Debug, PartialEq)]
pub enum LexErrorKind {
    InvalidChar(char),
}

pub type LexError = Annot<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
}
