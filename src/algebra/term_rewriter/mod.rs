// https://stackoverflow.com/a/7542438

use std::borrow::Borrow;
use std::iter::zip;

use crate::{
    parser::trees::{Tree, TreeNode},
    types::symbol::Symbol,
};

type Expression = TreeNode<Symbol>;

struct Rule {
    input: Expression,
    output: Expression,
}

/// Checks if the abstract template matches the structure of the expression
fn expr_matches(exp: &Expression, template: &Expression) -> bool {
    fn recurse(exp: &Expression, template: &Expression) -> bool {
        zip(&exp.borrow().children, &template.borrow().children).all(|(exp_child, temp_child)| {
            expr_matches(&exp_child.0.borrow_mut(), &exp_child.0.borrow_mut())
        })
    }

    return match (
        &exp.borrow().data.symbol_type,
        &template.borrow().data.symbol_type,
    ) {
        (_, crate::types::symbol::SymbolType::Variable { name: temp_name }) => true,
        //a variable can be anything
        (
            crate::types::symbol::SymbolType::Operator(exp_op),
            crate::types::symbol::SymbolType::Operator(temp_op),
        ) => exp_op == temp_op && recurse(exp, template),
        (
            crate::types::symbol::SymbolType::Num { value: exp_value },
            crate::types::symbol::SymbolType::Num { value: temp_value },
        ) => exp_value == temp_value,
        (
            crate::types::symbol::SymbolType::Const(exp_const),
            crate::types::symbol::SymbolType::Const(temp_const),
        ) => exp_const == temp_const,
        (
            crate::types::symbol::SymbolType::Function(exp_func),
            crate::types::symbol::SymbolType::Function(temp_func),
        ) => exp_func == temp_func && recurse(exp, template),
        _ => false,
    };
}

fn apply_rule(exp: Expression, rule: &Rule) -> Expression {
    todo!();
}
