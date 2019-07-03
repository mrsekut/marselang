use crate::lexer::{Loc, Token, TokenKind};
use crate::parser::ast::{Ast, BinOp};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    RedundantExpression(Token),
    UnclosedOpenParen(Token),
    NotExpression(Token),
    Eof,
}

pub fn parser(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ast = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ast),
    }
}

// expr ::= term expr_loop
// expr ::= ("+" | "-") expr_loop | ε
fn parse_expr<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let mut lhs = parse_term(tokens)?;
    loop {
        match tokens.peek().map(|tok| tok.value) {
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

// term ::= factor term_loop
// term ::= ("*" | "/") factor term_loop | ε
fn parse_term<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let mut lhs = parse_factor(tokens)?;
    loop {
        match tokens.peek().map(|tok| tok.value) {
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
                let rhs = parse_factor(tokens)?;
                let loc = lhs.loc.merge(&rhs.loc);
                lhs = Ast::binop(op, lhs, rhs, loc)
            }
            _ => return Ok(lhs),
        }
    }
}

// factor ::= nat | "(" expr ")"
fn parse_factor<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            TokenKind::Number(n) => Ok(Ast::num(n, tok.loc)),
            TokenKind::Lparen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::Rparen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

#[test]
fn test_parser() {
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
    )
}
