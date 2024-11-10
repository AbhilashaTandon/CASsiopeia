use phf_macros::phf_map;

pub enum Constant<'a> {
    ResConst(ResConst),
    Const { name: &'a str },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ResConst {
    Pi,
    Phi,
    E,
    Tau,
    I,
}

pub const RESERVED_CONSTANTS: phf::Map<&'static str, ResConst> = phf_map! {
    "pi" => ResConst::Pi,
    "e" => ResConst::E,
    "phi" => ResConst::Phi,
    "tau" => ResConst::Tau,
    "i" => ResConst::I
};
