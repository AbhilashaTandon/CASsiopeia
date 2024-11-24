use std::{cell::RefCell, collections::HashMap, rc::Rc};

use num_traits::Zero;

///Contains functionality for simplifying mathematical expressions. i.e.
/// ```
/// x + x => x * 2
/// x * 0 => 0,
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
    fn simplify(&mut self) {
        TreeNode::<Symbol>::simplify(&mut self.root.borrow_mut());
    }
}
impl TreeNode<Symbol> {
    fn simplify(&mut self) {
        for child in &self.children {
            Self::simplify(&mut child.borrow_mut());
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

    let mut arg_counts: HashMap<TreeNode<Symbol>, u32> = HashMap::new();
    for arg in &node.children {
        let TreeNode {
            data: Symbol { symbol_type, .. },
            children,
        } = &arg.borrow().to_owned();

        if *symbol_type == (Num { value: ZERO }) {
            continue;
        }
        //adding 0 does nothing

        match symbol_type {
            SymbolType::Operator(Operator::Add) => {
                args.extend(children.clone());
                for child in children {
                    *arg_counts.entry(child.into_inner()).or_insert(0) += 1;
                    //adds 1 to entry if it exists, and creates it if not
                }
            } //reduce
            //a + (b + c) = a + b + c
            _ => {
                arg_counts.entry(arg.into_inner());
            }
        }
    }

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
