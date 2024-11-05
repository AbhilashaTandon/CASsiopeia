
use std::string::ToString;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum CASErrorKind {
    NoError,
    SyntaxError,
    TypeError,
    MalformedNumericLiteral,
    MalformedVariableName,
}

impl ToString for CASErrorKind {
    fn to_string(&self) -> String {
        return String::from(match self {
            CASErrorKind::NoError => "No Error",
            CASErrorKind::SyntaxError
            | CASErrorKind::MalformedNumericLiteral
            | CASErrorKind::MalformedVariableName => "Syntax Error",
            CASErrorKind::TypeError => "Type Error",
        });
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct CASError {
    pub(crate) line_pos: usize,
    pub(crate) kind: CASErrorKind,
}

fn get_message(err_kind: CASErrorKind) -> String {
    return String::from(match err_kind {
            CASErrorKind::NoError => "nothing to see here!",
            CASErrorKind::SyntaxError => "unspecified syntax error.",
            CASErrorKind::TypeError => "unspecified type error.",
            CASErrorKind::MalformedNumericLiteral => "malformed numerical literal.",
            CASErrorKind::MalformedVariableName => "variable names must begin with an alphabetic character, and must only contain alphanumeric characters, _, or -.",
        });
}

pub(crate) fn print_error(err: CASError, line: &str, line_num: usize) {
    eprintln!("{} on line {}.", err.kind.to_string(), line_num + 1);
    //we number lines starting w 1 instead of 0
    eprintln!("{}", line);
    eprintln!("{:>width$}", "^", width = err.line_pos);
    eprintln!("{}", get_message(err.kind));
}
