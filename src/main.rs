// use std::env;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::path::Path;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
    iter::zip,
    path::Path,
};

use parser::vars::Var;
use parser::*;
use scanner::tokenize;
use trees::Tree;
use types::{cas_error::print_error, cas_num::CASNum, symbol::SymbolType};

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

    let expressions = vec![
        "3 * (-x) ^ 2 + 5 * x + 7",
        "1 / 238 * sin(x^x^x)",
        "2 - 2 - 2 - 2",
        "-3.12351 * 9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999 - sin(54943248923042 * pi)",

        //TODO: write custom int and float literal parsers, because the rust native ones wont work for arbitrary precision
    ];
    let var_tables: Vec<Option<HashMap<&str, Var<'_>>>> = vec![
        Some(HashMap::from([(
            "x",
            Var {
                expr: Tree::from(SymbolType::Num {
                    value: CASNum::from(2),
                }),

                args: Box::new([]),
            },
        )])),
        Some(HashMap::from([(
            "x",
            Var {
                expr: Tree::from(SymbolType::Num {
                    value: CASNum::from(2),
                }),

                args: Box::new([]),
            },
        )])),
        None,
        None,
    ];

    for (expr, var_table) in zip(&expressions, var_tables) {
        let scan = tokenize(&expr);

        if let Ok(tokens) = scan {
            let hash_table = match var_table {
                Some(x) => x,
                None => HashMap::new(),
            };

            let parse = parse(&tokens, &hash_table, vec![]);

            if let Ok(tree) = parse {
                println!("{}", tree);
            } else {
                print_error(parse.unwrap_err(), &expr, 1);
            }
        }
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
