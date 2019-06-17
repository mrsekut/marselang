#[derive(Debug, PartialEq)]
struct Loc(usize, usize);

#[derive(Debug, PartialEq)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Number(u64),
    Plus,
    Asterisk,
}

type Token = Annot<TokenKind>;
impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
}

#[derive(Debug, PartialEq)]
enum LexErrorKind {
    InvalidChar(char),
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
}

fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    while pos < input.len() {
        match input[pos] {
            b'0'...b'9' => {
                use std::str::from_utf8;
                let start = pos;
                while pos < input.len() && b"1234567890".contains(&input[pos]) {
                    pos += 1;
                }
                let n = from_utf8(&input[start..pos]).unwrap().parse().unwrap();
                tokens.push(Token::number(n, Loc(start, pos)));
            }
            b'+' => {
                tokens.push(Token::plus(Loc(pos, pos + 1)));
                pos = pos + 1;
            }
            b'*' => {
                tokens.push(Token::asterisk(Loc(pos, pos + 1)));
                pos = pos + 1;
            }
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

#[test]
fn test_lex() {
    assert_eq!(
        lex("12+3*123+3"),
        Ok(vec![
            Token::number(12, Loc(0, 2)),
            Token::plus(Loc(2, 3)),
            Token::number(3, Loc(3, 4)),
            Token::asterisk(Loc(4, 5)),
            Token::number(123, Loc(5, 8)),
            Token::plus(Loc(8, 9)),
            Token::number(3, Loc(9, 10)),
        ])
    )
}

fn main() {
    let l = lex("12+3*123+3").unwrap();
    println!("{:?}", l);
}
