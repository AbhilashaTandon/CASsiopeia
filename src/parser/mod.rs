use expression::{into_postfix, shunting_yard};
use trees::Tree;
use vars::VarTable;

use crate::types::{cas_error::CASError, cas_num::CASNum, symbol::Symbol, token::Token};

mod expression;
mod test;
pub(crate) mod trees;
pub(crate) mod vars;

//here we're mixing semantic parsing with syntactic parsing
//this makes it easier for functions since we can ensure they're given the right number of arguments

pub(crate) fn into_expr<'a>(
    tokens: Vec<Token>,
    var_table: &'a VarTable<'a>,
    args: Vec<&str>,
) -> Result<Tree<Symbol<'a>>, CASError> {
    return shunting_yard(&mut into_postfix(tokens, var_table, args)?);
}
