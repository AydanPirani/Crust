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
    let (_, function) = parse_function(tokens, offset)?;
    Ok(Program::FunctionCall { function: function })

}

fn parse_function(tokens: &Vec<Token>, mut offset: usize) -> Result<(usize, Function), String> {
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

    let (mut offset, body) = parse_statement(tokens, offset)?;

    expect_token!(tokens.get(offset).unwrap(), Token::CloseBrace => ());
    offset += 1;

    Ok((offset, Function::Define { name: name, body: body }))
    
}


fn parse_statement(tokens: &Vec<Token>, mut offset: usize) -> Result<(usize, Statement), String> {
    let token =  tokens.get(offset).unwrap();
    expect_token!(token, Token::Keyword { ty: Keyword::Return } => ());
    offset += 1;

    let (mut offset, expression) = parse_expression(tokens, offset)?;

    expect_token!(tokens.get(offset).unwrap(), Token::Semicolon => ());
    offset += 1;

    Ok((offset, Statement::Return { expression: expression }))
}


fn parse_expression(tokens: &Vec<Token>, mut offset: usize) -> Result<(usize, Expression), String> {
    let token =  tokens.get(offset).unwrap();
    let val = expect_token!(token, Token::Literal { ty: Literal::Int, val } => val.clone());
    offset += 1;

    let int = val.parse::<i32>().unwrap();

    Ok((offset, Expression::Int { int }))
}
