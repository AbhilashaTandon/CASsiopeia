#[cfg(test)]

mod test {

    use super::super::{tokenize, Tokenization};

    use crate::types::cas_error::{CASError, CASErrorKind};

    use crate::types::symbol::operator::Operator::*;

    use crate::types::token::Token::*;

    fn run_test(line_of_code: &str, desired_output: Tokenization) {
        let computed_tokens: Tokenization = tokenize(line_of_code);

        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn variable_declarations() {
        run_test(
            "x = 2",
            Ok(vec![Name("x".to_string()), Operator(Assign), Int(2)]),
        );
    }

    #[test]
    fn function_declarations() {
        run_test(
            "f(x,y) = 2 * x + 3 * y",
            Ok(vec![
                Name("f".to_string()),
                Operator(LeftParen),
                Name("x".to_string()),
                Operator(Comma),
                Name("y".to_string()),
                Operator(RightParen),
                Operator(Assign),
                Int(2),
                Operator(Mult),
                Name("x".to_string()),
                Operator(Add),
                Int(3),
                Operator(Mult),
                Name("y".to_string()),
            ]),
        );
    }

    #[test]
    fn keywords() {
        run_test(
            "calc 3 * x - 5",
            Ok(vec![
                Calc,
                Int(3),
                Operator(Mult),
                Name("x".to_string()),
                Operator(Sub),
                Int(5),
            ]),
        );

        run_test(
            "sim 3 * x - 5",
            Ok(vec![
                Sim,
                Int(3),
                Operator(Mult),
                Name("x".to_string()),
                Operator(Sub),
                Int(5),
            ]),
        );

        run_test(
            "der 3 * x - 5, x",
            Ok(vec![
                Der,
                Int(3),
                Operator(Mult),
                Name("x".to_string()),
                Operator(Sub),
                Int(5),
                Operator(Comma),
                Name("x".to_string()),
            ]),
        );
    }

    #[test]
    fn dashes() {
        run_test(
            "x-y_z = -5 + 3 - 2 - -4",
            Ok(vec![
                Name("x-y_z".to_string()),
                Operator(Assign),
                Operator(Sub),
                Int(5),
                Operator(Add),
                Int(3),
                Operator(Sub),
                Int(2),
                Operator(Sub),
                Operator(Sub),
                Int(4),
            ]),
        );
    }

    #[test]
    fn invalid_names() {
        //variable names can't begin w underscore
        // let mut tokens: Vec<Token> = );
        // process_line("_x = 2", &mut tokens, 0);

        run_test(
            "_x = 2",
            Err(vec![CASError {
                line_pos: 1,
                kind: CASErrorKind::MalformedVariableName {
                    name: "_x".to_string(),
                },
            }]),
        );

        run_test(
            "-x = 2",
            Ok(vec![
                Operator(Sub),
                Name("x".to_string()),
                Operator(Assign),
                Int(2),
            ]),
        );
    }

    #[test]
    fn floats() {
        run_test(
            "x = 3.3343",
            Ok(vec![Name("x".to_string()), Operator(Assign), Float(3.3343)]),
        );

        run_test(
            "y = -102342.",
            Ok(vec![
                Name("y".to_string()),
                Operator(Assign),
                Operator(Sub),
                Float(102342.0),
            ]),
        );

        run_test(
            "y = .102342",
            Ok(vec![
                Name("y".to_string()),
                Operator(Assign),
                Float(0.102342),
            ]),
        );

        run_test(
            "y = .10.2342",
            Err(vec![CASError {
                line_pos: 12,
                kind: CASErrorKind::MalformedNumericLiteral {
                    lit: ".10.2342".to_string(),
                },
            }]),
        );
    }

    #[test]
    fn comparison() {
        run_test(
            "x == y",
            Ok(vec![
                Name("x".to_string()),
                Operator(Equal),
                Name("y".to_string()),
            ]),
        );

        run_test(
            "x <= y",
            Ok(vec![
                Name("x".to_string()),
                Operator(LessEqual),
                Name("y".to_string()),
            ]),
        );

        run_test(
            "x != y",
            Ok(vec![
                Name("x".to_string()),
                Operator(NotEqual),
                Name("y".to_string()),
            ]),
        );

        run_test(
            "x >= y",
            Ok(vec![
                Name("x".to_string()),
                Operator(GreaterEqual),
                Name("y".to_string()),
            ]),
        );

        run_test(
            "x < y",
            Ok(vec![
                Name("x".to_string()),
                Operator(Less),
                Name("y".to_string()),
            ]),
        );

        run_test(
            "x > y",
            Ok(vec![
                Name("x".to_string()),
                Operator(Greater),
                Name("y".to_string()),
            ]),
        );
    }
}
