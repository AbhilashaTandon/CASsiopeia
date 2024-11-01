#[cfg(test)]
pub(crate) mod test {
    use scanner::{tokenize, TokenItem, Tokenization, Value};

    use crate::{scanner::*, spec::spec::TokenType, spec::spec::TokenType::*};

    fn make_token(
        token_name: TokenType,
        // token_text: &str,
        token_value: Option<Value>,
    ) -> TokenItem {
        return TokenItem::Token {
            token_name,
            // token_text: String::from(token_text),
            token_value,
        };
    }

    #[test]
    fn variable_declarations() {
        let computed_tokens: Tokenization = tokenize(String::from("x = 2"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Equal, None),
            make_token(Int, Some(Value::Int(2))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn function_declarations() {
        let computed_tokens: Tokenization = tokenize(String::from("f(x,y) = 2 * x + 3 * y"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("f")))),
            make_token(LeftParen, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Comma, None),
            make_token(Name, Some(Value::String(String::from("y")))),
            make_token(RightParen, None),
            make_token(Equal, None),
            make_token(Int, Some(Value::Int(2))),
            make_token(Mult, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Add, None),
            make_token(Int, Some(Value::Int(3))),
            make_token(Mult, None),
            make_token(Name, Some(Value::String(String::from("y")))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn keywords() {
        let computed_tokens: Tokenization = tokenize(String::from("calc 3 * x - 5"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Calc, None),
            make_token(Int, Some(Value::Int(3))),
            make_token(Mult, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(5))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);

        let computed_tokens: Tokenization = tokenize(String::from("sim 3 * x - 5"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Sim, None),
            make_token(Int, Some(Value::Int(3))),
            make_token(Mult, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(5))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);

        let computed_tokens: Tokenization = tokenize(String::from("der 3 * x - 5, x"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Calc, None),
            make_token(Int, Some(Value::Int(3))),
            make_token(Mult, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(5))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn dashes() {
        let computed_tokens: Tokenization = tokenize(String::from("x-y_z = -5 + 3 - 2 - -4"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("x-y_z")))),
            make_token(Equal, None),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(5))),
            make_token(Add, None),
            make_token(Int, Some(Value::Int(3))),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(2))),
            make_token(Sub, None),
            make_token(Sub, None),
            make_token(Int, Some(Value::Int(4))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn invalid_names() {
        //variable names can't begin w underscore
        // let computed_tokens: Tokenization = tokenize(String::from("_x = 2"));
        // let desired_tokens: Vec<TokenItem> = vec![
        //     TokenItem::TokenError {
        //         error_code: 1,
        //         error_value: String::from("_x"),
        //     },
        //     make_token(Equal, None),
        //     make_token(Int, Some(Value::Int(2))),
        // ];
        // let desired_output: Tokenization = Tokenization {
        //     tokens: desired_tokens,
        //     error_code: 1,
        // };
        // assert_eq!(computed_tokens, desired_output);

        let computed_tokens: Tokenization = tokenize(String::from("-x = 2"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Sub, None),
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Equal, None),
            make_token(Int, Some(Value::Int(2))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn floats() {
        //variable names can't begin w underscore
        let computed_tokens: Tokenization = tokenize(String::from("x = 3.3343"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("x")))),
            make_token(Equal, None),
            make_token(Float, Some(Value::Float(3.3343))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);

        let computed_tokens: Tokenization = tokenize(String::from("y = -102342."));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("y")))),
            make_token(Equal, None),
            make_token(Sub, None),
            make_token(Float, Some(Value::Float(102342.0))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);

        let computed_tokens: Tokenization = tokenize(String::from("y = .102342"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(Name, Some(Value::String(String::from("y")))),
            make_token(Equal, None),
            make_token(Float, Some(Value::Float(0.102342))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: vec![],
        };
        assert_eq!(computed_tokens, desired_output);

        // let computed_tokens: Tokenization = tokenize(String::from("y = .10.2342"));
        // let desired_tokens: Vec<TokenItem> = vec![
        //     make_token(Name, Some(Value::String(String::from("y")))),
        //     make_token(Equal, None),
        //     TokenItem::TokenError {
        //         error_code: 1,
        //         error_value: ".10.2342".to_string(),
        //     },
        // ];
        // let desired_output: Tokenization = Tokenization {
        //     tokens: desired_tokens,
        //     error_code: 1,
        // };
        // assert_eq!(computed_tokens, desired_output);
    }
}
