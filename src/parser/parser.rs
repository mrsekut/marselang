use crate::lexer::{Token, TokenKind};
use crate::parser::ast::{Ast, BinOp, UniOp};
use crate::parser::error::ParserError;
use std::iter::Peekable;

pub fn parser(tokens: Vec<Token>) -> Result<Ast, ParserError> {
    let mut tokens = tokens.into_iter().peekable();
    let ast = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParserError::RedundantExpression(tok)),
        None => Ok(ast),
    }
}

// expr ::= term expr_loop
// expr ::= ("+" | "-") expr_loop | ε
fn parse_expr<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParserError> {
    let mut lhs = parse_term(tokens)?;
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
// term ::= ("*" | "/") unnary term_loop | ε
fn parse_term<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
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

fn parse_unary<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
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
    tokens: &mut Peekable<Tokens>,
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
fn test_uniop_jparser() {
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
