use std::fmt::Display;

use crate::types::CASNum;
use function::Func;
use operator::Operator;

use constant::{Const, ResConst};
use std::hash::Hash;
pub(crate) mod constant;
pub(crate) mod function;
pub(crate) mod operator;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]

pub(crate) enum SymbolType {
    //type of tokens of output of parsing
    Variable { name: String },
    Operator(Operator),
    Function(Func),
    Num { value: CASNum },
    Const(Const),
}

#[derive(Debug, Clone, Eq)]
pub(crate) struct Symbol {
    pub(crate) symbol_type: SymbolType,
    pub(crate) line_pos: usize,
}
//since the variable table is a hash map we can store variables and functions with their names and still have constant lookups
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbol_type == other.symbol_type
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol_type.hash(state);
    }
}

impl PartialOrd for SymbolType {
    /** Compares SymbolTypes, used for sorting arguments of commutative operators.
     *
     * The enum variants of SymbolType are sorted as below:
     *
     * Num
     * Const
     * Function
     * Variable
     * Operator
     *
     * Variables, functions, and constants are sorted lexicographically by their name, nums are sorted by value, and operators are sorted by precedence from lowest to highest.
     *
     * Note that in practice, when expressions are simplified arguments of multiplication are sorted in ascending order, while arguments of addition are sorted in descending order. This imitates the way polynomials are traditionally formatted:
     *
     * 3 * x ^ 2 + 2 * x + 5
     *
     * # Examples
     *
     * ```
     * ```
     */
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use crate::types::symbol::Const::ResConst;
        use crate::types::symbol::ResConst::*;
        todo!()
    }
}

impl SymbolType {
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

impl Display for SymbolType {
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

impl Symbol {
    pub(crate) fn num_args(&self) -> usize {
        self.symbol_type.num_args()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol_type)
    }
}
