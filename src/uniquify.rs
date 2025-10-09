use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::format;

use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;

type VariableScope = HashMap<String, String>;

type VariableEnvironment = Vec<VariableScope>;

struct VariableCollection {
    names: HashSet<String>,
    numbered_names: HashMap<String, usize>,
}

pub fn uniquify(program: &mut (Vec<BaseExpr<Type>>, Vec<FunctionType>)) {
    let base_expressions = &mut program.0;
    let functions = &mut program.1;

    let mut env = VariableEnvironment::new();
    env.push(VariableScope::new());

    let mut variable_collection = VariableCollection {
        names: HashSet::new(),
        numbered_names: HashMap::new(),
    };

    for function in functions.iter_mut() {
        uniquify_function(function, &mut variable_collection);
    }

    for base_expr in base_expressions.iter_mut() {
        uniquify_base_expr(base_expr, &mut env, &mut variable_collection);
    }
}

fn exists_in_environment(name: &String, env: &VariableEnvironment) -> Option<String> {
    for scope in env.iter().rev() {
        if let Some(unique_name) = scope.get(name) {
            return Some(unique_name.clone());
        }
    }
    return None;
}

fn generate_unique_name(
    base_name: &String,
    variable_collection: &mut VariableCollection,
) -> String {
    // Check the counter for this base name, or start it at 0 if it doesn't exist
    let counter = variable_collection
        .numbered_names
        .entry(base_name.clone())
        .or_insert(0);
    let unique_name = format!("{}{}", base_name, counter);
    *counter += 1;
    variable_collection.names.insert(unique_name.clone());
    unique_name
}

fn uniquify_function(
    function: &mut FunctionType,
    variable_collection: &mut VariableCollection,
) {
    let mut env = VariableEnvironment::new();
    env.push(VariableScope::new());

    // Adding all the parameters to the environment
    for param in function.param_names.iter_mut() {
        // If it has been defined in the current scope, we can just take its alias
        if let Some(unique_name) = exists_in_environment(param, &env) {
            *param = unique_name;
        }
        // Otherwise, we need to create a new unique name for it
        else {
            // Generate a unique name
            let unique_name = generate_unique_name(param, variable_collection);
            // and add it to the current scope
            env.last_mut()
                .unwrap()
                .insert(param.clone(), unique_name.clone());
            // and rename the variable
            *param = unique_name;
        }
    }

    for base_expr in function.content.iter_mut() {
        uniquify_base_expr(base_expr, &mut env, variable_collection);
    }
}

fn uniquify_base_expr(
    base_expr: &mut BaseExpr<Type>,
    env: &mut VariableEnvironment,
    variable_collection: &mut VariableCollection,
) {
    match &mut base_expr.data {
        BaseExprData::VariableAssignment { var_name, expr } => {
            // If it has been defined in the current scope, we can just take its alias
            if let Some(unique_name) = exists_in_environment(var_name, env) {
                *var_name = unique_name;
            }
            // Otherwise, we need to create a new unique name for it
            else {
                // Generate a unique name
                let unique_name = generate_unique_name(var_name, variable_collection);
                // and add it to the current scope
                env.last_mut()
                    .unwrap()
                    .insert(var_name.clone(), unique_name.clone());
                // and rename the variable
                *var_name = unique_name;
            }

            // Now uniquify the expression being assigned
            uniquify_rec_expr(expr, env, &mut variable_collection.names);
        }
        BaseExprData::Simple { expr } => {
            uniquify_rec_expr(expr, env, &mut variable_collection.names);
        }
        BaseExprData::Return { return_value } => {
            if let Some(ret_val) = return_value {
                uniquify_rec_expr(ret_val, env, &mut variable_collection.names);
            }
        }
        BaseExprData::ForLoop { var_name, until, body } => {
            // If it has been defined in the current scope, we can just take its alias
            if let Some(unique_name) = exists_in_environment(var_name, env) {
                *var_name = unique_name;
            }
            // Otherwise, we need to create a new unique name for it
            else {
                // Generate a unique name
                let unique_name = generate_unique_name(var_name, variable_collection);
                // and add it to the current scope
                env.last_mut()
                    .unwrap()
                    .insert(var_name.clone(), unique_name.clone());
                // and rename the variable
                *var_name = unique_name;
            }

            // Uniquify the until expression
            uniquify_rec_expr(until, env, &mut variable_collection.names);

            // Uniquify the body in a new scope
            env.push(VariableScope::new());
            for expr in body.iter_mut() {
                uniquify_base_expr(expr, env, variable_collection);
            }
            env.pop();
        }
        BaseExprData::IfStatement { condition, body, else_statement } => {
            uniquify_rec_expr(condition, env, &mut variable_collection.names);
            env.push(VariableScope::new());
            for expr in body.iter_mut() {
                uniquify_base_expr(expr, env, variable_collection);
            }
            env.pop();

            if let Some(else_stmt) = else_statement {
                env.push(VariableScope::new());
                uniquify_base_expr(else_stmt, env, variable_collection);
                env.pop();
            }
        }
        BaseExprData::ElseIfStatement { condition, body, else_statement } => {
            uniquify_rec_expr(condition, env, &mut variable_collection.names);
            env.push(VariableScope::new());
            for expr in body.iter_mut() {
                uniquify_base_expr(expr, env, variable_collection);
            }
            env.pop();

            if let Some(else_stmt) = else_statement {
                env.push(VariableScope::new());
                uniquify_base_expr(else_stmt, env, variable_collection);
                env.pop();
            }
        }
        BaseExprData::ElseStatement { body } => {
            env.push(VariableScope::new());
            for expr in body.iter_mut() {
                uniquify_base_expr(expr, env, variable_collection);
            }
            env.pop();
        }
        _ => {}
    }
}

fn uniquify_rec_expr(
    rec_expr: &mut RecExpr<Type>,
    env: &mut VariableEnvironment,
    collected_names: &mut HashSet<String>,
) {
    match &mut rec_expr.data {
        RecExprData::Number { .. } => {}
        RecExprData::Boolean { .. } => {}
        RecExprData::String { .. } => {}
        RecExprData::Variable { name } => {
            if let Some(unique_name) = exists_in_environment(name, env) {
                *name = unique_name;
            } else {
                panic!("Variable '{}' is not defined in the current scope", name);
            }
        }
        RecExprData::FunctionCall { function_name: _, args } => {
            for arg in args.iter_mut() {
                uniquify_rec_expr(arg, env, collected_names);
            }
            print!("{:?}", args);
        }
        RecExprData::Add { left , right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Subtract { left , right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Multiply { left , right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Divide { left , right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Power { left , right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Minus { right } => uniquify_rec_expr(right, env, collected_names),
        RecExprData::And { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Or { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Not { right } => uniquify_rec_expr(right, env, collected_names),
        RecExprData::GreaterThan { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::LessThan { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::GreaterThanOrEqual { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::LessThanOrEqual { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::Equals { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::NotEquals { left, right } => uniquify_binary_operator(left, right, env, collected_names),
        RecExprData::List { elements } => {
            for element in elements.iter_mut() {
                uniquify_rec_expr(element, env, collected_names);
            }
        }
        RecExprData::ListAccess { variable, index } => {
            // Check the variable we are indexing into
            if let Some(unique_name) = exists_in_environment(variable, env) {
                *variable = unique_name;
            } else {
                panic!("Variable '{}' is not defined in the current scope", variable);
            }
            uniquify_rec_expr(index, env, collected_names);
        }
        _ => {}
    }
}

fn uniquify_binary_operator(
    left: &mut RecExpr<Type>,
    right: &mut RecExpr<Type>,
    env: &mut VariableEnvironment,
    collected_names: &mut HashSet<String>,
) {
    uniquify_rec_expr(left, env, collected_names);
    uniquify_rec_expr(right, env, collected_names);
}
