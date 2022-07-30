use std::iter::Peekable;

use self::expression::Expression;
use crate::tokeniser::{self, Token, Tokeniser};

mod expression;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<Option<Token>>,
        got: Option<Token>,
    },
}

use ParseError::*;

pub fn parse(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    let exp = parse_exp(tokeniser)?;
    match tokeniser.next() {
        None => Ok(exp),
        t => Err(UnexpectedToken {
            expected: vec![None],
            got: t,
        }),
    }
}

fn parse_exp(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    let a = parse_or(tokeniser)?;
    match tokeniser.peek() {
        Some(Token::Imp) => {
            tokeniser.next();
            let b = parse_exp(tokeniser)?;
            Ok(Expression::Imp(Box::new(a), Box::new(b)))
        }
        _ => Ok(a),
    }
}

fn parse_or(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    let a = parse_and(tokeniser)?;
    match tokeniser.peek() {
        Some(Token::Or) => {
            tokeniser.next();
            let b = parse_or(tokeniser)?;
            Ok(Expression::Or(Box::new(a), Box::new(b)))
        }
        _ => Ok(a),
    }
}

fn parse_and(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    let a = parse_not(tokeniser)?;
    match tokeniser.peek() {
        Some(Token::And) => {
            tokeniser.next();
            let b = parse_and(tokeniser)?;
            Ok(Expression::And(Box::new(a), Box::new(b)))
        }
        _ => Ok(a),
    }
}

fn parse_not(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    match tokeniser.peek() {
        Some(Token::Not) => {
            tokeniser.next();
            let a = parse_literal(tokeniser)?;
            Ok(Expression::Not(Box::new(a)))
        }
        _ => parse_literal(tokeniser),
    }
}
fn parse_literal(tokeniser: &mut Peekable<Tokeniser>) -> Result<Expression, ParseError> {
    match tokeniser.next() {
        Some(Token::True) => Ok(Expression::Literal(true)),
        Some(Token::False) => Ok(Expression::Literal(false)),
        Some(Token::Var(name)) => Ok(Expression::Variable(name)),
        Some(Token::LBracket) => {
            let exp = parse_exp(tokeniser);
            match tokeniser.next() {
                Some(Token::RBracket) => exp,
                other => Err(UnexpectedToken {
                    expected: vec![Some(Token::RBracket)],
                    got: other,
                }),
            }
        }
        other => Err(UnexpectedToken {
            expected: vec![
                Some(Token::True),
                Some(Token::False),
                Some(Token::Var("var".to_string())),
                Some(Token::LBracket),
            ],
            got: other,
        }),
    }
}

#[test]
fn expression_parsing() {
    let text = "a and b and c g";
    let mut tokeniser = Tokeniser::new(&text);
    let exp = parse(&mut tokeniser);
    println!("{:?}", &exp);
}
