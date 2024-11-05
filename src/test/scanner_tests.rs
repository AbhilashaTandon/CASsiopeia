#[cfg(test)]
pub(crate) mod scanner_tests {
    use crate::scanner::{tokenize, TokenItem, Tokenization, Value};

    use crate::{
        error::CASError,
        spec::TokenType::{self, *},
    };

    fn make_token(
        token_name: TokenType,
        // token_text: &str,
        token_value: Option<Value>,
    ) -> TokenItem {
        return TokenItem::Token {
            token_name,
            token_value,
        };
    }

    fn run_test(line_of_code: &str, desired_tokens: Vec<TokenItem>, desired_errors: Vec<CASError>) {
        let computed_tokens: Tokenization = tokenize(String::from(line_of_code));

        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            errors: desired_errors,
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn variable_declarations() {
        run_test(
            "x = 2",
            vec![
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Equal, None),
                make_token(Int, Some(Value::Int(2))),
            ],
            vec![],
        );
    }

    #[test]
    fn function_declarations() {
        run_test(
            "f(x,y) = 2 * x + 3 * y",
            vec![
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
            ],
            vec![],
        );
    }

    #[test]
    fn keywords() {
        run_test(
            "calc 3 * x - 5",
            vec![
                make_token(Calc, None),
                make_token(Int, Some(Value::Int(3))),
                make_token(Mult, None),
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Sub, None),
                make_token(Int, Some(Value::Int(5))),
            ],
            vec![],
        );

        run_test(
            "sim 3 * x - 5",
            vec![
                make_token(Sim, None),
                make_token(Int, Some(Value::Int(3))),
                make_token(Mult, None),
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Sub, None),
                make_token(Int, Some(Value::Int(5))),
            ],
            vec![],
        );

        run_test(
            "der 3 * x - 5, x",
            vec![
                make_token(Der, None),
                make_token(Int, Some(Value::Int(3))),
                make_token(Mult, None),
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Sub, None),
                make_token(Int, Some(Value::Int(5))),
                make_token(Comma, None),
                make_token(Name, Some(Value::String(String::from("x")))),
            ],
            vec![],
        );
    }

    #[test]
    fn dashes() {
        run_test(
            "x-y_z = -5 + 3 - 2 - -4",
            vec![
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
            ],
            vec![],
        );
    }

    #[test]
    fn invalid_names() {
        //variable names can't begin w underscore
        // let mut tokens: Vec<TokenItem> = vec![];
        // process_line("_x = 2", &mut tokens, 0);

        run_test(
            "_x = 2",
            vec![
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Equal, None),
                make_token(Int, Some(Value::Int(2))),
            ],
            vec![CASError {
                line_pos: 1,
                kind: crate::error::CASErrorKind::MalformedVariableName,
            }],
        );

        run_test(
            "-x = 2",
            vec![
                make_token(Sub, None),
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Equal, None),
                make_token(Int, Some(Value::Int(2))),
            ],
            vec![],
        );
    }

    #[test]
    fn floats() {
        run_test(
            "x = 3.3343",
            vec![
                make_token(Name, Some(Value::String(String::from("x")))),
                make_token(Equal, None),
                make_token(Float, Some(Value::Float(3.3343))),
            ],
            vec![],
        );

        run_test(
            "y = -102342.",
            vec![
                make_token(Name, Some(Value::String(String::from("y")))),
                make_token(Equal, None),
                make_token(Sub, None),
                make_token(Float, Some(Value::Float(102342.0))),
            ],
            vec![],
        );

        run_test(
            "y = .102342",
            vec![
                make_token(Name, Some(Value::String(String::from("y")))),
                make_token(Equal, None),
                make_token(Float, Some(Value::Float(0.102342))),
            ],
            vec![],
        );

        run_test(
            "y = .10.2342",
            vec![
                make_token(Name, Some(Value::String(String::from("y")))),
                make_token(Equal, None),
            ],
            vec![CASError {
                line_pos: 12,
                kind: crate::error::CASErrorKind::MalformedNumericLiteral,
            }],
        );
    }
}
