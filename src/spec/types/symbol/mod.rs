use operator::Operator;

use crate::spec::CASNum;

pub mod operator;

#[derive(Debug, PartialEq, Clone)]

pub enum Symbol<'a> {
    //type of tokens of output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function { num_args: usize, name: &'a str },
    Num { value: CASNum },
    Const { name: &'a str },
}

impl Symbol<'_> {
    pub fn num_args(&self) -> usize {
        match self {
            Symbol::Variable { .. } => 0,
            Symbol::Operator(..) => 2,
            Symbol::Function { num_args, .. } => *num_args,
            Symbol::Num { .. } => 0,
            Symbol::Const { .. } => 0,
        }
    }
}
