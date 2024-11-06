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

use scanner::{process_line, TokenItem};
use types::cas_num::{align, CASNum, Sign};

pub mod parser;
pub mod scanner;
pub mod spec;
pub mod types;

use crate::types::cas_num;

fn main() {
    //cli stuff
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        writeln!(io::stderr(), "Usage: {} <filename> for compilation, or run without any arguments to start an interpreter session.", args[0]).unwrap();
        return;
    }

    // if args.len() == 1 {
    //     prompt();
    // }

    let mut tokens: Vec<TokenItem> = vec![];
    process_line("y = .10.2342", &mut tokens, 0);

    for (idx, arg) in args.iter().enumerate() {
        println!("arg #{}: {}", idx, arg);
    }

    let path: &Path = Path::new(&args[1]);
    let display: std::path::Display<'_> = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Error: couldn't open {}. {}", display, why),
        Ok(f) => f,
    };

    let mut s: String = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("Error: couldn't read {}. {}", display, why),
        Ok(_) => run(s),
    };
}

fn run(code: String) {
    //TODO: change this to iterate over each line of input, that way we can print the whole line out in case of an error
    //and just add the tokens to a growing vec<token> for each line

    let mut tokens: Vec<TokenItem> = vec![];

    for (line_num, line) in code.lines().enumerate() {
        process_line(line, &mut tokens, line_num);
    }
}
