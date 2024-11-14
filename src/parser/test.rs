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
            symbol::Symbol,
            symbol::{
                operator::Operator::*,
                SymbolType::{self, *},
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
        } else if let Ok(tokens) = tokens {
            for token in &tokens {
                print!("{} ", token);
            }
            println!();
            let post_fix = to_postfix(&tokens, map, vec![]);

            assert_eq!(post_fix, desired_result);
        }
    }

    fn make_symbol<'a>(symbol_type: SymbolType<'a>, line_pos: usize) -> Symbol<'a> {
        return Symbol {
            symbol_type,
            line_pos,
        };
    }

    #[test]
    fn basic_test() {
        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                0,
            ),
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (Operator(Add), 0),
        ]);

        let postfix = symbols
            .iter()
            .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
            .collect();

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
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 3,
        });

        test_parser("(2 +", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 3,
        });

        test_parser("2 +)", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 2,
        });

        test_parser("())()()))", err, None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
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
                    expr: Tree::from(SymbolType::Num {
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
                    expr: Tree::from(SymbolType::Num {
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
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),
                    args: Box::new([]),
                },
            )])),
        );
    }

    #[test]
    fn operator_precedence() {
        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                0,
            ),
            (
                Num {
                    value: CASNum::from(3),
                },
                2,
            ),
            (
                Num {
                    value: CASNum::from(5.05),
                },
                0,
            ),
            (Operator(Mult), 0),
            (Operator(Add), 0),
        ]);

        let postfix = symbols
            .iter()
            .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
            .collect();

        test_parser("2 + 3 * 5.05", Ok(postfix), None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(230),
                },
                0,
            ),
            (
                Num {
                    value: CASNum::from(0.012),
                },
                2,
            ),
            (
                Num {
                    value: CASNum::from(23.2),
                },
                0,
            ),
            (Operator(Neg), 0),
            (Operator(Exp), 0),
            (Operator(Mult), 0),
        ]);

        let postfix = symbols
            .iter()
            .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
            .collect();

        test_parser("230 * 0.012 ^ -23.2", Ok(postfix), None);
    }
}
