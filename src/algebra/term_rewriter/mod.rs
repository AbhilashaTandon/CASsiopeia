// https://stackoverflow.com/a/7542438

use std::collections::HashMap;
use std::iter::zip;
use std::rc::Rc;

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
fn matches_expr<'a>(
    exp: &Expression,
    template: &'a Expression,
    var_subtrees: &mut HashMap<&'a String, Expression>,
) -> bool {
    fn recurse<'a>(
        exp: &Expression,
        template: &'a Expression,
        var_subtrees: &mut HashMap<&'a String, Expression>,
    ) -> bool {
        zip(&exp.children, &template.children).all(|(exp_child, temp_child)| {
            let temp_node = &*temp_child.0;
            let matches = matches_expr(&*exp_child.0.borrow(), temp_node, var_subtrees);

            matches
        })
    }

    return match (&exp.data.symbol_type, &template.data.symbol_type) {
        (_, crate::types::symbol::SymbolType::Variable(temp_name)) => {
            //check that variable matches

            if let Some((_, prev_exp)) = var_subtrees.get_key_value(temp_name) {
                return exp == prev_exp;
            } else {
                var_subtrees.insert(temp_name, exp.clone());
                //we probably need to clone here
                return true;
            }
        }

        //a variable can be anything
        (
            crate::types::symbol::SymbolType::Operator(exp_op),
            crate::types::symbol::SymbolType::Operator(temp_op),
        ) => exp_op == temp_op && recurse(exp, template, var_subtrees),
        (
            crate::types::symbol::SymbolType::Num(exp_value),
            crate::types::symbol::SymbolType::Num(temp_value),
        ) => exp_value == temp_value,
        (
            crate::types::symbol::SymbolType::Const(exp_const),
            crate::types::symbol::SymbolType::Const(temp_const),
        ) => exp_const == temp_const,
        (
            crate::types::symbol::SymbolType::Function(exp_func),
            crate::types::symbol::SymbolType::Function(temp_func),
        ) => exp_func == temp_func && recurse(exp, template, var_subtrees),
        _ => false,
    };
}

fn apply_rule(exp: Expression, rule: &Rule) -> Expression {
    todo!();
}
