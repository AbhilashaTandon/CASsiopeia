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

use crate::types::symbol::SymbolType::*;

fn simplify(expr: &mut Tree<Symbol<'_>>) {
    // if let Some(*root_node) = expr.root {
    // simplify_rec(&mut *root_node);
    // }
}

fn simplify_rec<'a>(node: &mut TreeNode<Symbol<'a>>) -> TreeNode<Symbol<'a>> {
    // node.children = node
    //     .children
    //     .iter()
    //     .map(|child| Box::new(simplify_rec(**child)))
    //     .collect();

    // node = match node.data.symbol_type {
    //     Operator(Add) => simplify_add(node),
    //     Operator(Sub) => simplify_sub(node),
    //     Operator(Mult) => simplify_mult(node),
    //     Operator(Div) => simplify_div(node),
    //     _ => node,
    // };

    // return node;

    todo!();
}

fn simplify_add(mut node: TreeNode<Symbol<'_>>) -> TreeNode<Symbol<'_>> {
    let children = vec![];

    for child in node.children {
        match child.data.symbol_type {
            Variable { name } => todo!(),
            Operator(operator) => todo!(),
            Function(func) => todo!(),
            Num { value } => todo!(),
            Const(_) => todo!(),
        }
    }

    node.children = children;
    node
}

fn simplify_sub(node: TreeNode<Symbol<'_>>) -> TreeNode<Symbol<'_>> {
    todo!();
}

fn simplify_mult(node: TreeNode<Symbol<'_>>) -> TreeNode<Symbol<'_>> {
    todo!();
}

fn simplify_div(node: TreeNode<Symbol<'_>>) -> TreeNode<Symbol<'_>> {
    todo!();
}
