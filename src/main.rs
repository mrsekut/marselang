mod interp;
mod lexer;
mod parser;

fn main() {
    let mut interp = interp::Interpreter::new();
    let l = lexer::lexer("10+3*9").unwrap();
    let p = parser::parser(l).unwrap();
    let i = interp.eval(&p);
    println!("{:?}", i);
}
