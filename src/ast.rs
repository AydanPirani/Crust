pub enum Program {
    FunctionCall {function: Function}
}

pub enum Function {
    Define {name: String, body: Statement},
}

pub enum Statement {
    Expression {expression: Expression},
}

pub enum Expression {
    Int {int: i32},
}