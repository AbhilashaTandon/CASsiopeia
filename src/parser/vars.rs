use std::{collections::HashMap, iter::zip};

use crate::types::{
    cas_error::CASErrorKind,
    cas_num::CASNum,
    symbol::{Symbol, SymbolType},
};

use super::trees::{Tree, TreeNodeRef};

type Expression<'a> = Tree<Symbol<'a>>;

#[derive(PartialEq, Debug)]
pub(crate) struct Var<'a> {
    pub(crate) expr: Tree<SymbolType<'a>>,
    pub args: Box<[&'a str]>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
pub type VarTable<'a> = HashMap<&'a str, Var<'a>>;

impl<'a> Var<'a> {
    pub fn apply<'b>(
        self,
        func_name: String,
        arg_vals: Box<[CASNum]>,
    ) -> Result<Tree<SymbolType<'b>>, CASErrorKind>
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

        if self.expr.root.is_none() {
            return Err(CASErrorKind::UnknownSymbol { symbol: func_name });
        }
        let mut args_map: HashMap<&'_ str, CASNum> = HashMap::new();
        for (name, value) in zip(self.args, arg_vals) {
            args_map.insert(name, value);
        }
        let mut expression = self.expr.root.unwrap();
        apply(&mut expression, args_map);
        return Ok(Tree {
            root: Some(expression),
        });
    }
}

fn apply<'a>(expr: &mut TreeNodeRef<SymbolType<'a>>, args: HashMap<&'a str, CASNum>) {
    if expr.children.len() == 0 {
        if let SymbolType::Variable { name } = expr.data {
            if let Some(value) = args.get(name) {
                expr.data = SymbolType::Num {
                    value: value.clone(),
                    //TODO: get rid of this clone
                };
            }
        }
    } else {
        for child in &mut expr.children {
            apply(child, args.clone());
            //TODO: get rid of this clone
        }
    }
}
