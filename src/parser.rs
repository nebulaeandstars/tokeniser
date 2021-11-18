use std::collections::VecDeque;
use std::fmt;

use super::token::Token;

#[derive(Debug)]
pub enum Expression {
    Add(MExpression, Box<Expression>),
    Subtract(MExpression, Box<Expression>),
    MExpression(MExpression),
}

#[derive(Debug)]
pub enum MExpression {
    Multiply(Atom, Box<MExpression>),
    Divide(Atom, Box<MExpression>),
    Atoms(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Expression(Box<Expression>),
    Integer(i64),
    Variable(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Add(e1, e2) => write!(f, "{} + {}", e1, e2),
            Expression::Subtract(e1, e2) => write!(f, "{} - {}", e1, e2),
            Expression::MExpression(exp) => write!(f, "{}", exp),
        }
    }
}

impl fmt::Display for MExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MExpression::Multiply(e1, e2) => write!(f, "({} * {})", e1, e2),
            MExpression::Divide(e1, e2) => write!(f, "({} / {})", e1, e2),
            MExpression::Atoms(atoms) => write!(f, "{}", atoms),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Expression(exp) => write!(f, "({})", exp),
            Atom::Integer(int) => write!(f, "{}", int),
            Atom::Variable(var) => write!(f, "{}", var),
        }
    }
}

impl From<Vec<Token>> for Expression {
    fn from(stream: Vec<Token>) -> Self {
        let mut stream: VecDeque<Token> = stream.into();
        parse_expression(&mut stream).unwrap()
    }
}

fn parse_expression<'a>(stream: &mut VecDeque<Token>) -> Option<Expression> {
    if let Some(exp1) = parse_m_expression(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Add) =>
                if let Some(exp2) = parse_expression(stream) {
                    Some(Expression::Add(exp1, Box::new(exp2)))
                }
                else {
                    None
                },
            Some(Token::Subtract) =>
                if let Some(exp2) = parse_expression(stream) {
                    Some(Expression::Subtract(exp1, Box::new(exp2)))
                }
                else {
                    None
                },

            Some(_) => {
                stream.push_front(token.unwrap());
                Some(Expression::MExpression(exp1))
            },
            None => Some(Expression::MExpression(exp1)),
        }
    }
    else {
        None
    }
}

fn parse_m_expression<'a>(stream: &mut VecDeque<Token>) -> Option<MExpression> {
    if let Some(atom) = parse_atom(stream) {
        let token = stream.pop_front();

        match token {
            Some(Token::Multiply) =>
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Multiply(atom, Box::new(exp)))
                }
                else {
                    None
                },
            Some(Token::Divide) =>
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Divide(atom, Box::new(exp)))
                }
                else {
                    None
                },

            Some(_) => {
                stream.push_front(token.unwrap());
                if let Some(exp) = parse_m_expression(stream) {
                    Some(MExpression::Multiply(atom, Box::new(exp)))
                }
                else {
                    Some(MExpression::Atoms(atom))
                }
            },
            None => Some(MExpression::Atoms(atom)),
        }
    }
    else {
        None
    }
}

fn parse_atom<'a>(stream: &mut VecDeque<Token>) -> Option<Atom> {
    let token = stream.pop_front();

    match token {
        Some(Token::Integer(num)) => Some(Atom::Integer(num)),
        Some(Token::Variable(var)) => Some(Atom::Variable(var.clone())),

        Some(Token::LeftParenthesis) => {
            if let Some(exp) = parse_expression(stream) {
                if let Some(Token::RightParenthesis) = stream.pop_front() {
                    Some(Atom::Expression(Box::new(exp)))
                }
                else {
                    panic!("Unmatched brackets!")
                }
            }
            else {
                None
            }
        },

        Some(_) => {
            stream.push_front(token.unwrap());
            None
        },
        None => None,
    }
}
