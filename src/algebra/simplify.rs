///Contains functionality for simplifying mathematical expressions. i.e.
/// ```
/// x + x => x * 2
/// x * 0 => 0,
/// etc.
/// ```
use crate::{
    parser::trees::{Tree, TreeNode},
    types::symbol::Symbol,
};

use crate::types::symbol::operator::Operator::*;

use crate::types::symbol::SymbolType::*;

fn simplify<'a>(expr: &'a mut Tree<Symbol<'a>>) {
    simplify_rec(&mut expr.root);
}

fn simplify_rec<'a>(node: &'a mut TreeNode<Symbol<'a>>) {
    for mut child in node.children {
        simplify_rec(&mut child);
    }

    match node.data.symbol_type {
        Operator(Add) => simplify_add(node),
        Operator(Sub) => simplify_sub(node),
        Operator(Mult) => simplify_mult(node),
        Operator(Div) => simplify_div(node),
        _ => {}
    };
}

fn simplify_add<'a>(node: &'a mut TreeNode<Symbol<'a>>) {
    let children = vec![];

    for child in &node.children {
        match &child.data.symbol_type {
            Variable { name } => todo!(),
            Operator(operator) => todo!(),
            Function(func) => todo!(),
            Num { value } => todo!(),
            Const(_) => todo!(),
        }
    }

    node.children = children;
}

fn simplify_sub(node: &mut TreeNode<Symbol<'_>>) {
    todo!();
}

fn simplify_mult(node: &mut TreeNode<Symbol<'_>>) {
    todo!();
}

fn simplify_div(node: &mut TreeNode<Symbol<'_>>) {
    todo!();
}
