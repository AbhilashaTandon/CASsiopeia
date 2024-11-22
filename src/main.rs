// use std::env;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::path::Path;

mod algebra;
mod parser;
mod scanner;
pub(crate) mod types;

fn main() {
    //cli stuff
    // let args: Vec<String> = env::args().collect();

    // if args.len() > 2 {
    //     writeln!(io::stderr(), "Usage: {} <filename> for compilation, or run without any arguments to start an interpreter session.", args[0]).unwrap();
    //     return;
    // }
}

// fn run(code: String) {
//     //TODO: change this to iterate over each line of input, that way we can print the whole line out in case of an error
//     //and just add the tokens to a growing vec<token> for each line

//     let mut tokens: Vec<TokenItem> = vec![];

//     for (line_num, line) in code.lines().enumerate() {
//         process_line(line, &mut tokens, line_num);
//     }
// }

// for (expr, var_table) in zip(&expressions, var_tables) {
//         let scan = tokenize(&expr);

//         if let Ok(tokens) = scan {
//             let hash_table = match var_table {
//                 Some(x) => x,
//                 None => HashMap::new(),
//             };

//             let parse = parse_expr(&tokens, &hash_table, vec![]);

//             if let Ok(tree) = parse {
//                 println!("{}", tree);
//             } else {
//                 print_error(parse.unwrap_err(), &expr, 1);
//             }
//         }
//     }
