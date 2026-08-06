use crate::diagnostics::KalawangError;
use crate::interpreter::environment::Environment;
use crate::interpreter::value::Value;
use crate::parser::ast::{BinaryOp, Expr, LiteralValue, Stmt};

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), KalawangError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), KalawangError> {
        match stmt {
            Stmt::Expression(expr) => {
                let _val = self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                println!("{}", val);
                Ok(())
            }
            Stmt::VarDeclaration { name, initializer } => {
                let val = if let Some(init) = initializer {
                    self.evaluate(init)?
                } else {
                    Value::Nil
                };
                self.env.define(name.clone(), val);
                Ok(())
            }
            Stmt::Block(stmts) => {
                for statement in stmts {
                    self.execute(statement)?;
                }
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let val = self.evaluate(condition)?;
                if val.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, KalawangError> {
        match expr {
            Expr::Literal(lit) => match lit {
                LiteralValue::Number(n) => Ok(Value::Number(*n)),
                LiteralValue::String(s) => Ok(Value::String(s.clone())),
                LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
                LiteralValue::Nil => Ok(Value::Nil),
            },
            Expr::Variable(name) => self
                .env
                .get(name)
                .ok_or_else(|| KalawangError::RuntimeError {
                    message: format!("Undefined variable '{}'", name),
                    line: 0,
                    column: 0,
                }),
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                match (left_val, op, right_val) {
                    (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), BinaryOp::Subtract, Value::Number(r)) => {
                        Ok(Value::Number(l - r))
                    }
                    (Value::Number(l), BinaryOp::Multiply, Value::Number(r)) => {
                        Ok(Value::Number(l * r))
                    }
                    (Value::Number(l), BinaryOp::Divide, Value::Number(r)) => {
                        Ok(Value::Number(l / r))
                    }
                    (l, BinaryOp::Equal, r) => Ok(Value::Boolean(l == r)),
                    (l, BinaryOp::NotEqual, r) => Ok(Value::Boolean(l != r)),
                    (Value::Number(l), BinaryOp::LessThan, Value::Number(r)) => {
                        Ok(Value::Boolean(l < r))
                    }
                    (Value::Number(l), BinaryOp::LessEqual, Value::Number(r)) => {
                        Ok(Value::Boolean(l <= r))
                    }
                    (Value::Number(l), BinaryOp::GreaterThan, Value::Number(r)) => {
                        Ok(Value::Boolean(l > r))
                    }
                    (Value::Number(l), BinaryOp::GreaterEqual, Value::Number(r)) => {
                        Ok(Value::Boolean(l >= r))
                    }
                    (Value::String(l), BinaryOp::Add, right) => {
                        Ok(Value::String(format!("{}{}", l, right)))
                    }
                    (left, BinaryOp::Add, Value::String(r)) => {
                        Ok(Value::String(format!("{}{}", left, r)))
                    }
                    _ => Err(KalawangError::RuntimeError {
                        message: "Invalid operand types for binary operation".to_string(),
                        line: 0,
                        column: 0,
                    }),
                }
            }
            Expr::Assign { name, value } => {
                let val = self.evaluate(value)?;
                if self.env.assign(name, val.clone()) {
                    Ok(val)
                } else {
                    Err(KalawangError::RuntimeError {
                        message: format!("Undefined variable '{}'", name),
                        line: 0,
                        column: 0,
                    })
                }
            }
            _ => Ok(Value::Nil),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_conditional_execution() {
        let source = r#"
            si a = 10 ᜵
            si res = 0 ᜵
            kung (a > 5) {
                res = 100 ᜵
            } kundi {
                res = 200 ᜵
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();
        assert_eq!(
            interpreter.env.get("res"),
            Some(crate::interpreter::value::Value::Number(100.0))
        );
    }

    #[test]
    fn test_else_if_execution() {
        let source = r#"
            si a = 5 ᜵
            si res = 0 ᜵
            ᜃᜓᜅ᜔ (a == 10) {
                res = 100 ᜵
            } ᜂᜃᜌ (a == 5) {
                res = 50 ᜵
            } ᜃᜓᜈ᜔ᜇᜒ {
                res = 0 ᜵
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();
        assert_eq!(
            interpreter.env.get("res"),
            Some(crate::interpreter::value::Value::Number(50.0))
        );
    }

    #[test]
    fn test_ay_and_triple_equal_operators() {
        let source = r#"
            si x = 10 ᜵
            si res1 = mali ᜵
            si res2 = mali ᜵
            si res3 = mali ᜵
            kung (x ay 10) { res1 = tama ᜵ }
            kung (x == 10) { res2 = tama ᜵ }
            kung (x === 10) { res3 = tama ᜵ }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();
        assert_eq!(
            interpreter.env.get("res1"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("res2"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("res3"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
    }
}
