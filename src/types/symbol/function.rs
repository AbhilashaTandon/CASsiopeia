use phf_macros::phf_map;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Func {
    ResFun(ResFun),
    Function { num_args: usize, name: String },
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
