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

fn prompt(){
    loop{
        print!("> ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        if let Err(message) = handle.read_line(&mut buffer){
            println!("Error: {message}");
        }

        
        let _ = match buffer.as_str(){
            "exit" => return,
            "syntax error" => error::get_error(buffer, 0, error::CASErrorKind::SyntaxError),
            _ => Ok(run_line(&buffer)),
        };
        
    }
}

fn run_line(code : &String){
    println!("> {}", code);
}
