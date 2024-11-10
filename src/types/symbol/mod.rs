use function::Func;
use operator::Operator;

use crate::types::CASNum;

use constant::Const;

pub mod constant;
pub mod function;
pub mod operator;

#[derive(Debug, PartialEq, Clone)]

pub enum Symbol<'a> {
    //type of tokens of output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function(Func),
    Num { value: CASNum },
    Const(Const<'a>),
}
//since the variable table is a hash map we can store variables and functions with their names and still have constant lookups

impl Symbol<'_> {
    pub fn num_args(&self) -> usize {
        match self {
            Symbol::Variable { .. } => 0,
            Symbol::Operator(..) => 2,
            Symbol::Num { .. } => 0,
            Symbol::Const { .. } => 0,
            Symbol::Function(Func::Function { num_args, .. }) => *num_args,
            Self::Function(Func::ResFun(res_fun)) => res_fun.num_args(),
        }
    }
}
