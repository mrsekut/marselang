use crate::lexer::{Token, TokenKind};
use crate::parser::ast::{Ast, BinOp, UniOp};
use crate::parser::error::ParserError;
use itertools::{multipeek, MultiPeek};
use std::iter::IntoIterator;

pub fn parser(tokens: Vec<Token>) -> Result<Ast, ParserError> {
    let mut tokens = multipeek(tokens.into_iter());
    let ast = parse_stmt(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParserError::RedundantExpression(tok)),
        None => Ok(ast),
    }
}

// stmt ::= expr
fn parse_stmt<Tokens: Iterator<Item = Token>>(
    tokens: &mut MultiPeek<Tokens>,
) -> Result<Ast, ParserError> {
    match tokens.peek().map(|tok| tok.value.clone()) {
        Some(TokenKind::Var(_)) => match tokens.peek().map(|tok| tok.value.clone()) {
            Some(TokenKind::Bind) => {
                let var = match tokens.next() {
                    Some(Token {
                        value: TokenKind::Var(s),
                        loc,
                    }) => (s, loc),
                    _ => unreachable!(),
                };
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::Bind,
                        loc: _,
                    }) => (),
                    _ => unreachable!(),
                };
                // TODO:
                let body = parse_expr(tokens)?;
                let loc = var.1.merge(&body.loc);
                Ok(Ast::bind(var.0, Box::new(body), loc))
            }
            _ => parse_expr(tokens),
        },
        _ => parse_expr(tokens),
    }
}

// expr ::= term expr_loop
// expr_loop ::= ("+" | "-") expr_loop | ε
fn parse_expr<Tokens: Iterator<Item = Token>>(
    tokens: &mut MultiPeek<Tokens>,
) -> Result<Ast, ParserError> {
    tokens.reset_peek();
    let mut lhs = parse_term(tokens)?;
    tokens.reset_peek();
    loop {
        match tokens.peek().map(|tok| tok.value.clone()) {
            Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                let op = match tokens.next() {
                    Some(Token {
                        value: TokenKind::Plus,
                        loc,
                    }) => BinOp::add(loc),
                    Some(Token {
                        value: TokenKind::Minus,
                        loc,
                    }) => BinOp::sub(loc),
                    _ => unreachable!(),
                };
                let rhs = parse_term(tokens)?;
                let loc = lhs.loc.merge(&rhs.loc);
                lhs = Ast::binop(op, lhs, rhs, loc)
            }
            _ => return Ok(lhs),
        }
    }
}

// term ::= unnary term_loop
// term_loop ::= ("*" | "/") unnary term_loop | ε
fn parse_term<Tokens: Iterator<Item = Token>>(
    tokens: &mut MultiPeek<Tokens>,
) -> Result<Ast, ParserError> {
    let mut lhs = parse_unary(tokens)?;
    loop {
        match tokens.peek().map(|tok| tok.value.clone()) {
            Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
                let op = match tokens.next() {
                    Some(Token {
                        value: TokenKind::Asterisk,
                        loc,
                    }) => BinOp::mul(loc),
                    Some(Token {
                        value: TokenKind::Slash,
                        loc,
                    }) => BinOp::div(loc),
                    _ => unreachable!(),
                };
                let rhs = parse_unary(tokens)?;
                let loc = lhs.loc.merge(&rhs.loc);
                lhs = Ast::binop(op, lhs, rhs, loc)
            }
            _ => return Ok(lhs),
        }
    }
}

// unary ::= factor | ("+" | "-") factor
fn parse_unary<Tokens: Iterator<Item = Token>>(
    tokens: &mut MultiPeek<Tokens>,
) -> Result<Ast, ParserError> {
    match tokens.peek().map(|tok| tok.value.clone()) {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Plus,
                    loc,
                }) => UniOp::plus(loc),
                Some(Token {
                    value: TokenKind::Minus,
                    loc,
                }) => UniOp::minus(loc),
                _ => unreachable!(),
            };
            let e = parse_factor(tokens)?;
            let loc = e.loc.merge(&e.loc);
            Ok(Ast::uniop(op, e, loc))
        }
        _ => parse_factor(tokens),
    }
}

// factor ::= nat | "(" expr ")"
fn parse_factor<Tokens: Iterator<Item = Token>>(
    tokens: &mut MultiPeek<Tokens>,
) -> Result<Ast, ParserError> {
    tokens
        .next()
        .ok_or(ParserError::Eof)
        .and_then(|tok| match tok.value {
            TokenKind::Number(n) => Ok(Ast::num(n, tok.loc)),
            TokenKind::Lparen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::Rparen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParserError::RedundantExpression(t)),
                    _ => Err(ParserError::UnclosedOpenParen(tok)),
                }
            }
            TokenKind::Var(s) => Ok(Ast::var(s, tok.loc)),
            _ => Err(ParserError::NotExpression(tok)),
        })
}

#[test]
fn test_binop_parser() {
    use crate::lexer::{Loc, Token};

    // "12 + (3 - 123) * 3 / 4",
    let ast = parser(vec![
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
    ]);

    assert_eq!(
        ast,
        Ok(Ast::binop(
            BinOp::add(Loc(3, 4)),
            Ast::num(12, Loc(0, 2)),
            Ast::binop(
                BinOp::div(Loc(19, 20)),
                Ast::binop(
                    BinOp::mul(Loc(15, 16)),
                    Ast::binop(
                        BinOp::sub(Loc(8, 9)),
                        Ast::num(3, Loc(6, 7)),
                        Ast::num(123, Loc(10, 13)),
                        Loc(6, 13)
                    ),
                    Ast::num(3, Loc(17, 18)),
                    Loc(6, 18)
                ),
                Ast::num(4, Loc(21, 22)),
                Loc(6, 22)
            ),
            Loc(0, 22)
        ))
    );
}

#[test]
fn test_uniop_parser() {
    use crate::lexer::{Loc, Token};

    // "-2+(+3)"
    let ast = parser(vec![
        Token::minus(Loc(0, 1)),
        Token::number(2, Loc(1, 2)),
        Token::plus(Loc(2, 3)),
        Token::lparen(Loc(3, 4)),
        Token::plus(Loc(4, 5)),
        Token::number(3, Loc(5, 6)),
        Token::rparen(Loc(6, 7)),
    ]);

    assert_eq!(
        ast,
        Ok(Ast::binop(
            BinOp::add(Loc(2, 3)),
            Ast::uniop(UniOp::minus(Loc(0, 1)), Ast::num(2, Loc(1, 2)), Loc(1, 2)),
            Ast::uniop(UniOp::plus(Loc(4, 5)), Ast::num(3, Loc(5, 6)), Loc(5, 6)),
            Loc(1, 6)
        ))
    );
}

#[test]
fn test_bind_parser() {
    use crate::lexer::{Loc, Token};

    // "hoge := 40 + 2"
    let ast = parser(vec![
        Token::var("hoge", Loc(0, 4)),
        Token::bind(Loc(5, 7)),
        Token::number(40, Loc(8, 10)),
        Token::plus(Loc(11, 12)),
        Token::number(2, Loc(13, 14)),
    ]);

    assert_eq!(
        ast,
        Ok(Ast::bind(
            "hoge".to_string(),
            Box::new(Ast::binop(
                BinOp::add(Loc(11, 12)),
                Ast::num(40, Loc(8, 10)),
                Ast::num(2, Loc(13, 14)),
                Loc(8, 14)
            )),
            Loc(0, 14)
        ))
    );
}

#[test]
fn test_bin0d_parser() {
    use crate::lexer::{Loc, Token};

    // "x + x"
    let ast = parser(vec![
        Token::var("x", Loc(0, 1)),
        Token::plus(Loc(2, 3)),
        Token::var("x", Loc(4, 5)),
    ]);

    assert_eq!(
        ast,
        Ok(Ast::binop(
            BinOp::add(Loc(2, 3)),
            Ast::var("x".to_string(), Loc(0, 1)),
            Ast::var("x".to_string(), Loc(4, 5)),
            Loc(0, 5)
        ))
    );
}
