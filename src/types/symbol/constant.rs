use std::fmt::Display;

use phf_macros::phf_map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Const {
    ResConst(ResConst),
    Const { name: String },
}

impl Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::ResConst(res_const) => write!(f, "{}", res_const),
            Const::Const { name } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ResConst {
    Pi,
    Phi,
    E,
    Tau,
    I,
    C, //for indefinite integration
    Inf,
    NegInf,
}

pub(crate) const RESERVED_CONSTANTS: phf::Map<&'static str, ResConst> = phf_map! {
    "pi" => ResConst::Pi,
    "e" => ResConst::E,
    "phi" => ResConst::Phi,
    "tau" => ResConst::Tau,
    "i" => ResConst::I,
    "π" => ResConst::Pi,
    "ϕ" => ResConst::Phi,
    "τ" => ResConst::Tau,
    "C" => ResConst::C,
    "∞" => ResConst::Inf,
    "-∞" => ResConst::NegInf,
};

impl Display for ResConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResConst::Pi => "π",
                ResConst::Phi => "ϕ",
                ResConst::E => "e",
                ResConst::Tau => "τ",
                ResConst::I => "i",
                ResConst::C => "C",
                ResConst::Inf => "∞",
                ResConst::NegInf => "-∞",
            },
        )
    }
}
