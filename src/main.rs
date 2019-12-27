mod tokenizer;
mod parser;
mod program;
mod generator;

use std::fs;

use crate::tokenizer::tokenize;
use crate::parser::parse_program;
use crate::generator::generate_assembly;

fn main() {
    let contents = fs::read_to_string("samples/return_2.c").expect("Error: Unable to read sample #1.");

    println!("Read: {}", contents.trim());

    let mut tokens = tokenize(contents.as_str().chars());

    println!("Tokens: {:?}", tokens);

    let program = parse_program(&mut tokens);

    println!("Program: {:?}", program);

    let assembly = generate_assembly(program);

    println!("Assembly: ");
    println!("{}", assembly);
}
