#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    Keyword {ty: Keyword},
    Identifier {val: String},
    Literal {ty: Literal, val: String},
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::OpenBrace => 1,
            Token::CloseBrace => 1,
            Token::OpenParen => 1,
            Token::CloseParen => 1,
            Token::Semicolon => 1,
            Token::Keyword {ty} => match ty {
                Keyword::Int => 3,
                Keyword::Return => 6,
            },
            Token::Identifier {val } => val.len(),
            Token::Literal {ty: _, val} => val.len(),
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