use std::{collections::HashMap, iter::zip};

use crate::types::{cas_error::CASErrorKind, cas_num::CASNum, symbol::SymbolType};

use super::trees::{Tree, TreeNodeRef};

#[derive(PartialEq, Debug)]
pub(crate) struct Var<'a> {
    pub(crate) expr: Tree<SymbolType<'a>>,
    pub(crate) args: Box<[&'a str]>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
pub(crate) type VarTable<'a> = HashMap<&'a str, Var<'a>>;

impl<'a> Var<'a> {
    pub(crate) fn apply(
        mut self,
        func_name: String,
        arg_vals: Box<[CASNum]>,
    ) -> Result<Tree<SymbolType<'a>>, CASErrorKind> {
        if arg_vals.len() != self.args.len() {
            return Err(CASErrorKind::WrongNumberOfArgs {
                args_given: arg_vals.len(),
                args_needed: self.args.len(),
                func_name,
            });
        }

        let mut args_map: HashMap<&'_ str, CASNum> = HashMap::new();
        for (name, value) in zip(self.args, arg_vals) {
            args_map.insert(name, value);
        }
        apply(&mut self.expr.root, &args_map);
        Ok(self.expr)
    }
}

fn apply<'a>(expr: &mut TreeNodeRef<SymbolType<'a>>, args: &HashMap<&'a str, CASNum>) {
    if expr.borrow().children.is_empty() {
        if let SymbolType::Variable { name } = expr.borrow().data {
            if let Some(value) = args.get(name) {
                expr.borrow_mut().data = SymbolType::Num {
                    value: value.clone(),
                    //TODO: get rid of this clone
                };
            }
        }
    } else {
        for child in &mut expr.borrow_mut().children {
            apply(child, args);
        }
    }
}
