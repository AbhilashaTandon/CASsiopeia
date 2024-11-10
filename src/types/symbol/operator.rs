use std::collections::HashMap;

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
    Comma,
    Assign,
}

pub fn left_associative(operator: &Operator) -> bool {
    match operator {
        Operator::Exp | Operator::Assign => false,
        _ => true,
    }
}

pub fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Less
        | Operator::Greater
        | Operator::Equal
        | Operator::NotEqual
        | Operator::LessEqual
        | Operator::GreaterEqual => 4,

        Operator::Add | Operator::Sub => 5,
        Operator::Mult | Operator::Div => 6,
        Operator::Exp => 7,
        Operator::LeftBracket
        | Operator::LeftParen
        | Operator::RightBracket
        | Operator::RightParen => 9,
        Operator::Comma => todo!(),
        Operator::Assign => todo!(),
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
