pub enum TokenType {
    // Single-character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Minus,
    Plus,
    Star,
    Slash,
    Colon,
    Semicolon,

    // Single- or Double-character
    ExclMark,
    ExclMarkEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    Def,

    // Others
    Identifier(String),
    Number(u32),
}

pub struct Token {
    pub typ: TokenType,
    pub line: usize,
    pub column: usize,
}
