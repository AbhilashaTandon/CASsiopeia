use std::fmt::Display;

use phf_macros::phf_map;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Func<'a> {
    ResFun(ResFun),
    Function { num_args: usize, name: &'a str },
}

impl<'a> Display for Func<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::ResFun(res_fun) => write!(f, "{}", res_fun), //use resfun display
            Func::Function { name, .. } => write!(f, "{}()", name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResFun {
    Sqrt,
    Cbrt,
    Log2,
    Log10,
    Ln,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Asin,
    Acos,
    Atan,
    Acsc,
    Asec,
    Acot,
}

impl ResFun {
    pub fn num_args(self) -> usize {
        return 1;
    }
}

impl Display for ResFun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ResFun::Sqrt => "sqrt()",
            ResFun::Cbrt => "cbrt()",
            ResFun::Log2 => "log2()",
            ResFun::Log10 => "log10()",
            ResFun::Ln => "ln()",
            ResFun::Sin => "sin()",
            ResFun::Cos => "cos()",
            ResFun::Tan => "tan()",
            ResFun::Csc => "csc()",
            ResFun::Sec => "sec()",
            ResFun::Cot => "cot()",
            ResFun::Asin => "asin()",
            ResFun::Acos => "acos()",
            ResFun::Atan => "atan()",
            ResFun::Acsc => "acsc()",
            ResFun::Asec => "asec()",
            ResFun::Acot => "acot()",
        };
        return write!(f, "{}", name);
    }
    //TODO: find some way of making a double sided hashmap to lookup this stuff
}

pub(crate) static RESERVED_FUNCTIONS: phf::Map<&'static str, ResFun> = phf_map! {
    "sqrt" => ResFun::Sqrt,
    "cbrt" => ResFun::Cbrt,
    "log2" => ResFun::Log2,
    "log10" => ResFun::Log10,
    "ln" => ResFun::Ln,
    "sin" => ResFun::Sin,
    "cos" => ResFun::Cos,
    "tan" => ResFun::Tan,
    "csc" => ResFun::Csc,
    "sec" => ResFun::Sec,
    "cot" => ResFun::Cot,
    "asin" => ResFun::Asin,
    "acos" => ResFun::Acos,
    "atan" => ResFun::Atan,
    "acsc" => ResFun::Acsc,
    "asec" => ResFun::Asec,
    "acot" => ResFun::Acot,
};
