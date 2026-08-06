#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Print,
    Var,
    If,
    ElseIf,
    Else,
    While,
    Return,

    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    EqualEqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters & Punctuation
    SingleDanda, // ᜵
    DoubleDanda, // ᜶
    EndStatement,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
            column,
        }
    }
}
