use std::collections::VecDeque;

use crate::program::Program;
use crate::program::Function;
use crate::program::Statement;
use crate::program::Expression;
use crate::program::UnaryOp;

use crate::tokenizer::Token;

use crate::utility::append_tabs;

pub fn unparse_program(program: Program) -> String {
    unparse_function(program.function)
}

fn unparse_function(function: Function) -> String {
    let mut result = append_tabs(0);

    result.push_str("int ");
    result.push_str(&function.name);
    result.push_str("() {\n");
    result.push_str(&unparse_statement(function.statement, 4));
    result.push_str("}");

    result
}

fn unparse_statement(statement: Statement, indent_level: u8) -> String {
    let mut result = append_tabs(indent_level);

    result.push_str("return ");
    result.push_str(&unparse_expression(statement.exp, 0));
    result.push_str(";\n");

    result
}

fn unparse_expression(expression: Expression, indent_level: u8) -> String {
    let mut result = append_tabs(indent_level);

    if expression.value.is_some() {
        result.push_str(&expression.value.unwrap().to_string());
    } else {
        result.push_str(&unparse_unary_op(*expression.unary_op.unwrap(), indent_level));
    }

    result
}

fn unparse_unary_op(unary_op: UnaryOp, indent_level: u8) -> String {
    let mut result = append_tabs(indent_level);

    match unary_op.token {
        Token::Keyword(keyword) => {
            result.push_str(&keyword);
            result.push_str(&unparse_expression(*unary_op.exp, indent_level));
        },

        _tmp => println!("Error generating disassembly! Unary op value: {:?}", _tmp),
    }

    result
}


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

    let mut value: u8 = 0;
    let mut unary_op = UnaryOp {
        token: Token::Keyword(String::from("")),
        exp: Box::new(Expression {
            value: None,
            unary_op: None,
        }),
    };

    let mut num = false;

    match token {
        Token::Keyword(keyword) => {
            if keyword == String::from("-") {
                unary_op.token = Token::Keyword(keyword);
                unary_op.exp = Box::new(parse_expression(tokens));
            } else if keyword == String::from("~") {
                unary_op.token = Token::Keyword(keyword);
                unary_op.exp = Box::new(parse_expression(tokens));
            } else if keyword == String::from("!") {
                unary_op.token = Token::Keyword(keyword);
                unary_op.exp = Box::new(parse_expression(tokens));
            } else {
                println!("Error parsing expression! Value: {:?}", Token::Keyword(keyword));
            }
        },

        Token::Number(i) => {
            num = true;
            value = i;
        },

        _tmp => println!("Error parsing expression! Value: {:?}", _tmp),
    };

    if num {
        Expression {
            value: Some(value),

            unary_op: None,
        }
    } else {
        Expression {
            value: None,

            unary_op: Some(Box::new(unary_op)),
        }
    }
}
