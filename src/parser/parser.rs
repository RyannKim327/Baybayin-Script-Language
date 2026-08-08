use crate::diagnostics::KalawangError;
use crate::lexer::{Token, TokenType};
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Stmt, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, KalawangError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, KalawangError> {
        if self.match_types(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, KalawangError> {
        let name_token = self.peek().clone();
        let name = if let TokenType::Identifier(ref n) = name_token.token_type {
            self.advance();
            n.clone()
        } else {
            return Err(KalawangError::ParseError {
                message: "Expect variable name after 'si' / 'that'".to_string(),
                line: name_token.line,
                column: name_token.column,
            });
        };

        let initializer = if self.match_types(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume_optional_danda();
        Ok(Stmt::VarDeclaration { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, KalawangError> {
        if self.match_types(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_types(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_types(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_types(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, KalawangError> {
        let condition = self.expression()?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_types(&[TokenType::ElseIf]) {
            Some(Box::new(self.if_statement()?))
        } else if self.match_types(&[TokenType::Else]) {
            if self.match_types(&[TokenType::If]) {
                Some(Box::new(self.if_statement()?))
            } else {
                Some(Box::new(self.statement()?))
            }
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, KalawangError> {
        let condition = self.expression()?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, KalawangError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, KalawangError> {
        let value = self.expression()?;
        self.consume_optional_danda();
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, KalawangError> {
        let expr = self.expression()?;
        self.consume_optional_danda();
        Ok(Stmt::Expression(expr))
    }

    fn consume_optional_danda(&mut self) {
        if self.check(&TokenType::SingleDanda)
            || self.check(&TokenType::DoubleDanda)
            || self.check(&TokenType::EndStatement)
        {
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr, KalawangError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, KalawangError> {
        let expr = self.logical_or()?;

        if self.match_types(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            if let Expr::Index { target, index } = expr {
                return Ok(Expr::IndexAssign {
                    target,
                    index,
                    value: Box::new(value),
                });
            }

            return Err(KalawangError::ParseError {
                message: "Invalid assignment target.".to_string(),
                line: equals.line,
                column: equals.column,
            });
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.logical_and()?;

        while self.match_types(&[TokenType::Or]) {
            let right = self.logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::LogicalOr,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.equality()?;

        while self.match_types(&[TokenType::And]) {
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::LogicalAnd,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.comparison()?;

        while self.match_types(&[
            TokenType::EqualEqual,
            TokenType::EqualEqualEqual,
            TokenType::BangEqual,
        ]) {
            let operator_token = self.previous().clone();
            let right = self.comparison()?;
            let op = match operator_token.token_type {
                TokenType::EqualEqual | TokenType::EqualEqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.term()?;

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator_token = self.previous().clone();
            let right = self.term()?;
            let op = match operator_token.token_type {
                TokenType::Greater => BinaryOp::GreaterThan,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::LessThan,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.factor()?;

        while self.match_types(&[TokenType::Plus, TokenType::Minus]) {
            let operator_token = self.previous().clone();
            let right = self.factor()?;
            let op = match operator_token.token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.unary()?;

        while self.match_types(&[TokenType::Star, TokenType::Slash]) {
            let operator_token = self.previous().clone();
            let right = self.unary()?;
            let op = match operator_token.token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, KalawangError> {
        if self.match_types(&[TokenType::Minus]) {
            let right = self.unary()?;
            Ok(Expr::Unary {
                op: UnaryOp::Negate,
                right: Box::new(right),
            })
        } else {
            self.call_or_index()
        }
    }

    fn call_or_index(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_types(&[TokenType::LeftBracket]) {
                let index = self.expression()?;
                self.consume(TokenType::RightBracket, "Expect ']' after index.")?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_types(&[TokenType::LeftParen]) {
                let mut arguments = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.match_types(&[TokenType::Comma]) {
                            break;
                        }
                        if self.check(&TokenType::RightParen) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    arguments,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, KalawangError> {
        let token = self.peek().clone();

        match token.token_type {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(n)))
            }
            TokenType::String(ref s) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::String(s.clone())))
            }
            TokenType::Boolean(b) => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(b)))
            }
            TokenType::Identifier(ref name) => {
                self.advance();
                Ok(Expr::Variable(name.clone()))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !self.check(&TokenType::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_types(&[TokenType::Comma]) {
                            break;
                        }
                        if self.check(&TokenType::RightBracket) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightBracket, "Expect ']' after array elements.")?;
                Ok(Expr::Array(elements))
            }
            TokenType::Mga => {
                self.advance();
                if self.match_types(&[TokenType::LeftParen]) {
                    let mut elements = Vec::new();
                    if !self.check(&TokenType::RightParen) {
                        loop {
                            elements.push(self.expression()?);
                            if !self.match_types(&[TokenType::Comma]) {
                                break;
                            }
                            if self.check(&TokenType::RightParen) {
                                break;
                            }
                        }
                    }
                    self.consume(
                        TokenType::RightParen,
                        "Expect ')' after arguments in 'mga(...)'.",
                    )?;
                    Ok(Expr::Array(elements))
                } else if self.check(&TokenType::LeftBracket) {
                    self.primary()
                } else {
                    Ok(Expr::Variable("mga".to_string()))
                }
            }
            TokenType::Input => {
                self.advance();
                let prompt = if self.match_types(&[TokenType::LeftParen]) {
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expect ')' after prompt parameter in 'pahingi' / 'ask'.",
                    )?;
                    expr
                } else {
                    self.primary()?
                };
                Ok(Expr::Input(Box::new(prompt)))
            }
            TokenType::Convert => {
                self.advance();
                if self.match_types(&[TokenType::LeftParen]) {
                    let value = self.expression()?;
                    self.consume(
                        TokenType::Comma,
                        "Expect ',' between parameters in 'isalin' / 'convert'.",
                    )?;
                    let target_type = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "Expect ')' after parameters in 'isalin' / 'convert'.",
                    )?;
                    Ok(Expr::Convert {
                        value: Box::new(value),
                        target_type: Box::new(target_type),
                    })
                } else {
                    let value = self.expression()?;
                    self.match_types(&[TokenType::Comma]);
                    let target_type = self.expression()?;
                    Ok(Expr::Convert {
                        value: Box::new(value),
                        target_type: Box::new(target_type),
                    })
                }
            }
            _ => Err(KalawangError::ParseError {
                message: format!("Unexpected token '{}'", token.lexeme),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<&Token, KalawangError> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            let token = self.peek();
            Err(KalawangError::ParseError {
                message: message.to_string(),
                line: token.line,
                column: token.column,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::ast::{Expr, LiteralValue, Stmt};

    #[test]
    fn test_parse_input_expression() {
        let source = r#"
            si name = pahingi("Pangalan: ");
            that age = ask("Age: ");
            ᜐᜒ ᜂᜐᜒᜇ᜔ = ᜉᜑᜒᜅᜒ("ᜂᜐᜒᜇ᜔: ");
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse().unwrap();

        assert_eq!(
            stmts[0],
            Stmt::VarDeclaration {
                name: "name".to_string(),
                initializer: Some(Expr::Input(Box::new(Expr::Literal(LiteralValue::String(
                    "Pangalan: ".to_string()
                ))))),
            }
        );
        assert_eq!(
            stmts[1],
            Stmt::VarDeclaration {
                name: "age".to_string(),
                initializer: Some(Expr::Input(Box::new(Expr::Literal(LiteralValue::String(
                    "Age: ".to_string()
                ))))),
            }
        );
        assert_eq!(
            stmts[2],
            Stmt::VarDeclaration {
                name: "ᜂᜐᜒᜇ᜔".to_string(),
                initializer: Some(Expr::Input(Box::new(Expr::Literal(LiteralValue::String(
                    "ᜂᜐᜒᜇ᜔: ".to_string()
                ))))),
            }
        );
    }

    #[test]
    fn test_parse_array_expressions() {
        let source = r#"
            si a = [1, 2, 3];
            si b = mga(4, 5, 6);
            si c = bilang(7, 8, 9);
            si x = a[0];
            a[1] = 99;
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse().unwrap();

        assert_eq!(
            stmts[0],
            Stmt::VarDeclaration {
                name: "a".to_string(),
                initializer: Some(Expr::Array(vec![
                    Expr::Literal(LiteralValue::Number(1.0)),
                    Expr::Literal(LiteralValue::Number(2.0)),
                    Expr::Literal(LiteralValue::Number(3.0)),
                ])),
            }
        );

        assert_eq!(
            stmts[1],
            Stmt::VarDeclaration {
                name: "b".to_string(),
                initializer: Some(Expr::Array(vec![
                    Expr::Literal(LiteralValue::Number(4.0)),
                    Expr::Literal(LiteralValue::Number(5.0)),
                    Expr::Literal(LiteralValue::Number(6.0)),
                ])),
            }
        );

        assert_eq!(
            stmts[3],
            Stmt::VarDeclaration {
                name: "x".to_string(),
                initializer: Some(Expr::Index {
                    target: Box::new(Expr::Variable("a".to_string())),
                    index: Box::new(Expr::Literal(LiteralValue::Number(0.0))),
                }),
            }
        );

        assert_eq!(
            stmts[4],
            Stmt::Expression(Expr::IndexAssign {
                target: Box::new(Expr::Variable("a".to_string())),
                index: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                value: Box::new(Expr::Literal(LiteralValue::Number(99.0))),
            })
        );
    }
}
