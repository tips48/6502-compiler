use crate::tokenizer::Token;

use crate::program::Program;
use crate::program::Function;
use crate::program::Statement;
use crate::program::Expression;
use crate::program::UnaryOp;

use crate::utility::append_tabs;

pub fn generate_assembly(program: Program) -> String {
    let mut result = String::from("jsr main\nbrk\n");

    let func = generate_function(program.function);

    result.push_str(&func);

    result
}

fn generate_function(function: Function) -> String {
    let mut result = function.name;
    result.push_str(":\n");

    result.push_str(&generate_statement(function.statement, 4));

    result
}

fn generate_statement(statement: Statement, indent_level: u8) -> String {
    let mut result = generate_expression(statement.exp, indent_level);

    // Return sub program
    result.push_str(&append_tabs(indent_level));
    result.push_str("tax\n");
    result.push_str(&append_tabs(indent_level));
    result.push_str("rts\n");

    result
}

fn generate_expression(expression: Expression, indent_level: u8) -> String {
    let mut result = append_tabs(indent_level);

    if expression.value.is_some() {
        // Load value into A
        result.push_str("lda #$");
        result.push_str(&expression.value.unwrap().to_string());
        result.push_str("\n");
    } else {
        result.push_str(&generate_unary_op(*expression.unary_op.unwrap(), indent_level));
    }

    result
}

// TODO look at NEG_16 to fix the label issue
fn generate_unary_op(unary_op: UnaryOp, indent_level: u8) -> String {
    let mut result = append_tabs(0);

    match unary_op.token {
        Token::Keyword(keyword) => {
            result.push_str(&generate_expression(*unary_op.exp, 0));
            result.push_str(&append_tabs(indent_level));
            // TODO make this a match
            if keyword.to_string() == String::from("-") {
                result.push_str("eor #$FF\n");
            } else if keyword.to_string() == String::from("~") {
                result.push_str("clc\n");
                result.push_str(&append_tabs(indent_level));
                result.push_str("eor #$FF\n");
                result.push_str(&append_tabs(indent_level));
                result.push_str("adc #1\n");
            } else if keyword.to_string() == String::from("!") {
                result.push_str("cmp #0\n"); // Check if equal to 0
                result.push_str(&append_tabs(indent_level));
                result.push_str("bne ZERO\n"); // If not zero
                result.push_str(&append_tabs(indent_level));
                result.push_str("lda #0\nZERO:\n"); // Set to zero; if zero
                result.push_str(&append_tabs(indent_level));
                result.push_str("lda #1\n"); // Set to one
            } else {
                println!("Error parsing expression! Value: {:?}", Token::Keyword(keyword));
            }
        },

        _tmp => println!("Error generating assembly! Unary op value: {:?}", _tmp),
    }

    result
}
