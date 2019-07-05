use crate::lexer::{LexerError, Loc, Token};

// fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
//     while pos < input.len() && f(input[pos]) {
//         println!("pos: {:?}", pos);
//         println!("len: {:?}", input.len());
//         pos += 1;
//     }
//     pos
// }

pub fn lexer(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;
    macro_rules! lex_a_token {
        ($token_method:ident, $pos:ident) => {{
            tokens.push(Token::$token_method(Loc(pos, pos + 1)));
            pos = $pos + 1;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'0'...b'9' => {
                // TODO: clean
                use std::str::from_utf8;
                let start = pos;
                while pos < input.len() && b"1234567890".contains(&input[pos]) {
                    pos += 1;
                }
                let n = from_utf8(&input[start..pos]).unwrap().parse().unwrap();
                tokens.push(Token::number(n, Loc(start, pos)));
                // let end = recognize_many(input, pos, |b| b"0123456789".contains(&b));
                // let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();
                // tokens.push(Token::number(n, Loc(start, end)));
            }
            b'a'...b'z' => {
                // TODO: clean
                use std::str::from_utf8;
                let start = pos;
                // let end = recognize_many(input, start, |b| b"abcdefghijklmnopqrstuvwxyz".contains(&b));
                // let s = from_utf8(&input[start..end]).unwrap();
                // tokens.push(Token::var(s, Loc(start, end)));
                while pos < input.len() && b"abcdefghijklmnopqrstuvwxyz".contains(&input[pos]) {
                    pos += 1;
                }
                let s = from_utf8(&input[start..pos]).unwrap();
                tokens.push(Token::var(s, Loc(start, pos)));
            }
            b'+' => lex_a_token!(plus, pos),
            b'-' => lex_a_token!(minus, pos),
            b'*' => lex_a_token!(asterisk, pos),
            b'/' => lex_a_token!(slash, pos),
            b'(' => lex_a_token!(lparen, pos),
            b')' => lex_a_token!(rparen, pos),
            b':' => {
                // TODO: clean
                let start = pos;
                // let end = recognize_many(input, start, |b| b"=".contains(&b));
                // tokens.push(Token::bind(Loc(start, end)));
                // FIXME:
                while pos < input.len() && b":=".contains(&input[pos]) {
                    pos += 1;
                }
                tokens.push(Token::bind(Loc(start, pos)));
            }
            b' ' | b'\n' | b'\t' => {
                pos = pos + 1;
            }
            b => return Err(LexerError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

#[test]
fn test_lexer() {
    assert_eq!(
        lexer("12 + (3 - 123) * 3 / 4"),
        Ok(vec![
            Token::number(12, Loc(0, 2)),
            Token::plus(Loc(3, 4)),
            Token::lparen(Loc(5, 6)),
            Token::number(3, Loc(6, 7)),
            Token::minus(Loc(8, 9)),
            Token::number(123, Loc(10, 13)),
            Token::rparen(Loc(13, 14)),
            Token::asterisk(Loc(15, 16)),
            Token::number(3, Loc(17, 18)),
            Token::slash(Loc(19, 20)),
            Token::number(4, Loc(21, 22)),
        ])
    )
}

#[test]
fn test_bind_lexer() {
    assert_eq!(
        lexer("hoge := 42"),
        Ok(vec![
            Token::var("hoge", Loc(0, 4)),
            Token::bind(Loc(5, 7)),
            Token::number(42, Loc(8, 10)),
        ])
    )
}
