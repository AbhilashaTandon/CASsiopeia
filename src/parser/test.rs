#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        parser::{
            expression::shunting_yard,
            trees::{construct_tree, Parsing},
            CASNum,
        },
        types::{
            cas_error::{print_error, CASErrorKind},
            symbol::Symbol::*,
        },
    };

    use crate::types::symbol::operator::Operator::*;

    use crate::scanner::tokenize;

    fn test_parser(expression: &str, desired_result: Parsing) {
        let tokens = tokenize(expression);

        if let Err(errors) = tokens {
            for err in errors {
                print_error(err, "", 0);
            }
            assert!(false);
        } else {
            assert_eq!(
                shunting_yard(&tokens.unwrap(), HashMap::new(), vec![]),
                desired_result
            );
        }
    }

    #[test]
    fn basic_tests() {
        let ast = construct_tree(
            Operator(Add),
            vec![
                Num {
                    value: CASNum::from(2),
                },
                Num {
                    value: CASNum::from(2),
                },
            ],
        );

        test_parser("2 + 2", Ok(ast));
    }

    #[test]
    fn errors() {
        let err = Err(CASErrorKind::NoExpressionGiven);

        test_parser("", err);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("(2 +", err);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("2 +)", err);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("())()()))", err);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("[][][]][", err);

        let err = Err(CASErrorKind::AssignmentInExpression);

        test_parser("x = 2", err);
    }
}
