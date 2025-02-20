use std::fmt::format;

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
        Value::Number(value) => return format!("{value}"),
        Value::Bool(value) => return format!("{value}"),
        Value::String(value) => return format!("{value}"),
        Value::Function { .. } => return String::from("function"),
    }
}

fn value_type_to_string(value: &Value) -> String {
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

pub fn interpret(base_expressions: Vec<BaseExpr>) -> Result<String, String> {
    let mut env: Environment = Vec::new();

    env.push(Vec::new());

    for base_expression in base_expressions {
        match interpret_base_expr(base_expression, &mut env) {
            Ok(InterpretationResult::Return { value }) => {
                match value {
                    Some(value) => print!("{}", value_to_string(&value)),
                    None => {}
                };
            }
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    return Ok(String::from("Interpretation successful"));
}

fn interpret_base_expr(
    base_expression: BaseExpr,
    env: &mut Environment,
) -> Result<InterpretationResult, String> {
    match base_expression {
        BaseExpr::Simple { expr } => {
            interpret_expr(expr, env);
            return Ok(InterpretationResult::Empty);
        }
        BaseExpr::VariableAssignment { var_name, expr } => {
            let value = match interpret_expr(expr, env) {
                Ok(right) => match right {
                    Some(value) => value,
                    None => return Err(String::from("Cannot assign to empty")),
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            let scope = env.last_mut().unwrap();
            scope.push(Binding {
                name: var_name,
                value: value,
            });
            return Ok(InterpretationResult::Empty);
        }
        BaseExpr::IfStatement {
            condition,
            body,
            else_statement,
        } => {
            let condition = match interpret_expr(condition, env) {
                Ok(Some(Value::Bool(condition))) => condition,
                Ok(Some(other_value)) => {
                    return Err(format!(
                        "Cannot use {} as a condition for an if statement",
                        value_type_to_string(&other_value)
                    ))
                }
                Ok(None) => {
                    return Err(format!(
                        "Cannot use empty as the condition for an if statement"
                    ))
                }
                Err(e) => return Err(e),
            };

            // If the condition is false, use the else option
            if !condition {
                let else_statement_real = match else_statement {
                    Some(expr) => *expr,
                    None => return Ok(InterpretationResult::Empty),
                };

                return interpret_base_expr(else_statement_real, env);
            }

            for base_expression in body {
                let interp_result = match interpret_base_expr(base_expression, env) {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result {
                    InterpretationResult::Return {
                        value: return_value,
                    } => {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break => {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty => {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr::ElseIfStatement {
            condition,
            body,
            else_statement,
        } => {
            let condition = match interpret_expr(condition, env) {
                Ok(Some(Value::Bool(condition))) => condition,
                Ok(Some(other_value)) => {
                    return Err(format!(
                        "Cannot use {} as a condition for an if statement",
                        value_type_to_string(&other_value)
                    ))
                }
                Ok(None) => {
                    return Err(format!(
                        "Cannot use empty as the condition for an if statement"
                    ))
                }
                Err(e) => return Err(e),
            };

            // If the condition is false, use the else option
            if !condition {
                let else_statement_real = match else_statement {
                    Some(expr) => *expr,
                    None => return Ok(InterpretationResult::Empty),
                };

                return interpret_base_expr(else_statement_real, env);
            }

            for base_expression in body {
                let interp_result = match interpret_base_expr(base_expression, env) {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result {
                    InterpretationResult::Return {
                        value: return_value,
                    } => {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break => {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty => {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr::ElseStatement { body } => {
            for base_expression in body {
                let interp_result = match interpret_base_expr(base_expression, env) {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result {
                    InterpretationResult::Return {
                        value: return_value,
                    } => {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break => {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty => {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr::PlusEqualsStatement { var_name, expr } => {
            let value = match interpret_expr(expr, env) {
                Ok(right) => match right {
                    Some(value) => value,
                    None => return Err(String::from("Cannot assign to empty")),
                },
                Err(e) => return Err(e),
            };

            let current_value = match find_in_env(&var_name, env) {
                Some(value) => value,
                None => return Err(format!("Variable {} not found", var_name)),
            };

            let new_value = match add(&Some(current_value), &Some(value)) {
                Ok(new_value) => match new_value {
                    Some(value) => value,
                    None => return Err(String::from("Cannot assign to empty")),
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            let scope = env.last_mut().unwrap();
            match update_in_scope(&new_value, &var_name, scope) {
                true => {}
                false => {
                    return Err(format!("Variable {} not found", var_name));
                }
            }
            return Ok(InterpretationResult::Empty);
        }

        BaseExpr::FunctionDefinition {
            fun_name,
            args,
            body,
        } => {
            let function = Value::Function {
                name: fun_name.clone(),
                args,
                body,
            };

            let scope = env.last_mut().unwrap();
            scope.push(Binding {
                name: fun_name,
                value: function,
            });

            return Ok(InterpretationResult::Empty);
        }

        BaseExpr::Return { return_value } => {
            let return_value = match return_value {
                Some(expr) => expr,
                None => return Ok(InterpretationResult::Return { value: None }),
            };

            let return_value = match interpret_expr(return_value, env) {
                Ok(Some(value)) => value,
                Ok(None) => return Ok(InterpretationResult::Return { value: None }),
                Err(e) => return Err(e),
            };

            return Ok(InterpretationResult::Return {
                value: Some(return_value),
            });
        }

        BaseExpr::Break => {
            return Ok(InterpretationResult::Break);
        }

        BaseExpr::ForLoop {
            var_name,
            until,
            body,
        } => {
            let until = match interpret_expr(until, env) {
                Ok(Some(Value::Number(until))) => until,
                Ok(Some(other_value)) => {
                    return Err(format!(
                        "Cannot use {} as a condition for a for loop",
                        value_type_to_string(&other_value)
                    ))
                }
                Ok(None) => {
                    return Err(format!("Cannot use empty as the condition for a for loop"))
                }
                Err(e) => return Err(e),
            };

            let scope = env.last_mut().unwrap();

            scope.push(Binding {
                name: var_name.clone(),
                value: Value::Number(0),
            });

            for i in 0..until {
                let scope = env.last_mut().unwrap();
                match update_in_scope(&Value::Number(i), &var_name, scope) {
                    true => {}
                    false => {
                        return Err(format!("Variable {} not found", var_name));
                    }
                }
                /*
                for base_expression in body.iter() {
                    let interp_result = match interpret_base_expr(base_expression, env) {
                        Ok(result) => result,
                        Err(e) => return Err(e),
                    };

                    match interp_result {
                        InterpretationResult::Return {
                            value: return_value,
                        } => {
                            return Ok(InterpretationResult::Return { value: return_value });
                        }
                        InterpretationResult::Break => {
                            return Ok(InterpretationResult::Break);
                        }
                        InterpretationResult::Empty => {}
                    }
                }
                */
            }

            return Ok(InterpretationResult::Empty);
        }
    }
}

fn add(left: &Option<Value>, right: &Option<Value>) -> Result<Option<Value>, String> {
    match (left, right) {
        (Some(Value::Number(left)), Some(Value::Number(right))) => {
            let result = left + right;
            return Ok(Some(Value::Number(result)));
        }
        (Some(Value::String(left)), Some(Value::String(right))) => {
            let result = left.clone() + right;
            return Ok(Some(Value::String(result)));
        }
        (Some(left), Some(right)) => {
            return Err(format!(
                "Cannot apply operator + on types {} and {}",
                value_type_to_string(left),
                value_type_to_string(right)
            ));
        }
        _ => return Err(String::from("Attempted to apply operator + on empty")),
    }
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

            return add(&left_value, &right_value);
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                        value_type_to_string(&left_value),
                        value_type_to_string(&right_value)
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
                    return Err(format!("Variable {} not found", function_name));
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
                        value_type_to_string(&other),
                        function_name
                    ));
                }
            }
        }

        RecExpr::Assign {
            variable_name,
            right,
        } => {
            let value = match interpret_expr(*right, env) {
                Ok(right) => match right {
                    Some(value) => value,
                    None => return Err(String::from("Cannot assign to empty")),
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

fn update_in_env(value: &Value, name: &String, env: &mut Environment) -> bool {
    for scope in env.iter_mut().rev() {
        if update_in_scope(value, name, scope) {
            return true;
        }
    }
    return false;
}

fn update_in_scope(value: &Value, name: &String, scope: &mut Scope) -> bool {
    for binding in scope.iter_mut() {
        if binding.name == *name {
            binding.value = value.clone();
            return true;
        }
    }
    return false;
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
