use std::fmt::Display;

use crate::types::CASNum;
use function::Func;
use operator::Operator;

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
            Symbol::Operator(..) => 2, //this is technically untrue for - because it can also be used for negation, but we will handle that separately
            Symbol::Num { .. } => 0,
            Symbol::Const { .. } => 0,
            Symbol::Function(Func::Function { num_args, .. }) => *num_args,
            Self::Function(Func::ResFun(res_fun)) => res_fun.num_args(),
        }
    }
}

impl Display for Symbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Variable { name } => write!(f, "{}", name),
            Symbol::Operator(operator) => write!(f, "{}", operator),
            Symbol::Function(func) => write!(f, "{}", func),
            Symbol::Num { value } => todo!(),
            Symbol::Const(constant) => write!(f, "{}", constant),
        }
    }
}
