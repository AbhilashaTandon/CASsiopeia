pub(crate) mod scanner {
    use std::iter::{Enumerate, Peekable};
    use std::{fmt, str};

    use crate::error::error::{print_error, CASError, CASErrorKind};
    use crate::spec;
    use crate::spec::{to_token_name, TokenType};

    #[derive(PartialEq, Debug)]
    pub(crate) enum Value {
        //for numerical literals and variable names
        Int(i64),
        Float(f64), //TODO: replace these with arbitrary precision types, either custom or in some crate
        String(String),
    }

    impl fmt::Display for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Value::Int(int) => write!(f, "{}", int),
                Value::Float(float) => write!(f, "{}", float),
                Value::String(string) => write!(f, "{}", string),
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub(crate) enum TokenItem {
        //stores each token or error we find in file
        Token {
            token_name: TokenType,
            // token_text: String,
            token_value: Option<Value>,
        },

        Error {
            err: CASErrorKind,
        },
    }

    #[derive(PartialEq, Debug)]
    pub struct Tokenization {
        //result of tokenizing code
        pub tokens: Vec<TokenItem>,
        pub errors: Vec<CASError>,
    }

    pub(crate) fn process_line(line: &str, tokens: &mut Vec<TokenItem>, line_num: usize) {
        let result = tokenize(line.to_string());
        if result.errors.len() == 0 {
            for token in &result.tokens {
                match token {
                    TokenItem::Token {
                        token_name,
                        // token_text,
                        token_value,
                    } => println!(
                        // "{} {} {}",
                        "{} {}",
                        token_name.to_string(),
                        // token_text,
                        match token_value {
                            Some(value) => value.to_string(),
                            None => String::from("None"),
                        }
                    ),
                    _ => (),
                }
            }
            tokens.extend(result.tokens);
        } else {
            //if theres any error print it out
            for error in result.errors {
                print_error(error, line, line_num);
            }
        }
    }

    pub(crate) fn tokenize(line_of_code: String) -> Tokenization {
        //splits file into tokens
        let mut char_iter: Peekable<Enumerate<str::Chars>> =
            line_of_code.chars().enumerate().peekable(); //peekable to look forward for multichar tokens

        let mut tokens: Vec<TokenItem> = vec![];
        let mut errors: Vec<CASError> = vec![];

        while let Some(_) = char_iter.peek() {
            //while not at the end of file

            let current_token: TokenItem = get_token(&mut char_iter);
            match current_token {
                TokenItem::Token { .. } => tokens.push(current_token),
                TokenItem::Error { err } => {
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

        return Tokenization { tokens, errors };
    }

    fn get_token(iter: &mut Peekable<Enumerate<str::Chars>>) -> TokenItem {
        //END OF FILE
        if iter.peek().is_none() {
            return TokenItem::Token {
                token_name: TokenType::Eof,
                // token_text: String::from(""),
                token_value: None,
            };
        }

        let mut next_char = iter.next().unwrap().1;

        //what we're doing here is trying to parse these, if we succeed we return the token, if we fail it must be something else

        //WHITESPACE
        if let Some(value) = skip_over_whitespace(&mut next_char, iter) {
            //this only happens when we get to the end of the file, where we return EOF
            return value;
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

        return TokenItem::Token {
            token_name: to_token_name(String::from(next_char).to_lowercase().as_str()),
            // token_text: next_char.to_string(),
            token_value: None,
        };
    }

    fn parse_number(
        next_char: char,
        iter: &mut Peekable<Enumerate<str::Chars>>,
    ) -> Option<TokenItem> {
        //parses numerical literals like 3.4, 1234, -1523
        if next_char.is_numeric() || next_char == '.' {
            match get_next_number(next_char, iter) {
                //check if its a float, int, or something that cant be either
                Ok(Value::Float(float)) => {
                    return Some(TokenItem::Token {
                        token_name: TokenType::Float,
                        // token_text: String::from("float"),
                        token_value: Some(Value::Float(float)),
                    });
                }
                Ok(Value::Int(int)) => {
                    return Some(TokenItem::Token {
                        token_name: TokenType::Int,
                        // token_text: String::from("int"),
                        token_value: Some(Value::Int(int)),
                    });
                }
                Ok(Value::String(_)) => {
                    return Some(TokenItem::Error {
                        err: CASErrorKind::MalformedNumericLiteral,
                    })
                }
                Err(_) => {
                    return Some(TokenItem::Error {
                        err: CASErrorKind::MalformedNumericLiteral,
                    })
                }
            }
        }
        None
    }

    fn get_next_number(
        chr: char,
        iter: &mut Peekable<Enumerate<str::Chars>>,
    ) -> Result<Value, String> {
        let mut num: String = String::from(chr);
        while let Some(&(_, chr)) = iter.peek() {
            if !chr.is_numeric() && chr != '.' {
                break;
            }
            num.push(chr);
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

    fn skip_over_whitespace(
        next_char: &mut char,
        iter: &mut Peekable<Enumerate<str::Chars>>,
    ) -> Option<TokenItem> {
        while next_char.is_whitespace() {
            //should never be '\n' or '\r' since we parse one line at a time
            assert!(*next_char != '\n' && *next_char != '\r');
            if let Some((_, chr)) = iter.next() {
                *next_char = chr;
            } else {
                //at end of file
                return Some(TokenItem::Token {
                    token_name: TokenType::Eof,
                    // token_text: String::from(""),
                    token_value: None,
                });
            }
        }
        None //we return none once we reach a non whitespace char
    }

    fn parse_comp_ops(
        next_char: char,
        iter: &mut Peekable<Enumerate<str::Chars>>,
    ) -> Option<TokenItem> {
        //gets comparison operators
        if "<>!".contains(next_char) {
            if iter.peek().is_some() {
                if iter.peek().unwrap().1 == '=' {
                    iter.next();
                    return Some(TokenItem::Token {
                        token_name: match next_char {
                            '<' => TokenType::LessEqual,
                            '>' => TokenType::GreaterEqual,
                            '!' => TokenType::NotEqual,
                            _ => TokenType::Error,
                        },
                        // token_text: next_char.to_string() + "=",
                        token_value: None,
                    });
                }
            }
        }
        None
    }

    fn parse_ops(next_char: char) -> Option<TokenItem> {
        //parses operators that are one character
        if spec::OPERATORS.contains(&next_char) {
            return Some(TokenItem::Token {
                token_name: to_token_name(String::from(next_char).to_lowercase().as_str()),
                // token_text: next_char.to_string(),
                token_value: None,
            });
        }
        None
    }

    fn parse_names(
        next_char: char,
        iter: &mut Peekable<Enumerate<str::Chars>>,
    ) -> Option<TokenItem> {
        //parses variable or function names or constants (alphabetic chars)

        if next_char.is_alphabetic() {
            let word: String = next_char.to_string() + &get_next_word(iter);
            if spec::KEYWORDS.contains(&word.as_str()) {
                return Some(TokenItem::Token {
                    token_name: to_token_name(word.to_lowercase().as_str()),
                    // token_text: word,
                    token_value: None,
                });
            } else if spec::RESERVED_FUNCTIONS.contains(&word.as_str()) {
                return Some(TokenItem::Token {
                    token_name: TokenType::ResFun,
                    // token_text: word,
                    token_value: None,
                });
            } else if spec::RESERVED_CONSTANTS.contains(&word.as_str()) {
                return Some(TokenItem::Token {
                    token_name: TokenType::Const,
                    // token_text: word,
                    token_value: None,
                });
            } else {
                return Some(TokenItem::Token {
                    token_name: TokenType::Name,
                    // token_text: word,
                    token_value: Some(Value::String(word)),
                });
            }
        } else if next_char == '_' {
            //if variable name starts with _ or -
            return Some(TokenItem::Error {
                err: CASErrorKind::MalformedVariableName,
            });
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
}
