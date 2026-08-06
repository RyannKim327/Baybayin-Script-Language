use crate::diagnostics::KalawangError;
use crate::lexer::{Token, TokenType};
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Stmt};

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
                message: "Expect variable name after 'si' / 'var'".to_string(),
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
        if self.check(&TokenType::SingleDanda) || self.check(&TokenType::DoubleDanda) {
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr, KalawangError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, KalawangError> {
        let expr = self.equality()?;

        if self.match_types(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
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

    fn equality(&mut self) -> Result<Expr, KalawangError> {
        let mut expr = self.comparison()?;

        while self.match_types(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator_token = self.previous().clone();
            let right = self.comparison()?;
            let op = match operator_token.token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
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
        let mut expr = self.primary()?;

        while self.match_types(&[TokenType::Star, TokenType::Slash]) {
            let operator_token = self.previous().clone();
            let right = self.primary()?;
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
