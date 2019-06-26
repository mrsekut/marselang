use crate::lexer::Annot;
use crate::parser::{Ast, BinOp};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<u64, InterpreterError> {
        use crate::parser::AstKind::*;
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

pub type InterpreterError = Annot<InterpreterErrorKind>;

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
