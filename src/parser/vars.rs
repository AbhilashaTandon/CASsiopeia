use std::{collections::HashMap, iter::zip};

use crate::spec::types::{cas_error::CASErrorKind, cas_num::CASNum, symbol::Symbol};

use super::trees::{Tree, TreeNodeRef};

type Expression<'a> = Tree<Symbol<'a>>;

#[derive(PartialEq, Debug)]
pub struct Var<'a> {
    expr: Expression<'a>,
    pub args: Box<[&'a str]>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
pub type VarTable<'a> = HashMap<&'a str, Var<'a>>;

impl<'a> Var<'a> {
    pub fn apply<'b>(self, arg_vals: Box<[CASNum]>) -> Result<Expression<'b>, CASErrorKind>
    where
        'a: 'b,
    {
        if arg_vals.len() != self.args.len() {
            return Err(CASErrorKind::WrongNumberOfArgs);
        }

        if self.expr.root.is_none() {
            return Err(CASErrorKind::UndefinedFunction);
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

fn apply<'a>(expr: &mut TreeNodeRef<Symbol>, args: HashMap<&'a str, CASNum>) {
    if expr.children.len() == 0 {
        if let Symbol::Variable { name } = expr.data {
            if let Some(value) = args.get(name) {
                expr.data = Symbol::Num {
                    value: value.clone(),
                };
            }
        }
    } else {
        for child in &mut expr.children {
            apply(child, args.clone());
        }
    }
}