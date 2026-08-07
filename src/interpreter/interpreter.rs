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
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
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
                match op {
                    BinaryOp::LogicalOr => {
                        let left_val = self.evaluate(left)?;
                        if left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    BinaryOp::LogicalAnd => {
                        let left_val = self.evaluate(left)?;
                        if !left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    _ => {
                        let left_val = self.evaluate(left)?;
                        let right_val = self.evaluate(right)?;
                        match (left_val, op, right_val) {
                            (Value::Number(l), BinaryOp::Add, Value::Number(r)) => {
                                Ok(Value::Number(l + r))
                            }
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
            Expr::Input(prompt_expr) => {
                let prompt_val = self.evaluate(prompt_expr)?;
                print!("{}", prompt_val);
                use std::io::Write;
                std::io::stdout().flush().ok();

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).map_err(|e| {
                    KalawangError::RuntimeError {
                        message: format!("Failed to read user input: {}", e),
                        line: 0,
                        column: 0,
                    }
                })?;

                let trimmed = input.trim_end_matches(['\r', '\n']).to_string();
                Ok(Value::String(trimmed))
            }
            Expr::Convert { value, target_type } => {
                let val_res = self.evaluate(value)?;
                let target_str = match target_type.as_ref() {
                    Expr::Variable(name) => {
                        if let Some(val) = self.env.get(name) {
                            val.to_string()
                        } else {
                            name.clone()
                        }
                    }
                    _ => {
                        let type_val = self.evaluate(target_type)?;
                        type_val.to_string()
                    }
                };

                let target_lower = target_str.trim().to_lowercase();
                match target_lower.as_str() {
                    "int" | "numero" | "integer" | "bilang" | "ᜊᜒᜎᜅ᜔" => {
                        match val_res {
                            Value::String(s) => {
                                let trimmed = s.trim();
                                if let Ok(i) = trimmed.parse::<i64>() {
                                    Ok(Value::Number(i as f64))
                                } else if let Ok(f) = trimmed.parse::<f64>() {
                                    Ok(Value::Number(f.trunc()))
                                } else {
                                    Err(KalawangError::RuntimeError {
                                        message: format!("Cannot convert string '{}' to int", s),
                                        line: 0,
                                        column: 0,
                                    })
                                }
                            }
                            Value::Number(n) => Ok(Value::Number(n.trunc())),
                            Value::Boolean(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
                            Value::Nil => Ok(Value::Number(0.0)),
                        }
                    }
                    "float" | "decimal" | "hatian" | "ᜑᜆᜒᜀᜈ᜔" => match val_res {
                        Value::String(s) => {
                            let trimmed = s.trim();
                            if let Ok(f) = trimmed.parse::<f64>() {
                                Ok(Value::Number(f))
                            } else {
                                Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "Cannot convert string '{}' to float/decimal",
                                        s
                                    ),
                                    line: 0,
                                    column: 0,
                                })
                            }
                        }
                        Value::Number(n) => Ok(Value::Number(n)),
                        Value::Boolean(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
                        Value::Nil => Ok(Value::Number(0.0)),
                    },
                    "boolean" | "bool" | "booleano" | "tamao-mali" | "tamaomali" => match val_res {
                        Value::String(s) => {
                            let s_lower = s.trim().to_lowercase();
                            match s_lower.as_str() {
                                "true" | "tama" | "1" | "t" | "yes" | "oo" | "ᜆᜋ" => {
                                    Ok(Value::Boolean(true))
                                }
                                "false" | "mali" | "0" | "f" | "no" | "hindi" | "ᜋᜎᜒ" => {
                                    Ok(Value::Boolean(false))
                                }
                                _ => {
                                    if let Ok(b) = s_lower.parse::<bool>() {
                                        Ok(Value::Boolean(b))
                                    } else if let Ok(n) = s_lower.parse::<f64>() {
                                        Ok(Value::Boolean(n != 0.0))
                                    } else {
                                        Ok(Value::Boolean(!s_lower.is_empty()))
                                    }
                                }
                            }
                        }
                        Value::Number(n) => Ok(Value::Boolean(n != 0.0)),
                        Value::Boolean(b) => Ok(Value::Boolean(b)),
                        Value::Nil => Ok(Value::Boolean(false)),
                    },
                    _ => Err(KalawangError::RuntimeError {
                        message: format!("Unknown target datatype '{}' for conversion", target_str),
                        line: 0,
                        column: 0,
                    }),
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

    #[test]
    fn test_type_conversions() {
        let source = r#"
            si num1 = isalin("123", numero) ᜵
            si num2 = convert("456", "int") ᜵
            si num3 = ᜁᜐᜎᜒᜈ᜔("789", "ᜈᜓᜋ᜔ᜁᜇᜓ") ᜵
            si flt1 = isalin("12.34", hatian) ᜵
            si flt2 = convert("56.78", "decimal") ᜵
            si flt3 = convert("99.9", float) ᜵
            si bool1 = isalin("tama", boolean) ᜵
            si bool2 = convert("false", "boolean") ᜵
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();

        assert_eq!(
            interpreter.env.get("num1"),
            Some(crate::interpreter::value::Value::Number(123.0))
        );
        assert_eq!(
            interpreter.env.get("num2"),
            Some(crate::interpreter::value::Value::Number(456.0))
        );
        assert_eq!(
            interpreter.env.get("num3"),
            Some(crate::interpreter::value::Value::Number(789.0))
        );
        assert_eq!(
            interpreter.env.get("flt1"),
            Some(crate::interpreter::value::Value::Number(12.34))
        );
        assert_eq!(
            interpreter.env.get("flt2"),
            Some(crate::interpreter::value::Value::Number(56.78))
        );
        assert_eq!(
            interpreter.env.get("flt3"),
            Some(crate::interpreter::value::Value::Number(99.9))
        );
        assert_eq!(
            interpreter.env.get("bool1"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("bool2"),
            Some(crate::interpreter::value::Value::Boolean(false))
        );
    }

    #[test]
    fn test_logical_operators_execution() {
        let source = r#"
            si res1 = tama o mali ᜵
            si res2 = mali at tama ᜵
            si res3 = (10 > 5) at (20 == 20) ᜵
            si res4 = (5 > 10) ᜂ (3 < 8) ᜵
            si res5 = (10 > 2) and (100 == 100) ᜵
            si res6 = (1 == 2) or (2 == 2) ᜵
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
            Some(crate::interpreter::value::Value::Boolean(false))
        );
        assert_eq!(
            interpreter.env.get("res3"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("res4"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("res5"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("res6"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
    }
}
