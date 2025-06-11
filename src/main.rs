mod lexer;
mod token;

fn main() {
    let filepath = "programs/return_2.c";

    let tokens = match lexer::lex_file(filepath) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    for token in tokens {
        println!("{:?}", token);
    }
}
