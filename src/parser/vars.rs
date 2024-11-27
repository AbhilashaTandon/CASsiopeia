use std::{collections::HashMap, iter::zip};

use crate::types::{cas_error::CASErrorKind, cas_num::CASNum, symbol::SymbolType};

use super::trees::{Tree, TreeNodeRef};

#[derive(PartialEq, Debug)]
pub(crate) struct Var {
    pub(crate) expr: Tree<SymbolType>,
    pub(crate) args: Vec<String>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
pub(crate) type VarTable<'a> = HashMap<String, Var>;

impl<'a> Var {
    pub(crate) fn apply<'b>(
        mut self,
        func_name: String,
        arg_vals: Box<[CASNum]>,
    ) -> Result<Tree<SymbolType>, CASErrorKind>
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

        let mut args_map: HashMap<String, CASNum> = HashMap::new();
        for (name, value) in zip(self.args, arg_vals) {
            args_map.insert(name, value);
        }

        apply(&mut self.expr.root, &args_map);
        Ok(Tree {
            root: self.expr.root,
        })
    }
}

fn apply(expr: &mut TreeNodeRef<SymbolType>, args: &HashMap<String, CASNum>) {
    //replaces variables in expression with values given in args
    if expr.0.borrow().children.is_empty() {
        if let SymbolType::Variable { name } = &expr.0.borrow().data {
            if let Some(value) = args.get(name) {
                expr.0.borrow_mut().data = SymbolType::Num {
                    value: value.clone(),
                    //TODO: get rid of this clone
                };
            }
        }
    } else {
        for child in &mut expr.0.borrow_mut().children {
            apply(child, args);
        }
    }
}
