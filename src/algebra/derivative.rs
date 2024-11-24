use std::{cell::RefCell, rc::Rc};

use num_traits::Zero;

///Contains functionality for finding derivatives of mathematical expressions. i.e.
/// ```
/// d/dx x^2 => x * 2
/// d/dx sin x => cos x,
/// etc.
/// ```
use crate::{
    parser::trees::{Tree, TreeNode},
    types::{
        cas_num::ZERO,
        symbol::{Symbol, SymbolType},
    },
};

use crate::types::symbol::operator::Operator;
use crate::types::symbol::SymbolType::*;

impl Tree<Symbol> {
    fn derivative(&mut self) {
        TreeNode::<Symbol>::derivative(&mut self.root.borrow_mut());
    }
}
impl TreeNode<Symbol> {
    fn derivative(&mut self) {
        for child in &self.children {
            Self::derivative(&mut child.borrow_mut());
        }

        match self.data.symbol_type {
            SymbolType::Operator(Operator::Add) => simplify_add(self),
            SymbolType::Operator(Operator::Sub) => simplify_sub(self),
            SymbolType::Operator(Operator::Mult) => simplify_mult(self),
            SymbolType::Operator(Operator::Div) => simplify_div(self),
            _ => {}
        };

        todo!();
    }
}

fn simplify_add(node: &mut TreeNode<Symbol>) {
    let mut args: Vec<Rc<RefCell<TreeNode<Symbol>>>> = vec![];

    node.children = args;
}

fn simplify_sub(node: &mut TreeNode<Symbol>) {
    todo!();
}

fn simplify_mult(node: &mut TreeNode<Symbol>) {
    todo!();
}

fn simplify_div(node: &mut TreeNode<Symbol>) {
    todo!();
}
