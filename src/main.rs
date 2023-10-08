use runtime::Runtime;

mod lexer;
mod parser;
mod runtime;

use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Error on flush output");
    
    let mut input = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Error on read input");

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer.tokenizer());
    let mut runtime = Runtime::new(String::from("./tables"));

    runtime.query(parser.parse());
    runtime.execute();
}
