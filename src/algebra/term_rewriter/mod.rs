// https://stackoverflow.com/a/7542438

use crate::types::cas_error::CASErrorKind;
use crate::types::symbol::SymbolType;
use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;

use crate::{
    parser::{
        trees::AST,
        vars::{Var, VarTable},
    },
    types::symbol::Symbol,
};

struct Rule {
    input: AST,
    output: AST,
}

/// Checks if the abstract template matches the structure of the expression
/// Requires that arguments be in normal form (i.e. with arguments of commutative operators sorted the same way)
fn matches_expr<'a>(
    exp: &AST,
    // this should have no variables I think
    template: &'a AST,
    var_subtrees: &mut HashMap<&'a String, AST>,
) -> Result<bool, CASErrorKind> {
    let mut var_table: VarTable = HashMap::new();

    for (exp_child, temp_child) in exp.0.iter().zip(template.0.iter()) {
        match (exp_child.data.symbol_type, temp_child.data.symbol_type) {
            (SymbolType::Num(x), SymbolType::Num(y)) => {
                if (x != y) {
                    return Ok(false);
                }
            }

            (SymbolType::Operator(x), SymbolType::Operator(y)) => {
                if (x != y) {
                    return Ok(false);
                }
            }

            (SymbolType::Function(x), SymbolType::Function(y)) => {
                if (x != y) {
                    return Ok(false);
                }
            }

            (SymbolType::Variable(x), SymbolType::Variable(y)) => {}

            (SymbolType::Const(x), SymbolType::Const(y)) => {
                if (x != y) {
                    return Ok(false);
                }
            }

            (SymbolType::Num(x), SymbolType::Variable(y))
            | (SymbolType::Variable(y), SymbolType::Num(x)) => {
                if let Some(y_val) = var_table.get(&y) {
                    if (y_val != x) {
                        return Ok(false);
                    }
                }
                // if (x != y) {
                //     return Ok(false);
                // }
            }
        }

        if (exp_child.children != temp_child.children) {
            return Ok(false);
        }
    }

    return Ok(true);

    // return match (&exp.data.symbol_type, &template.data.symbol_type) {
    //     (_, crate::types::symbol::SymbolType::Variable(temp_name)) => {
    //         //check that variable matches

    //         if let Some((_, prev_exp)) = var_subtrees.get_key_value(temp_name) {
    //             return exp == prev_exp;
    //         } else {
    //             var_subtrees.insert(temp_name, exp.clone());
    //             //we probably need to clone here
    //             return true;
    //         }
    //     }

    //     //a variable can be anything
    //     (
    //         crate::types::symbol::SymbolType::Operator(exp_op),
    //         crate::types::symbol::SymbolType::Operator(temp_op),
    //     ) => exp_op == temp_op && recurse(exp, template, var_subtrees),
    //     (
    //         crate::types::symbol::SymbolType::Num(exp_value),
    //         crate::types::symbol::SymbolType::Num(temp_value),
    //     ) => exp_value == temp_value,
    //     (
    //         crate::types::symbol::SymbolType::Const(exp_const),
    //         crate::types::symbol::SymbolType::Const(temp_const),
    //     ) => exp_const == temp_const,
    //     (
    //         crate::types::symbol::SymbolType::Function(exp_func),
    //         crate::types::symbol::SymbolType::Function(temp_func),
    //     ) => exp_func == temp_func && recurse(exp, template, var_subtrees),
    //     _ => false,
    // };
}

fn apply_rule(exp: AST, rule: &Rule) -> AST {
    todo!();
}
