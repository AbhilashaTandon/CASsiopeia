use std::fmt;

use super::symbol::operator::Operator::{self, *};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //type of tokens outputted by scanner
    Name(String), //variable name
    Int(i128),    //integer literal
    Float(f64),   //floating point literal
    Eof,          //end of file
    //operators
    Assign,
    Operator(Operator),
    Comma,
    Calc,
    Sim,
    Der,
    Integral,
    Const(String),  //constants like pi, e, etc.
    ResFun(String), //reserved function
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match self {
            Token::Name(name) | Token::Const(name) | Token::ResFun(name) => name,
            Token::Int(value) => &format!("{}", value),
            Token::Float(value) => &format!("{}", value),
            Token::Eof => "EOF",
            Token::Assign => "ASSIGN",
            Token::Operator(op) => match op {
                Add => "ADD",
                Sub => "SUB",
                Mult => "MULT",
                Div => "DIV",
                Exp => "EXP",
                LeftBracket => "LEFT_BRACKET",
                LeftParen => "LEFT_PAREN",
                RightBracket => "RIGHT_BRACKET",
                RightParen => "RIGHT_PAREN",
                Less => "LESS",
                Greater => "GREATER",
                Equal => "EQUAL",
                NotEqual => "NOT_EQUAL",
                LessEqual => "LESS_EQUAL",
                GreaterEqual => "GREATER_EQUAL",
            },
            Token::Comma => "COMMA",
            Token::Calc => "CALC",
            Token::Sim => "SIM",
            Token::Der => "DER",
            Token::Integral => "INTEGRAL",
            Token::Error => "ERR",
        };
        write!(f, "{}", display)
    }
}

pub fn to_token_name(symbol: &str) -> Token {
    match symbol {
        "=" => Token::Assign,
        "+" => Token::Operator(Add),
        "-" => Token::Operator(Sub),
        "*" => Token::Operator(Mult),
        "/" => Token::Operator(Div),
        "^" => Token::Operator(Exp),
        "(" => Token::Operator(LeftParen),
        ")" => Token::Operator(RightParen),
        "," => Token::Comma,
        "<" => Token::Operator(Less),
        ">" => Token::Operator(Greater),
        "calc" => Token::Calc,
        "sim" => Token::Sim,
        "der" => Token::Der,
        "int" => Token::Integral,
        "[" => Token::Operator(LeftBracket),
        "]" => Token::Operator(RightBracket),
        _ => Token::Error,
    }
}

//TODO: specify error codes
