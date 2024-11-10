use std::iter::{Enumerate, Peekable};
use std::str;

use crate::spec;
use crate::spec::types::cas_error::{CASError, CASErrorKind};
use crate::spec::types::symbol::operator::Operator;
use crate::spec::types::token::{to_token_name, Token};

mod test;

pub type Tokenization = Result<Vec<Token>, Vec<CASError>>;

pub fn tokenize(line_of_code: &str) -> Tokenization {
    //splits file into tokens
    let mut char_iter: Peekable<Enumerate<str::Chars>> =
        line_of_code.chars().enumerate().peekable(); //peekable to look forward for multichar tokens

    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<CASError> = vec![];

    while let Some(_) = char_iter.peek() {
        //while not at the end of file

        let current_token: Result<Token, CASErrorKind> = get_token(&mut char_iter);
        match current_token {
            Ok(token) => tokens.push(token),
            Err(err) => {
                if let Some(&(line_pos, _)) = char_iter.peek() {
                    errors.push(CASError {
                        line_pos,
                        kind: err,
                    });
                } else {
                    errors.push(CASError {
                        line_pos: line_of_code.len(),
                        kind: err,
                    })
                }
            }
        }
    }
    //add token for end of file if not already present
    match errors.len() {
        0 => return Ok(tokens),
        _ => return Err(errors),
    }
}

fn get_token(iter: &mut Peekable<Enumerate<str::Chars>>) -> Result<Token, CASErrorKind> {
    //END OF FILE
    if iter.peek().is_none() {
        return Ok(Token::Eof);
    }

    let mut next_char = iter.next().unwrap().1;

    //what we're doing here is trying to parse these, if we succeed we return the token, if we fail it must be something else

    //WHITESPACE
    if let Some(value) = skip_over_whitespace(&mut next_char, iter) {
        //this only happens when we get to the end of the file, where we return EOF
        return Ok(value);
    }

    //VARIABLE/FUNCTION NAMES
    if let Some(value) = parse_names(next_char, iter) {
        return value;
    }

    //NUMERICAL LITERALS
    if let Some(value) = parse_number(next_char, iter) {
        return value;
    }

    //COMPARISON OPERATORS
    if let Some(value) = parse_comp_ops(next_char, iter) {
        return value;
    }

    //OTHER OPERATORS
    if let Some(value) = parse_ops(next_char) {
        return value;
    }

    return Ok(to_token_name(
        String::from(next_char).to_lowercase().as_str(),
    ));
}

fn parse_number(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Result<Token, CASErrorKind>> {
    //parses numerical literals like 3.4, 1234, -1523
    if next_char.is_numeric() || next_char == '.' {
        match get_next_number(next_char, iter) {
            //check if its a float, int, or something that cant be either
            Ok(Token::Float(float)) => {
                return Some(Ok(Token::Float(float)));
            }
            Ok(Token::Int(int)) => {
                return Some(Ok(Token::Int(int)));
            }
            _ => return Some(Err(CASErrorKind::MalformedNumericLiteral)),
        }
    }
    None
}

fn get_next_number(chr: char, iter: &mut Peekable<Enumerate<str::Chars>>) -> Result<Token, String> {
    let mut num: String = chr.to_string();
    while let Some(&(_, chr)) = iter.peek() {
        if !chr.is_numeric() && chr != '.' {
            break;
        }
        num.push(chr);
        iter.next();
    }
    let int_parse = num.parse::<i64>();
    if let Ok(int) = int_parse {
        return Ok(Token::Int(int.into()));
    }
    let float_parse = num.parse::<f64>();
    match float_parse {
        Ok(float) => return Ok(Token::Float(float)),
        Err(_) => return Err(num),
    }
}

fn skip_over_whitespace(
    next_char: &mut char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Token> {
    while next_char.is_whitespace() {
        //should never be '\n' or '\r' since we parse one line at a time
        assert!(*next_char != '\n' && *next_char != '\r');
        if let Some((_, chr)) = iter.next() {
            *next_char = chr;
        } else {
            //at end of file
            return Some(Token::Eof);
        }
    }
    None //we return none once we reach a non whitespace char
}

fn parse_comp_ops(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Result<Token, CASErrorKind>> {
    //gets comparison operators
    if "=<>!".contains(next_char) {
        if iter.peek().is_some() {
            if iter.peek().unwrap().1 == '=' {
                iter.next();
                return match next_char {
                    '<' => Some(Ok(Token::Operator(Operator::LessEqual))),
                    '>' => Some(Ok(Token::Operator(Operator::GreaterEqual))),
                    '!' => Some(Ok(Token::Operator(Operator::NotEqual))),
                    '=' => Some(Ok(Token::Operator(Operator::Equal))),
                    _ => Some(Err(CASErrorKind::InvalidCharacter)),
                };
            }
        }
    }
    None
}

fn parse_ops(next_char: char) -> Option<Result<Token, CASErrorKind>> {
    //parses operators that are one character
    if spec::OPERATORS.contains(&next_char) {
        return Some(Ok(to_token_name(
            String::from(next_char).to_lowercase().as_str(),
        )));
    }
    None
}

fn parse_names(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Result<Token, CASErrorKind>> {
    //parses variable or function names or constants (alphabetic chars)

    if next_char.is_alphabetic() {
        let word: String = next_char.to_string() + &get_next_word(iter);
        if spec::KEYWORDS.contains(&word.as_str()) {
            return Some(Ok(to_token_name(word.to_lowercase().as_str())));
        } else if spec::RESERVED_FUNCTIONS.contains(&word.as_str()) {
            return Some(Ok(Token::ResFun(word)));
        } else if spec::RESERVED_CONSTANTS.contains(&word.as_str()) {
            return Some(Ok(Token::Const(word)));
        } else {
            return Some(Ok(Token::Name(word)));
        }
    } else if next_char == '_' {
        //if variable name starts with _ or -
        return Some(Err(CASErrorKind::MalformedVariableName));
    }
    None
}

fn get_next_word(iter: &mut Peekable<Enumerate<str::Chars>>) -> String {
    let mut word: String = String::from("");
    while let Some(&(_, chr)) = iter.peek() {
        if !chr.is_alphabetic() && chr != '_' && chr != '-' && !chr.is_numeric() {
            return word;
        }
        word.push(chr);
        iter.next();
    }
    return word;
}
