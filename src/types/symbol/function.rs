use std::fmt::Display;

use phf_macros::phf_map;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub(crate) enum Func {
    ResFun(ResFun),
    Function { num_args: usize, name: String },
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::ResFun(res_fun) => write!(f, "{}", res_fun), //use resfun display
            Func::Function { name, .. } => write!(f, "{}()", name),
        }
    }
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub(crate) enum ResFun {
    Sqrt,
    Cbrt,
    Log2,
    Log10,
    Log,
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
    Calc, //calculates value of expression without arbitrary precision, uses values from variable table
    Der,  //computes derivatives, 2 args, var and expression
    //der(x, x^2) -> 2 * x
    Grad, //finds gradient, returns vector of expressions
    Div,
    Curl,
    Jacob,
    SymInt,
    DefInt,
    // "der", //derivative
    // "grad", "div", "curl", "jacob", "sym_int", //symbolic integration
    // "def_int",
}

impl ResFun {
    pub(crate) fn num_args(self) -> usize {
        match self {
            ResFun::Der => 2,    //der(x^2, x) -> 2*x
            ResFun::SymInt => 2, //sym_int(x^2, x) -> x^3/3 + C
            ResFun::DefInt => 4,
            ResFun::Log => 2,
            _ => 1,
        }
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
            ResFun::Calc => "calc ",
            ResFun::Der => "d ",
            ResFun::Grad => "∇",
            ResFun::Div => "∇ · ",
            ResFun::Curl => "∇ ×",
            ResFun::Jacob => "J ",
            ResFun::SymInt => "∫ ",
            ResFun::DefInt => "∫ ",
            ResFun::Log => "log()",
        };
        write!(f, "{}", name)
    }
    //TODO: find some way of making a double sided hashmap to lookup this stuff
}

pub(crate) static RESERVED_FUNCTIONS: phf::Map<&'static str, ResFun> = phf_map! {
    "sqrt" => ResFun::Sqrt,
    "cbrt" => ResFun::Cbrt,
    "log2" => ResFun::Log2,
    "log10" => ResFun::Log10,
    "log" => ResFun::Log,
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
    "calc" => ResFun::Calc,
    "der" => ResFun::Der,
    "grad" => ResFun::Grad,
    "div" => ResFun::Div,
    "curl" => ResFun::Curl,
    "jacob" => ResFun::Jacob,
    "sym_int" => ResFun::SymInt,
    "def_int" => ResFun::DefInt,
};
