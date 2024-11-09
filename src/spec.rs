use std::fmt;

use crate::types::cas_num::CASNum;

pub const KEYWORDS: [&'static str; 4] = [
    //for calculating the value of an expression, uses variable values from symbol table
    "calc", //calculates value of expression, gives arbitrary precision fp
    "sim",
    //simplifies expression, ignores values of variables and results of functions
    "der",
    // acts on expressions, finds derivative wrt to all inputs
    // if 1 input just a function, if multiple inputs returns gradient vector
    "int",
    // integrates, indefinite
];

pub const RESERVED_FUNCTIONS: [&'static str; 17] = [
    "sqrt", "cbrt", "log2", "log10", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "asin",
    "acos", "atan", "acsc", "asec", "acot",
];

pub const RESERVED_CONSTANTS: [&'static str; 5] = ["pi", "e", "phi", "tau", "i"];
pub const OPERATORS: [char; 13] = [
    '+', '-', '*', '/', '^', '(', ')', ',', '<', '=', '>', '[', ']',
];

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
}

pub fn left_associative(operator: &Operator) -> bool {
    match operator {
        Operator::Exp => false,
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
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match self {
            TokenType::Name(name) | TokenType::Const(name) | TokenType::ResFun(name) => name,
            TokenType::Int(value) => &format!("{}", value),
            TokenType::Float(value) => &format!("{}", value),
            TokenType::Eof => "EOF",
            TokenType::Assign => "ASSIGN",
            TokenType::Operator(op) => match op {
                Operator::Add => "ADD",
                Operator::Sub => "SUB",
                Operator::Mult => "MULT",
                Operator::Div => "DIV",
                Operator::Exp => "EXP",
                Operator::LeftBracket => "LEFT_BRACKET",
                Operator::LeftParen => "LEFT_PAREN",
                Operator::RightBracket => "RIGHT_BRACKET",
                Operator::RightParen => "RIGHT_PAREN",
                Operator::Less => "LESS",
                Operator::Greater => "GREATER",
                Operator::Equal => "EQUAL",
                Operator::NotEqual => "NOT_EQUAL",
                Operator::LessEqual => "LESS_EQUAL",
                Operator::GreaterEqual => "GREATER_EQUAL",
            },
            TokenType::Comma => "COMMA",
            TokenType::Calc => "CALC",
            TokenType::Sim => "SIM",
            TokenType::Der => "DER",
            TokenType::Integral => "INTEGRAL",
            TokenType::Error => "ERR",
        };
        write!(f, "{}", display)
    }
}

pub fn to_token_name(symbol: &str) -> TokenType {
    match symbol {
        "=" => TokenType::Assign,
        "+" => TokenType::Operator(Operator::Add),
        "-" => TokenType::Operator(Operator::Sub),
        "*" => TokenType::Operator(Operator::Mult),
        "/" => TokenType::Operator(Operator::Div),
        "^" => TokenType::Operator(Operator::Exp),
        "(" => TokenType::Operator(Operator::LeftParen),
        ")" => TokenType::Operator(Operator::RightParen),
        "," => TokenType::Comma,
        "<" => TokenType::Operator(Operator::Less),
        ">" => TokenType::Operator(Operator::Greater),
        "calc" => TokenType::Calc,
        "sim" => TokenType::Sim,
        "der" => TokenType::Der,
        "int" => TokenType::Integral,
        "[" => TokenType::Operator(Operator::LeftBracket),
        "]" => TokenType::Operator(Operator::RightBracket),
        _ => TokenType::Error,
    }
}

//TODO: specify error codes

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol<'a> {
    //type of tokens of output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function { num_args: usize, name: &'a str },
    Num { value: CASNum },
    Const { name: &'a str },
}

impl Symbol<'_> {
    pub fn num_args(&self) -> usize {
        match self {
            Symbol::Variable { .. } => 0,
            Symbol::Operator(..) => 2,
            Symbol::Function { num_args, .. } => *num_args,
            Symbol::Num { .. } => 0,
            Symbol::Const { .. } => 0,
        }
    }
}
