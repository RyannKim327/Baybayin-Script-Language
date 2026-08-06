#[derive(Debug, Clone, PartialEq)]
pub enum KalawangError {
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },
    RuntimeError {
        message: String,
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for KalawangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KalawangError::LexerError {
                message,
                line,
                column,
            } => {
                write!(f, "[Lexer Error L{}:{}] {}", line, column, message)
            }
            KalawangError::ParseError {
                message,
                line,
                column,
            } => {
                write!(f, "[Parse Error L{}:{}] {}", line, column, message)
            }
            KalawangError::RuntimeError {
                message,
                line,
                column,
            } => {
                write!(f, "[Runtime Error L{}:{}] {}", line, column, message)
            }
        }
    }
}

impl std::error::Error for KalawangError {}
