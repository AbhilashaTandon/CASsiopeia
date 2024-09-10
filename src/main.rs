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

    let path: &Path = Path::new(&args[1]);
    let display: std::path::Display<'_> = path.display();

    let mut file: File = match File::open(&path){
        Err(why) => panic!("Error: couldn't open {}. {}", display, why),
        Ok(file) => file, //TODO: auto format this 
    };

    let mut s: String = String::new();

    match file.read_to_string(&mut s){
        Err(why) => panic!("Error: couldn't read {}. {}", display, why),
        Ok(_) => run(&s),
    };
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

        let line: String = trim_whitespace(&buffer.as_str());
        
        run_line(line);
        
    }
}

fn run(code : &String){
    for line in code.lines(){ //TODO: split on semicolon
        run_line(String::from(line));
    }
}

fn run_line(line: String ) -> Result<String, error::CASError>{
    //add 

    let output : Result<String, error::CASError> = Ok(line); //replace this with actually running the code

    match output{
        Ok(code) => 
        return Ok(code), 
    Err(error) => return ,
    };
}
