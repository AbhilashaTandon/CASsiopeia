pub mod spec {
    pub(crate) const KEYWORDS: [&'static str; 6] = [
        "var", // for declaring a variable,
        "fun", //for declaring a function
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

    const RESERVED_FUNCTIONS: [&'static str; 17] = [
        "sqrt", "cbrt", "log2", "log10", "ln", "sin", "cos", "tan", "csc", "sec", "cot", "asin",
        "acos", "atan", "acsc", "asec", "acot",
    ];

    const RESERVED_CONSTANTS: [&'static str; 4] = ["pi", "e", "phi", "tau"];
    pub const OPERATORS: [char; 11] = ['+', '-', '*', '/', '^', '(', ')', ',', '<', '=', '>'];
    pub const COMP: [&'static str; 3] = ["!=", "<=", ">="];
}
