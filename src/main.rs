use runtime::Runtime;

mod lexer;
mod parser;
mod runtime;

use std::env;
use std::io::{self, Write};
use std::process::exit;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn execute(source: String, storage_path: String) {
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer.tokenizer());
    let mut runtime = Runtime::new(storage_path);
    runtime.query(parser.parse());
    runtime.execute()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let storage_path = match args.get(1) {
        Some(value) => value,
        _ => {
            eprintln!("ERROR: arguments is missing\nTry: <storage_path>");
            exit(1);
        } 
    };

    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Error on flush output");
        
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("Error on read input");

        if input.eq(&".exit\n".to_string()) {
            break;
        }
    
        execute(input, storage_path.to_string()); 
    }   
}
