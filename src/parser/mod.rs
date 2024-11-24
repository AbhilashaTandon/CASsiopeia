use crate::types::{cas_error::CASError, cas_num::CASNum, symbol::Symbol, token::Token};
use expression::{shunting_yard, to_postfix};
use slab_tree::tree::Tree;
use vars::VarTable;

mod expression;
mod test;
pub(crate) mod vars;

//here we're mixing semantic parsing with syntactic parsing
//this makes it easier for functions since we can ensure they're given the right number of arguments

pub(crate) fn parse_expr<'a>(
    tokens: Vec<Token>,
    var_table: &'a VarTable<'a>,
    args: Vec<String>,
) -> Result<Tree<Symbol>, CASError> {
    shunting_yard(&mut to_postfix(tokens, var_table, args)?)
}
