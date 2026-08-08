
// TODO: Naming for General Call
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // INFO: Keywords
    Print,
    Var,
    If,
    ElseIf,
    Else,
    While,
    Return,
    Input,
    Convert,
    Mga,

    // INFO: Literals
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),

    // INFO: Operators
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
    Or,
    And,

    // INFO: Delimiters & Punctuation
    SingleDanda, // ᜵
    DoubleDanda, // ᜶
    EndStatement,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,  // [
    RightBracket, // ]
    Comma,

    // INFO: Special
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
