use std::fmt::Display;

use crate::types::CASNum;
use function::Func;
use operator::Operator;

use constant::Const;

pub(crate) mod constant;
pub(crate) mod function;
pub(crate) mod operator;

#[derive(Debug, PartialEq, Clone)]

pub(crate) enum SymbolType<'a> {
    //type of tokens of output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function(Func<'a>),
    Num { value: CASNum },
    Const(Const<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Symbol<'a> {
    pub(crate) symbol_type: SymbolType<'a>,
    pub(crate) line_pos: usize,
}
//since the variable table is a hash map we can store variables and functions with their names and still have constant lookups

impl SymbolType<'_> {
    pub(crate) fn num_args(&self) -> usize {
        match self {
            SymbolType::Variable { .. } | SymbolType::Num { .. } | SymbolType::Const { .. } => 0,
            SymbolType::Operator(Operator::Neg) => 1,
            SymbolType::Operator(..) => 2,
            SymbolType::Function(Func::Function { num_args, .. }) => *num_args,
            Self::Function(Func::ResFun(res_fun)) => res_fun.num_args(),
        }
    }
}

impl Display for SymbolType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolType::Variable { name } => write!(f, "{}", name),
            SymbolType::Operator(operator) => write!(f, "{}", operator),
            SymbolType::Function(func) => write!(f, "{}", func),
            SymbolType::Num { value } => write!(f, "{}", value),
            SymbolType::Const(constant) => write!(f, "{}", constant),
        }
    }
}

impl Symbol<'_> {
    pub(crate) fn num_args(&self) -> usize {
        self.symbol_type.num_args()
    }
}

impl Display for Symbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol_type)
    }
}
