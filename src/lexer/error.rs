use crate::lexer::token::{Annot, Loc};

#[derive(Debug, PartialEq)]
pub enum LexerErrorKind {
    InvalidChar(char),
}

pub type LexerError = Annot<LexerErrorKind>;

impl LexerError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        LexerError::new(LexerErrorKind::InvalidChar(c), loc)
    }
}

use std::fmt;
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::lexer::LexerErrorKind::*;
        let loc = &self.loc;
        match &self.value {
            InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
        }
    }
}
