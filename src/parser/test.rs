#[cfg(test)]
mod test {
    use std::collections::{HashMap, VecDeque};

    use crate::{
        parser::{
            expression::{to_postfix, PostFix},
            trees::Tree,
            vars::{Var, VarTable},
            CASNum,
        },
        types::{
            cas_error::{print_error, CASError, CASErrorKind},
            symbol::{
                operator::Operator::*,
                Symbol::{self, *},
            },
        },
    };

    use crate::scanner::tokenize;

    fn test_parser<'a>(
        expression: &'a str,
        desired_result: PostFix,
        var_table: Option<VarTable<'a>>,
    ) {
        let map = match var_table {
            Some(map) => map,
            None => HashMap::new(),
        };
        let tokens = tokenize(expression);

        if let Err(errors) = tokens {
            for err in errors {
                print_error(err, "", 0);
            }
            assert!(false);
        } else {
            assert_eq!(to_postfix(&tokens.unwrap(), map, vec![]), desired_result);
        }
    }

    #[test]
    fn basic_test() {
        let postfix = VecDeque::from([
            Num {
                value: CASNum::from(2),
            },
            Num {
                value: CASNum::from(2),
            },
            Operator(Add),
        ]);

        test_parser("2 + 2", Ok(postfix), None);
    }
    #[test]
    fn no_expression() {
        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 0,
        });

        test_parser("", err, None);
    }

    #[test]
    fn mismatched_parens() {
        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 3,
        });

        test_parser("(2 +", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 3,
        });

        test_parser("2 +)", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 2,
        });

        test_parser("())()()))", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 6,
        });

        test_parser("[][][]][", err, None);
    }

    #[test]
    fn var_table() {
        let err = Err(CASError {
            kind: CASErrorKind::AssignmentInExpression,
            line_pos: 2,
        });

        test_parser(
            "x = 2",
            err,
            Some(HashMap::from([(
                "x",
                Var {
                    expr: Tree::from(Symbol::Num {
                        value: CASNum::from(2),
                    }),
                    args: Box::new([]),
                },
            )])),
        );

        let err = Err(CASError {
            kind: CASErrorKind::UnknownSymbol {
                symbol: "x".to_string(),
            },
            line_pos: 0,
        });

        test_parser(
            "x + 2",
            err,
            Some(HashMap::from([(
                "y",
                Var {
                    expr: Tree::from(Symbol::Num {
                        value: CASNum::from(2),
                    }),
                    args: Box::new([]),
                },
            )])),
        );

        let err = Err(CASError {
            kind: CASErrorKind::UnknownSymbol {
                symbol: "y".to_string(),
            },
            line_pos: 0,
        });

        test_parser(
            "y + 2",
            err,
            Some(HashMap::from([(
                "x",
                Var {
                    expr: Tree::from(Symbol::Num {
                        value: CASNum::from(2),
                    }),
                    args: Box::new([]),
                },
            )])),
        );
    }

    #[test]
    fn operator_precedence() {
        // let postfix = VecDeque::from([
        //     Num {
        //         value: CASNum::from(2),
        //     },
        //     Num {
        //         value: CASNum::from(5),
        //     },
        //     Num {
        //         value: CASNum::from(3.3),
        //     },
        //     Operator(Mult),
        //     Operator(Add),
        // ]);
        // test_parser("2 + 5 * 3.3", Ok(postfix), None);

        let postfix = VecDeque::from([
            Num {
                value: CASNum::from(230),
            },
            Num {
                value: CASNum::from(0.012),
            },
            Num {
                value: CASNum::from(23.2),
            },
            Operator(Sub),
            Operator(Exp),
            Operator(Mult),
        ]);
        test_parser("230 * 0.012 ^ -23.2", Ok(postfix), None);
    }
}
