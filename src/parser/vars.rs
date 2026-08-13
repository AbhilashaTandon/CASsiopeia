use core::num;
use std::{collections::HashMap, iter::zip};

use crate::types::{
    cas_error::CASErrorKind,
    cas_num::CASNum,
    symbol::{Symbol, SymbolType},
};

use super::trees::{ASTNode, AST};

#[derive(PartialEq, Debug)]
pub(crate) struct Var {
    pub(crate) expr: AST,
    pub(crate) args: Vec<String>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

/// table storing predefined variables (numericals and functions)
pub(crate) type VarTable<'a> = HashMap<String, Var>;

impl<'a> Var {
    /// replaces variables in expression with values given in args
    pub(crate) fn apply<'b>(
        //only for functions
        mut self,
        func_name: String,
        arg_vals: Box<[CASNum]>,
    ) -> Result<Var, CASErrorKind>
    where
        'a: 'b,
    {
        if arg_vals.len() != self.args.len() {
            return Err(CASErrorKind::WrongNumberOfArgs {
                args_given: arg_vals.len(),
                args_needed: self.args.len(),
                func_name,
            });
        }

        if (arg_vals.len() == 0) {
            //no arguments given, so we don't do any replacing
            return Ok(self);
        }

        if (self.expr.get_length() == 0) {
            //AST has no nodes
            return Err(CASErrorKind::NoExpressionGiven);
        }

        let mut args_map: HashMap<String, CASNum> = HashMap::new();
        for (name, value) in zip(self.args.clone(), arg_vals) {
            args_map.insert(name, value);
        }

        let result = self.apply_recurse(0, &args_map);

        if let Some(err) = result {
            return Err(err);
        } else {
            return Ok(self);
        }
        //start at 0 since its the root
    }

    // this function should stay private
    fn apply_recurse(
        &mut self,
        node_idx: usize,
        args: &HashMap<String, CASNum>,
    ) -> Option<CASErrorKind> {
        let num_children = self.expr.get_num_children(node_idx);
        if let Some(node) = self.expr.get_data(node_idx) {
            if (node.num_args() != num_children) {
                return Some(CASErrorKind::WrongNumberOfArgs {
                    args_given: num_children,
                    args_needed: node.num_args(),
                    func_name: node.to_string(),
                });
            }
            match &node.symbol_type {
                SymbolType::Variable(name) => {
                    //replace variable with value from args
                    if let Some(value) = args.get(name) {
                        self.expr.set_data(
                            node_idx,
                            Symbol {
                                symbol_type: SymbolType::Num(value.clone()),
                                line_pos: node.line_pos,
                            },
                        );
                    }
                }
                SymbolType::Operator(_) | SymbolType::Function(_) => {
                    for child in self.expr.get_children(node_idx) {
                        return self.apply_recurse(*child, args);
                    }
                }
                SymbolType::Num(_) | SymbolType::Const(_) | SymbolType::EOF => {}
            };
        } else {
            panic!("out of bounds error in apply")
        }
        return None;
    }
}
