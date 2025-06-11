use crate::token::{self, Keyword, Literal, Token};
use regex::Regex;

const IDENT_PATTERN: &str = "^[a-zA-Z]+";
const INT_PATTERN: &str = "^[0-9]+";

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

    match tokenize(file_contents, &mut tokens) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    
    Ok(tokens)
}

fn tokenize(token_str: &str, tokens: &mut Vec<Token>) -> Result<(), String> {
    if token_str.is_empty() {
        return Ok(());
    }

    let token_bytes = token_str.as_bytes();

    if token_bytes[0].is_ascii_whitespace() {
        return tokenize(&token_str[1..], tokens);
    }

    match token_bytes[0] {
        b'{' => return step(Token::OpenBrace, token_str, tokens),
        b'}' => return step(Token::CloseBrace, token_str, tokens),
        b'(' => return step(Token::OpenParen, token_str, tokens),
        b')' => return step(Token::CloseParen, token_str, tokens),
        b';' => return step(Token::Semicolon, token_str, tokens),
        _ => ()
    }

    if token_str.starts_with("int") {
        let token = Token::Keyword {r#type: Keyword::Int};
        return step(token, token_str, tokens);
    }

    if token_str.starts_with("return") {
        let token = Token::Keyword {r#type: Keyword::Return};
        return step(token, token_str, tokens);
    }

    if let Some(prefix) = split_token(token_str, INT_PATTERN) {
        let token = Token::Literal {r#type: Literal::Int, value: prefix.to_string() };
        return step(token, token_str, tokens);
    }

    if let Some(prefix) = split_token(token_str, IDENT_PATTERN) {
        let token = Token::Identifier {name: prefix.to_string()};
        return step(token, token_str, tokens);
    }

    Ok(())

}


fn step(token: Token, token_str: &str, tokens: &mut Vec<Token>) -> Result<(), String> {
    let offset = token.len();
    tokens.push(token);
    return tokenize(&token_str[offset..], tokens);
}

fn split_token<'a>(s: &'a str, pattern: &str) -> Option<(&'a str)> {
    let re = Regex::new(&format!(r"^(?:{})", pattern)).unwrap();

    if let Some(mat) = re.find(s) {
        let end = mat.end();
        let prefix = &s[..end];
        Some(prefix)
    } else {
        None
    }

}