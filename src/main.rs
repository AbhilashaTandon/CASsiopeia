use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub mod error;
pub mod scanner;
pub mod spec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        writeln!(io::stderr(), "Usage: {} <filename> for compilation, or run without any arguments to start an interpreter session.", args[0]).unwrap();
        return;
    }

    // if args.len() == 1 {
    //     prompt();
    // }

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
    let result = scanner::scanner::tokenize(code);
    if result.error_code == 0 {
        for token in result.tokens {
            match token {
                scanner::scanner::TokenItem::Token {
                    token_name,
                    token_text,
                    token_value,
                } => println!(
                    "{} {} {}",
                    token_name,
                    token_text,
                    match token_value {
                        Some(value) => value.to_string(),
                        None => String::from("None"),
                    }
                ),

                scanner::scanner::TokenItem::TokenError { .. } => (),
            }
        }
    } else {
        for token in result.tokens {
            match token {
                scanner::scanner::TokenItem::Token { .. } => (),

                scanner::scanner::TokenItem::TokenError {
                    line_num,
                    error_code,
                    error_value,
                } => println!(
                    "[line {}] Error code {}: {}",
                    line_num + 1,
                    error_code,
                    error_value
                ),
            }
        }
    }
}
