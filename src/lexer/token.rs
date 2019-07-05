use crate::lexer::Loc;
use crate::util::Annot;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(u64), // 0..9
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Lparen,      // (
    Rparen,      // )
    Bind,        // :=
    Var(String), // hoge
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;
        match self {
            Number(n) => n.fmt(f),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            Lparen => write!(f, "("),
            Rparen => write!(f, ")"),
            Bind => write!(f, ":="),
            Var(s) => s.fmt(f),
        }
    }
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    pub fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::Lparen, loc)
    }

    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::Rparen, loc)
    }

    pub fn bind(loc: Loc) -> Self {
        Self::new(TokenKind::Bind, loc)
    }

    pub fn var(s: impl Into<String>, loc: Loc) -> Self {
        Self::new(TokenKind::Var(s.into()), loc)
    }
}
