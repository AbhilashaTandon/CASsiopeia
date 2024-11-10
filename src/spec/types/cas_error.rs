use std::string::ToString;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
 pub(crate) enum CASErrorKind {
    NoError,
    SyntaxError,
    TypeError,
    MalformedNumericLiteral,
    MalformedVariableName,
    AssignmentInExpression,
    UnknownSymbol,
    MismatchedParentheses,
    NoExpressionGiven,
    WrongNumberOfArgs,
    UndefinedFunction,
    InvalidCharacter,
}

impl ToString for CASErrorKind {
    fn to_string(&self) -> String {
        return String::from(match self {
            CASErrorKind::NoError => "No Error",
            CASErrorKind::TypeError => "Type Error",
            CASErrorKind::SyntaxError
            | CASErrorKind::MalformedNumericLiteral
            | CASErrorKind::MalformedVariableName | CASErrorKind::AssignmentInExpression | CASErrorKind::UnknownSymbol | CASErrorKind::MismatchedParentheses | CASErrorKind::NoExpressionGiven => "Syntax Error",
            CASErrorKind::WrongNumberOfArgs | CASErrorKind::UndefinedFunction => "Runtime Error",
            CASErrorKind::InvalidCharacter => "Syntax Error",
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
    return String::from(match self {
            CASErrorKind::NoError => "nothing to see here!",
            CASErrorKind::SyntaxError => "unspecified syntax error.",
            CASErrorKind::TypeError => "unspecified type error.",
            CASErrorKind::MalformedNumericLiteral => "malformed numerical literal.",
            CASErrorKind::MalformedVariableName => "variable names must begin with an alphabetic character, and must only contain alphanumeric characters, _, or -.",
            CASErrorKind::AssignmentInExpression => "variable or function assignments cannot be made inside expressions. Perhaps you meant to use the equality operator '=='?",
            CASErrorKind::UnknownSymbol => "use of unknown variable or function.",
            CASErrorKind::MismatchedParentheses => "expression contains mismatched parentheses.",
            CASErrorKind::NoExpressionGiven => "a variable or command was given an empty expression.",
            CASErrorKind::WrongNumberOfArgs => "the wrong number of arguments were given to a function.",
            CASErrorKind::UndefinedFunction => "arguments were passed to an undefined function.",
            CASErrorKind::InvalidCharacter => "an invalid character was entered.",
            
        });
    }
}


 pub fn print_error(err: CASError, line: &str, line_num: usize) {
    eprintln!("{} on line {}.", err.kind.to_string(), line_num + 1);
    //we number lines starting w 1 instead of 0
    eprintln!("{}", line);
    eprintln!("{:>width$}", "^", width = err.line_pos);
    eprintln!("{}", err.kind.get_message());
}
