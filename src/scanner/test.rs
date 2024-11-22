#[cfg(test)]

mod test {

    use super::super::{tokenize, Tokenization};

    use crate::types::cas_error::{CASError, CASErrorKind};

    use crate::types::cas_num::CASNum;
    use crate::types::symbol::function::ResFun;
    use crate::types::symbol::operator::Operator::*;
    use crate::types::token::Token;
    use crate::types::token::TokenType::{self, *};

    fn run_test(
        line_of_code: &str,
        desired_output: Result<Vec<(TokenType, usize)>, Vec<CASError>>,
    ) {
        let computed_tokens: Tokenization = tokenize(line_of_code);
        match desired_output {
            Err(err) => assert_eq!(computed_tokens, Err(err)),
            Ok(tokens) => {
                let desired_tokens: Tokenization = Ok(tokens
                    .iter()
                    .map(|(token_type, line_pos)| make_token(token_type.clone(), *line_pos))
                    .collect());
                assert_eq!(computed_tokens, desired_tokens);
            }
        }
    }

    fn make_token(token_type: TokenType, line_pos: usize) -> Token {
        Token {
            token_type,
            line_pos,
        }
    }

    #[test]
    fn variable_declarations() {
        run_test(
            "x = 2",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(Assign), 2),
                (Num(CASNum::from(2)), 4),
            ]),
        );
    }

    #[test]
    fn function_declarations() {
        run_test(
            "f(x,y) = 2 * x + 3 * y",
            Ok(vec![
                (Name("f".to_string()), 0),
                (Operator(LeftParen), 1),
                (Name("x".to_string()), 2),
                (Operator(Comma), 3),
                (Name("y".to_string()), 4),
                (Operator(RightParen), 5),
                (Operator(Assign), 7),
                (Num(CASNum::from(2)), 9),
                (Operator(Mult), 11),
                (Name("x".to_string()), 13),
                (Operator(Add), 15),
                (Num(CASNum::from(3)), 17),
                (Operator(Mult), 19),
                (Name("y".to_string()), 21),
            ]),
        );
    }

    #[test]
    fn keywords() {
        run_test(
            "calc 3 * x - 5",
            Ok(vec![
                (ResFun(ResFun::Calc), 3),
                (Num(CASNum::from(3)), 5),
                (Operator(Mult), 7),
                (Name("x".to_string()), 9),
                (Operator(Sub), 11),
                (Num(CASNum::from(5)), 13),
            ]),
        );

        run_test(
            "der 3 * x - 5, x",
            Ok(vec![
                (ResFun(ResFun::Der), 2),
                (Num(CASNum::from(3)), 4),
                (Operator(Mult), 6),
                (Name("x".to_string()), 8),
                (Operator(Sub), 10),
                (Num(CASNum::from(5)), 12),
                (Operator(Comma), 13),
                (Name("x".to_string()), 15),
            ]),
        );
    }

    #[test]
    fn dashes() {
        run_test(
            "x-y_z = -5 + 3 - 2 - -4",
            Ok(vec![
                (Name("x-y_z".to_string()), 4),
                (Operator(Assign), 6),
                (Operator(Sub), 8),
                (Num(CASNum::from(5)), 9),
                (Operator(Add), 11),
                (Num(CASNum::from(3)), 13),
                (Operator(Sub), 15),
                (Num(CASNum::from(2)), 17),
                (Operator(Sub), 19),
                (Operator(Sub), 21),
                (Num(CASNum::from(4)), 22),
            ]),
        );
    }

    #[test]
    fn invalid_names() {
        run_test(
            "-x = 2",
            Ok(vec![
                (Operator(Sub), 0),
                (Name("x".to_string()), 1),
                (Operator(Assign), 3),
                (Num(CASNum::from(2)), 5),
            ]),
        );
    }

    #[test]
    fn floats() {
        run_test(
            "y = -102342.",
            Ok(vec![
                (Name("y".to_string()), 0),
                (Operator(Assign), 2),
                (Operator(Sub), 4),
                (Num(CASNum::from(102342.0)), 11),
            ]),
        );

        run_test(
            "x = 3.3343",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(Assign), 2),
                (Num(CASNum::from(3.3343)), 9),
            ]),
        );

        run_test(
            "y = .102342",
            Ok(vec![
                (Name("y".to_string()), 0),
                (Operator(Assign), 2),
                (Num(CASNum::from(0.102342)), 10),
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
                (Name("x".to_string()), 0),
                (Operator(Equal), 3),
                (Name("y".to_string()), 5),
            ]),
        );

        run_test(
            "x <= y",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(LessEqual), 3),
                (Name("y".to_string()), 5),
            ]),
        );

        run_test(
            "x != y ",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(NotEqual), 3),
                (Name("y".to_string()), 5),
            ]),
        );

        run_test(
            "x >= y",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(GreaterEqual), 3),
                (Name("y".to_string()), 5),
            ]),
        );

        run_test(
            "x < y",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(Less), 2),
                (Name("y".to_string()), 4),
            ]),
        );

        run_test(
            "x > y",
            Ok(vec![
                (Name("x".to_string()), 0),
                (Operator(Greater), 2),
                (Name("y".to_string()), 4),
            ]),
        );
    }
}
