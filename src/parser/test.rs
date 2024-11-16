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

    fn symbols_to_postfix(symbols: VecDeque<(SymbolType, usize)>) -> VecDeque<Symbol> {
        return symbols
            .iter()
            .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
            .collect();
    }

    fn test_parser<'a>(
        expression: &'a str,
        desired_result: PostFix,
        var_table: &Option<VarTable<'a>>,
    ) {
        let map = match var_table {
            Some(map) => map,
            None => &HashMap::new(),
        };
        let tokens = tokenize(expression);

        if let Err(errors) = tokens {
            for err in errors {
                print_error(err, "", 0);
            }
            assert!(false);
        } else if let Ok(tokens) = tokens {
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
                4,
            ),
            (Operator(Add), 2),
        ]);

        test_parser("2 + 2", Ok(symbols_to_postfix(symbols)), &None);
    }
    #[test]
    fn no_expression() {
        let err = Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 0,
        });

        test_parser("", err, &None);
    }

    #[test]
    fn mismatched_parens() {
        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 0,
        });

        test_parser("(2 +", err, &None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 3,
        });

        test_parser("2 +)", err, &None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 2,
        });

        test_parser("())()()))", err, &None);

        let err = Err(CASError {
            kind: CASErrorKind::MismatchedParentheses,
            line_pos: 6,
        });

        test_parser("[][][]][", err, &None);
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
            &Some(HashMap::from([(
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
            &Some(HashMap::from([(
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
            &Some(HashMap::from([(
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
                4,
            ),
            (
                Num {
                    value: CASNum::from(5.05),
                },
                11,
            ),
            (Operator(Mult), 6),
            (Operator(Add), 2),
        ]);

        test_parser("2 + 3 * 5.05", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(230),
                },
                2,
            ),
            (
                Num {
                    value: CASNum::from(0.012),
                },
                10,
            ),
            (
                Num {
                    value: CASNum::from(23.2),
                },
                17,
            ),
            (Operator(Exp), 12),
            (Operator(Mult), 4),
        ]);

        let postfix = symbols
            .iter()
            .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
            .collect();

        test_parser("230 * 0.012 ^ 23.2", Ok(postfix), &None);

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
                5,
            ),
            (
                Num {
                    value: CASNum::from(5.05),
                },
                12,
            ),
            (Operator(Mult), 7),
            (Operator(Add), 2),
        ]);

        test_parser("2 + (3 * 5.05)", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                1,
            ),
            (
                Num {
                    value: CASNum::from(3),
                },
                5,
            ),
            (Operator(Add), 3),
            (
                Num {
                    value: CASNum::from(5.05),
                },
                13,
            ),
            (Operator(Mult), 8),
        ]);

        test_parser("(2 + 3) * 5.05", Ok(symbols_to_postfix(symbols)), &None);
    }

    #[test]
    fn minus_sign() {
        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                1,
            ),
            (Operator(Neg), 0),
        ]);

        test_parser("-2", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (Operator(Neg), 0),
        ]);

        test_parser("- 2", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (Operator(Neg), 0),
            (
                Num {
                    value: CASNum::from(3),
                },
                6,
            ),
            (Operator(Add), 4),
        ]);

        test_parser("- 2 + 3", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([(Variable { name: "x" }, 1), (Operator(Neg), 0)]);

        let var_table = Some(HashMap::from([(
            "x",
            Var {
                expr: Tree::from(SymbolType::Num {
                    value: CASNum::from(2),
                }),
                args: Box::new([]),
            },
        )]));

        test_parser("-x", Ok(symbols_to_postfix(symbols)), &var_table);

        let symbols = VecDeque::from([(Variable { name: "x" }, 2), (Operator(Neg), 0)]);

        test_parser("- x", Ok(symbols_to_postfix(symbols)), &var_table);

        let symbols = VecDeque::from([
            (Variable { name: "x" }, 2),
            (Operator(Neg), 0),
            (
                Num {
                    value: CASNum::from(3),
                },
                6,
            ),
            (Operator(Add), 4),
        ]);

        test_parser("- x + 3", Ok(symbols_to_postfix(symbols)), &var_table);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (Operator(Neg), 0),
            (
                Num {
                    value: CASNum::from(3),
                },
                7,
            ),
            (Operator(Neg), 6),
            (Operator(Add), 4),
        ]);

        test_parser("- 2 + -3", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (Operator(Neg), 0),
            (
                Num {
                    value: CASNum::from(3),
                },
                6,
            ),
            (Operator(Sub), 4),
        ]);

        test_parser("- 2 - 3", Ok(symbols_to_postfix(symbols)), &None);
    }
}
