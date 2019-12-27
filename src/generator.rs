use crate::program::Program;
use crate::program::Function;
use crate::program::Statement;
use crate::program::Expression;

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
    let value = generate_expression(statement.exp);

    let mut result = append_tabs(indent_level);

    // Load value into X
    result.push_str("LDX #$");
    result.push_str(&value);
    result.push_str("\n");

    result.push_str(&append_tabs(indent_level));
    result.push_str("rts\n");

    result
}

fn generate_expression(expression: Expression) -> String {
    expression.value.to_string()
}

fn append_tabs(amount: u8) -> String {
    let mut result = String::new();

    for _ in 0..amount {
        result.push(' ');
    }

    result
}
