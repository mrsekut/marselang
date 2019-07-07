use crate::interp::{InterpreterError, InterpreterErrorKind};
use crate::parser::{Ast, BinOp};
use std::collections::HashMap;

pub struct Interpreter(HashMap<String, u64>);

impl Interpreter {
    pub fn new() -> Self {
        Interpreter(HashMap::new())
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
            Bind { ref var, ref body } => {
                // TODO: clean
                let e = self.eval(body)?;
                self.0.insert(var.clone(), e);
                Ok(0)
            }
            Var(ref s) => self.0.get(s).cloned().ok_or(InterpreterError::new(
                InterpreterErrorKind::UnboundVariable(s.clone()),
                expr.loc.clone(),
            )),
            _ => unreachable!(),
        }
    }

    fn eval_binop(&mut self, op: &BinOp, lhs: u64, rhs: u64) -> Result<u64, InterpreterErrorKind> {
        use crate::parser::BinOpKind::*;
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
