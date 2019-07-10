use crate::interp::{InterpreterError, InterpreterErrorKind};
use crate::parser::{Ast, BinOp};
use std::collections::HashMap;

pub struct Interpreter(HashMap<String, i32>);

impl Interpreter {
    pub fn new() -> Self {
        Interpreter(HashMap::new())
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<i32, InterpreterError> {
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

    fn eval_binop(&mut self, op: &BinOp, lhs: i32, rhs: i32) -> Result<i32, InterpreterErrorKind> {
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

#[test]
fn test_eval() {
    use crate::lexer::Loc;
    let mut interp = Interpreter::new();
    use crate::parser::{Ast, BinOp};

    // "1 + 2"
    let ast = Ast::binop(
        BinOp::add(Loc(2, 3)),
        Ast::num(1, Loc(0, 1)),
        Ast::num(2, Loc(4, 5)),
        Loc(0, 5),
    );

    let result = interp.eval(&ast).unwrap();
    assert_eq!(result, 3);
}

// #[test]
// fn test_eval_neg() {
//     use crate::lexer::Loc;
//     let mut interp = Interpreter::new();
//     use crate::parser::{Ast, BinOp, UniOp};

//     // "2 + (-1)"
//     let ast = Ast::binop(
//         BinOp::add(Loc(2, 3)),
//         Ast::num(2, Loc(0, 1)),
//         Ast::uniop(UniOp::minus(Loc(5, 6)), Ast::num(1, Loc(6, 7)), Loc(6, 7)),
//         Loc(0, 7),
//     );

//     let result = interp.eval(&ast).unwrap();
//     assert_eq!(result, 1);
// }

#[test]
fn test_eval_in_0() {
    use crate::lexer::Loc;
    let mut interp = Interpreter::new();
    use crate::parser::{Ast, BinOp};

    // "1 + 2 - 3 * 2"
    let ast = Ast::binop(
        BinOp::sub(Loc(6, 7)),
        Ast::binop(
            BinOp::add(Loc(2, 3)),
            Ast::num(1, Loc(0, 1)),
            Ast::num(2, Loc(4, 5)),
            Loc(0, 5),
        ),
        Ast::binop(
            BinOp::mul(Loc(10, 11)),
            Ast::num(3, Loc(8, 9)),
            Ast::num(2, Loc(12, 13)),
            Loc(8, 13),
        ),
        Loc(0, 13),
    );

    let result = interp.eval(&ast).unwrap();
    assert_eq!(result, -3);
}

// #[test]
// fn test_eval_in_var() {
//     use crate::lexer::Loc;
//     let mut interp = Interpreter::new();
//     use crate::parser::{Ast, BinOp};

//     // "hoge := 4\n hoge"
//     let ast = Ast::binop(
//         BinOp::sub(Loc(6, 7)),
//         Ast::binop(
//             BinOp::add(Loc(2, 3)),
//             Ast::num(1, Loc(0, 1)),
//             Ast::num(2, Loc(4, 5)),
//             Loc(0, 5),
//         ),
//         Ast::binop(
//             BinOp::mul(Loc(10, 11)),
//             Ast::num(3, Loc(8, 9)),
//             Ast::num(2, Loc(12, 13)),
//             Loc(8, 13),
//         ),
//         Loc(0, 13),
//     );

//     let result = interp.eval(&ast).unwrap();
//     assert_eq!(result, 0);
// }
