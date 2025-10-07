use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;

// Function type checking works as follows:
// 1. We first preload all function definitions into a separate function environment
//    This environment keeps all functions but without specific parameter types
// 2. When we encounter a function call, we look for a function with the same name in the type environment
//    If we find one with the same parameter types, we return its return type
// 3. If we do not find one with the same parameter types, we look for a function with the same name in the function environment
//    If we find one, we type-check it with the new parameter types and add it to the type environment if successful

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Undefined,
    Integer,
    Float,
    Boolean,
    String,
    List(Box<Type>),
}

struct TypeBinding {
    name: String,
    value_type: Type,
}

type TypeScope = Vec<TypeBinding>;

#[derive(Clone, PartialEq, Debug)]
struct FunctionBinding {
    name: String,
    param_names: Vec<String>,
    content: Vec<BaseExpr<()>>,
}
type FunctionEnvironment = Vec<FunctionBinding>;

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionType {
    pub(crate) name: String,
    pub(crate) param_names: Vec<String>,
    pub(crate) param_types: Vec<Type>,
    pub(crate) return_type: Type,
    pub(crate) content: Vec<BaseExpr<Type>>, // The content of the function with types filled in
}

struct TypeEnvironment {
    scopes: Vec<TypeScope>,
    functions: Vec<FunctionType>,
}

fn print_type_env(env: &TypeEnvironment) {
    print!("Type Environment: ");
    for (i, scope) in env.scopes.iter().enumerate()
    {
        print!("scope {}: [", i);
        for binding in scope.iter()
        {
            print!("{}: {:?}, ", binding.name, binding.value_type);
        }
        print!("], ");
    }
    print!("Functions: [");
    for func in env.functions.iter()
    {
        print!(
            "({}: {:?} -> {:?}), ",
            func.name, func.param_types, func.return_type
        );
    }
    print!("\n");
}

fn print_function_env(func_env: &FunctionEnvironment) {
    print!("Function Environment: [");
    for func in func_env.iter()
    {
        print!("{}({:?}), ", func.name, func.param_names);
    }
    print!("]\n");
}

fn add_default_functions_to_env(env: &mut TypeEnvironment) {
    env.functions.push(FunctionType {
        name: String::from("print"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::String],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("print"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Integer],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("print"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Float],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("print"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Boolean],
        return_type: Type::Undefined,
        content: Vec::new(),
    });

    env.functions.push(FunctionType {
        name: String::from("println"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::String],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("println"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Integer],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("println"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Float],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
    env.functions.push(FunctionType {
        name: String::from("println"),
        param_names: vec![String::from("value")],
        param_types: vec![Type::Boolean],
        return_type: Type::Undefined,
        content: Vec::new(),
    });
}

fn preload_functions(base_expressions: &Vec<BaseExpr<()>>, func_env: &mut FunctionEnvironment) {
    for base_expr in base_expressions.iter()
    {
        match &base_expr.data
        {
            BaseExprData::FunctionDefinition {
                fun_name,
                args,
                body,
            } =>
            {
                let func_binding = FunctionBinding {
                    name: fun_name.clone(),
                    param_names: args.clone(),
                    content: body.clone(),
                };
                func_env.push(func_binding);
            }
            _ =>
            {}
        }
    }
}

fn find_matching_function_in_function_env(
    name: &String,
    param_types: &Vec<Type>,
    func_env: &FunctionEnvironment,
) -> Option<FunctionBinding> {
    for func in func_env.iter()
    {
        if func.name == *name
        {
            // We have found a function with the correct name, now we need to check the parameter types
            if func.param_names.len() == param_types.len()
            {
                return Some(func.clone());
            }
        }
    }
    return None;
}

// Find a function in the environment with the given name and parameter types
// It then returns the return type of that function
// If a function exists with the same name but different parameter types, it type-checks that function with the new parameter types
// and adds it to the environment if successful
fn find_matching_function_in_env(
    name: &String,
    param_types: &Vec<Type>,
    env: &mut TypeEnvironment,
    func_env: &FunctionEnvironment,
) -> Result<Type, Error> {
    for function in env.functions.iter()
    {
        if function.name == *name
        {
            if function.param_types == *param_types
            {
                return Ok(function.return_type.clone());
            }
        }
    }

    // If we cannot find a function with that name and parameter types, we type-check the function with the given name
    // but with these new parameter types
    match find_matching_function_in_function_env(name, param_types, func_env)
    {
        Some(func) =>
        {
            // We have found a function with the correct name, now we need to type-check it with the given parameter types
            let mut new_env: TypeEnvironment = TypeEnvironment {
                scopes: Vec::new(),
                functions: env.functions.clone(),
            };
            new_env.scopes.push(Vec::new());

            // So we add the parameter types to the new environment
            // with the names given in the function definition
            for (i, param_name) in func.param_names.iter().enumerate()
            {
                new_env.scopes.last_mut().unwrap().push(TypeBinding {
                    name: param_name.clone(),
                    value_type: param_types[i].clone(),
                });
            }

            let mut expected_return_type: Option<Type> = None;
            match type_check(
                func.content.clone(),
                &mut new_env,
                func_env,
                false,
                &mut expected_return_type,
            )
            {
                Ok(typed_base_expressions) =>
                {
                    // If the function has no return statement, we set the return type to undefined
                    let return_type = match expected_return_type
                    {
                        Some(rt) => rt,
                        None => Type::Undefined,
                    };

                    // The function is successfully type-checked with the new parameter types
                    env.functions.push(FunctionType {
                        name: name.clone(),
                        param_names: func.param_names.clone(),
                        param_types: param_types.clone(),
                        return_type: return_type.clone(),
                        content: typed_base_expressions.0,
                    });
                    return Ok(return_type);
                }
                Err(error) =>
                {
                    return Err(error);
                }
            }
        }
        None =>
        {
            return Err(Error::SimpleError {
                message: format!(
                    "Function '{}' with parameter types {:?} not found",
                    name, param_types
                ),
            });
        }
    }
}

fn update_in_env(value: &Type, name: &String, env: &mut TypeEnvironment) -> bool {
    for scope in env.scopes.iter_mut().rev()
    {
        if update_in_scope(value, name, scope)
        {
            return true;
        }
    }
    return false;
}

fn update_in_scope(value: &Type, name: &String, scope: &mut TypeScope) -> bool {
    for binding in scope.iter_mut()
    {
        if binding.name == *name
        {
            binding.value_type = value.clone();
            return true;
        }
    }
    return false;
}

fn update_or_add_in_scope(value: &Type, name: &String, scope: &mut TypeScope) {
    if update_in_scope(value, name, scope)
    {
        return;
    }

    scope.push(TypeBinding {
        name: name.clone(),
        value_type: value.clone(),
    });
}

fn find_in_env(name: &String, env: &TypeEnvironment) -> Option<Type> {
    for scope in env.scopes.iter().rev()
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

fn find_in_scope(name: &String, scope: &TypeScope) -> Option<Type> {
    for binding in scope.iter()
    {
        if binding.name == *name
        {
            return Some(binding.value_type.clone());
        }
    }

    return None;
}

pub fn type_check_program(
    base_expressions: Vec<BaseExpr<()>>,
    print_results: bool,
) -> Result<(Vec<BaseExpr<Type>>, Vec<FunctionType>), Error> {
    let mut env: TypeEnvironment = TypeEnvironment {
        scopes: Vec::new(),
        functions: Vec::new(),
    };

    env.scopes.push(Vec::new());

    add_default_functions_to_env(&mut env);

    let mut func_env: FunctionEnvironment = Vec::new();
    preload_functions(&base_expressions, &mut func_env);
    print_function_env(&func_env);

    let mut expected_return_type: Option<Type> = None;

    match type_check(
        base_expressions,
        &mut env,
        &func_env,
        print_results,
        &mut expected_return_type,
    )
    {
        Ok((typed_base_expressions, typed_functions)) => Ok((typed_base_expressions, typed_functions)),
        Err(error) => Err(error),
    }
}

// Type check a set of base expressions in the given environment
// If print_results is true, it will print the types of variable assignments
// It returns the expected return type of the program if there is one
fn type_check(
    base_expressions: Vec<BaseExpr<()>>,
    env: &mut TypeEnvironment,
    func_env: &FunctionEnvironment,
    print_results: bool,
    expected_return_type: &mut Option<Type>,
) -> Result<(Vec<BaseExpr<Type>>, Vec<FunctionType>), Error> {
    let mut typed_base_expressions: Vec<BaseExpr<Type>> = Vec::new();

    for base_expr in base_expressions
    {
        print_type_env(&env);
        match base_expr.data
        {
            BaseExprData::Simple { expr: rec_expr } =>
            {
                let rec_expr_typed = check_type_rec(rec_expr, env, func_env)?;
                let rec_expr_type = rec_expr_typed.generic_data.clone();
                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::Simple {
                        expr: rec_expr_typed,
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: rec_expr_type,
                });
            }
            BaseExprData::VariableAssignment { var_name, expr } =>
            {
                let expr_typed = check_type_rec(expr, env, func_env)?;
                let expr_type = expr_typed.generic_data.clone();
                update_or_add_in_scope(&expr_type, &var_name, env.scopes.last_mut().unwrap());
                if print_results
                {
                    println!("Variable '{}' has type {:?}", var_name, expr_type);
                }
                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::VariableAssignment {
                        var_name: var_name.clone(),
                        expr: expr_typed,
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of variable assignments
                });
            }
            BaseExprData::FunctionDefinition {
                fun_name,
                args,
                body,
            } =>
            {
                // We don't need to do anything here, as functions are handled separately at the start of type-checking
                // They will also not be included in the list of typed base expressions returned
            }
            BaseExprData::IfStatement {
                condition,
                body,
                else_statement,
            } =>
            {
                let condition_row = condition.row;
                let condition_col_start = condition.col_start;
                let condition_col_end = condition.col_end;

                let cond_typed = check_type_rec(condition, env, func_env)?;
                let cond_type = cond_typed.generic_data.clone();

                if cond_type != Type::Boolean
                {
                    return Err(Error::TypeError {
                        message: "If condition must be of type Boolean".to_string(),
                        expected: Type::Boolean,
                        found: cond_type,
                        row: condition_row,
                        col_start: condition_col_start,
                        col_end: condition_col_end,
                    });
                }

                // Typecheck the body in a new scope
                env.scopes.push(Vec::new());
                let body_typed =
                    type_check(body, env, func_env, print_results, expected_return_type)?.0;
                env.scopes.pop();

                let else_typed = match else_statement
                {
                    Some(else_expr) =>
                    {
                        env.scopes.push(Vec::new());
                        let else_typed = type_check(
                            vec![*else_expr],
                            env,
                            func_env,
                            print_results,
                            expected_return_type,
                        )?.0;
                        env.scopes.pop();
                        Some(Box::new(else_typed[0].clone()))
                    }
                    None => None,
                };
                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::IfStatement {
                        condition: cond_typed,
                        body: body_typed,
                        else_statement: else_typed,
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of if statements
                });
            }
            BaseExprData::ElseIfStatement {
                condition,
                body,
                else_statement,
            } =>
            {
                let condition_row = condition.row;
                let condition_col_start = condition.col_start;
                let condition_col_end = condition.col_end;

                let cond_typed = check_type_rec(condition, env, func_env)?;
                let cond_type = cond_typed.generic_data.clone();

                if cond_type != Type::Boolean
                {
                    return Err(Error::TypeError {
                        message: "If condition must be of type Boolean".to_string(),
                        expected: Type::Boolean,
                        found: cond_type,
                        row: condition_row,
                        col_start: condition_col_start,
                        col_end: condition_col_end,
                    });
                }

                // Typecheck the body in a new scope
                env.scopes.push(Vec::new());
                let body_typed =
                    type_check(body, env, func_env, print_results, expected_return_type)?.0;
                env.scopes.pop();

                let else_typed = match else_statement
                {
                    Some(else_expr) =>
                    {
                        env.scopes.push(Vec::new());
                        let else_typed = type_check(
                            vec![*else_expr],
                            env,
                            func_env,
                            print_results,
                            expected_return_type,
                        )?.0;
                        env.scopes.pop();
                        Some(Box::new(else_typed[0].clone()))
                    }
                    None => None,
                };
                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::ElseIfStatement {
                        condition: cond_typed,
                        body: body_typed,
                        else_statement: else_typed,
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of if statements
                });
            }
            BaseExprData::ElseStatement { body } =>
            {
                // Typecheck the body in a new scope
                env.scopes.push(Vec::new());
                let body_typed =
                    type_check(body, env, func_env, print_results, expected_return_type)?.0;
                env.scopes.pop();

                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::ElseStatement { body: body_typed },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of else statements
                });
            }
            BaseExprData::Return {
                return_value: optional_return_value,
            } =>
            {
                let return_value = match optional_return_value
                {
                    Some(rv) => rv,
                    None =>
                    {
                        // Define the function to return undefined (no return value)
                        *expected_return_type = Some(Type::Undefined);

                        // Continue on to the next statement
                        continue;
                    }
                };

                let return_value_row = return_value.row;
                let return_value_col_start = return_value.col_start;
                let return_value_col_end = return_value.col_end;

                // There is a return value
                // Therefore we type-check it and compare it to the expected return type
                // If there is no expected return type, we set it to the type of this return value
                let return_typed = check_type_rec(return_value, env, func_env)?;
                let return_type = return_typed.generic_data.clone();

                match &expected_return_type
                {
                    Some(expected_type) =>
                    {
                        if *expected_type != return_type
                        {
                            return Err(Error::TypeError {
                                message: "Return type does not match expected return type"
                                    .to_string(),
                                expected: expected_type.clone(),
                                found: return_type,
                                row: return_value_row,
                                col_start: return_value_col_start,
                                col_end: return_value_col_end,
                            });
                        }
                    }
                    None =>
                    {
                        // If there was no expected return type, we set it to the current return type
                        *expected_return_type = Some(return_type.clone());
                    }
                }

                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::Return {
                        return_value: Some(return_typed),
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: return_type,
                });
            }
            BaseExprData::ForLoop {
                var_name,
                until,
                body,
            } =>
            {
                let until_row = until.row;
                let until_col_start = until.col_start;
                let until_col_end = until.col_end;

                let iteration_typed = check_type_rec(until, env, func_env)?;
                let iteration_variable_type = match iteration_typed.generic_data.clone()
                {
                    Type::Integer => Type::Integer,
                    Type::List(list_type) => *list_type,
                    other_type =>
                    {
                        return Err(Error::LocationError {
                            message: format!(
                                "For loop iteration cannot be of type {:?}",
                                other_type
                            ),
                            row: until_row,
                            col_start: until_col_start,
                            col_end: until_col_end,
                        });
                    }
                };

                // Typechecking the body with the iteration variable included in the scope
                env.scopes.push(Vec::new());
                update_or_add_in_scope(
                    &iteration_variable_type,
                    &var_name,
                    env.scopes.last_mut().unwrap(),
                );
                let body_typed =
                    type_check(body, env, func_env, print_results, expected_return_type)?.0;
                env.scopes.pop();

                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::ForLoop {
                        var_name: var_name.clone(),
                        until: iteration_typed,
                        body: body_typed,
                    },
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of for loops
                });
            }
            BaseExprData::Break =>
            {
                typed_base_expressions.push(BaseExpr {
                    data: BaseExprData::Break,
                    row: base_expr.row,
                    col_start: base_expr.col_start,
                    col_end: base_expr.col_end,
                    generic_data: Type::Undefined, // We do not store the type of break statements
                });
            }
            _ =>
            {
                unimplemented!(
                    "Only RecExpr is implemented in type_check, not {:?}",
                    base_expr.data
                );
            }
        }
    }
    print_type_env(&env);

    // If we have an expected return type, we return it
    Ok((typed_base_expressions, env.functions.clone()))
}

// This function allows entry into type-checking a single rec-expr from a test
pub fn get_type(base_expr: BaseExpr<()>) -> Result<BaseExpr<Type>, Error> {
    let mut env: TypeEnvironment = TypeEnvironment {
        scopes: Vec::new(),
        functions: Vec::new(),
    };

    env.scopes.push(Vec::new());

    add_default_functions_to_env(&mut env);

    let func_env: FunctionEnvironment = Vec::new();

    match base_expr.data
    {
        BaseExprData::Simple { expr: rec_expr } =>
        {
            let rec_expr_typed = check_type_rec(rec_expr, &mut env, &func_env)?;
            let rec_expr_type = rec_expr_typed.generic_data.clone();
            return Ok(BaseExpr {
                data: BaseExprData::Simple {
                    expr: rec_expr_typed,
                },
                row: base_expr.row,
                col_start: base_expr.col_start,
                col_end: base_expr.col_end,
                generic_data: rec_expr_type,
            });
        }
        _ =>
        {
            unimplemented!("Only RecExpr is implemented in get_type");
        }
    }
}

fn check_type_rec(
    rec_expr: RecExpr<()>,
    env: &mut TypeEnvironment,
    func_env: &FunctionEnvironment,
) -> Result<RecExpr<Type>, Error> {
    let rec_expr_row = rec_expr.row;
    let rec_expr_col_start = rec_expr.col_start;
    let rec_expr_col_end = rec_expr.col_end;

    return match rec_expr.data
    {
        RecExprData::Number { number } => Ok(RecExpr {
            data: RecExprData::Number { number },
            row: rec_expr_row,
            col_start: rec_expr_col_start,
            col_end: rec_expr_col_end,
            generic_data: Type::Integer,
        }),
        RecExprData::Boolean { value } => Ok(RecExpr {
            data: RecExprData::Boolean { value },
            row: rec_expr_row,
            col_start: rec_expr_col_start,
            col_end: rec_expr_col_end,
            generic_data: Type::Boolean,
        }),
        RecExprData::String { value } => Ok(RecExpr {
            data: RecExprData::String { value },
            row: rec_expr_row,
            col_start: rec_expr_col_start,
            col_end: rec_expr_col_end,
            generic_data: Type::String,
        }),
        RecExprData::List { elements } =>
        {
            if elements.len() == 0
            {
                return Ok(RecExpr {
                    data: RecExprData::List {
                        elements: Vec::new(),
                    },
                    row: rec_expr_row,
                    col_start: rec_expr_col_start,
                    col_end: rec_expr_col_end,
                    generic_data: Type::List(Box::new(Type::Undefined)),
                });
            }
            let first_elem_typed = check_type_rec(elements[0].clone(), env, func_env)?;
            let first_elem_type = first_elem_typed.generic_data.clone();
            let mut typed_elements = Vec::<RecExpr<Type>>::new();
            typed_elements.push(first_elem_typed);

            for elem in elements.iter().skip(1)
            {
                let elem_typed = check_type_rec(elem.clone(), env, func_env)?;
                let elem_type = elem_typed.generic_data.clone();
                if elem_type != first_elem_type
                {
                    return Err(Error::TypeError {
                        message: "List elements must be of the same type".to_string(),
                        expected: first_elem_type,
                        found: elem_type,
                        row: elem.row,
                        col_start: elem.col_start,
                        col_end: elem.col_end,
                    });
                }
                typed_elements.push(elem_typed);
            }
            return Ok(RecExpr {
                data: RecExprData::List {
                    elements: typed_elements,
                },
                row: rec_expr_row,
                col_start: rec_expr_col_start,
                col_end: rec_expr_col_end,
                generic_data: Type::List(Box::new(first_elem_type)),
            });
        }
        RecExprData::Add { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;
            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Integer && right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Add {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Integer,
                });
            }
            else if (left_type == Type::Integer || left_type == Type::Float)
                && (right_type == Type::Integer || right_type == Type::Float)
            {
                return Ok(RecExpr {
                    data: RecExprData::Add {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Float,
                });
            }
            else if left_type == Type::String && right_type == Type::String
            {
                return Ok(RecExpr {
                    data: RecExprData::Add {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::String,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for addition".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Multiply { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;
            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Integer && right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Multiply {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Integer,
                });
            }
            else if (left_type == Type::Integer || left_type == Type::Float)
                && (right_type == Type::Integer || right_type == Type::Float)
            {
                return Ok(RecExpr {
                    data: RecExprData::Multiply {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Float,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for multiplication".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Divide { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;
            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Integer && right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Divide {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Integer,
                });
            }
            else if (left_type == Type::Integer || left_type == Type::Float)
                && (right_type == Type::Integer || right_type == Type::Float)
            {
                return Ok(RecExpr {
                    data: RecExprData::Divide {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Float,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for division".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Subtract { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;
            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Integer && right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Subtract {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Integer,
                });
            }
            else if (left_type == Type::Integer || left_type == Type::Float)
                && (right_type == Type::Integer || right_type == Type::Float)
            {
                return Ok(RecExpr {
                    data: RecExprData::Subtract {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Float,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for subtraction".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Power { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;
            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Integer && right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Power {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Integer,
                });
            }
            else if (left_type == Type::Integer || left_type == Type::Float)
                && (right_type == Type::Integer || right_type == Type::Float)
            {
                return Ok(RecExpr {
                    data: RecExprData::Power {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Float,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for exponentiation".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Minus { right } =>
        {
            let row = right.row;
            let col_start = right.col_start;
            let col_end = right.col_end;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let right_type = right_typed.generic_data.clone();

            if right_type == Type::Integer
            {
                return Ok(RecExpr {
                    data: RecExprData::Minus {
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: col_start,
                    col_end: col_end,
                    generic_data: Type::Integer,
                });
            }
            else if right_type == Type::Float
            {
                return Ok(RecExpr {
                    data: RecExprData::Minus {
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: col_start,
                    col_end: col_end,
                    generic_data: Type::Float,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand type for negation".to_string(),
                    expected: Type::Integer,
                    found: right_type,
                    row: row,
                    col_start: col_start,
                    col_end: col_end,
                });
            }
        }
        RecExprData::Or { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Boolean && right_type == Type::Boolean
            {
                return Ok(RecExpr {
                    data: RecExprData::Or {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Boolean,
                });
            }
            else if left_type != Type::Boolean
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for logical OR".to_string(),
                    expected: Type::Boolean,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for logical OR".to_string(),
                    expected: Type::Boolean,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::And { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == Type::Boolean && right_type == Type::Boolean
            {
                return Ok(RecExpr {
                    data: RecExprData::And {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Boolean,
                });
            }
            else if left_type != Type::Boolean
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for logical AND".to_string(),
                    expected: Type::Boolean,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for logical AND".to_string(),
                    expected: Type::Boolean,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::Not { right } =>
        {
            let row = right.row;
            let col_start = right.col_start;
            let col_end = right.col_end;

            let right_typed = check_type_rec(*right, env, func_env)?;
            let right_type = right_typed.generic_data.clone();

            if right_type == Type::Boolean
            {
                return Ok(RecExpr {
                    data: RecExprData::Not {
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: col_start,
                    col_end: col_end,
                    generic_data: Type::Boolean,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand type for logical NOT".to_string(),
                    expected: Type::Boolean,
                    found: right_type,
                    row: row,
                    col_start: col_start,
                    col_end: col_end,
                });
            }
        }
        RecExprData::Equals { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == right_type
            {
                return Ok(RecExpr {
                    data: RecExprData::Equals {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Boolean,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for equality check".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::NotEquals { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type == right_type
            {
                return Ok(RecExpr {
                    data: RecExprData::NotEquals {
                        left: Box::new(left_typed),
                        right: Box::new(right_typed),
                    },
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                    generic_data: Type::Boolean,
                });
            }
            else
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for inequality check".to_string(),
                    expected: left_type,
                    found: right_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: right_col_end,
                });
            }
        }
        RecExprData::GreaterThan { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type != Type::Integer && left_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for greater-than check".to_string(),
                    expected: Type::Integer,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            if right_type != Type::Integer && right_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for greater-than check".to_string(),
                    expected: Type::Integer,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }

            return Ok(RecExpr {
                data: RecExprData::GreaterThan {
                    left: Box::new(left_typed),
                    right: Box::new(right_typed),
                },
                row: row,
                col_start: left_col_start,
                col_end: right_col_end,
                generic_data: Type::Boolean,
            });
        }
        RecExprData::LessThan { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type != Type::Integer && left_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for less-than check".to_string(),
                    expected: Type::Integer,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            if right_type != Type::Integer && right_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for less-than check".to_string(),
                    expected: Type::Integer,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }

            return Ok(RecExpr {
                data: RecExprData::LessThan {
                    left: Box::new(left_typed),
                    right: Box::new(right_typed),
                },
                row: row,
                col_start: left_col_start,
                col_end: right_col_end,
                generic_data: Type::Boolean,
            });
        }
        RecExprData::GreaterThanOrEqual { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type != Type::Integer && left_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for greater-than-or-equal check".to_string(),
                    expected: Type::Integer,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            if right_type != Type::Integer && right_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for greater-than-or-equal check".to_string(),
                    expected: Type::Integer,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }

            return Ok(RecExpr {
                data: RecExprData::GreaterThanOrEqual {
                    left: Box::new(left_typed),
                    right: Box::new(right_typed),
                },
                row: row,
                col_start: left_col_start,
                col_end: right_col_end,
                generic_data: Type::Boolean,
            });
        }
        RecExprData::LessThanOrEqual { left, right } =>
        {
            let row = left.row;
            let left_col_start = left.col_start;
            let right_col_start = right.col_start;
            let left_col_end = left.col_end;
            let right_col_end = right.col_end;

            let left_typed = check_type_rec(*left, env, func_env)?;
            let right_typed = check_type_rec(*right, env, func_env)?;
            let left_type = left_typed.generic_data.clone();
            let right_type = right_typed.generic_data.clone();

            if left_type != Type::Integer && left_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for less-than-or-equal check".to_string(),
                    expected: Type::Integer,
                    found: left_type,
                    row: row,
                    col_start: left_col_start,
                    col_end: left_col_end,
                });
            }
            if right_type != Type::Integer && right_type != Type::Float
            {
                return Err(Error::TypeError {
                    message: "Invalid operand types for less-than-or-equal check".to_string(),
                    expected: Type::Integer,
                    found: right_type,
                    row: row,
                    col_start: right_col_start,
                    col_end: right_col_end,
                });
            }

            return Ok(RecExpr {
                data: RecExprData::LessThanOrEqual {
                    left: Box::new(left_typed),
                    right: Box::new(right_typed),
                },
                row: row,
                col_start: left_col_start,
                col_end: right_col_end,
                generic_data: Type::Boolean,
            });
        }
        RecExprData::FunctionCall {
            function_name,
            args,
        } =>
        {
            // First we collect all of the given parameter types so we can match against them
            let mut arg_types: Vec<Type> = Vec::new();
            let mut args_typed: Vec<RecExpr<Type>> = Vec::new();
            for (i, arg) in args.iter().enumerate()
            {
                let arg_typed = check_type_rec(arg.clone(), env, func_env)?;
                args_typed.push(arg_typed.clone());
                let arg_type = arg_typed.generic_data;
                arg_types.push(arg_type);
            }

            // Then we look for a matching function in the environment
            let function_type =
                find_matching_function_in_env(&function_name, &arg_types, env, func_env);
            match function_type
            {
                Ok(return_type) =>
                {
                    // Check that the number of arguments matches the number of parameters
                    if arg_types.len() != args.len()
                    {
                        return Err(Error::LocationError {
                            message: format!(
                                "Function '{}' expects {} arguments, but {} were provided",
                                function_name,
                                arg_types.len(),
                                args.len()
                            ),
                            row: rec_expr_row,
                            col_start: rec_expr_col_start,
                            col_end: rec_expr_col_end,
                        });
                    }

                    return Ok(RecExpr {
                        data: RecExprData::FunctionCall {
                            function_name,
                            args: args_typed,
                        },
                        row: rec_expr_row,
                        col_start: rec_expr_col_start,
                        col_end: rec_expr_col_end,
                        generic_data: return_type,
                    });
                }
                Err(error) =>
                {
                    return Err(error);
                }
            }
        }
        RecExprData::ListAccess { variable, index } =>
        {
            let var_type = find_in_env(&variable, &env);
            let index_row = index.row;
            let index_col_start = index.col_start;
            let index_col_end = index.col_end;

            match var_type
            {
                Some(Type::List(elem_type)) =>
                {
                    let index_typed = check_type_rec(*index, env, func_env)?;
                    let index_type = index_typed.generic_data.clone();
                    if index_type != Type::Integer
                    {
                        return Err(Error::TypeError {
                            message: "List index must be an integer".to_string(),
                            expected: Type::Integer,
                            found: index_type,
                            row: index_row,
                            col_start: index_col_start,
                            col_end: index_col_end,
                        });
                    }
                    return Ok(RecExpr {
                        data: RecExprData::ListAccess {
                            variable,
                            index: Box::new(index_typed),
                        },
                        row: rec_expr_row,
                        col_start: rec_expr_col_start,
                        col_end: rec_expr_col_end,
                        generic_data: *elem_type,
                    });
                }
                Some(other_type) =>
                {
                    return Err(Error::TypeError {
                        message: format!(
                            "Variable '{}' is of type {:?}, not a list",
                            variable, other_type
                        ),
                        expected: Type::List(Box::new(Type::Undefined)),
                        found: other_type,
                        row: rec_expr_row,
                        col_start: rec_expr_col_start,
                        col_end: rec_expr_col_end,
                    });
                }
                None =>
                {
                    return Err(Error::TypeError {
                        message: format!("Variable '{}' is not defined", variable),
                        expected: Type::Undefined,
                        found: Type::Undefined,
                        row: rec_expr_row,
                        col_start: rec_expr_col_start,
                        col_end: rec_expr_col_end,
                    });
                }
            }
        }
        RecExprData::Variable { name } =>
        {
            let var_type = find_in_env(&name, &env);
            match var_type
            {
                Some(t) =>
                {
                    return Ok(RecExpr {
                        data: RecExprData::Variable { name },
                        row: rec_expr_row,
                        col_start: rec_expr_col_start,
                        col_end: rec_expr_col_end,
                        generic_data: t,
                    });
                }
                None => Err(Error::TypeError {
                    message: format!("Variable '{}' is not defined", name),
                    expected: Type::Undefined,
                    found: Type::Undefined,
                    row: rec_expr_row,
                    col_start: rec_expr_col_start,
                    col_end: rec_expr_col_end,
                }),
            }
        }

        _ =>
        {
            unimplemented!(
                "check_type_rec not implemented for this RecExprData variant: {:?}",
                rec_expr.data
            );
        }
    };
}
