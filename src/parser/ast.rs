use crate::lexer::{Annot, Loc};

#[derive(Debug, PartialEq)]
pub enum UniOpKind {
    Plus,
    Minus,
}

pub type UniOp = Annot<UniOpKind>;

impl UniOp {
    pub fn plus(loc: Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }
    pub fn minus(loc: Loc) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

pub type BinOp = Annot<BinOpKind>;

impl BinOp {
    pub fn add(loc: Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }
    pub fn sub(loc: Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }
    pub fn mul(loc: Loc) -> Self {
        Self::new(BinOpKind::Mul, loc)
    }
    pub fn div(loc: Loc) -> Self {
        Self::new(BinOpKind::Div, loc)
    }
}

#[derive(Debug, PartialEq)]
pub enum AstKind {
    Num(u64),
    UniOp {
        op: UniOp,
        e: Box<Ast>,
    },
    BinOp {
        op: BinOp,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
}

pub type Ast = Annot<AstKind>;

impl Ast {
    pub fn num(n: u64, loc: Loc) -> Self {
        Self::new(AstKind::Num(n), loc)
    }

    pub fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }

    pub fn binop(op: BinOp, lhs: Ast, rhs: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            loc,
        )
    }
}