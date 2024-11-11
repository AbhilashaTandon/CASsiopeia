#[cfg(test)]
mod test {
    use core::f64;
    use std::collections::HashMap;

    use crate::{
        parser::{
            expression::shunting_yard,
            trees::{construct_node, construct_tree, Parsing, Tree, TreeNode},
            vars::{Var, VarTable},
            CASNum,
        },
        types::{
            cas_error::{print_error, CASErrorKind},
            symbol::{
                operator::Operator::{self, *},
                Symbol::{self, *},
            },
        },
    };

    use crate::types::symbol::operator::Operator::*;

    use crate::scanner::tokenize;

    fn test_parser<'a>(
        expression: &'a str,
        desired_result: Parsing,
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
            assert_eq!(shunting_yard(&tokens.unwrap(), map, vec![]), desired_result);
        }
    }

    #[test]
    fn basic_test() {
        let ast = construct_tree(
            Operator(Add),
            vec![
                Num {
                    value: CASNum::from(2),
                }
                .into(),
                Num {
                    value: CASNum::from(2),
                }
                .into(),
            ],
        );

        test_parser("2 + 2", Ok(ast), None);
    }
    #[test]
    fn no_expression() {
        let err = Err(CASErrorKind::NoExpressionGiven);

        test_parser("", err, None);
    }

    #[test]
    fn mismatched_parens() {
        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("(2 +", err, None);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("2 +)", err, None);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("())()()))", err, None);

        let err = Err(CASErrorKind::MismatchedParentheses);

        test_parser("[][][]][", err, None);
    }

    #[test]
    fn var_table() {
        let err = Err(CASErrorKind::AssignmentInExpression);

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

        let err = Err(CASErrorKind::UnknownSymbol {
            symbol: "x".to_string(),
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

        let err = Err(CASErrorKind::UnknownSymbol {
            symbol: "y".to_string(),
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
        let ast = construct_node(
            Operator(Mult),
            vec![
                Num {
                    value: CASNum::from(5),
                },
                Num {
                    value: CASNum::from(3.3),
                },
            ],
        );

        let mut branch = construct_node(
            Operator(Add),
            vec![Num {
                value: CASNum::from(2),
            }],
        );

        branch.add_child(ast);
        test_parser("2 + 5 * 3.3", Ok(Tree::from(branch)), None);

        let ast = construct_node(
            Operator(Exp),
            vec![
                Num {
                    value: CASNum::from(0.012),
                },
                Num {
                    value: CASNum::from(-23.2),
                },
            ],
        );

        let mut branch = construct_node(
            Operator(Mult),
            vec![Num {
                value: CASNum::from(230),
            }],
        );

        branch.add_child(ast);
        test_parser("230 * 0.012 ^ -23.2", Ok(Tree::from(branch)), None);
    }
}
