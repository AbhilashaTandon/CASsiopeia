pub(crate) mod scanner {
    use std::iter::Peekable;
    use std::{cmp, fmt, str};

    use crate::error::CASError;
    use crate::spec;

    pub(crate) enum Value {
        Int(i64),
        Float(f64), //TODO: replace these with arbitrary precision types, either custom or in some crate
    }

    impl fmt::Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Value::Int(int) => write!(f, "{}", int),
                Value::Float(float) => write!(f, "{}", float),
            }
        }
    }

    pub(crate) enum TokenItem {
        Token {
            token_name: String,
            token_text: String,
            token_value: Option<Value>,
        },
        TokenError {
            line_num: u32,
            error_code: i32,
            error_value: String,
        },
    }

    pub struct Tokenization {
        pub tokens: Vec<TokenItem>,
        pub error_code: i32,
    }

    pub(crate) fn tokenize(file_contents: String) -> Tokenization {
        let mut err_code: i32 = 0;
        let mut char_iter: Peekable<str::Chars<'_>> = file_contents.chars().peekable();

        let mut line_num: u32 = 0;

        let mut tokens: Vec<TokenItem> = vec![];

        while let Some(_) = char_iter.peek() {
            //while not at the end of file

            let current_token = get_token(&mut char_iter, line_num);
            match current_token.0 {
                TokenItem::Token { .. } => (),
                TokenItem::TokenError { error_code, .. } => {
                    err_code = cmp::max(err_code, error_code)
                    //define error codes so that most serious error is highest
                    //we'll still show all the errors but the code will be the one for the most severe error
                }
            }
            line_num = current_token.1;
            tokens.push(current_token.0);
        }

        if let TokenItem::Token { token_name, .. } = tokens.last().unwrap() {
            if token_name != "EOF" {
                tokens.push(TokenItem::Token {
                    token_name: "EOF".to_string(),
                    token_text: String::from(""),
                    token_value: None,
                });
            }
        } else if let TokenItem::TokenError { .. } = tokens.last().unwrap() {
            tokens.push(TokenItem::Token {
                token_name: "EOF".to_string(),
                token_text: String::from(""),
                token_value: None,
            });
        }
        //add token for end of file if not already present

        return Tokenization {
            tokens,
            error_code: err_code,
        };
    }

    fn get_token(iter: &mut Peekable<str::Chars>, mut line_num: u32) -> (TokenItem, u32) {
        let mut next_char = iter.next().unwrap();
        while next_char.is_whitespace() {
            match next_char {
                '\n' => {
                    line_num += 1;
                }
                _ => (), //tab or space
            }
            if let Some(chr) = iter.next() {
                next_char = chr;
            } else {
                return (
                    TokenItem::Token {
                        token_name: "EOF".to_string(),
                        token_text: String::from(""),
                        token_value: None,
                    },
                    line_num,
                );
            }
        }

        if "<>!".contains(next_char) {
            if iter.peek() == Some(&'=') {
                iter.next();
                return (
                    TokenItem::Token {
                        token_name: to_token_name(next_char) + "_EQUAL",
                        token_text: next_char.to_string() + "=",
                        token_value: None,
                    },
                    line_num,
                );
            }
        }
        if spec::spec::OPERATORS.contains(&next_char) {
            return (
                TokenItem::Token {
                    token_name: to_token_name(next_char),
                    token_text: next_char.to_string(),
                    token_value: None,
                },
                line_num,
            );
        } else if next_char.is_alphabetic() {
            let word: String = next_char.to_string() + &get_next_word(iter);
            if spec::spec::KEYWORDS.contains(&word.as_str()) {
                return (
                    TokenItem::Token {
                        token_name: word.to_uppercase(),
                        token_text: word,
                        token_value: None,
                    },
                    line_num,
                );
            } else {
                return (
                    TokenItem::Token {
                        token_name: String::from("NAME"),
                        token_text: word,
                        token_value: None,
                    },
                    line_num,
                );
            }
        } else if next_char.is_numeric() || next_char == '.' {
            match get_next_number(next_char, iter) {
                Ok(Value::Float(float)) => {
                    return (
                        TokenItem::Token {
                            token_name: String::from("FLOAT"),
                            token_text: String::from("float"),
                            token_value: Some(Value::Float(float)),
                        },
                        line_num,
                    );
                }
                Ok(Value::Int(int)) => {
                    return (
                        TokenItem::Token {
                            token_name: String::from("INT"),
                            token_text: String::from("int"),
                            token_value: Some(Value::Int(int)),
                        },
                        line_num,
                    );
                }
                Err(failed_to_parse) => {
                    return (
                        TokenItem::TokenError {
                            line_num,
                            error_code: 1,
                            error_value: failed_to_parse,
                        },
                        line_num,
                    );
                }
            }
        }

        if iter.peek().is_none() {
            return (
                TokenItem::Token {
                    token_name: "EOF".to_string(),
                    token_text: String::from(""),
                    token_value: None,
                },
                line_num,
            );
        }
        return (
            TokenItem::Token {
                token_name: to_token_name(next_char),
                token_text: next_char.to_string(),
                token_value: None,
            },
            line_num,
        );
    }

    fn to_token_name(chr: char) -> String {
        match chr {
            '+' => String::from("ADD"),
            '-' => String::from("SUB"),
            '*' => String::from("MULT"),
            '/' => String::from("DIV"),
            '^' => String::from("EXP"),
            '(' => String::from("LEFT_PAREN"),
            ')' => String::from("RIGHT_PAREN"),
            ',' => String::from("COMMA"),
            '<' => String::from("LESS"),
            '>' => String::from("GREATER"),
            '=' => String::from("EQUAL"),
            '!' => String::from("NOT"),
            _ => String::from(chr),
        }
    }

    fn get_next_word(iter: &mut Peekable<str::Chars>) -> String {
        let mut word: String = String::from("");
        while let Some(chr) = iter.peek() {
            if !chr.is_alphabetic() {
                return word;
            }
            word.push(*chr);
            iter.next();
        }
        return word;
    }

    fn get_next_number(chr: char, iter: &mut Peekable<str::Chars>) -> Result<Value, String> {
        let mut num: String = String::from(chr);
        while let Some(chr) = iter.peek() {
            if !chr.is_numeric() && *chr != '.' {
                break;
            }
            num.push(*chr);
            iter.next();
        }
        let int_parse = num.parse::<i64>();
        if let Ok(int) = int_parse {
            return Ok(Value::Int(int));
        }
        let float_parse = num.parse::<f64>();
        match float_parse {
            Ok(float) => return Ok(Value::Float(float)),
            Err(_) => return Err(num),
        }
    }
}
