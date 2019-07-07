use crate::error::print_annot;
use crate::util::Annot;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    DivisionByZero,
    UnboundVariable(String),
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

use std::fmt;
impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => write!(f, "division by zero"),
            UnboundVariable(ref v) => write!(f, "variable {} is not bound", v),
        }
    }
}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        eprintln!("{}", self);
        print_annot(input, self.loc.clone());
    }
}

use std::error::Error as StdError;
impl StdError for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => "the right hand expression of the division evaluates to zero",
            UnboundVariable(_) => "variable is not bound",
        }
    }
}
