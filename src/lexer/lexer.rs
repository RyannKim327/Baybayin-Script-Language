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
                '[' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::LeftBracket,
                        "[",
                        start_line,
                        start_col,
                    ));
                }
                ']' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::RightBracket,
                        "]",
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
                            message: "Unexpected character '!'".to_string(),
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
                '|' => {
                    self.advance();
                    if self.match_char('|') {
                        tokens.push(Token::new(TokenType::Or, "||", start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Or, "|", start_line, start_col));
                    }
                }
                '&' => {
                    self.advance();
                    if self.match_char('&') {
                        tokens.push(Token::new(TokenType::And, "&&", start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::And, "&", start_line, start_col));
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
            if self.peek() == '\\' {
                self.advance(); // consume '\\'
                if self.is_at_end() {
                    return Err(KalawangError::LexerError {
                        message: "Unterminated string literal".to_string(),
                        line: start_line,
                        column: start_col,
                    });
                }
                match self.peek() {
                    'n' => {
                        self.advance();
                        value.push('\n');
                    }
                    't' => {
                        self.advance();
                        value.push('\t');
                    }
                    'r' => {
                        self.advance();
                        value.push('\r');
                    }
                    '\\' => {
                        self.advance();
                        value.push('\\');
                    }
                    '"' => {
                        self.advance();
                        value.push('"');
                    }
                    _ => {
                        value.push(self.advance());
                    }
                }
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                value.push(self.advance());
            }
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
            "tama" | "ᜆᜋ" | "true" => TokenType::Boolean(true),
            "mali" | "ᜋᜎᜒ" | "false" => TokenType::Boolean(false),
            "sabihin" | "ᜐᜊᜒᜑᜒᜈ᜔" | "ipaliwanag" | "ᜁᜉᜎᜒᜏᜈᜄ᜔" | "tell" | "say" | "print" => {
                TokenType::Print
            }
            "si" | "ᜐᜒ" | "ipangalan" | "ᜁᜉᜅᜎᜈ᜔" | "that" => TokenType::Var,
            "kung" | "ᜃᜓᜅ᜔" | "if" => TokenType::If,
            "okaya" | "ukaya" | "ᜂᜃᜌ" | "elseif" => TokenType::ElseIf,
            "kundi" | "ᜃᜓᜈ᜔ᜇᜒ" | "else" => TokenType::Else,
            "hindi" | "ᜑᜒᜈ᜔ᜇᜒ" | "not" => TokenType::BangEqual,
            "habang" | "ᜑᜊᜅ᜔" | "while" => TokenType::While,
            "ibalik" | "ᜁᜊᜎᜒᜃ᜔" | "return" => TokenType::Return,
            "pahingi" | "ᜉᜑᜒᜅᜒ" | "ask" => TokenType::Input,
            "isalin" | "ᜁᜐᜎᜒᜈ᜔" | "convert" => TokenType::Convert,
            "mga" | "ᜋᜅ" | "array" | "list" => TokenType::Mga,
            "ay" | "ᜀᜌ᜔" | "is" => TokenType::Equal,
            "aytalagang" | "ᜀᜌ᜔ᜆᜎᜄᜅ᜔" | "isliterally" => TokenType::EqualEqual,
            "o" | "ᜂ" | "or" => TokenType::Or,
            "at" | "ᜀᜆ᜔" | "and" => TokenType::And,
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
        let mut lexer = Lexer::new("== === aytalagang ᜀᜌ᜔ᜆᜎᜄᜅ᜔ ay ᜀᜌ᜔");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[1].token_type, TokenType::EqualEqualEqual);
        assert_eq!(tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[3].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[4].token_type, TokenType::Equal);
        assert_eq!(tokens[5].token_type, TokenType::Equal);
    }

    #[test]
    fn test_string_newline_escape() {
        let mut lexer = Lexer::new("\"Unang linya\\nPangalawang linya\"");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type,
            TokenType::String("Unang linya\nPangalawang linya".to_string())
        );
    }

    #[test]
    fn test_english_keywords() {
        let mut lexer = Lexer::new(
            "tell say print that if elseif else not while return is isliterally true false",
        );
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Print);
        assert_eq!(tokens[1].token_type, TokenType::Print);
        assert_eq!(tokens[2].token_type, TokenType::Print);
        assert_eq!(tokens[3].token_type, TokenType::Var);
        assert_eq!(tokens[4].token_type, TokenType::If);
        assert_eq!(tokens[5].token_type, TokenType::ElseIf);
        assert_eq!(tokens[6].token_type, TokenType::Else);
        assert_eq!(tokens[7].token_type, TokenType::BangEqual);
        assert_eq!(tokens[8].token_type, TokenType::While);
        assert_eq!(tokens[9].token_type, TokenType::Return);
        assert_eq!(tokens[10].token_type, TokenType::Equal);
        assert_eq!(tokens[11].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[12].token_type, TokenType::Boolean(true));
        assert_eq!(tokens[13].token_type, TokenType::Boolean(false));
    }

    #[test]
    fn test_input_keywords() {
        let mut lexer = Lexer::new("pahingi ᜉᜑᜒᜅᜒ ask");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Input);
        assert_eq!(tokens[1].token_type, TokenType::Input);
        assert_eq!(tokens[2].token_type, TokenType::Input);
    }

    #[test]
    fn test_convert_keywords() {
        let mut lexer = Lexer::new("isalin ᜁᜐᜎᜒᜈ᜔ convert");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Convert);
        assert_eq!(tokens[1].token_type, TokenType::Convert);
        assert_eq!(tokens[2].token_type, TokenType::Convert);
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("o ᜂ or || | at ᜀᜆ᜔ and && &");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Or);
        assert_eq!(tokens[1].token_type, TokenType::Or);
        assert_eq!(tokens[2].token_type, TokenType::Or);
        assert_eq!(tokens[3].token_type, TokenType::Or);
        assert_eq!(tokens[4].token_type, TokenType::Or);
        assert_eq!(tokens[5].token_type, TokenType::And);
        assert_eq!(tokens[6].token_type, TokenType::And);
        assert_eq!(tokens[7].token_type, TokenType::And);
        assert_eq!(tokens[8].token_type, TokenType::And);
        assert_eq!(tokens[9].token_type, TokenType::And);
    }

    #[test]
    fn test_array_tokens() {
        let mut lexer = Lexer::new("[1, 2, 3] mga ᜋᜅ array list");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LeftBracket);
        assert_eq!(tokens[1].token_type, TokenType::Number(1.0));
        assert_eq!(tokens[2].token_type, TokenType::Comma);
        assert_eq!(tokens[3].token_type, TokenType::Number(2.0));
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::Number(3.0));
        assert_eq!(tokens[6].token_type, TokenType::RightBracket);
        assert_eq!(tokens[7].token_type, TokenType::Mga);
        assert_eq!(tokens[8].token_type, TokenType::Mga);
        assert_eq!(tokens[9].token_type, TokenType::Mga);
        assert_eq!(tokens[10].token_type, TokenType::Mga);
    }
}
