#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Keyword {r#type: Keyword},
    Identifier {name: String},
    Literal {r#type: Literal, value: String},
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::OpenBrace => 1,
            Token::CloseBrace => 1,
            Token::OpenParen => 1,
            Token::CloseParen => 1,
            Token::Semicolon => 1,
            Token::Keyword {r#type} => match r#type {
                Keyword::Int => 3,
                Keyword::Return => 6,
            },
            Token::Identifier {name } => name.len(),
            Token::Literal {r#type: _, value} => value.len(),
        }
    }
}

#[derive(Debug)]
pub enum Keyword {
    Int,
    Return,
}

#[derive(Debug)]
pub enum Literal {
    Int,
}