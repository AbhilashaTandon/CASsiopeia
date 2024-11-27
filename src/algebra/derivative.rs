use std::{borrow::Borrow, cell::RefCell, ops::Mul, rc::Rc};

use num_traits::Zero;

///Contains functionality for finding derivatives of mathematical expressions. i.e.
/// ```
/// d/dx x^2 => x * 2
/// d/dx sin x => cos x,
/// etc.
/// ```
use crate::{
    parser::{
        trees::{Tree, TreeNode, TreeNodeRef},
        vars::Var,
    },
    types::{
        cas_num::ZERO,
        symbol::{Symbol, SymbolType},
    },
};

use crate::types::symbol::operator::Operator;
use crate::types::symbol::SymbolType::*;

impl Tree<Symbol> {
    fn derivative(&mut self, wrt: &Var) {
        TreeNode::<Symbol>::derivative(&mut self.root.0.borrow_mut(), wrt);
    }
}
impl TreeNode<Symbol> {
    fn derivative(&mut self, wrt: &Var) {
        for child in &self.children {
            Self::derivative(&mut child.0.borrow_mut(), wrt);
        }

        match self.data.symbol_type {
            SymbolType::Operator(Operator::Add) => simplify_add(self, wrt),
            SymbolType::Operator(Operator::Sub) => simplify_add(self, wrt),
            //addition works the same way as subtraction
            //d/dx f + g -> d/dx f + d/dx g
            //d/dx f - g -> d/dx f - d/dx g
            SymbolType::Operator(Operator::Mult) => simplify_mult(self, wrt),
            SymbolType::Operator(Operator::Div) => simplify_div(self, wrt),
            SymbolType::Const(_) | SymbolType::Num { .. } => {
                self.data = Symbol {
                    symbol_type: Num { value: ZERO },
                    line_pos: self.data.line_pos,
                };
                //derivative of constant is 0
            }
            _ => {}
        };

        todo!();
    }
}

fn simplify_add(node: &mut TreeNode<Symbol>, wrt: &Var) {
    for child in &node.children {
        child.0.borrow_mut().derivative(wrt);
        // d /dx (f(x) + g(x)) = d/dx f(x) + d/dx g(x)
    }
}

fn simplify_mult(node: &mut TreeNode<Symbol>, wrt: &Var) {
    let derivatives: Vec<TreeNodeRef<Symbol>> = node
        .children
        .iter()
        .map(|child| {
            let der = child.clone();
            der.0.borrow_mut().derivative(wrt);
            //makes a clone and takes the derivative
            der
        })
        .collect();

    let new_children: Vec<TreeNodeRef<Symbol>> = vec![];

    for (idx, diff) in derivatives.iter().enumerate() {
        let mut new_node = TreeNode::from(Symbol {
            symbol_type: Operator(Operator::Mult),
            line_pos: node.data.line_pos,
        });
        new_node.children.push(diff.clone());
        for (idx_2, arg) in node.children.iter().enumerate() {
            if idx_2 == idx {
                continue;
            }
            new_node.children.push(arg.clone());
        }
    }

    // (f * g * h)' -> f' * (g * h) + f * (g * h)' -> f' * g * h + f * g' * h + f * g * h'
    node.children = new_children;
}

fn simplify_div(node: &mut TreeNode<Symbol>, wrt: &Var) {
    todo!();
}
