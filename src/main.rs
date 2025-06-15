mod ast;
mod token;

mod lexer;
mod parser;
mod generator;

fn main() {
    let filepath = "programs/return_2.c";

    let tokens = lexer::lex_file(filepath).unwrap();

    let ast = parser::parse(&tokens).unwrap();
    
    println!("{:?}", ast);

    let _ = generator::generate(ast, "output.s");
}
