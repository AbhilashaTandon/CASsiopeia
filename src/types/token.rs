use std::fmt;

use super::{
    cas_error::CASErrorKind,
    symbol::{
        constant::ResConst,
        function::{Function, ResFun},
        operator::Operator::{self, *},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //type of tokens outputted by scanner
    Name(String),       //variable name
    Int(i128),          //integer literal
    Float(f64),         //floating point literal
    Operator(Operator), //operators
    Const(ResConst),    //constants like pi, e, etc.
    ResFun(ResFun),     //reserved function
    Calc,
    Sim,
    Der,
    Integral,
    Eof, //end of file
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match self {
            Token::Name(name) => name,
            Token::Const(_) => "CONST",
            Token::ResFun(_) => "RES_FUN",
            Token::Int(value) => &format!("{}", value),
            Token::Float(value) => &format!("{}", value),
            Token::Eof => "EOF",

            Token::Operator(op) => match op {
                Assign => "ASSIGN",
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
                Comma => "COMMA",
            },
            Token::Calc => "CALC",
            Token::Sim => "SIM",
            Token::Der => "DER",
            Token::Integral => "INTEGRAL",
        };
        write!(f, "{}", display)
    }
}

//TODO: specify error codes
