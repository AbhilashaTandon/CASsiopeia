pub enum Constant<'a> {
    ResConst(ResConst),
    Const { name: &'a str },
}

pub enum ResConst {
    Pi,
    Phi,
    E,
    Tau,
    I,
}

pub const RESERVED_CONSTANTS: [&'static str; 5] = ["pi", "e", "phi", "tau", "i"];
