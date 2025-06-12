mod ast;
mod token;

mod lexer;
mod parser;

fn main() {
    let filepath = "programs/multi_digit.c";

    let tokens = lexer::lex_file(filepath).unwrap();

    let ast = parser::parse(&tokens).unwrap();
    
    println!("{:?}", ast);
}
