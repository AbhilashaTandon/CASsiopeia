use std::fmt::Display;

use phf_macros::phf_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Const<'a> {
    ResConst(ResConst),
    Const { name: &'a str },
}

impl<'a> Display for Const<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Const::ResConst(res_const) => write!(f, "{}", res_const),
            Const::Const { name } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ResConst {
    Pi,
    Phi,
    E,
    Tau,
    I,
    C, //for indefinite integration
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
            },
        )
    }
}
