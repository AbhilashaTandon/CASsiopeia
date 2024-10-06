use std::fmt;
use std::string::ToString;

#[derive(Debug)]
pub(crate) enum CASErrorKind {
    SyntaxError,
    TypeError,
}

impl ToString for CASErrorKind {
    fn to_string(&self) -> String {
        return match self {
            CASErrorKind::SyntaxError => String::from("Syntax Error"),
            CASErrorKind::TypeError => String::from("Type Error"),
        };
    }
}

#[derive(Debug)]
pub(crate) struct CASError {
    line: String,
    line_num: u32,
    message: String,
    kind: CASErrorKind,
}

impl fmt::Display for CASError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.line_num {
            0 => write!(f, "{}: {}", self.kind.to_string(), self.message),
            _ => write!(
                f,
                "{} on line {}: {}\n\t{}",
                self.kind.to_string(),
                self.line_num,
                self.message,
                self.line
            ),
        }
    }
}

pub(crate) fn get_error(
    line: String,
    line_num: u32,
    kind: CASErrorKind,
) -> Result<String, CASError> {
    Err(CASError {
        line: line,
        line_num: line_num,
        message: kind.to_string(),
        kind: kind,
    })
}
