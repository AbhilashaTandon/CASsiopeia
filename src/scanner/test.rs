#[cfg(test)]

mod test {

    use super::super::{tokenize, TokenItem, Tokenization};

    use crate::{
        spec::TokenType::{self, *},
        types::error::{CASError, CASErrorKind},
    };

    use crate::spec::Operator;
    use crate::spec::Operator::*;

    fn make_token(token_name: TokenType) -> TokenItem {
        return TokenItem::Token(token_name);
    }

    fn run_test(line_of_code: &str, desired_tokens: Vec<TokenItem>, desired_errors: Vec<CASError>) {
        let computed_tokens: Tokenization = tokenize(line_of_code);

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
                make_token(Name("x".to_string())),
                make_token(Assign),
                make_token(Int(2)),
            ],
            vec![],
        );
    }

    #[test]
    fn function_declarations() {
        run_test(
            "f(x,y) = 2 * x + 3 * y",
            vec![
                make_token(Name("f".to_string())),
                make_token(Operator(LeftParen)),
                make_token(Name("x".to_string())),
                make_token(Comma),
                make_token(Name("y".to_string())),
                make_token(Operator(RightParen)),
                make_token(Assign),
                make_token(Int(2)),
                make_token(Operator(Mult)),
                make_token(Name("x".to_string())),
                make_token(Operator(Add)),
                make_token(Int(3)),
                make_token(Operator(Mult)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );
    }

    #[test]
    fn keywords() {
        run_test(
            "calc 3 * x - 5",
            vec![
                make_token(Calc),
                make_token(Int(3)),
                make_token(Operator(Mult)),
                make_token(Name("x".to_string())),
                make_token(Operator(Sub)),
                make_token(Int(5)),
            ],
            vec![],
        );

        run_test(
            "sim 3 * x - 5",
            vec![
                make_token(Sim),
                make_token(Int(3)),
                make_token(Operator(Mult)),
                make_token(Name("x".to_string())),
                make_token(Operator(Sub)),
                make_token(Int(5)),
            ],
            vec![],
        );

        run_test(
            "der 3 * x - 5, x",
            vec![
                make_token(Der),
                make_token(Int(3)),
                make_token(Operator(Mult)),
                make_token(Name("x".to_string())),
                make_token(Operator(Sub)),
                make_token(Int(5)),
                make_token(Comma),
                make_token(Name("x".to_string())),
            ],
            vec![],
        );
    }

    #[test]
    fn dashes() {
        run_test(
            "x-y_z = -5 + 3 - 2 - -4",
            vec![
                make_token(Name("x-y_z".to_string())),
                make_token(Assign),
                make_token(Operator(Sub)),
                make_token(Int(5)),
                make_token(Operator(Add)),
                make_token(Int(3)),
                make_token(Operator(Sub)),
                make_token(Int(2)),
                make_token(Operator(Sub)),
                make_token(Operator(Sub)),
                make_token(Int(4)),
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
                make_token(Name("x".to_string())),
                make_token(Assign),
                make_token(Int(2)),
            ],
            vec![CASError {
                line_pos: 1,
                kind: CASErrorKind::MalformedVariableName,
            }],
        );

        run_test(
            "-x = 2",
            vec![
                make_token(Operator(Sub)),
                make_token(Name("x".to_string())),
                make_token(Assign),
                make_token(Int(2)),
            ],
            vec![],
        );
    }

    #[test]
    fn floats() {
        run_test(
            "x = 3.3343",
            vec![
                make_token(Name("x".to_string())),
                make_token(Assign),
                make_token(Float(3.3343)),
            ],
            vec![],
        );

        run_test(
            "y = -102342.",
            vec![
                make_token(Name("y".to_string())),
                make_token(Assign),
                make_token(Operator(Sub)),
                make_token(Float(102342.0)),
            ],
            vec![],
        );

        run_test(
            "y = .102342",
            vec![
                make_token(Name("y".to_string())),
                make_token(Assign),
                make_token(Float(0.102342)),
            ],
            vec![],
        );

        run_test(
            "y = .10.2342",
            vec![make_token(Name("y".to_string())), make_token(Assign)],
            vec![CASError {
                line_pos: 12,
                kind: CASErrorKind::MalformedNumericLiteral,
            }],
        );
    }

    #[test]
    fn comparison() {
        run_test(
            "x == y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(Equal)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );

        run_test(
            "x <= y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(LessEqual)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );

        run_test(
            "x != y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(NotEqual)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );

        run_test(
            "x >= y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(GreaterEqual)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );

        run_test(
            "x < y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(Less)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );

        run_test(
            "x > y",
            vec![
                make_token(Name("x".to_string())),
                make_token(Operator(Greater)),
                make_token(Name("y".to_string())),
            ],
            vec![],
        );
    }
}
