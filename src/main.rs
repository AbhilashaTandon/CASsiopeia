// use std::env;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::path::Path;

use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use types::cas_num::CASNum;

mod parser;
mod scanner;
pub mod types;

fn main() {
    //cli stuff
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        writeln!(io::stderr(), "Usage: {} <filename> for compilation, or run without any arguments to start an interpreter session.", args[0]).unwrap();
        return;
    }

    let floats = vec![
        2.5325,
        0.0000019073486328125,
        -2.34844396355274555919e-22,
        1.04091361631528862002e-27,
        -1.83996007268899958108e+31,
        0.,
        902341.2532,
        0239402.2340923,
        55.592082977294921875,
        13.384548187255859375,
        36029084781772800.0,
        4.5741310728335148525e-26,
        5.35045224510513345425e-23,
        2582772973568.0,
        1.95604696469614937424e-16,
        3.942e192,
    ];

    for float in floats {
        println!("{} {:?}", float, CASNum::from(float));
    }
}

// fn run(code: String) {
//     //TODO: change this to iterate over each line of input, that way we can print the whole line out in case of an error
//     //and just add the tokens to a growing vec<token> for each line

//     let mut tokens: Vec<TokenItem> = vec![];

//     for (line_num, line) in code.lines().enumerate() {
//         process_line(line, &mut tokens, line_num);
//     }
// }
