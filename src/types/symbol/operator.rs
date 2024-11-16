use std::{
    collections::HashMap,
    fmt::{write, Display},
};

use phf_macros::phf_map;

#[derive(Clone, PartialEq, Debug, Copy)]

pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    LeftBracket,
    LeftParen,
    RightBracket,
    RightParen,
    Less,
    Greater,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Neg,
    Comma,
    Assign,
}

pub fn left_associative(operator: &Operator) -> bool {
    match operator {
        Operator::Exp | Operator::Assign | Operator::Neg => true,
        _ => false,
    }
}

pub fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Comma => 0,
        Operator::Assign => 1,
        Operator::Equal | Operator::NotEqual => 2,
        Operator::Less | Operator::LessEqual | Operator::Greater | Operator::GreaterEqual => 3,
        Operator::Add | Operator::Sub => 4,
        Operator::Mult | Operator::Div => 5,
        Operator::Exp => 6,
        Operator::Neg => 7,

        Operator::LeftBracket
        | Operator::LeftParen
        | Operator::RightBracket
        | Operator::RightParen => 8,
    }
}

pub(crate) static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    "+" => Operator::Add,
    "-" => Operator::Sub,
    "*" => Operator::Mult,
    "/" => Operator::Div,
    "^" => Operator::Exp,
    "[" => Operator::LeftBracket,
    "(" => Operator::LeftParen,
    "]" => Operator::RightBracket,
    ")" => Operator::RightParen,
    "<" => Operator::Less,
    ">" => Operator::Greater,
    "==" => Operator::Equal,
    "!=" => Operator::NotEqual,
    "<=" => Operator::LessEqual,
    ">=" => Operator::GreaterEqual,
    "," => Operator::Comma,
    "=" => Operator::Assign,
};

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Sub | Operator::Neg => "-",
                Operator::Mult => "*",
                Operator::Div => "/",
                Operator::Exp => "^",
                Operator::LeftBracket => "[",
                Operator::LeftParen => "(",
                Operator::RightBracket => "]",
                Operator::RightParen => ")",
                Operator::Less => "<",
                Operator::Greater => ">",
                Operator::Equal => "==",
                Operator::NotEqual => "!=",
                Operator::LessEqual => "<=",
                Operator::GreaterEqual => ">=",
                Operator::Comma => ",",
                Operator::Assign => "=",
            },
        )
    }
}
