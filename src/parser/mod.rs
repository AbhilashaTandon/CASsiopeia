use expression::{into_postfix, shunting_yard};
use trees::Tree;
use vars::VarTable;

use crate::types::{cas_error::CASError, cas_num::CASNum, symbol::Symbol};

mod expression;
mod test;
pub(crate) mod trees;
pub(crate) mod vars;

//here we're mixing semantic parsing with syntactic parsing
//this makes it easier for functions since we can ensure they're given the right number of arguments

pub(crate) fn parse_expr<'a>(
    symbols: Vec<Symbol>,
    var_table: &'a VarTable<'a>,
    args: Vec<String>,
) -> Result<Tree<Symbol>, Vec<CASError>> {
    shunting_yard(&mut into_postfix(&symbols, var_table, args)?)
}
