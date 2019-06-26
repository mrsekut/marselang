use crate::lexer::{Token, TokenKind};
use crate::parser::ast::{Ast, BinOp};

#[derive(Debug)]
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

use std::iter::Peekable;

// expr ::= term ( "+" term | "-" term) *
fn parse_expr<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let lhs = parse_term(tokens)?;
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
            Ok(Ast::binop(op, lhs, rhs, loc))
        }
        _ => Ok(lhs),
    }
}

// term ::= factor ("*" factor | "/" factor ) *
fn parse_term<Tokens: Iterator<Item = Token>>(
    tokens: &mut Peekable<Tokens>,
) -> Result<Ast, ParseError> {
    let lhs = parse_factor(tokens)?;
    match tokens.peek().map(|tok| tok.value) {
        Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Asterisk,
                    loc,
                }) => BinOp::mul(loc),
                Some(Token {
                    value: TokenKind::Minus,
                    loc,
                }) => BinOp::div(loc),
                _ => unreachable!(),
            };
            let rhs = parse_factor(tokens)?;
            let loc = lhs.loc.merge(&rhs.loc);
            Ok(Ast::binop(op, lhs, rhs, loc))
        }
        _ => Ok(lhs),
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
