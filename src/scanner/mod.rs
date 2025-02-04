use std::iter::{Enumerate, Peekable};
use std::str;

use crate::types::cas_error::{CASError, CASErrorKind};
use crate::types::symbol::constant::RESERVED_CONSTANTS;
use crate::types::symbol::function::RESERVED_FUNCTIONS;
use crate::types::symbol::operator::*;

use crate::types::token::TokenType::*;
use crate::types::token::{Token, TokenType};

mod test;

pub(crate) type Tokenization = Result<Vec<Token>, Vec<CASError>>;

pub(crate) fn tokenize(line_of_code: &str) -> Tokenization {
    //splits file into tokens
    let mut char_iter: Peekable<Enumerate<str::Chars>> =
        line_of_code.chars().enumerate().peekable(); //peekable to look forward for multichar tokens

    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<CASError> = vec![];

    while char_iter.peek().is_some() {
        //while not at the end of file
        let current_token: Result<Token, CASError> = get_token(&mut char_iter);
        match current_token {
            Ok(Token {
                token_type: Eof, ..
            }) => {}
            Ok(token) => tokens.push(token),
            Err(err) => {
                if char_iter.peek().is_some() {
                    errors.push(err);
                } else {
                    errors.push(CASError {
                        line_pos: line_of_code.len(),
                        kind: err.kind,
                    })
                }
            }
        }
    }
    //add token for end of file if not already present
    match errors.len() {
        0 => Ok(tokens),
        _ => Err(errors),
    }
}

fn get_token(iter: &mut Peekable<Enumerate<str::Chars>>) -> Result<Token, CASError> {
    let mut next_char;
    let mut line_pos;

    match iter.next() {
        None => {
            //END OF FILE
            return Ok(Token {
                token_type: Eof,
                line_pos: iter.count(),
            });
        }
        Some((pos, char)) => {
            line_pos = pos;
            next_char = char;
        }
    }

    //minus 1 since peek is following char

    //what we're doing here is trying to parse these, if we succeed we return the token, if we fail it must be something else

    //WHITESPACE
    if let Some(value) = skip_over_whitespace(&mut next_char, iter, &mut line_pos) {
        //this only happens when we get to the end of the file, where we return EOF

        return Ok(value);
    }

    //VARIABLE/FUNCTION NAMES
    if let Some(value) = parse_names(next_char, iter, &mut line_pos) {
        return Ok(value);
    }

    //NUMERICAL LITERALS
    if let Some(value) = parse_number(next_char, iter, &mut line_pos) {
        return value;
    }

    //OTHER OPERATORS
    if let Some(value) = parse_ops(next_char, iter, &mut line_pos) {
        return Ok(value);
    }

    Err(CASError {
        line_pos,
        kind: CASErrorKind::InvalidCharacter { chr: next_char },
    })
}

fn parse_number(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
    line_pos: &mut usize,
) -> Option<Result<Token, CASError>> {
    //parses numerical literals like 3.4, 1234, -1523

    //minus 1 since peek is following char

    if next_char.is_numeric() || next_char == '.' {
        let token_type = match get_next_number(next_char, iter, line_pos) {
            //check if its a float, int, or something that cant be either
            Ok(Token {
                token_type: Num(number),
                ..
            }) => Num(number),

            Err(lit) => {
                return Some(Err(CASError {
                    line_pos: *line_pos,
                    kind: CASErrorKind::MalformedNumericLiteral { lit },
                }))
            }
            _ => {
                return Some(Err(CASError {
                    line_pos: *line_pos,
                    kind: CASErrorKind::SyntaxError,
                }))
            }
        };
        return Some(Ok(Token {
            token_type,
            line_pos: *line_pos,
        }));
    }
    None
}

fn get_next_number(
    chr: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
    line_pos: &mut usize,
) -> Result<Token, String> {
    let mut num: String = chr.to_string();

    while let Some(&(_, chr)) = iter.peek() {
        if !chr.is_numeric() && chr != '.' {
            break;
        }
        num.push(chr);
        iter.next();
        *line_pos += 1;
    }
    let int_parse = num.parse::<i64>();
    if let Ok(int) = int_parse {
        return Ok(Token {
            token_type: Num(int.into()),
            line_pos: *line_pos,
        });
    }
    let float_parse = num.parse::<f64>();
    match float_parse {
        Ok(float) => Ok(Token {
            token_type: Num(float.into()),
            line_pos: *line_pos,
        }),
        Err(_) => Err(num),
    }
    /*
    let parse : Result<CASNum, Error> = num_lit::parse_lit(num);
    match parse {
        Ok(x) => {
            return Ok(Token {
                token_type: Num(x),
                line_pos: *line_pos,
            })
        }
        Err(_) => return Err(num),
    } */
}

fn skip_over_whitespace(
    next_char: &mut char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
    line_pos: &mut usize,
) -> Option<Token> {
    //minus 1 since peek is following char
    while next_char.is_whitespace() {
        //should never be '\n' or '\r' since we parse one line at a time
        assert!(*next_char != '\n' && *next_char != '\r');
        if let Some((_, chr)) = iter.next() {
            *line_pos += 1;
            *next_char = chr;
        } else {
            //at end of file
            return Some(Token {
                token_type: Eof,
                line_pos: iter.count(),
            });
        }
    }
    None //we return none once we reach a non whitespace char
}

fn parse_ops(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
    line_pos: &mut usize,
) -> Option<Token> {
    //parses operators that are one character
    let one_char = next_char.to_string();

    if let Some((_, '=')) = iter.peek() {
        if let Some(op) = OPERATORS.get(&(next_char.to_string() + "=")) {
            iter.next(); //advance iterator if success
            return Some(Token {
                token_type: Operator(*op),
                line_pos: *line_pos + 1,
            });
        }
    }
    OPERATORS.get(&one_char).map(|op| Token {
        token_type: Operator(*op),
        line_pos: *line_pos,
    })
}

fn parse_names(
    next_char: char,
    iter: &mut Peekable<Enumerate<str::Chars>>,
    line_pos: &mut usize,
) -> Option<Token> {
    //parses variable or function names or constants (alphabetic chars)

    if next_char.is_alphabetic() || next_char == '_' {
        let mut token_type: TokenType = Eof;
        let word: String = next_char.to_string() + &get_next_word(iter, line_pos);
        if let Some(func) = RESERVED_FUNCTIONS.get(&word) {
            token_type = ResFun(*func);
        } else if let Some(res_const) = RESERVED_CONSTANTS.get(&word) {
            token_type = Const(*res_const);
        } else {
            token_type = Name(word);
        }
        return Some(Token {
            token_type,
            line_pos: *line_pos,
        });
    }
    None
}

fn get_next_word(iter: &mut Peekable<Enumerate<str::Chars>>, line_pos: &mut usize) -> String {
    let mut word: String = String::from("");
    while let Some(&(_, chr)) = iter.peek() {
        if !chr.is_alphabetic() && chr != '_' && chr != '-' && !chr.is_numeric() {
            return word;
        }
        word.push(chr);
        iter.next();
        *line_pos += 1;
    }
    word
}
