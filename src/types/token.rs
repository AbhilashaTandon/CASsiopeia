use std::fmt;

use super::symbol::{
    constant::ResConst,
    function::ResFun,
    operator::Operator::{self, *},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_pos: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match &self.token_type {
            TokenType::Name(name) => &name,
            TokenType::Const(_) => "CONST",
            TokenType::ResFun(_) => "RES_FUN",
            TokenType::Int(value) => &format!("{}", value),
            TokenType::Float(value) => &format!("{}", value),
            TokenType::Eof => "EOF",

            TokenType::Operator(op) => match op {
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
                Neg => "NEG",
            },
            TokenType::Calc => "CALC",
            TokenType::Sim => "SIM",
            TokenType::Der => "DER",
            TokenType::Integral => "INTEGRAL",
        };
        write!(f, "{}", display)
    }
}

//TODO: specify error codes
