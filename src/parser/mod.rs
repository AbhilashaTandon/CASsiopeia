use expression::{shunting_yard, to_postfix};
use trees::Tree;
use vars::VarTable;

use crate::types::{cas_error::CASError, cas_num::CASNum, symbol::Symbol, token::Token};

mod expression;
mod test;
pub mod trees;
pub(crate) mod vars;

pub fn parse<'a>(
    tokens: &'a Vec<Token>,
    var_table: &'a VarTable<'a>,
    args: Vec<&str>,
) -> Result<Tree<Symbol<'a>>, CASError> {
    return shunting_yard(to_postfix(tokens, var_table, args)?);
}
