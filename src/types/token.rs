use std::fmt;

use super::{
    symbol::{
        constant::ResConst,
        function::ResFun,
        operator::Operator::{self, *},
    },
    CASNum,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
    //type of tokens outputted by scanner
    Name(String), //variable name
    Num(CASNum),
    Operator(Operator), //operators
    Const(ResConst),    //constants like pi, e, etc.
    ResFun(ResFun),     //reserved function
    Eof,                //end of file
}
/** */
#[derive(Debug, Clone, PartialEq)]
/// A type that is used for elements outputted by the scanner/lexer.
///
/// The first member is the type of the token, the enum 'TokenType'. The second member is the line number of the file it was found on, used for printing helpful error messages.
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) line_pos: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match &self.token_type {
            TokenType::Name(name) => name,
            TokenType::Const(constant) => &format!("{}", constant),
            TokenType::ResFun(fun) => &format!("{}", fun),
            TokenType::Num(value) => &format!("{}", value),
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
        };
        write!(f, "{}", display)
    }
}

//TODO: specify error codes
