mod lexer;
use lexer::lexer;
// TODO: {Token}で読み込む

#[derive(Debug, PartialEq)]
pub enum UniOpKind {
    Plus,
    Minus,
}

type UniOp = lexer::Annot<UniOpKind>;

impl UniOp {
    fn plus(loc: lexer::Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }
    fn minus(loc: lexer::Loc) -> Self {
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

type BinOp = lexer::Annot<BinOpKind>;

impl BinOp {
    fn add(loc: lexer::Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }
    fn sub(loc: lexer::Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }
    fn mul(loc: lexer::Loc) -> Self {
        Self::new(BinOpKind::Mul, loc)
    }
    fn div(loc: lexer::Loc) -> Self {
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

pub type Ast = lexer::Annot<AstKind>;

impl Ast {
    fn num(n: u64, loc: lexer::Loc) -> Self {
        Self::new(AstKind::Num(n), loc)
    }

    fn uniop(op: UniOp, e: Ast, loc: lexer::Loc) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }

    fn binop(op: BinOp, lhs: Ast, rhs: Ast, loc: lexer::Loc) -> Self {
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

#[derive(Debug)]
pub enum ParseError {
    RedundantExpression(lexer::Token),
    UnclosedOpenParen(lexer::Token),
    NotExpression(lexer::Token),
    Eof,
}

pub fn parser(tokens: Vec<lexer::Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ast = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ast),
    }
}

use std::iter::Peekable;

// expr ::= term ( "+" term | "-" term) *
fn parse_expr<Tokens: Iterator<Item = lexer::Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let lhs = parse_term(tokens)?;
    match tokens.peek().map(|tok| tok.value) {
        Some(lexer::TokenKind::Plus) | Some(lexer::TokenKind::Minus) => {
            let op = match tokens.next() {
                Some(lexer::Token {
                    value: lexer::TokenKind::Plus,
                    loc,
                }) => BinOp::add(loc),
                Some(lexer::Token {
                    value: lexer::TokenKind::Minus,
                    loc,
                }) => BinOp::sub(loc),
                _ => unreachable!(),
            };
            let rhs = parse_term(tokens)?;
            let loc = lhs.loc.merge(&rhs.loc);
            Ok(Ast::binop(op, lhs, rhs, loc))
        }
        _ => Ok(lhs),
    }
}

// term ::= factor ("*" factor | "/" factor ) *
fn parse_term<Tokens: Iterator<Item = lexer::Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let lhs = parse_factor(tokens)?;
    match tokens.peek().map(|tok| tok.value) {
        Some(lexer::TokenKind::Asterisk) | Some(lexer::TokenKind::Slash) => {
            let op = match tokens.next() {
                Some(lexer::Token {
                    value: lexer::TokenKind::Asterisk,
                    loc,
                }) => BinOp::mul(loc),
                Some(lexer::Token {
                    value: lexer::TokenKind::Minus,
                    loc,
                }) => BinOp::div(loc),
                _ => unreachable!(),
            };
            let rhs = parse_factor(tokens)?;
            let loc = lhs.loc.merge(&rhs.loc);
            Ok(Ast::binop(op, lhs, rhs, loc))
        }
        _ => Ok(lhs),
    }
}

// factor ::= nat | "(" expr ")"
fn parse_factor<Tokens: Iterator<Item = lexer::Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            lexer::TokenKind::Number(n) => Ok(Ast::num(n, tok.loc)),
            lexer::TokenKind::Lparen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(lexer::Token {
                        value: lexer::TokenKind::Rparen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<u64, InterpreterError> {
        use crate::AstKind::*;
        match expr.value {
            Num(n) => Ok(n),
            BinOp {
                ref op,
                ref lhs,
                ref rhs,
            } => {
                let l = self.eval(lhs)?;
                let r = self.eval(rhs)?;
                self.eval_binop(op, l, r)
                    .map_err(|e| InterpreterError::new(e, expr.loc.clone()))
            }
            _ => unreachable!(),
        }
    }

    fn eval_binop(&mut self, op: &BinOp, lhs: u64, rhs: u64) -> Result<u64, InterpreterErrorKind> {
        match op.value {
            Add => Ok(lhs + rhs),
            Sub => Ok(lhs - rhs),
            Mul => Ok(lhs * rhs),
            Div => {
                if rhs == 0 {
                    Err(InterpreterErrorKind::DivisionByZero)
                } else {
                    Ok(lhs / rhs)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    DivisionByZero,
}

pub type InterpreterError = lexer::Annot<InterpreterErrorKind>;

use std::error::Error as StdError;
use std::fmt;

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl StdError for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => "the right hand expression of the division evaluates to zero",
        }
    }
}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        eprintln!("{}", self);
        // print_annot(input, self.loc.clone());
    }
}

fn main() {
    let mut interp = Interpreter::new();
    let l = lexer::lexer("10+3*9").unwrap();
    let p = parser(l).unwrap();
    let i = interp.eval(&p);
    println!("{:?}", i);
}
