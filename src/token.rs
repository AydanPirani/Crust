pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword {keyword_type: KeywordType},
    Identifier,
    Literal {literal_type: LiteralType, value: String},
}

pub enum KeywordType {
    Int,
    Return,
}

pub enum LiteralType {
    Int,
}