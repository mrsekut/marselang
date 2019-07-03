mod interp;
mod lexer;
mod parser;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};

    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    // let tokens = lexer::lexer("1+2+4").unwrap();
    let tokens = lexer::lexer("12 + (3 - 123) * 3 / 4").unwrap();
    let p = parser::parser(tokens).unwrap();
    format!("{:?}", p);
    // use std::io::{self, BufRead, BufReader};
    // let mut interp = interp::Interpreter::new();

    // let stdin = io::stdin();
    // let stdin = stdin.lock();
    // let stdin = BufReader::new(stdin);
    // let mut lines = stdin.lines();

    // loop {
    //     prompt("> ").unwrap();
    //     if let Some(Ok(line)) = lines.next() {
    //         let tokens = lexer::lexer(&line).unwrap();
    //         let p = parser::parser(tokens).unwrap();
    //         let i = interp.eval(&p);
    //         println!("{:?}", i.unwrap());
    //     } else {
    //         break;
    //     }
    // }
}
