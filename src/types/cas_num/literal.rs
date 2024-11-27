//used for parsing string literals to CASNums

//parses numerical literals
//0.02, -1,304,774, 1.264e-13

use std::iter::{Enumerate, Peekable};

use crate::types::{
    cas_error::{CASError, CASErrorKind},
    cas_num::Sign,
};

use super::CASNum;

pub(super) fn parse_lit(
    first_char: char,
    iter: &mut Peekable<Enumerate<std::str::Chars>>,
    line_pos: &mut usize,
) -> Option<Result<CASNum, CASError>>
//if number returns Some(Ok(CASNum), if not returns None, if error returns Some(Err)
{
    //we don't match unary minus or plus

    let mut num: &CASNum = &CASNum::from(0);

    let mut int: String = if first_char.is_ascii_digit() {
        first_char.to_string()
    } else {
        "".to_string()
    }; //digits before decimal pt
    let mut frac: String = String::from(""); //digits after decimal pt
    let mut exp: String = String::from(""); //digits after e
    let mut lit: String = first_char.to_string(); //literal of full number to return on error
    let sign: Sign;

    let mut in_frac: bool = false; //if true and exp is false, we are in the fractional part
    let mut in_exp: bool = false; //if true we are after e in the exponent

    if first_char == '-' {
        sign = Sign::Neg;
    } else if first_char == '.' {
        in_frac = true;
        sign = Sign::Pos;
    } else if first_char == '+' || first_char.is_ascii_digit() {
        sign = Sign::Pos;
    } else {
        return None; //not number
    }

    while let Some(&(char_pos, chr)) = iter.peek() {
        iter.next();
        lit.push(chr);
        *line_pos = char_pos;

        match chr {
            chr if chr.is_ascii_digit() => {
                if in_exp {
                    exp.push(chr);
                } else if in_frac {
                    frac.push(chr);
                } else {
                    int.push(chr);
                }
            }
            '.' => {
                if in_frac {
                    return Some(Err(CASError {
                        line_pos: *line_pos,
                        kind: CASErrorKind::MalformedNumericLiteral { lit },
                    }));
                } else {
                    in_frac = true;
                }
            }
            'e' => {
                if in_exp {
                    return Some(Err(CASError {
                        line_pos: *line_pos,
                        kind: CASErrorKind::MalformedNumericLiteral { lit },
                    }));
                } else {
                    in_exp = true;
                    match iter.peek() {
                        Some(&(_, '-')) => {
                            exp.push('-');
                            iter.next();
                        }
                        Some(&(_, '+')) => {
                            iter.next();
                        }
                        None => break,
                        _ => {}
                    }
                }
            }
            '-' => {
                return Some(Err(CASError {
                    line_pos: *line_pos,
                    kind: CASErrorKind::MalformedNumericLiteral { lit },
                }));
            }
            ',' | '_' => {
                //separators
                continue;
            }

            _ => break,
        }
    }

    if in_exp && exp.is_empty() {
        return Some(Err(CASError {
            line_pos: *line_pos,
            kind: CASErrorKind::MalformedNumericLiteral { lit },
        }));
        //we just randomly have an e at the end of our number
    }

    let mut chunks = vec![];
    let mut current = int.as_str();
    while current.len() >= 19 {
        let (rest, chunk) = current.split_at(current.len() - 19);
        chunks.push(chunk);
        current = rest;
    }
    chunks.push(current);

    //splits int into chunks of 19 digits or less, with remainder at front

    let mut power = &CASNum::from(1);
    let factor = &CASNum::from(10000000000000000000_u64);

    for chunk in chunks.iter() {
        let chunk_val = chunk.parse::<u64>();
        if chunk_val.is_err() {
            return Some(Err(CASError {
                line_pos: *line_pos,
                kind: CASErrorKind::MalformedNumericLiteral { lit },
            }));
        }

        num += &(CASNum::from(chunk_val.unwrap()) * power);
        power *= factor;
    }
    //if it has 19 base 10 digits or less we can ensure it is less than the max u64 size
    //we parse each chunk as a u64, convert it to a CASNum, multiply it by a power of 10, and add it to an accumulator

    todo!();
}

// fn parse_number(
//     next_char: char,
//     iter: &mut Peekable<Enumerate<str::Chars>>,
//     line_pos: &mut usize,
// ) -> Option<Result<Token, CASError>> {
//     //parses numerical literals like 3.4, 1234, -1523

//     //minus 1 since peek is following char

//     if next_char.is_numeric() || next_char == '.' {
//         let token_type = match get_next_number(next_char, iter, line_pos) {
//             //check if its a float, int, or something that cant be either
//             Ok(Token {
//                 token_type: Float(float),
//                 ..
//             }) => Float(float),
//             Ok(Token {
//                 token_type: Int(int),
//                 ..
//             }) => Int(int),

//             Err(lit) => {
//                 return Some(Err(CASError {
//                     line_pos: *line_pos,
//                     kind: CASErrorKind::MalformedNumericLiteral { lit },
//                 }))
//             }
//             _ => {
//                 return Some(Err(CASError {
//                     line_pos: *line_pos,
//                     kind: CASErrorKind::SyntaxError,
//                 }))
//             }
//         };
//         return Some(Ok(Token {
//             token_type,
//             line_pos: *line_pos,
//         }));
//     }
//     None
// }

// fn get_next_number(
//     chr: char,
//     iter: &mut Peekable<Enumerate<str::Chars>>,
//     line_pos: &mut usize,
// ) -> Result<Token, String> {
//     let mut num: String = chr.to_string();

//     while let Some(&(_, chr)) = iter.peek() {
//         if !chr.is_numeric() && chr != '.' {
//             break;
//         }
//         num.push(chr);
//         iter.next();
//         *line_pos += 1;
//     }
//     let int_parse = num.parse::<i64>();
//     if let Ok(int) = int_parse {
//         return Ok(Token {
//             token_type: Int(int.into()),
//             line_pos: *line_pos,
//         });
//     }
//     let float_parse = num.parse::<f64>();
//     match float_parse {
//         Ok(float) => {
//             return Ok(Token {
//                 token_type: Float(float),
//                 line_pos: *line_pos,
//             })
//         }
//         Err(_) => return Err(num),
//     }
// }
