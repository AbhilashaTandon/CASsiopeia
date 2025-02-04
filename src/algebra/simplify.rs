use std::{cell::RefCell, collections::HashMap, ops::Mul, rc::Rc};

use num_traits::Zero;

///Contains functionality for simplifying mathematical expressions. i.e.
/// ```
/// x + x => x * 2
/// x * 0 => 0,
/// etc.
/// ```
use crate::{
    parser::trees::{Tree, TreeNode, TreeNodeRef},
    types::{
        cas_num::{CASNum, ZERO},
        symbol::{Symbol, SymbolType},
    },
};

use crate::types::symbol::operator::Operator;
use crate::types::symbol::SymbolType::*;

impl Tree<Symbol> {
    fn simplify(&mut self) {
        TreeNode::<Symbol>::simplify(&mut self.root.0.borrow_mut());
    }
}
impl TreeNode<Symbol> {
    fn simplify(&mut self) {
        for child in &self.children {
            Self::simplify(&mut child.0.borrow_mut());
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
    let mut args = vec![];

    let mut arg_counts: HashMap<TreeNodeRef<Symbol>, u32> = HashMap::new();
    for arg in &node.children {
        let TreeNode {
            data: Symbol { symbol_type, .. },
            children,
        } = &arg.0.borrow().to_owned();

        if *symbol_type == (Num { value: ZERO }) {
            continue;
        }
        //adding 0 does nothing

        match symbol_type {
            SymbolType::Operator(Operator::Add) => {
                args.extend(children.clone());
                for child in children {
                    *arg_counts.entry(child.clone()).or_insert(0) += 1;
                    //adds 1 to entry if it exists, and creates it if not
                }
            } //reduce
            //a + (b + c) = a + b + c
            _ => {
                arg_counts.entry(arg.clone());
            }
        }
    }

    for (item, count) in arg_counts {
        if count == 1 {
            args.push(item);
        } else {
            //if the same expr occurs multiple times, replace it w multiplication
            //a + a + a -> 3 * a
            let coeff = TreeNode::from(Symbol {
                symbol_type: Num {
                    value: CASNum::from(count),
                },
                line_pos: item.0.borrow().data.line_pos,
            });
            let mult = TreeNodeRef::new(Symbol {
                symbol_type: Operator(Operator::Mult),
                line_pos: item.0.borrow().data.line_pos,
            });
            mult.0.borrow_mut().children.push(item);

            mult.0.borrow_mut().add_children(vec![coeff]);
            args.push(mult);
        }
    }

    node.children = args;
}

fn simplify_sub(node: &mut TreeNode<Symbol>) {}

fn simplify_mult(node: &mut TreeNode<Symbol>) {
    let mut args = vec![];

    let mut arg_counts: HashMap<TreeNodeRef<Symbol>, u32> = HashMap::new();
    for arg in &node.children {
        let TreeNode {
            data: Symbol { symbol_type, .. },
            children,
        } = &arg.0.borrow().to_owned();

        if *symbol_type
            == (Num {
                value: CASNum::from(1),
            })
        {
            continue;
        }
        //multiplying by 1 does nothing
        else if *symbol_type == (Num { value: ZERO }) {
            args = vec![TreeNodeRef::new(Symbol {
                symbol_type: Num { value: ZERO },
                line_pos: arg.0.borrow().data.line_pos,
            })];
            break;
        }

        match symbol_type {
            SymbolType::Operator(Operator::Mult) => {
                args.extend(children.clone());
                for child in children {
                    *arg_counts.entry(child.clone()).or_insert(0) += 1;
                    //adds 1 to entry if it exists, and creates it if not
                }
            } //reduce
            //a * (b * c) = a * b * c
            _ => {
                arg_counts.entry(arg.clone());
            }
        }
    }

    if args.len() == 1 {
        //if multiplying by 0
        //this is messy fix this logic later
        node.children == args;
        return;
    }

    for (item, count) in arg_counts {
        if count == 1 {
            args.push(item);
        } else {
            //if the same expr occurs multiple times, replace it w exponentiation
            //a * a * a -> a ^ 3
            let coeff = TreeNode::from(Symbol {
                symbol_type: Num {
                    value: CASNum::from(count),
                },
                line_pos: item.0.borrow().data.line_pos,
            });
            let exp = TreeNodeRef::new(Symbol {
                symbol_type: Operator(Operator::Exp),
                line_pos: item.0.borrow().data.line_pos,
            });
            exp.0.borrow_mut().children.push(item);

            exp.0.borrow_mut().add_children(vec![coeff]);
            args.push(exp);
        }
    }

    node.children = args;
}

fn simplify_div(node: &mut TreeNode<Symbol>) {}
