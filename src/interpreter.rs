use std::fmt::format;

use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;

#[derive(Clone)]
enum StandardFunction {
    Print,
    PrintLine,
}

fn add_default_functions_to_env(env: &mut Environment) {
    let scope = env.last_mut().unwrap();

    scope.push(Binding {
        name: String::from("print"),
        value: Value::StandardFunction(StandardFunction::Print),
    });

    scope.push(Binding {
        name: String::from("println"),
        value: Value::StandardFunction(StandardFunction::PrintLine),
    });
}

#[derive(Clone)]
enum Value {
    Number(i32),
    Bool(bool),
    String(String),
    Function {
        name: String,
        args: Vec<String>,
        body: Vec<BaseExpr<()>>,
    },
    StandardFunction(StandardFunction),
    List(Vec<Value>),
}

fn value_to_string(value: &Value) -> String {
    match value
    {
        Value::Number(value) => return format!("{value}"),
        Value::Bool(value) => return format!("{value}"),
        Value::String(value) => return format!("{value}"),
        Value::Function { name, .. } => return format!("function {}", name),
        Value::StandardFunction(_) => return String::from("standard function"),
        Value::List(values) =>
        {
            let mut result = String::from("[");
            for (i, value) in values.iter().enumerate()
            {
                result.push_str(&value_to_string(value));
                if i != values.len() - 1
                {
                    result.push_str(", ");
                }
            }
            result.push_str("]");
            return result;
        }
    }
}

fn value_type_to_string(value: &Value) -> String {
    match value
    {
        Value::Number(_) => return String::from("integer"),
        Value::Bool(_) => return String::from("boolean"),
        Value::String(_) => return String::from("string"),
        Value::Function { .. } => return String::from("function"),
        Value::StandardFunction(_) => return String::from("standard function"),
        Value::List(_) => return String::from("list"),
    }
}

struct Binding {
    name: String,
    value: Value,
}

type Scope = Vec<Binding>;

type Environment = Vec<Scope>;

pub type Terminal = Vec<String>;

enum InterpretationResult {
    Return { value: Option<Value> },
    Break,
    Empty,
}

pub fn interpret(base_expressions: Vec<BaseExpr<()>>) -> Result<Terminal, Error> {
    let mut env: Environment = Vec::new();

    env.push(Vec::new());

    add_default_functions_to_env(&mut env);

    let mut terminal: Terminal = Vec::new();

    terminal.push(String::new());

    for base_expression in &base_expressions
    {
        match interpret_base_expr(base_expression, &mut env, &mut terminal)
        {
            Ok(_) =>
            {}
            Err(e) => return Err(e),
        }
    }

    return Ok(terminal);
}

fn interpret_base_expr(
    base_expression: &BaseExpr<()>,
    env: &mut Environment,
    terminal: &mut Terminal,
) -> Result<InterpretationResult, Error> {
    match base_expression
    {
        BaseExpr {
            data: BaseExprData::Simple { expr },
            ..
        } => match interpret_expr(expr, env, terminal)
        {
            Ok(_) => return Ok(InterpretationResult::Empty),
            Err(e) => return Err(e),
        },
        BaseExpr {
            data: BaseExprData::VariableAssignment { var_name, expr },
            ..
        } =>
        {
            let value = match interpret_expr(expr, env, terminal)
            {
                Ok(right) => match right
                {
                    Some(value) => value,
                    None =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot assign to empty"),
                            row: base_expression.row,
                            col_start: base_expression.col_start,
                            col_end: base_expression.col_end,
                        })
                    }
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            update_or_add_in_scope(&value, &var_name, env.last_mut().unwrap());
            return Ok(InterpretationResult::Empty);
        }
        BaseExpr {
            data:
                BaseExprData::IfStatement {
                    condition,
                    body,
                    else_statement,
                },
            ..
        } =>
        {
            let row = condition.row;
            let col_start = condition.col_start;
            let col_end = condition.col_end;

            let condition = match interpret_expr(condition, env, terminal)
            {
                Ok(Some(Value::Bool(condition))) => condition,
                Ok(Some(other_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot use {} as a condition for an if statement",
                            value_type_to_string(&other_value)
                        ),
                        row,
                        col_start,
                        col_end,
                    })
                }
                Ok(None) =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot use empty as a condition for an if statement"),
                        row,
                        col_start,
                        col_end,
                    })
                }
                Err(e) => return Err(e),
            };

            // If the condition is false, use the else option
            if !condition
            {
                let else_statement_real = match else_statement
                {
                    Some(expr) => expr,
                    None => return Ok(InterpretationResult::Empty),
                };

                return interpret_base_expr(&*else_statement_real, env, terminal);
            }

            for base_expression in body
            {
                let interp_result = match interpret_base_expr(base_expression, env, terminal)
                {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result
                {
                    InterpretationResult::Return {
                        value: return_value,
                    } =>
                    {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break =>
                    {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty =>
                    {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr {
            data:
                BaseExprData::ElseIfStatement {
                    condition,
                    body,
                    else_statement,
                },
            ..
        } =>
        {
            let row = condition.row;
            let col_start = condition.col_start;
            let col_end = condition.col_end;

            let condition = match interpret_expr(condition, env, terminal)
            {
                Ok(Some(Value::Bool(condition))) => condition,
                Ok(Some(other_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot use {} as a condition for an if statement",
                            value_type_to_string(&other_value)
                        ),
                        row,
                        col_start,
                        col_end,
                    })
                }
                Ok(None) =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot use empty as a condition for an if statement"),
                        row,
                        col_start,
                        col_end,
                    })
                }
                Err(e) => return Err(e),
            };

            // If the condition is false, use the else option
            if !condition
            {
                let else_statement_real = match else_statement
                {
                    Some(expr) => expr,
                    None => return Ok(InterpretationResult::Empty),
                };

                return interpret_base_expr(&*else_statement_real, env, terminal);
            }

            for base_expression in body
            {
                let interp_result = match interpret_base_expr(base_expression, env, terminal)
                {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result
                {
                    InterpretationResult::Return {
                        value: return_value,
                    } =>
                    {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break =>
                    {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty =>
                    {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr {
            data: BaseExprData::ElseStatement { body },
            ..
        } =>
        {
            for base_expression in body
            {
                let interp_result = match interpret_base_expr(base_expression, env, terminal)
                {
                    Ok(result) => result,
                    Err(e) => return Err(e),
                };

                match interp_result
                {
                    InterpretationResult::Return {
                        value: return_value,
                    } =>
                    {
                        return Ok(InterpretationResult::Return {
                            value: return_value,
                        });
                    }
                    InterpretationResult::Break =>
                    {
                        return Ok(InterpretationResult::Empty);
                    }
                    InterpretationResult::Empty =>
                    {}
                }
            }

            return Ok(InterpretationResult::Empty);
        }
        BaseExpr {
            data: BaseExprData::PlusEqualsStatement { var_name, expr },
            ..
        } =>
        {
            let row = base_expression.row;
            let col_start = base_expression.col_start;
            let col_end = base_expression.col_end;

            let right_side_row = expr.row;
            let right_side_col_start = expr.col_start;
            let right_side_col_end = expr.col_end;

            let value = match interpret_expr(expr, env, terminal)
            {
                Ok(right) => match right
                {
                    Some(value) => value,
                    None =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot assign to empty"),
                            row,
                            col_start,
                            col_end,
                        });
                    }
                },
                Err(e) => return Err(e),
            };

            let current_value = match find_in_env(&var_name, env)
            {
                Some(value) => value,
                None =>
                {
                    return Err(Error::LocationError {
                        message: format!("Variable {} not found", var_name),
                        row,
                        col_start,
                        col_end,
                    });
                }
            };

            let new_value = match add(&Some(current_value), &Some(value), row, col_start, col_end)
            {
                Ok(new_value) => match new_value
                {
                    Some(value) => value,
                    None =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot assign to empty"),
                            row,
                            col_start,
                            col_end,
                        });
                    }
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            let scope = env.last_mut().unwrap();
            match update_in_scope(&new_value, &var_name, scope)
            {
                true =>
                {}
                false =>
                {
                    return Err(Error::LocationError {
                        message: format!("Variable {} not found", var_name),
                        row: right_side_row,
                        col_start: right_side_col_start,
                        col_end: right_side_col_end,
                    });
                }
            }
            return Ok(InterpretationResult::Empty);
        }

        BaseExpr {
            data:
                BaseExprData::FunctionDefinition {
                    fun_name,
                    args,
                    body,
                },
            ..
        } =>
        {
            let function = Value::Function {
                name: fun_name.clone(),
                args: args.clone(),
                body: body.clone(),
            };

            update_or_add_in_scope(&function, &fun_name, env.last_mut().unwrap());

            return Ok(InterpretationResult::Empty);
        }

        BaseExpr {
            data: BaseExprData::Return { return_value },
            ..
        } =>
        {
            let return_value = match return_value
            {
                Some(expr) => expr,
                None => return Ok(InterpretationResult::Return { value: None }),
            };

            let return_value = match interpret_expr(return_value, env, terminal)
            {
                Ok(Some(value)) => value,
                Ok(None) => return Ok(InterpretationResult::Return { value: None }),
                Err(e) => return Err(e),
            };

            return Ok(InterpretationResult::Return {
                value: Some(return_value),
            });
        }

        BaseExpr {
            data: BaseExprData::Break,
            ..
        } =>
        {
            return Ok(InterpretationResult::Break);
        }

        BaseExpr {
            data:
                BaseExprData::ForLoop {
                    var_name,
                    until: until_expr,
                    body,
                },
            ..
        } =>
        {
            let row = until_expr.row;
            let col_start = until_expr.col_start;
            let col_end = until_expr.col_end;

            let values = match interpret_expr(until_expr, env, terminal)
            {
                Ok(Some(Value::Number(until))) =>
                {
                    (0..until).map(|i| Value::Number(i)).into_iter().collect()
                }
                Ok(Some(Value::List(values))) => values,
                Ok(Some(other_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot use {} as a condition for a for loop",
                            value_type_to_string(&other_value)
                        ),
                        row,
                        col_start,
                        col_end,
                    });
                }
                Ok(None) =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot use empty as a condition for a for loop"),
                        row,
                        col_start,
                        col_end,
                    });
                }
                Err(e) => return Err(e),
            };

            update_or_add_in_scope(&Value::Number(0), var_name, env.last_mut().unwrap());

            for i in values
            {
                let scope = env.last_mut().unwrap();
                match update_in_scope(&i, &var_name, scope)
                {
                    true =>
                    {}
                    false =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Variable {} not found", var_name),
                            row,
                            col_start,
                            col_end,
                        });
                    }
                }

                for base_expression in body.iter()
                {
                    let interp_result = match interpret_base_expr(base_expression, env, terminal)
                    {
                        Ok(result) => result,
                        Err(e) => return Err(e),
                    };

                    match interp_result
                    {
                        InterpretationResult::Return {
                            value: return_value,
                        } =>
                        {
                            return Ok(InterpretationResult::Return {
                                value: return_value,
                            });
                        }
                        InterpretationResult::Break =>
                        {
                            return Ok(InterpretationResult::Break);
                        }
                        InterpretationResult::Empty =>
                        {}
                    }
                }
            }

            return Ok(InterpretationResult::Empty);
        }
    }
}

fn add(
    left: &Option<Value>,
    right: &Option<Value>,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> Result<Option<Value>, Error> {
    match (left, right)
    {
        (Some(Value::Number(left)), Some(Value::Number(right))) =>
        {
            let result = left + right;
            return Ok(Some(Value::Number(result)));
        }
        (Some(Value::String(left)), Some(Value::String(right))) =>
        {
            let result = left.clone() + right;
            return Ok(Some(Value::String(result)));
        }
        (Some(Value::List(left_elements)), Some(Value::List(right_elements))) =>
        {
            let mut result = left_elements.clone();
            for element in right_elements
            {
                result.push(element.clone());
            }
            return Ok(Some(Value::List(result)));
        }
        (Some(Value::List(elements)), Some(value)) =>
        {
            let mut result = elements.clone();
            result.push(value.clone());
            return Ok(Some(Value::List(result)));
        }
        (Some(left), Some(right)) =>
        {
            return Err(Error::LocationError {
                message: format!(
                    "Cannot apply operator + on types {} and {}",
                    value_type_to_string(left),
                    value_type_to_string(right)
                ),
                row,
                col_start,
                col_end,
            });
        }
        _ =>
        {
            return Err(Error::LocationError {
                message: format!("Cannot apply operator + on empty"),
                row,
                col_start,
                col_end,
            });
        }
    }
}

fn interpret_expr(
    expr: &RecExpr<()>,
    env: &mut Environment,
    terminal: &mut Terminal,
) -> Result<Option<Value>, Error> {
    match &expr.data
    {
        RecExprData::Variable { name } => match find_in_env(&name, env)
        {
            Some(value) => return Ok(Some(value)),
            None =>
            {
                return Err(Error::LocationError {
                    message: format!("Variable not found: {}", name),
                    row: expr.row,
                    col_start: expr.col_start,
                    col_end: expr.col_end,
                });
            }
        },
        RecExprData::Number { number } => return Ok(Some(Value::Number(*number))),
        RecExprData::Boolean { value } => return Ok(Some(Value::Bool(*value))),
        RecExprData::String { value } => return Ok(Some(Value::String(value.clone()))),
        RecExprData::Add { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            let row = expr.row;
            let col_start = expr.col_start;
            let col_end = expr.col_end;

            return add(&left_value, &right_value, row, col_start, col_end);
        }
        RecExprData::Subtract { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) =>
                {
                    let result = left_num - right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator - on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator - on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Multiply { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) =>
                {
                    let result = left_num * right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator * on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator * on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Divide { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left_num)), Some(Value::Number(right_num))) =>
                {
                    let result = left_num / right_num;
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator / on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator / on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Power { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    if right < 0
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot raise to a negative power"),
                            row: expr.row,
                            col_start: expr.col_start,
                            col_end: expr.col_end,
                        });
                    }

                    let result = i32::pow(left, right as u32);
                    return Ok(Some(Value::Number(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator ^ on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator ^ on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Minus { right } =>
        {
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match right_value
            {
                Some(Value::Number(value)) =>
                {
                    let result = -value;
                    return Ok(Some(Value::Number(result)));
                }
                Some(value) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator - on type {}",
                            value_type_to_string(&value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator - on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Equals { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::Bool(left)), Some(Value::Bool(right))) =>
                {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::String(left)), Some(Value::String(right))) =>
                {
                    let result = left == right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(_), Some(_)) =>
                {
                    // If the types are different, they are not equal
                    return Ok(Some(Value::Bool(false)));
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator == on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::NotEquals { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left != right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::Bool(left)), Some(Value::Bool(right))) =>
                {
                    let result = left != right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(Value::String(left)), Some(Value::String(right))) =>
                {
                    let result = left != right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(_), Some(_)) =>
                {
                    // If the types are different, they are not equal
                    return Ok(Some(Value::Bool(true)));
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator != on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::GreaterThan { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left > right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator > on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator > on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::GreaterThanOrEqual { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left >= right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator >= on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator >= on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::LessThan { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left < right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator < on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator < on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::LessThanOrEqual { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Number(left)), Some(Value::Number(right))) =>
                {
                    let result = left <= right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator <= on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator <= on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::And { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Bool(left)), Some(Value::Bool(right))) =>
                {
                    let result = left && right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator AND on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator AND on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Or { left, right } =>
        {
            let left_value = match interpret_expr(&*left, env, terminal)
            {
                Ok(left_value) => left_value,
                Err(e) => return Err(e),
            };
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match (left_value, right_value)
            {
                (Some(Value::Bool(left)), Some(Value::Bool(right))) =>
                {
                    let result = left || right;
                    return Ok(Some(Value::Bool(result)));
                }
                (Some(left_value), Some(right_value)) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator OR on types {} and {}",
                            value_type_to_string(&left_value),
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator OR on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Not { right } =>
        {
            let right_value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right_value) => right_value,
                Err(e) => return Err(e),
            };

            match right_value
            {
                Some(Value::Bool(right)) =>
                {
                    let result = !right;
                    return Ok(Some(Value::Bool(result)));
                }
                Some(right_value) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot apply operator NOT on type {}",
                            value_type_to_string(&right_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                _ =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot apply operator NOT on empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::FunctionCall {
            function_name,
            args,
        } =>
        {
            let env_variable = match find_in_env(&function_name, env)
            {
                Some(env_variable) => env_variable,
                None =>
                {
                    return Err(Error::LocationError {
                        message: format!("Function {} not found", function_name),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            };

            // We also need all values that we will pass
            let mut arg_values = Vec::new();
            for arg in args
            {
                let row = arg.row;
                let col_start = arg.col_start;
                let col_end = arg.col_end;

                match interpret_expr(&arg, env, terminal)
                {
                    Ok(Some(value)) =>
                    {
                        arg_values.push(value);
                    }
                    Ok(None) =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot pass Empty to a function"),
                            row,
                            col_start,
                            col_end,
                        });
                    }
                    Err(e) => return Err(e),
                }
            }

            match env_variable
            {
                Value::Function { name, args, body } =>
                {
                    // Matching the arguments values with the argument names
                    let mut function_scope: Scope = Vec::new();

                    if args.len() != arg_values.len()
                    {
                        return Err(Error::LocationError {
                            message: format!(
                                "Expected {} arguments, but got {}",
                                args.len(),
                                arg_values.len()
                            ),
                            row: expr.row,
                            col_start: expr.col_start,
                            col_end: expr.col_end,
                        });
                    }

                    for (name, value) in args.iter().zip(arg_values.iter())
                    {
                        function_scope.push(Binding {
                            name: name.clone(),
                            value: value.clone(),
                        });
                    }

                    // Adding this scope to the environment
                    env.push(function_scope);

                    // Run all sub statements
                    for base_expression in body
                    {
                        let row = base_expression.row;
                        let col_start = base_expression.col_start;
                        let col_end = base_expression.col_end;

                        let interp_result =
                            match interpret_base_expr(&base_expression, env, terminal)
                            {
                                Ok(result) => result,
                                Err(e) => return Err(e),
                            };

                        match interp_result
                        {
                            InterpretationResult::Return {
                                value: return_value,
                            } =>
                            {
                                return Ok(return_value);
                            }
                            InterpretationResult::Break =>
                            {
                                return Err(Error::LocationError {
                                    message: format!("Cannot break out of a function"),
                                    row,
                                    col_start,
                                    col_end,
                                });
                            }
                            InterpretationResult::Empty =>
                            {}
                        }
                    }

                    // Removing the scope
                    env.pop();

                    // No return statement was found, thus return empty
                    return Ok(None);
                }
                Value::StandardFunction(StandardFunction::Print) =>
                {
                    let last_terminal_line: &mut String = terminal.last_mut().unwrap();
                    for arg in arg_values
                    {
                        let value_string = value_to_string(&arg);
                        print!("{}", value_string);
                        last_terminal_line.push_str(&value_string);
                    }

                    return Ok(None);
                }
                Value::StandardFunction(StandardFunction::PrintLine) =>
                {
                    let last_terminal_line = terminal.last_mut().unwrap();
                    for arg in arg_values
                    {
                        let value_string = value_to_string(&arg);
                        print!("{}", value_string);
                        last_terminal_line.push_str(&value_string);
                    }
                    terminal.push(String::new());
                    println!();
                    return Ok(None);
                }
                other =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Expected function, found {} for variable {}",
                            value_type_to_string(&other),
                            function_name
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
        RecExprData::Assign {
            variable_name,
            right,
        } =>
        {
            let value = match interpret_expr(&*right, env, terminal)
            {
                Ok(right) => match right
                {
                    Some(value) => value,
                    None =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot assign to empty"),
                            row: expr.row,
                            col_start: expr.col_start,
                            col_end: expr.col_end,
                        });
                    }
                },
                Err(e) => return Err(e),
            };

            // Now we add this value to the scope
            let scope = env.last_mut().unwrap();
            scope.push(Binding {
                name: variable_name.clone(),
                value: value,
            });

            return Ok(None);
        }
        RecExprData::Access { object, variable } =>
        {
            return Err(Error::SimpleError {
                message: format!("not implemented"),
            });
        }
        RecExprData::List { elements } =>
        {
            let mut list = Vec::new();
            for element in elements
            {
                let value = match interpret_expr(&element, env, terminal)
                {
                    Ok(Some(value)) => value,
                    Ok(None) =>
                    {
                        return Err(Error::LocationError {
                            message: format!("Cannot add empty to a list"),
                            row: element.row,
                            col_start: element.col_start,
                            col_end: element.col_end,
                        });
                    }
                    Err(e) => return Err(e),
                };

                list.push(value);
            }

            return Ok(Some(Value::List(list)));
        }
        RecExprData::ListAccess { variable, index } =>
        {
            let variable_value = match find_in_env(&variable, env)
            {
                Some(value) => value,
                None =>
                {
                    return Err(Error::LocationError {
                        message: format!("Variable {} not found", variable),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            };

            let index_value = match interpret_expr(&*index, env, terminal)
            {
                Ok(Some(value)) => value,
                Ok(None) =>
                {
                    return Err(Error::LocationError {
                        message: format!("Cannot access list with empty"),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
                Err(e) => return Err(e),
            };

            match (variable_value, index_value)
            {
                (Value::List(list), Value::Number(index)) =>
                {
                    let index = index as usize;
                    let len = list.len();
                    if index >= len
                    {
                        return Err(Error::LocationError {
                            message: format!(
                                "Index {index} out of bounds for list of length {len}"
                            ),
                            row: expr.row,
                            col_start: expr.col_start,
                            col_end: expr.col_end,
                        });
                    }

                    return Ok(Some(list[index].clone()));
                }
                (variable_value, index_value) =>
                {
                    return Err(Error::LocationError {
                        message: format!(
                            "Cannot access list with types {} and {}",
                            value_type_to_string(&variable_value),
                            value_type_to_string(&index_value)
                        ),
                        row: expr.row,
                        col_start: expr.col_start,
                        col_end: expr.col_end,
                    });
                }
            }
        }
    }
}

fn update_in_env(value: &Value, name: &String, env: &mut Environment) -> bool {
    for scope in env.iter_mut().rev()
    {
        if update_in_scope(value, name, scope)
        {
            return true;
        }
    }
    return false;
}

fn update_in_scope(value: &Value, name: &String, scope: &mut Scope) -> bool {
    for binding in scope.iter_mut()
    {
        if binding.name == *name
        {
            binding.value = value.clone();
            return true;
        }
    }
    return false;
}

fn update_or_add_in_scope(value: &Value, name: &String, scope: &mut Scope) {
    if update_in_scope(value, name, scope)
    {
        return;
    }

    scope.push(Binding {
        name: name.clone(),
        value: value.clone(),
    });
}

fn find_in_env(name: &String, env: &Environment) -> Option<Value> {
    for scope in env.iter().rev()
    {
        match find_in_scope(name, scope)
        {
            Some(value) => return Some(value),
            None =>
            {}
        }
    }
    return None;
}

fn find_in_scope(name: &String, scope: &Scope) -> Option<Value> {
    for binding in scope.iter()
    {
        if binding.name == *name
        {
            return Some(binding.value.clone());
        }
    }

    return None;
}
