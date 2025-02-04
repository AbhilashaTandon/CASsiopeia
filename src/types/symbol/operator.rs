use std::fmt::Display;

use phf_macros::phf_map;

#[derive(Hash, Clone, PartialEq, Debug, Eq, Copy)]

pub(crate) enum Operator {
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

pub(crate) fn left_associative(operator: &Operator) -> bool {
    !matches!(operator, Operator::Exp | Operator::Assign | Operator::Neg)
}

pub(crate) fn precedence(op: &Operator) -> u8 {
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

pub(crate) fn commutative(op: &Operator) -> bool {
    match op {
        Operator::Add | Operator::Mult | Operator::Equal | Operator::NotEqual => true,
        Operator::Sub
        | Operator::Div
        | Operator::Exp
        | Operator::Less
        | Operator::Greater
        | Operator::LessEqual
        | Operator::GreaterEqual => false,
        Operator::LeftBracket
        | Operator::LeftParen
        | Operator::RightBracket
        | Operator::RightParen => false,
        Operator::Neg => false, //commutativity doesn't make sense for unary ops
        Operator::Comma => false, //function arguments can't be swapped around
        Operator::Assign => false, //a = b does a different thing than b = a
    }
}

//TODO: find some way of making a double ended map for this idk

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
