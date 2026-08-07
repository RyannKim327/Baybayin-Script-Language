use crate::diagnostics::KalawangError;
use crate::lexer::token::{Token, TokenType};

pub struct Lexer<'a> {
    _source: &'a str,
    chars: Vec<(usize, char)>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            _source: source,
            chars,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, KalawangError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let start_line = self.line;
            let start_col = self.column;
            let ch = self.peek();

            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                '+' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::Plus, "+", start_line, start_col));
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::Minus, "-", start_line, start_col));
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::Star, "*", start_line, start_col));
                }
                '/' => {
                    self.advance();
                    if self.match_char('/') {
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Slash, "/", start_line, start_col));
                    }
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::LeftParen, "(", start_line, start_col));
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::RightParen,
                        ")",
                        start_line,
                        start_col,
                    ));
                }
                '{' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::LeftBrace, "{", start_line, start_col));
                }
                '}' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::RightBrace,
                        "}",
                        start_line,
                        start_col,
                    ));
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::new(TokenType::Comma, ",", start_line, start_col));
                }
                ';' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::EndStatement,
                        ";",
                        start_line,
                        start_col,
                    ));
                }
                '᜵' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::SingleDanda,
                        "᜵",
                        start_line,
                        start_col,
                    ));
                }
                '᜶' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::DoubleDanda,
                        "᜶",
                        start_line,
                        start_col,
                    ));
                }
                '=' => {
                    self.advance();
                    if self.match_char('=') {
                        if self.match_char('=') {
                            tokens.push(Token::new(
                                TokenType::EqualEqualEqual,
                                "===",
                                start_line,
                                start_col,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::EqualEqual,
                                "==",
                                start_line,
                                start_col,
                            ));
                        }
                    } else {
                        tokens.push(Token::new(TokenType::Equal, "=", start_line, start_col));
                    }
                }
                '!' => {
                    self.advance();
                    if self.match_char('=') {
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            "!=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        return Err(KalawangError::LexerError {
                            message: format!("Unexpected character '!'"),
                            line: start_line,
                            column: start_col,
                        });
                    }
                }
                '<' => {
                    self.advance();
                    if self.match_char('=') {
                        tokens.push(Token::new(
                            TokenType::LessEqual,
                            "<=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(TokenType::Less, "<", start_line, start_col));
                    }
                }
                '>' => {
                    self.advance();
                    if self.match_char('=') {
                        tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            ">=",
                            start_line,
                            start_col,
                        ));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, ">", start_line, start_col));
                    }
                }
                '"' => {
                    let string_token = self.read_string(start_line, start_col)?;
                    tokens.push(string_token);
                }
                ch if ch.is_ascii_digit() => {
                    let number_token = self.read_number(start_line, start_col)?;
                    tokens.push(number_token);
                }
                ch if is_identifier_start(ch) => {
                    let id_token = self.read_identifier(start_line, start_col);
                    tokens.push(id_token);
                }
                unexpected => {
                    return Err(KalawangError::LexerError {
                        message: format!("Unexpected character '{}'", unexpected),
                        line: start_line,
                        column: start_col,
                    });
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));
        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.chars.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.position].1
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.peek();
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.position].1 != expected {
            false
        } else {
            self.position += 1;
            self.column += 1;
            true
        }
    }

    fn read_string(&mut self, start_line: usize, start_col: usize) -> Result<Token, KalawangError> {
        self.advance(); // consume opening quote
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(KalawangError::LexerError {
                message: "Unterminated string literal".to_string(),
                line: start_line,
                column: start_col,
            });
        }

        self.advance(); // consume closing quote
        Ok(Token::new(
            TokenType::String(value.clone()),
            format!("\"{}\"", value),
            start_line,
            start_col,
        ))
    }

    fn read_number(&mut self, start_line: usize, start_col: usize) -> Result<Token, KalawangError> {
        let mut num_str = String::new();
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            num_str.push(self.advance());
        }

        if !self.is_at_end() && self.peek() == '.' {
            num_str.push(self.advance());
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                num_str.push(self.advance());
            }
        }

        let val: f64 = num_str.parse().map_err(|_| KalawangError::LexerError {
            message: format!("Invalid number literal '{}'", num_str),
            line: start_line,
            column: start_col,
        })?;

        Ok(Token::new(
            TokenType::Number(val),
            num_str,
            start_line,
            start_col,
        ))
    }

    fn read_identifier(&mut self, start_line: usize, start_col: usize) -> Token {
        let mut ident = String::new();
        while !self.is_at_end() && is_identifier_part(self.peek()) {
            ident.push(self.advance());
        }

        let token_type = match ident.as_str() {
            "tama" | "ᜆᜋ" => TokenType::Boolean(true),
            "mali" | "ᜋᜎᜒ" => TokenType::Boolean(false),
            "sabihin" | "ᜐᜊᜒᜑᜒᜈ᜔" | "ipaliwanag" | "ᜁᜉᜎᜒᜏᜈᜄ᜔" | "print" => TokenType::Print,
            "si" | "ᜐᜒ" | "ipangalan" | "ᜁᜉᜅᜎᜈ᜔" | "var" => TokenType::Var,
            "kung" | "ᜃᜓᜅ᜔" => TokenType::If,
            "okaya" | "ukaya" | "ᜂᜃᜌ" => TokenType::ElseIf,
            "kundi" | "ᜃᜓᜈ᜔ᜇᜒ" => TokenType::Else,
            "habang" | "ᜑᜊ᜔" => TokenType::While,
            "ibalik" | "ᜁᜊᜎᜒᜃ᜔" => TokenType::Return,
            "ay" | "ᜀᜌ᜔" => TokenType::Equal,
            "aytalagang" | "ᜀᜌ᜔ᜆᜎᜄᜅ᜔" => TokenType::EqualEqual,
            _ => TokenType::Identifier(ident.clone()),
        };

        Token::new(token_type, ident, start_line, start_col)
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || is_baybayin(ch)
}

fn is_identifier_part(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || is_baybayin(ch)
}

fn is_baybayin(ch: char) -> bool {
    // Baybayin Unicode block: U+1700 to U+171F
    ('\u{1700}'..='\u{171F}').contains(&ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baybayin_lexer() {
        let mut lexer = Lexer::new("10 + 20 ᜵");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number(10.0));
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number(20.0));
        assert_eq!(tokens[3].token_type, TokenType::SingleDanda);
    }

    #[test]
    fn test_else_if_keywords() {
        let mut lexer = Lexer::new("okaya ukaya ᜂᜃᜌ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::ElseIf);
        assert_eq!(tokens[1].token_type, TokenType::ElseIf);
        assert_eq!(tokens[2].token_type, TokenType::ElseIf);
    }

    #[test]
    fn test_equality_tokens() {
        let mut lexer = Lexer::new("== === ay ᜀᜌ᜔ ᜀᜌ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[1].token_type, TokenType::EqualEqualEqual);
        assert_eq!(tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[3].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[4].token_type, TokenType::EqualEqual);
    }
}
