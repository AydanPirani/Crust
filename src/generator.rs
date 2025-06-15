use crate::ast::{Program, Function, Statement, Expression};

pub fn generate(ast: Program, filepath: &str) -> Result<(), String> {
    match generate_program(ast) {
        Ok(code) => {
            std::fs::write(filepath, code).unwrap();
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn generate_program(program: Program) -> Result<String, String> {
    let mut code = String::new();

    match program {
        Program::FunctionCall{function} => {
            generate_function(function, &mut code)?;
        }
    }

    Ok(code)
}

fn generate_function(function: Function, code: &mut String) -> Result<(), String> {
    match function {
        Function::Define { name, body } => {
            code.push_str(&format!(".globl _{}\n", name));
            code.push_str(&format!("_{}:\n", name));
            code.push_str(".align 2\n");
            generate_statement(body, code)?;
        }
    }

    Ok(())
}

fn generate_statement(statement: Statement, code: &mut String) -> Result<(), String> {
    match statement {
        Statement::Return { expression } => {
            code.push_str("mov x0, ");
            generate_expression(expression, code)?;
            code.push_str("\nret\n");
        }
    }

    Ok(())
}

fn generate_expression(expression: Expression, code: &mut String) -> Result<(), String> {
    match expression {
        Expression::Int { int } => {
            code.push_str(&format!("#{}", int));
        }
    }

    Ok(())
}