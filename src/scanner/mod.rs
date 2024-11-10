use std::iter::{Enumerate, Peekable};
use std::str;

use crate::types;
use crate::types::cas_error::{CASError, CASErrorKind};
use crate::types::symbol::constant::RESERVED_CONSTANTS;
use crate::types::symbol::function::RESERVED_FUNCTIONS;
use crate::types::symbol::operator::*;

use crate::types::token::Token;

mod test;

pub type Tokenization = Result<Vec<Token>, Vec<CASError>>;

pub fn tokenize(line_of_code: &str) -> Tokenization {
    //splits file into tokens
    let mut char_iter: Peekable<Enumerate<str::Chars>> =
        line_of_code.chars().enumerate().peekable(); //peekable to look forward for multichar tokens

    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<CASError> = vec![];

    while char_iter.peek().is_some() {
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

    //OTHER OPERATORS
    if let Some(value) = parse_ops(next_char, iter) {
        return value;
    }

    return Err(CASErrorKind::InvalidCharacter { chr: next_char });
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
            Err(lit) => return Some(Err(CASErrorKind::MalformedNumericLiteral { lit })),
            _ => return Some(Err(CASErrorKind::SyntaxError)),
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

fn parse_ops(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Result<Token, CASErrorKind>> {
    //parses operators that are one character
    let one_char = next_char.to_string();

    if iter.peek().is_some() {
        if iter.peek().unwrap().1 == '=' {
            match OPERATORS.get(&(next_char.to_string() + "=")) {
                Some(op) => {
                    iter.next(); //advance iterator if success
                    return Some(Ok(Token::Operator(*op)));
                }
                None => {}
            }
        }
    }
    match OPERATORS.get(&one_char) {
        Some(op) => {
            return Some(Ok(Token::Operator(*op)));
        }
        None => return None,
    };
}

fn parse_names(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
) -> Option<Result<Token, CASErrorKind>> {
    //parses variable or function names or constants (alphabetic chars)

    if next_char.is_alphabetic() || next_char == '_' {
        let word: String = next_char.to_string() + &get_next_word(iter);
        if types::KEYWORDS.contains(&word.as_str()) {
            match word.as_str() {
                "calc" => return Some(Ok(Token::Calc)),
                "sim" => return Some(Ok(Token::Sim)),
                "der" => return Some(Ok(Token::Der)),
                "int" => return Some(Ok(Token::Integral)),
                _ => {}
            }
        } else if let Some(func) = RESERVED_FUNCTIONS.get(&word) {
            return Some(Ok(Token::ResFun(*func)));
        } else if let Some(res_const) = RESERVED_CONSTANTS.get(&word) {
            return Some(Ok(Token::Const(*res_const)));
        } else {
            return Some(Ok(Token::Name(word)));
        }
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
