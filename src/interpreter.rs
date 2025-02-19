use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::RecExpr;

#[derive(Clone)]
enum Value {
    Number(i32),
    Bool(bool),
    String(String),
    Function {
        name: String,
        args: Vec<String>,
        body: Vec<BaseExpr>,
    },
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Number(_) => return String::from("integer"),
        Value::Bool(_) => return String::from("boolean"),
        Value::String(_) => return String::from("string"),
        Value::Function { .. } => return String::from("function"),
    }
}

struct Binding {
    name: String,
    value: Value,
}

type Scope = Vec<Binding>;

type Environment = Vec<Scope>;

enum InterpretationResult {
    Return { value: Option<Value> },
    Break,
    Empty,
}

fn interpret(base_expressions: Vec<BaseExpr>) -> Result<String, String> {}

fn interpret_rec(base_expressions: &[BaseExpr]) -> Result<String, String> {}

fn interpret_base_expr(
    base_expression: BaseExpr,
    env: &mut Environment,
) -> Result<InterpretationResult, String> {
}

fn interpret_expr(expr: RecExpr, env: &mut Environment) -> Result<Option<Value>, String> {
    match expr {
        RecExpr::Variable { name } => match find_in_env(&name, env) {
            Some(value) => return Ok(Some(value)),
            None => return Err(format!("Variable not found: {}", name)),
        },
        RecExpr::Number { number } => return Ok(Some(Value::Number(number))),
        RecExpr::Boolean { value } => return Ok(Some(Value::Bool(value))),
        RecExpr::String { value } => return Ok(Some(Value::String(value))),

        // Arithmetic
        RecExpr::Add { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) => {
                    let result = left_num + right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(Value::String(left_str)), Some(Value::String(right_str))) => {
                    let result = left_str + &right_str;
                    return Ok(Some(Value::String(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator + on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator + on empty")),
            }
        }
        RecExpr::Subtract { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) => {
                    let result = left_num - right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator - on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator - on empty")),
            }
        }
        RecExpr::Multiply { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) => {
                    let result = left_num * right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator * on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator * on empty")),
            }
        }
        RecExpr::Divide { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) => {
                    let result = left_num / right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator / on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator / on empty")),
            }
        }
        RecExpr::Power { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left)), Some(Value::Number(right))) => {
                    if right < 0 {
                        return Err(String::from("Cannot do power with power less than 0"));
                    }

                    let result = i32::pow(left, right as u32);
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator ^ on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator ^ on empty")),
            }
        }
        RecExpr::Minus { right } => {
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match right_value {
                Some(Value::Number(value)) => {
                    let result = -value;
                    return Ok(Some(Value::Number(result)));
                }
                Some(value) => {
                    return Err(format!(
                        "Cannot apply operator - on type {}",
                        value_to_string(&value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator - on empty")),
            }
        }

        RecExpr::Equals { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Number(left)), Some(Value::Number(right))) => {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::Bool(left)), Some(Value::Bool(right))) => {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::String(left)), Some(Value::String(right))) => {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator == on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator == on empty")),
            }
        }

        // Boolean operators
        RecExpr::And { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Bool(left)), Some(Value::Bool(right))) => {
                    let result = left && right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator AND on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator AND on empty")),
            }
        }
        RecExpr::Or { left, right } => {
            let left_value = match interpret_expr(*left, env) {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(*right, env) {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value) {
                (Some(Value::Bool(left)), Some(Value::Bool(right))) => {
                    let result = left || right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(format!(
                        "Cannot apply operator OR on types {} and {}",
                        value_to_string(&left_value),
                        value_to_string(&right_value)
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator OR on empty")),
            }
        }

        RecExpr::FunctionCall {
            function_name,
            args,
        } => {
            let env_variable = match find_in_env(&function_name, env) {
                Some(env_variable) => env_variable,
                None => {
                    return Err(format!(
                        "Variable {} not found",
                        function_name
                    ));
                }
            };

            // We also need all values that we will pass
            let mut arg_values = Vec::new();
            for arg in args {
                match interpret_expr(arg, env) {
                    Ok(Some(value)) => {
                        arg_values.push(value);
                    }
                    Ok(None) => return Err(String::from("Attempted to pass Empty to a function")),
                    Err(e) => return Err(e),
                }
            }

            match env_variable {
                Value::Function { name, args, body } => {
                    // Matching the arguments values with the argument names
                    let mut function_scope: Scope = Vec::new();

                    if args.len() != arg_values.len() {
                        return Err(format!(
                            "Expected {} arguments, but got {}",
                            args.len(),
                            arg_values.len()
                        ));
                    }

                    for (name, value) in args.iter().zip(arg_values.iter()) {
                        function_scope.push(Binding {
                            name: name.clone(),
                            value: value.clone(),
                        });
                    }

                    // Adding this scope to the environment
                    env.push(function_scope);

                    // Run all sub statements
                    for base_expression in body {
                        let interp_result = match interpret_base_expr(base_expression, env) {
                            Ok(result) => result,
                            Err(e) => return Err(e),
                        };

                        match interp_result {
                            InterpretationResult::Return {
                                value: return_value,
                            } => {
                                return Ok(return_value);
                            }
                            InterpretationResult::Break => {
                                return Err(String::from("Cannot break out of a funcion"));
                            }
                            InterpretationResult::Empty => {}
                        }
                    }

                    // Removing the scope
                    env.pop();

                    // No return statement was found, thus return empty
                    return Ok(None);
                }
                other => {
                    return Err(format!(
                        "Expected function, found {} for variable {}",
                        value_to_string(&other),
                        function_name
                    ));
                }
            }
        }

        RecExpr::Assign { variable_name, right } => {
            let value = match interpret_expr(*right, env) {
                Ok(right) => {
                    match right {
                        Some(value) => value,
                        None => return Err(String::from("Cannot assign to empty")),
                    }
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            let scope = env.last_mut().unwrap();
            scope.push(Binding {
                name: variable_name,
                value: value,
            });

            return Ok(None);
        }

        RecExpr::Access { object, variable } => return Err(String::from("Not implemented")),
    }
}

fn find_in_env(name: &String, env: &Environment) -> Option<Value> {
    for scope in env.iter().rev() {
        match find_in_scope(name, scope) {
            Some(value) => return Some(value),
            None => {}
        }
    }
    return None;
}

fn find_in_scope(name: &String, scope: &Scope) -> Option<Value> {
    for binding in scope.iter() {
        if binding.name == *name {
            return Some(binding.value.clone());
        }
    }

    return None;
}
