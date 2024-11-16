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
                function::Func,
                operator::Operator::*,
                Symbol,
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
        let symbols = VecDeque::from([(
            Num {
                value: CASNum::from(-2),
            },
            1,
        )]);

        test_parser("-2", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([(
            Num {
                value: CASNum::from(-2),
            },
            2,
        )]);

        test_parser("- 2", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(-2),
                },
                2,
            ),
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
                    value: CASNum::from(-2),
                },
                2,
            ),
            (
                Num {
                    value: CASNum::from(-3),
                },
                7,
            ),
            (Operator(Add), 4),
        ]);

        test_parser("- 2 + -3", Ok(symbols_to_postfix(symbols)), &None);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(-2),
                },
                2,
            ),
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

    #[test]
    fn functions() {
        let err = Err(CASError {
            kind: CASErrorKind::UnknownSymbol {
                symbol: "f".to_owned(),
            },
            line_pos: 0,
        });

        test_parser("f(2, 3, 4)", err, &None);

        let var_table = Some(HashMap::from([(
            "f",
            Var {
                expr: Tree::from(SymbolType::Num {
                    value: CASNum::from(2),
                }),

                args: Box::new(["x", "y", "z"]),
            },
        )]));

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                2,
            ),
            (
                Num {
                    value: CASNum::from(3),
                },
                5,
            ),
            (
                Num {
                    value: CASNum::from(4),
                },
                8,
            ),
            (
                Function(Func::Function {
                    num_args: 3,
                    name: "f",
                }),
                0,
            ),
        ]);

        test_parser("f(2, 3, 4)", Ok(symbols_to_postfix(symbols)), &var_table);

        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                4,
            ),
            (
                Num {
                    value: CASNum::from(3),
                },
                7,
            ),
            (
                Num {
                    value: CASNum::from(4),
                },
                10,
            ),
            (
                Function(Func::Function {
                    num_args: 3,
                    name: "foo",
                }),
                2,
            ),
        ]);

        let var_table = Some(HashMap::from([(
            "foo",
            Var {
                expr: Tree::from(SymbolType::Num {
                    value: CASNum::from(2),
                }),

                args: Box::new(["a", "b", "c"]),
            },
        )]));

        test_parser("foo(2, 3, 4)", Ok(symbols_to_postfix(symbols)), &var_table);

        let symbols = VecDeque::from([
            (Variable { name: "x" }, 12),
            (
                Function(Func::Function {
                    num_args: 1,
                    name: "baz",
                }),
                10,
            ),
            (
                Function(Func::Function {
                    num_args: 1,
                    name: "bar",
                }),
                6,
            ),
            (Variable { name: "y" }, 17),
            (
                Function(Func::Function {
                    num_args: 2,
                    name: "foo",
                }),
                2,
            ),
        ]);

        let var_table = Some(HashMap::from([
            (
                "foo",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),

                    args: Box::new(["a", "b"]),
                },
            ),
            (
                "bar",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(1),
                    }),

                    args: Box::new(["a"]),
                },
            ),
            (
                "baz",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(1),
                    }),

                    args: Box::new(["a"]),
                },
            ),
            (
                "x",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),

                    args: Box::new([]),
                },
            ),
            (
                "y",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),

                    args: Box::new([]),
                },
            ),
        ]));

        test_parser(
            "foo(bar(baz(x)), y)",
            Ok(symbols_to_postfix(symbols)),
            &var_table,
        );

        let var_table = Some(HashMap::from([
            (
                "foo",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),

                    args: Box::new(["a", "b"]),
                },
            ),
            (
                "bar",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(1),
                    }),

                    args: Box::new(["a"]),
                },
            ),
            (
                "baz",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(1),
                    }),

                    args: Box::new(["a"]),
                },
            ),
            (
                "x",
                Var {
                    expr: Tree::from(SymbolType::Num {
                        value: CASNum::from(2),
                    }),

                    args: Box::new([]),
                },
            ),
        ]));

        let err = Err(CASError {
            kind: CASErrorKind::UnknownSymbol {
                symbol: "y".to_string(),
            },
            line_pos: 17,
        });

        test_parser("foo(bar(baz(x)), y)", err, &var_table);
    }

    #[test]
    fn stress_test() {
        let symbols = VecDeque::from([
            (
                Num {
                    value: CASNum::from(2),
                },
                0,
            ),
            (
                Num {
                    value: CASNum::from(1),
                },
                4,
            ),
            (
                Num {
                    value: CASNum::from(2),
                },
                6,
            ),
            (Operator(Add), 5),
            (Operator(Neg), 2),
            (
                Num {
                    value: CASNum::from(2),
                },
                11,
            ),
            (
                Num {
                    value: CASNum::from(5),
                },
                13,
            ),
            (
                Num {
                    value: CASNum::from(2),
                },
                17,
            ),
            (
                Num {
                    value: CASNum::from(400),
                },
                21,
            ),
            (Operator(Add), 18),
            (Operator(Neg), 15),
            (Operator(Mult), 14),
            (Operator(Add), 12),
            (Operator(Neg), 9),
            (Operator(Exp), 8),
            (Operator(Mult), 1),
        ]);

        test_parser(
            "2*-(1+2)^-(2+5*-(2+400))",
            Ok(symbols_to_postfix(symbols)),
            &None,
        );
    }
}
