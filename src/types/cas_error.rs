use std::string::ToString;

use super::token::Token;

#[derive(Clone, PartialEq, Debug)]
 pub(crate) enum CASErrorKind {
    NoError,
    SyntaxError,
    TypeError,
    MalformedNumericLiteral{
        lit:String
    },
    MalformedVariableName{
        name:  String
    },
    AssignmentInExpression,
    UnknownSymbol{
        symbol: String
    },
    MismatchedParentheses,
    NoExpressionGiven,
    WrongNumberOfArgs{
        args_given: usize,
        args_needed: usize,
        func_name:String
    },
    InvalidCharacter{
        chr: char
    },
    CommandInExpression{
        command: Token
    },
}

impl ToString for CASErrorKind {
    fn to_string(&self) -> String {
        return String::from(match self {
            CASErrorKind::NoError => "No Error",
            CASErrorKind::TypeError => "Type Error",
            CASErrorKind::SyntaxError
            | CASErrorKind::MalformedNumericLiteral{..}
            | CASErrorKind::MalformedVariableName{..} | CASErrorKind::AssignmentInExpression | CASErrorKind::UnknownSymbol{..} | CASErrorKind::MismatchedParentheses | CASErrorKind::NoExpressionGiven | CASErrorKind::InvalidCharacter{..} | CASErrorKind::CommandInExpression { .. } => "Syntax Error",
            CASErrorKind::WrongNumberOfArgs{..} => "Runtime Error",
        });
    }
}

#[derive(PartialEq, Debug)]
 pub struct CASError {
     pub line_pos: usize,
     pub kind: CASErrorKind,
}

impl CASErrorKind{
    fn get_message(self: &Self) -> String {
    return match self {
            CASErrorKind::NoError => String::from("nothing to see here!"),
            CASErrorKind::SyntaxError => String::from("unspecified syntax error."),
            CASErrorKind::TypeError => String::from("unspecified type error."),
            CASErrorKind::MalformedNumericLiteral{
                lit
            } => format!("malformed numerical literal {}.", lit),
            CASErrorKind::MalformedVariableName{name} => format!("malformed variable name {}. variable names must begin with an alphabetic character, and must only contain alphanumeric characters, _, or -.", name),
            CASErrorKind::AssignmentInExpression => String::from("variable or function assignments cannot be made inside expressions. Perhaps you meant to use the equality operator '=='?"),
            CASErrorKind::UnknownSymbol{symbol} => format!("use of unknown variable or function {}.", symbol),
            CASErrorKind::MismatchedParentheses => String::from("expression contains mismatched parentheses."),
            CASErrorKind::NoExpressionGiven => String::from("a variable or command was given an empty expression."),
            CASErrorKind::WrongNumberOfArgs{args_given, args_needed, func_name} => format!("function {} requires {} arguments, but was given {}.", func_name, args_needed, args_given),
            CASErrorKind::InvalidCharacter{chr} => format!("an invalid character {} was entered.", chr),
            CASErrorKind::CommandInExpression { command } => format!("the {} command is not allowed within an expression.", command),
            
        };
    }
}


pub fn print_error(err: CASError, line: &str, line_num: usize) {
    eprintln!("{} on line {}.", err.kind.to_string(), line_num + 1);
    //we number lines starting w 1 instead of 0
    eprintln!("{}", line);
    eprintln!("{:>width$}", "^", width = err.line_pos);
    eprintln!("{}", err.kind.get_message());
}
