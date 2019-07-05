use crate::lexer::{LexerError, Loc, Token};

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
                use std::str::from_utf8;
                let start = pos;
                while pos < input.len() && b"1234567890".contains(&input[pos]) {
                    pos += 1;
                }
                let n = from_utf8(&input[start..pos]).unwrap().parse().unwrap();
                tokens.push(Token::number(n, Loc(start, pos)));
            }
            b'+' => lex_a_token!(plus, pos),
            b'-' => lex_a_token!(minus, pos),
            b'*' => lex_a_token!(asterisk, pos),
            b'/' => lex_a_token!(slash, pos),
            b'(' => lex_a_token!(lparen, pos),
            b')' => lex_a_token!(rparen, pos),
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
