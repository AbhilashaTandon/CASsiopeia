use std::fmt;

use crate::spec::types::cas_num::CASNum;

pub mod types;

pub const KEYWORDS: [&'static str; 4] = [
    //for calculating the value of an expression, uses variable values from symbol table
    "calc", //calculates value of expression, gives arbitrary precision fp
    "sim",
    //simplifies expression, ignores values of variables and results of functions
    "der",
    // acts on expressions, finds derivative wrt to all inputs
    // if 1 input just a function, if multiple inputs returns gradient vector
    "int",
    // integrates, indefinite
];

pub const RESERVED_FUNCTIONS: [&'static str; 17] = [
    "sqrt", "cbrt", "log2", "log10", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "asin",
    "acos", "atan", "acsc", "asec", "acot",
];

pub const RESERVED_CONSTANTS: [&'static str; 5] = ["pi", "e", "phi", "tau", "i"];
pub const OPERATORS: [char; 13] = [
    '+', '-', '*', '/', '^', '(', ')', ',', '<', '=', '>', '[', ']',
];
