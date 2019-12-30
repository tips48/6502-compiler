mod tokenizer;
mod parser;
mod program;
mod generator;
mod utility;

use std::fs;

use crate::tokenizer::tokenize;
use crate::parser::parse_program;
use crate::parser::unparse_program;
use crate::generator::generate_assembly;

fn load_and_assemble(filename: String) {
    println!("Compiling: {}", filename);
    let contents = fs::read_to_string(filename).expect("Error: Unable to read file.");

    println!("Read: {}", contents.trim());

    let mut tokens = tokenize(contents.as_str().chars());

    println!("Tokens: {:?}", tokens);

    let program = parse_program(&mut tokens);

    println!("Program: {:?}", program);

    let assembly = generate_assembly(program);

    println!("Assembly: ");
    println!("{}", assembly);
}

fn load_and_disassemble(filename: String) {
    println!("Compiling: {}", filename);
    let contents = fs::read_to_string(filename).expect("Error: Unable to read file.");

    println!("Read: {}", contents.trim());

    let mut tokens = tokenize(contents.as_str().chars());

    println!("Tokens: {:?}", tokens);

    let program = parse_program(&mut tokens);

    println!("Program: {:?}", program);

    let assembly = unparse_program(program);

    println!("Disassembly: ");
    println!("{}", assembly);
}

fn main() {
    println!("Starting compiler.");

    load_and_assemble("samples/return_not_2.c".to_string());
    load_and_disassemble("samples/return_not_2.c".to_string());
}
