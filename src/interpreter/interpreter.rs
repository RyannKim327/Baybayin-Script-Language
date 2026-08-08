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
            Expr::Unary { op, right } => {
                let right_val = self.evaluate(right)?;
                match (op, right_val) {
                    (crate::parser::ast::UnaryOp::Negate, Value::Number(n)) => {
                        Ok(Value::Number(-n))
                    }
                    (crate::parser::ast::UnaryOp::Not, val) => {
                        Ok(Value::Boolean(!val.is_truthy()))
                    }
                    _ => Err(KalawangError::RuntimeError {
                        message: "Invalid operand for unary operation".to_string(),
                        line: 0,
                        column: 0,
                    }),
                }
            }
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
                            (Value::Array(mut l), BinaryOp::Add, Value::Array(r)) => {
                                l.extend(r);
                                Ok(Value::Array(l))
                            }
                            (Value::Array(mut l), BinaryOp::Add, r) => {
                                l.push(r);
                                Ok(Value::Array(l))
                            }
                            (l, BinaryOp::Add, Value::Array(mut r)) => {
                                r.insert(0, l);
                                Ok(Value::Array(r))
                            }
                            (Value::Array(l), BinaryOp::Multiply, Value::Number(n)) => {
                                let times = if n < 0.0 { 0 } else { n as usize };
                                let mut result = Vec::with_capacity(l.len() * times);
                                for _ in 0..times {
                                    result.extend(l.clone());
                                }
                                Ok(Value::Array(result))
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
            Expr::Array(elements) => {
                let mut values = Vec::with_capacity(elements.len());
                for elem in elements {
                    values.push(self.evaluate(elem)?);
                }
                Ok(Value::Array(values))
            }
            Expr::Index { target, index } => {
                let target_val = self.evaluate(target)?;
                let index_val = self.evaluate(index)?;

                let idx_num = match index_val {
                    Value::Number(n) => n as i64,
                    _ => {
                        return Err(KalawangError::RuntimeError {
                            message: format!("Array index must be a number, found {}", index_val),
                            line: 0,
                            column: 0,
                        })
                    }
                };

                match target_val {
                    Value::Array(arr) => {
                        let len = arr.len() as i64;
                        let actual_idx = if idx_num < 0 {
                            len + idx_num
                        } else {
                            idx_num
                        };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(KalawangError::RuntimeError {
                                message: format!(
                                    "Index out of bounds: index {} on array of length {}",
                                    idx_num, len
                                ),
                                line: 0,
                                column: 0,
                            });
                        }

                        Ok(arr[actual_idx as usize].clone())
                    }
                    Value::String(s) => {
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i64;
                        let actual_idx = if idx_num < 0 {
                            len + idx_num
                        } else {
                            idx_num
                        };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(KalawangError::RuntimeError {
                                message: format!(
                                    "Index out of bounds: index {} on string of length {}",
                                    idx_num, len
                                ),
                                line: 0,
                                column: 0,
                            });
                        }

                        Ok(Value::String(chars[actual_idx as usize].to_string()))
                    }
                    _ => Err(KalawangError::RuntimeError {
                        message: format!("Cannot index into non-array/string value '{}'", target_val),
                        line: 0,
                        column: 0,
                    }),
                }
            }
            Expr::IndexAssign {
                target,
                index,
                value,
            } => {
                let val = self.evaluate(value)?;
                let idx_val = self.evaluate(index)?;
                let idx_num = match idx_val {
                    Value::Number(n) => n as i64,
                    _ => {
                        return Err(KalawangError::RuntimeError {
                            message: format!("Array index must be a number, found {}", idx_val),
                            line: 0,
                            column: 0,
                        })
                    }
                };

                if let Expr::Variable(name) = target.as_ref() {
                    if let Some(current_val) = self.env.get_mut(name) {
                        match current_val {
                            Value::Array(arr) => {
                                let len = arr.len() as i64;
                                let actual_idx = if idx_num < 0 {
                                    len + idx_num
                                } else {
                                    idx_num
                                };

                                if actual_idx < 0 || actual_idx >= len {
                                    return Err(KalawangError::RuntimeError {
                                        message: format!(
                                            "Index out of bounds on assignment: index {} on array of length {}",
                                            idx_num, len
                                        ),
                                        line: 0,
                                        column: 0,
                                    });
                                }

                                arr[actual_idx as usize] = val.clone();
                                Ok(val)
                            }
                            _ => Err(KalawangError::RuntimeError {
                                message: format!("Cannot assign index to non-array variable '{}'", name),
                                line: 0,
                                column: 0,
                            }),
                        }
                    } else {
                        Err(KalawangError::RuntimeError {
                            message: format!("Undefined variable '{}'", name),
                            line: 0,
                            column: 0,
                        })
                    }
                } else {
                    Err(KalawangError::RuntimeError {
                        message: "Invalid target for index assignment".to_string(),
                        line: 0,
                        column: 0,
                    })
                }
            }
            Expr::Call { callee, arguments } => {
                let mut evaluated_args = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    evaluated_args.push(self.evaluate(arg)?);
                }

                if let Expr::Variable(name) = callee.as_ref() {
                    let name_lower = name.to_lowercase();
                    match name_lower.as_str() {
                        "mga" | "ᜋ᜔ᜄ" | "ᜋᜄ" | "ᜋᜅ" | "array" | "list" => {
                            Ok(Value::Array(evaluated_args))
                        }
                        "bilang" | "ᜊᜒᜎᜅ᜔" => {
                            if evaluated_args.len() == 1 {
                                match &evaluated_args[0] {
                                    Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                                    Value::String(s) => {
                                        Ok(Value::Number(s.chars().count() as f64))
                                    }
                                    _ => Ok(Value::Array(evaluated_args)),
                                }
                            } else {
                                Ok(Value::Array(evaluated_args))
                            }
                        }
                        "haba" | "ᜑᜊ" | "sukat" | "ᜐᜓᜃᜆ᜔" | "length" | "len" | "count" | "size" => {
                            if let Some(first) = evaluated_args.first() {
                                match first {
                                    Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                                    Value::String(s) => {
                                        Ok(Value::Number(s.chars().count() as f64))
                                    }
                                    _ => Err(KalawangError::RuntimeError {
                                        message: format!(
                                            "'{}' expects an array or string argument",
                                            name
                                        ),
                                        line: 0,
                                        column: 0,
                                    }),
                                }
                            } else {
                                Err(KalawangError::RuntimeError {
                                    message: format!("'{}' expects 1 argument", name),
                                    line: 0,
                                    column: 0,
                                })
                            }
                        }
                        "dagdagan" | "ᜇᜄ᜔ᜇᜄᜈ᜔" | "idagdag" | "ᜁᜇᜄ᜔ᜇᜄ᜔" | "isuksok"
                        | "ᜁᜐᜓᜃ᜔ᜐᜓᜃ᜔" | "push" | "append" | "add" => {
                            if evaluated_args.len() < 2 {
                                return Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "'{}' expects at least 2 arguments (array, item)",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            let item = evaluated_args[1].clone();
                            if let Some(Expr::Variable(var_name)) = arguments.first() {
                                if let Some(val) = self.env.get_mut(var_name) {
                                    if let Value::Array(arr) = val {
                                        arr.push(item.clone());
                                        let updated = arr.clone();
                                        return Ok(Value::Array(updated));
                                    }
                                }
                            }
                            match &evaluated_args[0] {
                                Value::Array(arr) => {
                                    let mut new_arr = arr.clone();
                                    new_arr.push(item);
                                    Ok(Value::Array(new_arr))
                                }
                                _ => Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "First argument of '{}' must be an array",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                }),
                            }
                        }
                        "alisin" | "ᜀᜎᜒᜐᜒᜈ᜔" | "tanggalin" | "ᜆᜅ᜔ᜄᜎᜒᜈ᜔" | "pop" | "remove" | "delete" => {
                            if evaluated_args.is_empty() {
                                return Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "'{}' expects at least 1 argument (array)",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            if let Some(Expr::Variable(var_name)) = arguments.first() {
                                if let Some(val) = self.env.get_mut(var_name) {
                                    if let Value::Array(arr) = val {
                                        if evaluated_args.len() >= 2 {
                                            if let Value::Number(idx) = evaluated_args[1] {
                                                let i = idx as i64;
                                                let len = arr.len() as i64;
                                                let actual_i = if i < 0 { len + i } else { i };
                                                if actual_i >= 0 && actual_i < len {
                                                    return Ok(arr.remove(actual_i as usize));
                                                } else {
                                                    return Err(KalawangError::RuntimeError {
                                                        message: format!(
                                                            "Index {} out of bounds for remove",
                                                            idx
                                                        ),
                                                        line: 0,
                                                        column: 0,
                                                    });
                                                }
                                            }
                                        } else if let Some(removed) = arr.pop() {
                                            return Ok(removed);
                                        } else {
                                            return Ok(Value::Nil);
                                        }
                                    }
                                }
                            }
                            match &evaluated_args[0] {
                                Value::Array(arr) => {
                                    let mut new_arr = arr.clone();
                                    if evaluated_args.len() >= 2 {
                                        if let Value::Number(idx) = evaluated_args[1] {
                                            let i = idx as i64;
                                            let len = new_arr.len() as i64;
                                            let actual_i = if i < 0 { len + i } else { i };
                                            if actual_i >= 0 && actual_i < len {
                                                return Ok(new_arr.remove(actual_i as usize));
                                            } else {
                                                return Err(KalawangError::RuntimeError {
                                                    message: format!(
                                                        "Index {} out of bounds for remove",
                                                        idx
                                                    ),
                                                    line: 0,
                                                    column: 0,
                                                });
                                            }
                                        }
                                    }
                                    Ok(new_arr.pop().unwrap_or(Value::Nil))
                                }
                                _ => Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "First argument of '{}' must be an array",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                }),
                            }
                        }
                        "nandyan" | "ᜈᜈ᜔ᜇ᜔ᜌᜈ᜔" | "mayroon" | "ᜋᜌ᜔ᜇᜓᜂᜈ᜔" | "meron"
                        | "contains" | "includes" => {
                            if evaluated_args.len() < 2 {
                                return Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "'{}' expects 2 arguments (array/string, item)",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            match &evaluated_args[0] {
                                Value::Array(arr) => {
                                    Ok(Value::Boolean(arr.contains(&evaluated_args[1])))
                                }
                                Value::String(s) => {
                                    let search = evaluated_args[1].to_string();
                                    Ok(Value::Boolean(s.contains(&search)))
                                }
                                _ => Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "First argument of '{}' must be an array or string",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                }),
                            }
                        }
                        "pagsamahin" | "ᜉᜄ᜔ᜐᜋᜑᜒᜈ᜔" | "join" => {
                            if evaluated_args.is_empty() {
                                return Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "'{}' expects at least 1 argument (array)",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            let delim = if evaluated_args.len() >= 2 {
                                evaluated_args[1].to_string()
                            } else {
                                ", ".to_string()
                            };
                            match &evaluated_args[0] {
                                Value::Array(arr) => {
                                    let str_items: Vec<String> =
                                        arr.iter().map(|item| item.to_string()).collect();
                                    Ok(Value::String(str_items.join(&delim)))
                                }
                                _ => Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "First argument of '{}' must be an array",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                }),
                            }
                        }
                        "baligtad" | "ᜊᜎᜒᜄ᜔ᜆᜇ᜔" | "reverse" => {
                            if evaluated_args.is_empty() {
                                return Err(KalawangError::RuntimeError {
                                    message: format!("'{}' expects 1 argument (array)", name),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            if let Some(Expr::Variable(var_name)) = arguments.first() {
                                if let Some(val) = self.env.get_mut(var_name) {
                                    if let Value::Array(arr) = val {
                                        arr.reverse();
                                        return Ok(Value::Array(arr.clone()));
                                    }
                                }
                            }
                            match &evaluated_args[0] {
                                Value::Array(arr) => {
                                    let mut rev = arr.clone();
                                    rev.reverse();
                                    Ok(Value::Array(rev))
                                }
                                Value::String(s) => {
                                    let rev: String = s.chars().rev().collect();
                                    Ok(Value::String(rev))
                                }
                                _ => Err(KalawangError::RuntimeError {
                                    message: format!(
                                        "Argument of '{}' must be an array or string",
                                        name
                                    ),
                                    line: 0,
                                    column: 0,
                                }),
                            }
                        }
                        _ => Err(KalawangError::RuntimeError {
                            message: format!("Undefined function or array constructor '{}'", name),
                            line: 0,
                            column: 0,
                        }),
                    }
                } else {
                    Err(KalawangError::RuntimeError {
                        message: "Can only call functions and identifiers".to_string(),
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
                    "int" | "numero" | "integer" | "bilang" | "ᜊᜒᜎᜅ᜔" | "ᜈᜓᜋᜒᜇᜓ"
                    | "ᜈᜓᜋ᜔ᜁᜇᜓ" => match val_res {
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
                        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
                        Value::Nil => Ok(Value::Number(0.0)),
                    },
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
                        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
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
                        Value::Array(arr) => Ok(Value::Boolean(!arr.is_empty())),
                        Value::Nil => Ok(Value::Boolean(false)),
                    },
                    "string" | "str" | "salita" | "ᜐᜎᜒᜆ" => Ok(Value::String(val_res.to_string())),
                    "array" | "list" | "mga" | "ᜋ᜔ᜄ" | "ᜋᜄ" | "ᜋᜅ" => match val_res {
                        Value::Array(arr) => Ok(Value::Array(arr)),
                        Value::String(s) => {
                            let trimmed = s.trim();
                            let inner = if (trimmed.starts_with('[') && trimmed.ends_with(']'))
                                || (trimmed.starts_with('(') && trimmed.ends_with(')'))
                            {
                                &trimmed[1..trimmed.len() - 1]
                            } else {
                                trimmed
                            };
                            if inner.trim().is_empty() {
                                Ok(Value::Array(Vec::new()))
                            } else {
                                let items: Vec<Value> = inner
                                    .split(',')
                                    .map(|item| {
                                        let item_trim = item.trim();
                                        if let Ok(n) = item_trim.parse::<f64>() {
                                            Value::Number(n)
                                        } else if item_trim.starts_with('"')
                                            && item_trim.ends_with('"')
                                            && item_trim.len() >= 2
                                        {
                                            Value::String(
                                                item_trim[1..item_trim.len() - 1].to_string(),
                                            )
                                        } else if item_trim == "tama"
                                            || item_trim == "true"
                                            || item_trim == "ᜆᜋ"
                                        {
                                            Value::Boolean(true)
                                        } else if item_trim == "mali"
                                            || item_trim == "false"
                                            || item_trim == "ᜋᜎᜒ"
                                        {
                                            Value::Boolean(false)
                                        } else {
                                            Value::String(item_trim.to_string())
                                        }
                                    })
                                    .collect();
                                Ok(Value::Array(items))
                            }
                        }
                        Value::Number(n) => Ok(Value::Array(vec![Value::Number(n)])),
                        Value::Boolean(b) => Ok(Value::Array(vec![Value::Boolean(b)])),
                        Value::Nil => Ok(Value::Array(Vec::new())),
                    },
                    _ => Err(KalawangError::RuntimeError {
                        message: format!("Unknown target datatype '{}' for conversion", target_str),
                        line: 0,
                        column: 0,
                    }),
                }
            }
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

    #[test]
    fn test_array_creation_and_indexing() {
        let source = r#"
            si a ay [10, 20, 30] ᜵
            si unang = a[0] ᜵
            si pangalawa = a[1] ᜵
            si huli = a[-1] ᜵
            a[0] = 99 ᜵
            si bago_unang = a[0] ᜵
            si len = haba(a) ᜵
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();

        assert_eq!(
            interpreter.env.get("unang"),
            Some(crate::interpreter::value::Value::Number(10.0))
        );
        assert_eq!(
            interpreter.env.get("pangalawa"),
            Some(crate::interpreter::value::Value::Number(20.0))
        );
        assert_eq!(
            interpreter.env.get("huli"),
            Some(crate::interpreter::value::Value::Number(30.0))
        );
        assert_eq!(
            interpreter.env.get("bago_unang"),
            Some(crate::interpreter::value::Value::Number(99.0))
        );
        assert_eq!(
            interpreter.env.get("len"),
            Some(crate::interpreter::value::Value::Number(3.0))
        );
    }

    #[test]
    fn test_array_keywords_and_methods() {
        let source = r#"
            si x ay mga(1, 2, 3) ᜵
            si y ay bilang(4, 5, 6) ᜵
            si sum_arr = x + y ᜵
            si may_dalawa = nandyan(x, 2) ᜵
            si may_pito = nandyan(x, 7) ᜵
            dagdag(x, 4) ᜵
            si haba_x = sukat(x) ᜵
            si inalis = alis(x) ᜵
            si str_join = pagsamahin(y, "-") ᜵
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();

        assert_eq!(
            interpreter.env.get("sum_arr"),
            Some(crate::interpreter::value::Value::Array(vec![
                crate::interpreter::value::Value::Number(1.0),
                crate::interpreter::value::Value::Number(2.0),
                crate::interpreter::value::Value::Number(3.0),
                crate::interpreter::value::Value::Number(4.0),
                crate::interpreter::value::Value::Number(5.0),
                crate::interpreter::value::Value::Number(6.0),
            ]))
        );
        assert_eq!(
            interpreter.env.get("may_dalawa"),
            Some(crate::interpreter::value::Value::Boolean(true))
        );
        assert_eq!(
            interpreter.env.get("may_pito"),
            Some(crate::interpreter::value::Value::Boolean(false))
        );
        assert_eq!(
            interpreter.env.get("haba_x"),
            Some(crate::interpreter::value::Value::Number(4.0))
        );
        assert_eq!(
            interpreter.env.get("inalis"),
            Some(crate::interpreter::value::Value::Number(4.0))
        );
        assert_eq!(
            interpreter.env.get("str_join"),
            Some(crate::interpreter::value::Value::String("4-5-6".to_string()))
        );
    }

    #[test]
    fn test_baybayin_array_syntax() {
        let source = r#"
            ᜐᜒ a ᜀᜌ᜔ [10, 20, 30] ᜵
            ᜐᜒ b ᜀᜌ᜔ ᜋ᜔ᜄ(1, 2, 3) ᜵
            ᜐᜒ c ᜀᜌ᜔ ᜊᜒᜎᜅ᜔(4, 5) ᜵
            ᜐᜒ len_a = ᜑᜊ(a) ᜵
            ᜐᜒ len_b = ᜐᜓᜃᜆ᜔(b) ᜵
            ᜐᜒ elem = a[0] ᜵
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&statements).unwrap();

        assert_eq!(
            interpreter.env.get("len_a"),
            Some(crate::interpreter::value::Value::Number(3.0))
        );
        assert_eq!(
            interpreter.env.get("len_b"),
            Some(crate::interpreter::value::Value::Number(3.0))
        );
        assert_eq!(
            interpreter.env.get("elem"),
            Some(crate::interpreter::value::Value::Number(10.0))
        );
    }
}
