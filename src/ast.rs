use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Program {
    FunctionCall {function: Function}
}

#[derive(Debug)]
pub enum Function {
    Define {name: String, body: Statement},
}

#[derive(Debug)]
pub enum Statement {
    Expression {expression: Expression},
}

#[derive(Debug)]
pub enum Expression {
    Int {int: i32},
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Int { int } => write!(f, "Int({})", int),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expression { expression } =>
                write!(f, "Expression({})", expression),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Define { name, body } =>
                write!(f, "Define(name: {}, body: {})", name, body),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program::FunctionCall { function } =>
                write!(f, "FunctionCall({})", function),
        }
    }
}