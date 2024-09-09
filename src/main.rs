use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::io;

mod error;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() > 2{
        return;
    }

    if args.len() == 1{
        prompt();
    }

    for (idx, arg) in args.iter().enumerate(){
        println!("arg #{}: {}", idx, arg);
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("Error: couldn't open {}. {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s){
        Err(why) => panic!("Error: couldn't read {}. {}", display, why),
        Ok(_) => run(&s),
    };
}

fn run(code : &String){
    println!("{}", code);
}

pub fn trim_whitespace(s: &str) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn prompt(){
    loop{
        let mut buffer: String = String::new();
        let stdin: io::Stdin = io::stdin();
        let mut handle: io::StdinLock<'_> = stdin.lock();

        if let Err(message) = handle.read_line(&mut buffer){
            println!("Error: {message}");
        }

        let line = trim_whitespace(&buffer.as_str());
        println!("'{}'", line);
        
        let result: Result<String, error::CASError> = match line.as_str(){
            "exit" => return,
            "syntax error" => error::get_error(buffer, 0, error::CASErrorKind::SyntaxError),
            _ => Ok(buffer),
        };

        run_line(result);
        
    }
}

fn run_line(line: Result<String, error::CASError> ){
    match line{
        Ok(code) => 
    println!("> {}", code),
    Err(error) => println!("{}", error),
    };
}
