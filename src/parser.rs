use std::collections::VecDeque;

use crate::program::Program;
use crate::program::Function;
use crate::program::Statement;
use crate::program::Expression;

use crate::tokenizer::Token;

pub fn parse_program(tokens: &mut VecDeque<Token>) -> Program {
    Program {
        function: parse_function(tokens),
    }
}

fn parse_function(tokens: &mut VecDeque<Token>) -> Function {
    // Int
    let mut token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            if word != String::from("int") {
                println!("Error parsing function! {:?}", Token::Word(word));
            }
        },

        _tmp => println!("Error parsing function! No type: {:?}", _tmp),
    }

    // Name
    let mut name = "foo".to_owned();
    token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            name = word;
        }

        _tmp => println!("Error parsing function! No function name: {:?}", _tmp),
    }

    // (
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("(") {
                println!("Error parsing function! No (: {:?}", Token::Keyword(keyword));
            }
        }

        _tmp => println!("Error parsing function! No (: {:?}", _tmp),
    }

    // )
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from(")") {
                println!("Error parsing function! No ): {:?}", Token::Keyword(keyword));
            }
        }

        _tmp => println!("Error parsing function! No ): {:?}", _tmp),
    }

    // {
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("{") {
                println!("Error parsing function! No {{: {:?}", Token::Keyword(keyword));
            }
        }

        _tmp => println!("Error parsing function! No {{: {:?}", _tmp),
    }

    let statement = parse_statement(tokens);

    // }
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("}") {
                println!("Error parsing function! No }}: {:?}", Token::Keyword(keyword));
            }
        }

        _tmp => println!("Error parsing function! No }}: {:?}", _tmp),
    }

    Function {
        name: name,
        statement: statement,
    }
}

fn parse_statement(tokens: &mut VecDeque<Token>) -> Statement {
    let mut token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            if word != String::from("return") {
                println!("Error parsing statement! No return: {:?}", Token::Word(word));
            }
        },

        _tmp => println!("Error parsing statement! No return: {:?}", _tmp),
    }

    let exp = parse_expression(tokens);

    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from(";") {
                println!("Error parsing statement! No ;: {:?}", Token::Keyword(keyword));
            }
        }

        _tmp => println!("Error parsing statement! No ;: {:?}", _tmp),
    }

    Statement {
        exp: exp,
    }
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    let token = tokens.pop_front().unwrap();

    let mut value: u32 = 0;

    match token {
        Token::Number(i) => {
            value = i;
        },

        _tmp => println!("Error parsing expression! Value: {:?}", _tmp),
    };

    Expression {
        value: value,
    }
}
