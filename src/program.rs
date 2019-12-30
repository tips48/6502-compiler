use std::collections::VecDeque;

use crate::tokenizer::Token;
use crate::tokenizer::tokenize;
use crate::parser::parse_program;

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
    pub value: Option<u8>, // 8 for now, we need different assembly for 16 b/c registers are 8 bit

    pub unary_op: Option<Box<UnaryOp>>,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub token: Token,
    pub exp: Box<Expression>,
}
