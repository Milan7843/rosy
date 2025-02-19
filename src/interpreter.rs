use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::RecExpr;

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
        Value::Function(_) => return String::from("function"),
    }
}

struct Binding {
    name: String,
    value: Value,
}

struct Scope {
    bindings: Vec<Binding>,
}

struct Environment {
    scopes: Vec<Scope>,
}

enum InterpretationResult {
    Return {
        value: Option<Value>
    },
    Break,
    Empty,
}

fn interpret(base_expressions: Vec<BaseExpr>) -> Result<String, String> {}

fn interpret_rec(base_expressions: &[BaseExpr]) -> Result<String, String> {}

fn interpret_base_expr(base_expression: Vec<BaseExpr>) -> Result<InterpretationResult, String> {

}

fn interpret_expr(expr: RecExpr, env: &mut Environment) -> Result<Option<Value>, String> {
    match expr {
        RecExpr::Variable { name } => match find_in_env(&name, env) {
            Some(value) => return Ok(Some(value)),
            None => return Err(String::from("Variable not found: " + name)),
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
                    let result = left_num + right_num;
                    return Ok(Some(Value::String(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(String::from(
                        "Cannot apply operator + on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
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
                    return Err(String::from(
                        "Cannot apply operator - on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
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
                (Some(Value::String(left_str)), Some(Value::String(right_str))) => {
                    let result = left_num + right_num;
                    return Ok(Some(Value::String(result)));
                }
                (Some(left_value), Some(right_value)) => {
                    return Err(String::from(
                        "Cannot apply operator * on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
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
                    return Err(String::from(
                        "Cannot apply operator / on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator / on empty")),
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
                    return Err(String::from(
                        "Cannot apply operator AND on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
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
                    return Err(String::from(
                        "Cannot apply operator OR on types "
                            + value_to_string(&left_value)
                            + " and "
                            + value_to_string(&right_value),
                    ))
                }
                _ => return Err(String::from("Attempted to apply operator OR on empty")),
            }
        }

        RecExpr::FunctionCall { function_name, args } => {
            let env_variable = match find_in_env(&function_name, env) {
                Ok(env_variable) => env_variable,
                Err(e) => return Err(e),
            };

            match env_variable {
                Value::Function { name, args, body } => {

                    // Run all sub statements
                    for base_expression in body {
                        let interp_result = match interpret_base_expr(base_expression) {
                            Ok(result) => result,
                            Err(e) => return Err(e),
                        };

                        match interp_result {
                            InterpretationResult::Return{ value: return_value } => {
                                return Ok(Some(return_value));
                            }
                            InterpretationResult::Break => {
                                return Err(String::from("Cannot break out of a funcion"));
                            }
                            InterpretationResult::Empty => {}
                        }
                    }
                }
                other => return Err(String::from("Expected function, found " + value_to_string(&other) + " for variable " + function_name)),
            }
        }
    }
}

fn find_in_env(name: &String, env: &Environment) -> Option<&Value> {
    for scope in env.scopes.iter().rev() {
        match find_in_scope(name, scope) {
            Some(value) => return Some(value),
            None => {}
        }
    }
    return None;
}

fn find_in_scope(name: &String, scope: &Scope) -> Option<&Value> {
    for binding in scope.bindings.iter() {
        if binding.name == name {
            return Some(&binding.value);
        }
    }

    return None;
}
