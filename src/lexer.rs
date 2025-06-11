use crate::token::Token;

pub fn lex_file(filepath: &str) -> Result<Vec<Token>, String> {
    let metadata = match std::fs::metadata(filepath) {
        Ok(metadata) => metadata,
        Err(err) => return Err(format!("Failed to get metadata: {}", err)),
    };

    if metadata.is_dir() {
        return Err(format!("{} is a directory", filepath));
    }

    let file_contents = match std::fs::read_to_string(filepath) {
    Ok(file) => file,
        Err(err) => return Err(format!("Failed to read file: {}", err)),
    };

    get_tokens(&file_contents)
}

fn get_tokens(file_contents: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for line in file_contents.lines() {
        println!("{}", line);
        tokens.push(Token::Semicolon);
        // for token in line.split_whitespace() {
        //     tokens.push(Token::new(token));
        // }
    }

    Ok(tokens)
}