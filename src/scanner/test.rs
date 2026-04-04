#[cfg(test)]

mod test {

    use super::super::{tokenize, Tokenization};

    use crate::types;
    use crate::types::cas_error::{CASError, CASErrorKind};
    use crate::types::symbol::operator::Operator::{self, *};

    use crate::types::cas_num::CASNum;
    use crate::types::symbol::function::{self, *};
    use crate::types::symbol::{Symbol, SymbolType};

    fn run_test(
        line_of_code: &str,
        desired_output: Result<Vec<(SymbolType, usize)>, Vec<CASError>>,
    ) {
        let computed_symbols: Tokenization = tokenize(line_of_code);
        match desired_output {
            Err(err) => assert_eq!(computed_symbols, Err(err)),
            Ok(symbols) => {
                let desired_symbols: Tokenization = Ok(symbols
                    .iter()
                    .map(|(symbol_type, line_pos)| make_symbol(symbol_type.clone(), *line_pos))
                    .collect());
                assert_eq!(computed_symbols, desired_symbols);
            }
        }
    }

    fn make_symbol(symbol_type: SymbolType, line_pos: usize) -> Symbol {
        Symbol {
            symbol_type,
            line_pos,
        }
    }

    #[test]
    fn variable_declarations() {
        run_test(
            "x = 2",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(Assign), 2),
                (SymbolType::Num(CASNum::from(2)), 4),
            ]),
        );
    }

    #[test]
    fn function_declarations() {
        run_test(
            "f(x,y) = 2 * x + 3 * y",
            Ok(vec![
                (SymbolType::Variable("f".to_string()), 0),
                (SymbolType::Operator(LeftParen), 1),
                (SymbolType::Variable("x".to_string()), 2),
                (SymbolType::Operator(Comma), 3),
                (SymbolType::Variable("y".to_string()), 4),
                (SymbolType::Operator(RightParen), 5),
                (SymbolType::Operator(Assign), 7),
                (SymbolType::Num(CASNum::from(2)), 9),
                (SymbolType::Operator(Mult), 11),
                (SymbolType::Variable("x".to_string()), 13),
                (SymbolType::Operator(Add), 15),
                (SymbolType::Num(CASNum::from(3)), 17),
                (SymbolType::Operator(Mult), 19),
                (SymbolType::Variable("y".to_string()), 21),
            ]),
        );
    }

    #[test]
    fn keywords() {
        run_test(
            "calc 3 * x - 5",
            Ok(vec![
                (
                    types::symbol::SymbolType::Function(
                        crate::types::symbol::function::Func::ResFun(ResFun::Calc),
                    ),
                    3,
                ),
                (SymbolType::Num(CASNum::from(3)), 5),
                (SymbolType::Operator(Mult), 7),
                (SymbolType::Variable("x".to_string()), 9),
                (SymbolType::Operator(Sub), 11),
                (SymbolType::Num(CASNum::from(5)), 13),
            ]),
        );

        run_test(
            "der 3 * x - 5, x",
            Ok(vec![
                (
                    types::symbol::SymbolType::Function(types::symbol::function::Func::ResFun(
                        ResFun::Der,
                    )),
                    2,
                ),
                (SymbolType::Num(CASNum::from(3)), 4),
                (SymbolType::Operator(Mult), 6),
                (SymbolType::Variable("x".to_string()), 8),
                (SymbolType::Operator(Sub), 10),
                (SymbolType::Num(CASNum::from(5)), 12),
                (SymbolType::Operator(Comma), 13),
                (SymbolType::Variable("x".to_string()), 15),
            ]),
        );
    }

    #[test]
    fn dashes() {
        run_test(
            "x-y_z = -5 + 3 - 2 - -4",
            Ok(vec![
                (SymbolType::Variable("x-y_z".to_string()), 4),
                (SymbolType::Operator(Assign), 6),
                (SymbolType::Operator(Sub), 8),
                (SymbolType::Num(CASNum::from(5)), 9),
                (SymbolType::Operator(Add), 11),
                (SymbolType::Num(CASNum::from(3)), 13),
                (SymbolType::Operator(Sub), 15),
                (SymbolType::Num(CASNum::from(2)), 17),
                (SymbolType::Operator(Sub), 19),
                (SymbolType::Operator(Sub), 21),
                (SymbolType::Num(CASNum::from(4)), 22),
            ]),
        );
    }

    #[test]
    fn invalid_names() {
        run_test(
            "-x = 2",
            Ok(vec![
                (SymbolType::Operator(Sub), 0),
                (SymbolType::Variable("x".to_string()), 1),
                (SymbolType::Operator(Assign), 3),
                (SymbolType::Num(CASNum::from(2)), 5),
            ]),
        );
    }

    #[test]
    fn floats() {
        run_test(
            "y = -102342.",
            Ok(vec![
                (SymbolType::Variable("y".to_string()), 0),
                (SymbolType::Operator(Assign), 2),
                (SymbolType::Operator(Sub), 4),
                (SymbolType::Num(CASNum::from(102342.0)), 11),
            ]),
        );

        run_test(
            "x = 3.3343",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(Assign), 2),
                (SymbolType::Num(CASNum::from(3.3343)), 9),
            ]),
        );

        run_test(
            "y = .102342",
            Ok(vec![
                (SymbolType::Variable("y".to_string()), 0),
                (SymbolType::Operator(Assign), 2),
                (SymbolType::Num(CASNum::from(0.102342)), 10),
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
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(Equal), 3),
                (SymbolType::Variable("y".to_string()), 5),
            ]),
        );

        run_test(
            "x <= y",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(LessEqual), 3),
                (SymbolType::Variable("y".to_string()), 5),
            ]),
        );

        run_test(
            "x != y ",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(NotEqual), 3),
                (SymbolType::Variable("y".to_string()), 5),
            ]),
        );

        run_test(
            "x >= y",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(GreaterEqual), 3),
                (SymbolType::Variable("y".to_string()), 5),
            ]),
        );

        run_test(
            "x < y",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(Less), 2),
                (SymbolType::Variable("y".to_string()), 4),
            ]),
        );

        run_test(
            "x > y",
            Ok(vec![
                (SymbolType::Variable("x".to_string()), 0),
                (SymbolType::Operator(Greater), 2),
                (SymbolType::Variable("y".to_string()), 4),
            ]),
        );
    }
}
