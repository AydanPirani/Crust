mod lexer;
mod token;

fn main() {
    let filepath = "programs/return_2.c";

    let tokens = lexer::lex_file(filepath);

    // println!("{:?}", tokens);
}
