pub(crate) mod scanner {
    use std::iter::Peekable;
    use std::{cmp, fmt, str};

    use crate::spec;
    use crate::spec::spec::{to_token_name, TokenType};

    #[derive(PartialEq, Debug)]
    pub(crate) enum Value {
        //for numerical literals
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
        TokenError {
            error_code: i32, //TODO: change this to an error type enum, only convert to error code when we print them out at the end
            error_value: String,
        },
    }

    #[derive(PartialEq, Debug)]
    pub struct Tokenization {
        //result of tokenizing code
        pub tokens: Vec<TokenItem>,
        pub error_code: i32,
    }

    pub(crate) fn tokenize(line_of_code: String) -> Tokenization {
        //splits file into tokens
        let mut err_code: i32 = 0;
        let mut char_iter: Peekable<str::Chars<'_>> = line_of_code.chars().peekable(); //peekable to look forward for multichar tokens

        let mut tokens: Vec<TokenItem> = vec![];

        while let Some(_) = char_iter.peek() {
            //while not at the end of file

            let current_token = get_token(&mut char_iter);
            match current_token {
                TokenItem::Token { .. } => (),
                TokenItem::TokenError { error_code, .. } => {
                    err_code = cmp::max(err_code, error_code)
                    //define error codes so that most serious error is highest
                    //we'll still show all the errors but the code will be the one for the most severe error
                }
            }
            tokens.push(current_token);
        }
        //add token for end of file if not already present

        return Tokenization {
            tokens,
            error_code: err_code,
        };
    }

    fn get_token(iter: &mut Peekable<str::Chars>) -> TokenItem {
        //END OF FILE
        if iter.peek().is_none() {
            return TokenItem::Token {
                token_name: TokenType::Eof,
                // token_text: String::from(""),
                token_value: None,
            };
        }

        let mut next_char = iter.next().unwrap();

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

    fn parse_number(next_char: char, iter: &mut Peekable<str::Chars<'_>>) -> Option<TokenItem> {
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
                Ok(Value::String(string)) => {
                    return Some(TokenItem::TokenError {
                        error_code: 1,
                        error_value: string,
                    })
                }
                Err(malformed_lit) => {
                    return Some(TokenItem::TokenError {
                        error_code: 1,
                        error_value: malformed_lit,
                    })
                }
            }
        }
        None
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

    fn skip_over_whitespace(
        next_char: &mut char,
        iter: &mut Peekable<str::Chars<'_>>,
    ) -> Option<TokenItem> {
        while next_char.is_whitespace() {
            //should never be '\n' or '\r' since we parse one line at a time
            assert!(*next_char != '\n' && *next_char != '\r');
            if let Some(chr) = iter.next() {
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

    fn parse_comp_ops(next_char: char, iter: &mut Peekable<str::Chars<'_>>) -> Option<TokenItem> {
        //gets comparison operators
        if "<>!".contains(next_char) {
            if iter.peek() == Some(&'=') {
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
        None
    }

    fn parse_ops(next_char: char) -> Option<TokenItem> {
        //parses operators that are one character
        if spec::spec::OPERATORS.contains(&next_char) {
            return Some(TokenItem::Token {
                token_name: to_token_name(String::from(next_char).to_lowercase().as_str()),
                // token_text: next_char.to_string(),
                token_value: None,
            });
        }
        None
    }

    fn parse_names(next_char: char, iter: &mut Peekable<str::Chars<'_>>) -> Option<TokenItem> {
        //parses variable or function names or constants (alphabetic chars)
        if next_char.is_alphabetic() {
            let word: String = next_char.to_string() + &get_next_word(iter);
            if spec::spec::KEYWORDS.contains(&word.as_str()) {
                return Some(TokenItem::Token {
                    token_name: to_token_name(word.to_lowercase().as_str()),
                    // token_text: word,
                    token_value: None,
                });
            } else if spec::spec::RESERVED_FUNCTIONS.contains(&word.as_str()) {
                return Some(TokenItem::Token {
                    token_name: TokenType::ResFun,
                    // token_text: word,
                    token_value: None,
                });
            } else if spec::spec::RESERVED_CONSTANTS.contains(&word.as_str()) {
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
        }
        None
    }

    fn get_next_word(iter: &mut Peekable<str::Chars>) -> String {
        let mut word: String = String::from("");
        while let Some(chr) = iter.peek() {
            if !chr.is_alphabetic() && *chr != '_' && *chr != '-' {
                return word;
            }
            word.push(*chr);
            iter.next();
        }
        return word;
    }
}
