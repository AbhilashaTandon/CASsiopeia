use std::fmt;

pub(crate) const KEYWORDS: [&'static str; 4] = [
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

pub(crate) const RESERVED_FUNCTIONS: [&'static str; 17] = [
    "sqrt", "cbrt", "log2", "log10", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "asin",
    "acos", "atan", "acsc", "asec", "acot",
];

pub(crate) const RESERVED_CONSTANTS: [&'static str; 5] = ["pi", "e", "phi", "tau", "i"];
pub const OPERATORS: [char; 13] = [
    '+', '-', '*', '/', '^', '(', ')', ',', '<', '=', '>', '[', ']',
];
pub const COMP: [&'static str; 3] = ["!=", "<=", ">="];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum TokenType {
    Name,  //variable name
    Int,   //integer literal
    Float, //floating point literal
    Eof,   //end of file
    //operators
    Assign,
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Less,
    Greater,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Calc,
    Sim,
    Der,
    Integral,
    Const,  //constants like pi, e, etc.
    ResFun, //reserved function
    Error,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match self {
            TokenType::Name => "NAME",
            TokenType::Int => "INT",
            TokenType::Float => "FLOAT",
            TokenType::Eof => "EOF",
            TokenType::Assign => "ASSIGN",
            TokenType::Add => "ADD",
            TokenType::Sub => "SUB",
            TokenType::Mult => "MULT",
            TokenType::Div => "DIV",
            TokenType::Exp => "EXP",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::Comma => "COMMA",
            TokenType::Less => "LESS",
            TokenType::Greater => "GREATER",
            TokenType::Equal => "EQUAL",
            TokenType::NotEqual => "NOT_EQUAL",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Calc => "CALC",
            TokenType::Sim => "SIM",
            TokenType::Der => "DER",
            TokenType::Integral => "INTEGRAL",
            TokenType::Const => "CONST",
            TokenType::ResFun => "RESERVED_FUNCTION",
            TokenType::Error => "ERR",
            TokenType::LeftBracket => "LEFT_BRACKET",
            TokenType::RightBracket => "RIGHT_BRACKET",
        };
        write!(f, "{}", display)
    }
}

pub(crate) fn to_token_name(symbol: &str) -> TokenType {
    match symbol {
        "=" => TokenType::Assign,
        "+" => TokenType::Add,
        "-" => TokenType::Sub,
        "*" => TokenType::Mult,
        "/" => TokenType::Div,
        "^" => TokenType::Exp,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "," => TokenType::Comma,
        "<" => TokenType::Less,
        ">" => TokenType::Greater,
        "calc" => TokenType::Calc,
        "sim" => TokenType::Sim,
        "der" => TokenType::Der,
        "int" => TokenType::Integral,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,
        _ => TokenType::Error,
    }
}

//TODO: specify error codes

pub(crate) fn precedence(operator: TokenType) -> u32 {
    match operator {
        TokenType::Name => 0,
        TokenType::Int => 0,
        TokenType::Float => 0,
        TokenType::Const => 0,
        TokenType::Error => 0,
        TokenType::Eof => 0,

        TokenType::Comma => 1,

        TokenType::Assign => 2,

        TokenType::Equal => 3,
        TokenType::NotEqual => 3,

        TokenType::Less => 4,
        TokenType::Greater => 4,
        TokenType::LessEqual => 4,
        TokenType::GreaterEqual => 4,

        TokenType::Add => 5,
        TokenType::Sub => 5,

        TokenType::Mult => 6,
        TokenType::Div => 6,

        TokenType::Exp => 7,

        TokenType::Calc => 8,
        TokenType::Sim => 8,
        TokenType::Der => 8,
        TokenType::Integral => 8,

        TokenType::LeftParen => 9,
        TokenType::RightParen => 9,
        TokenType::LeftBracket => 9,
        TokenType::RightBracket => 9,

        TokenType::ResFun => 10,
    }
}
