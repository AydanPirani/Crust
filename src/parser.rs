use crate::token::{Token, Literal, Keyword};
use crate::ast::{Program, Function, Statement, Expression};

#[macro_export]
macro_rules! expect_token {
    // pattern plus a return expression
    ($actual:expr, $pattern:pat => $ret:expr) => {
        match $actual {
            $pattern => $ret,
            other => unreachable!(
                "expected `{}`, got `{:?}`",
                stringify!($pattern),
                other
            ),
        }
    };
}



pub fn parse(tokens: &Vec<Token>) -> Result<Program, String> {
    let program: Program = parse_program(tokens, 0)?;
    Ok(program)
}

fn parse_program(tokens: &Vec<Token>, offset: usize) -> Result<Program, String> {
let function = parse_function(tokens, offset)?;
    Ok(Program::FunctionCall { function: function })

}

fn parse_function(tokens: &Vec<Token>, mut offset: usize) -> Result<Function, String> {
    let token =  tokens.get(offset).unwrap();    
    
    expect_token!(token, Token::Keyword { ty: Keyword::Int } => ());
    offset += 1;

    let token =  tokens.get(offset).unwrap();

    let name: String = expect_token!(token, Token::Identifier { val } => val.clone());
    offset += 1;
    
    expect_token!(tokens.get(offset).unwrap(), Token::OpenParen => ());
    offset += 1;

    expect_token!(tokens.get(offset).unwrap(), Token::CloseParen => ());
    offset += 1;

    expect_token!(tokens.get(offset).unwrap(), Token::OpenBrace => ());
    offset += 1;

    let body = parse_statement(tokens, offset)?;

    expect_token!(tokens.get(offset).unwrap(), Token::CloseBrace => ());

    Ok(Function::Define { name: name, body: body })
    
}


fn parse_statement(tokens: &Vec<Token>, mut offset: usize) -> Result<Statement, String> {
    let token =  tokens.get(offset).unwrap();
    expect_token!(token, Token::Keyword { ty: Keyword::Return } => ());
    offset += 1;

    let expression = parse_expression(tokens, offset)?;
    Ok(Statement::Expression { expression: expression })
}


fn parse_expression(tokens: &Vec<Token>, offset: usize) -> Result<Expression, String> {
    let token =  tokens.get(offset).unwrap();
    let val = expect_token!(token, Token::Literal { ty: Literal::Int, val } => val.clone());

    let int = val.parse::<i32>().unwrap();

    Ok(Expression::Int { int })
}
