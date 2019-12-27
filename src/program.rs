use std::collections::VecDeque;

use crate::tokenizer::Token;
use crate::tokenizer::tokenize;

use std::str::FromStr;
use std::char::ParseCharError;

#[derive(Debug)]
pub struct Program {
    pub function: Function,
}
impl FromStr for Program {
    type Err = ParseCharError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tokens: &mut VecDeque<Token> = &mut tokenize(input.chars());

        Ok(parse_program(tokens))
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statement: Statement,
}

#[derive(Debug)]
pub struct Statement {
    pub exp: Expression,
}

#[derive(Debug)]
pub struct Expression {
    pub value: u32,
}

pub fn parse_program(tokens: &mut VecDeque<Token>) -> Program {
    Program {
        function: parse_function(tokens),
    }
}

pub fn parse_function(tokens: &mut VecDeque<Token>) -> Function {
    // Int
    let mut token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            if word != String::from("int") {
                println!("Error parsing statement! No type");
            }
        },

        _ => println!("Error parsing statement!"),
    }

    // Name
    let mut name = "foo".to_owned();
    token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            name = word;
        }

        _ => println!("Error parsing statement! No function name"),
    }

    // (
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("(") {
                println!("Error parsing statement! No (");
            }
        }

        _ => println!("Error parsing statement! No ("),
    }

    // )
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from(")") {
                println!("Error parsing statement! No )");
            }
        }

        _ => println!("Error parsing statement! No )"),
    }

    // {
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("{") {
                println!("Error parsing statement! No {{");
            }
        }

        _ => println!("Error parsing statement! No {{"),
    }

    let statement = parse_statement(tokens);

    // }
    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from("}") {
                println!("Error parsing statement! No }}");
            }
        }

        _ => println!("Error parsing statement! No }}"),
    }

    Function {
        name: name,
        statement: statement,
    }
}

pub fn parse_statement(tokens: &mut VecDeque<Token>) -> Statement {
    let mut token = tokens.pop_front().unwrap();

    match token {
        Token::Word(word) => {
            if word != String::from("return") {
                println!("Error parsing statement! No return");
            }
        },

        _ => println!("Error parsing statement!"),
    }

    let exp = parse_expression(tokens);

    token = tokens.pop_front().unwrap();

    match token {
        Token::Keyword(keyword) => {
            if keyword != String::from(";") {
                println!("Error parsing statement! No ;");
            }
        }

        _ => println!("Error parsing statement! No ;"),
    }

    Statement {
        exp: exp,
    }
}

pub fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    let token = tokens.pop_front().unwrap();

    let mut value: u32 = 0;

    match token {
        Token::Number(i) => {
            value = i;
        },

        _ => println!("Error parsing expression!"),
    };

    Expression {
        value: value,
    }
}
