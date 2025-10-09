use std::collections::HashMap;
use std::fmt::format;

use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;

#[derive(Debug, Clone)]
pub enum TacInstruction {
    Assign(String, TacValue),
    BinOp(String, TacValue, BinOp, TacValue),
    UnaryOp(String, UnOp, TacValue),
    Goto(String),
    If(TacValue, String),
    Label(String),
    FunctionLabel(String, Vec<String>), // Function entry point label with name and parameter names
    Call(String, Vec<TacValue>, Option<String>),
    Return(Option<TacValue>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum TacValue {
    Constant(i32),
    Variable(String),
    StringLiteral(String),
}

#[derive(Debug, Clone)]
pub struct TacFunction {
    name: String,
    params: Vec<Type>,
    return_type: Type,
    label: String,
}

struct TacFunctionEnvironment {
    functions: Vec<TacFunction>,
}

struct TacVariableScope {
    variables: HashMap<String, String>,
}

struct TacVariableEnvironment {
    scopes: Vec<TacVariableScope>,
}

// Finds a function in the environment by name and argument types
// Returns the function label if found
fn find_function(
    env: &TacFunctionEnvironment,
    name: &str,
    arg_types: Vec<Type>,
) -> Result<(String, Type), Error> {
    for func in &env.functions {
        if func.name == name && func.params == arg_types {
            return Ok((func.label.clone(), func.return_type.clone()));
        }
    }

    Err(Error::SimpleError {
        message: format!(
            "Function '{}' with specified argument types not found",
            name
        ),
    })
}

fn add_functions(
    functions: Vec<FunctionType>,
    function_env: &mut TacFunctionEnvironment,
    variable_env: &mut TacVariableEnvironment,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    label_counter: &mut i64,
) -> Result<(), Error> {
    for function in functions {
        let label = format!("func_{}", function.name);

        function_env.functions.push(TacFunction {
            name: function.name.clone(),
            params: function.param_types.clone(),
            return_type: function.return_type.clone(),
            label: label.clone(),
        });

        // Now we can generate TAC for the function body
        instructions.push(TacInstruction::FunctionLabel(label, function.param_names.clone()));

        for expr in function.content {
            generate_tac_for_base_expr(
                &expr,
                instructions,
                temp_counter,
                label_counter,
                function_env,
                variable_env,
            )?;
        }
    }
    Ok(())
}

fn find_variable_alias_in_current_scope(
    env: &TacVariableEnvironment,
    var_name: &str,
) -> Option<String> {
    match env.scopes.last() {
        Some(scope) => {
            if let Some(alias) = scope.variables.get(var_name) {
                return Some(alias.clone());
            }
        }
        None => return None,
    }
    None
}

fn find_variable_alias(env: &TacVariableEnvironment, var_name: &str) -> Option<String> {
    for scope in env.scopes.iter().rev() {
        if let Some(alias) = scope.variables.get(var_name) {
            return Some(alias.clone());
        }
    }
    None
}

pub fn generate_tac(
    program: Vec<BaseExpr<Type>>,
    functions: Vec<FunctionType>,
) -> Result<Vec<TacInstruction>, Error> {
    let mut instructions = Vec::new();
    let mut temp_counter = 0;
    let mut label_counter = 0;

    // First we preload the functions into the environment
    let mut function_env = TacFunctionEnvironment {
        functions: Vec::new(),
    };
    let mut variable_env = TacVariableEnvironment {
        scopes: vec![TacVariableScope {
            variables: HashMap::new(),
        }],
    };
    add_functions(
        functions,
        &mut function_env,
        &mut variable_env,
        &mut instructions,
        &mut temp_counter,
        &mut label_counter,
    )?;

    for expr in program {
        generate_tac_for_base_expr(
            &expr,
            &mut instructions,
            &mut temp_counter,
            &mut label_counter,
            &mut function_env,
            &mut variable_env,
        )?;
    }

    print_instructions(&instructions);

    Ok(instructions)
}

fn generate_tac_for_base_expr(
    expr: &BaseExpr<Type>,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    label_counter: &mut i64,
    function_env: &mut TacFunctionEnvironment,
    variable_env: &mut TacVariableEnvironment,
) -> Result<(), Error> {
    match &expr.data {
        BaseExprData::VariableAssignment { var_name, expr } => {
            let value = generate_tac_for_rec_expr(
                expr,
                instructions,
                temp_counter,
                function_env,
                variable_env,
            )?;
            instructions.push(TacInstruction::Assign(var_name.clone(), value));
        }
        BaseExprData::ForLoop {
            var_name,
            until,
            body,
        } => {
            let start_label = format!("L{}", label_counter);
            *label_counter += 1;
            let end_label = format!("L{}", label_counter);
            *label_counter += 1;

            // Initialize loop variable
            instructions.push(TacInstruction::Assign(
                var_name.clone(),
                TacValue::Constant(0),
            ));
            // Start of loop
            instructions.push(TacInstruction::Label(start_label.clone()));
            // Condition check
            let until_value = generate_tac_for_rec_expr(
                until,
                instructions,
                temp_counter,
                function_env,
                variable_env,
            )?;
            let cond_temp = format!("t{}", temp_counter);
            *temp_counter += 1;
            instructions.push(TacInstruction::BinOp(
                cond_temp.clone(),
                TacValue::Variable(var_name.clone()),
                BinOp::Lt,
                until_value,
            ));
            instructions.push(TacInstruction::If(
                TacValue::Variable(cond_temp),
                end_label.clone(),
            ));
            // Loop body
            for body_expr in body {
                generate_tac_for_base_expr(
                    body_expr,
                    instructions,
                    temp_counter,
                    label_counter,
                    function_env,
                    variable_env,
                )?;
            }
            // Increment loop variable
            let increment_temp = format!("t{}", temp_counter);
            *temp_counter += 1;
            instructions.push(TacInstruction::BinOp(
                increment_temp.clone(),
                TacValue::Variable(var_name.clone()),
                BinOp::Add,
                TacValue::Constant(1),
            ));
            instructions.push(TacInstruction::Assign(
                var_name.clone(),
                TacValue::Variable(increment_temp),
            ));
            // Jump back to start
            instructions.push(TacInstruction::Goto(start_label));
            // End of loop
            instructions.push(TacInstruction::Label(end_label));
        }
        BaseExprData::Return { return_value } => {
            if let Some(ret_expr) = return_value {
                let ret_value = generate_tac_for_rec_expr(
                    ret_expr,
                    instructions,
                    temp_counter,
                    function_env,
                    variable_env,
                )?;
                instructions.push(TacInstruction::Return(Some(ret_value)));
            } else {
                instructions.push(TacInstruction::Return(None));
            }
        }
        BaseExprData::IfStatement {
            condition,
            body,
            else_statement,
        } => {
            let next_label = format!("L{}", label_counter);
            *label_counter += 1;
            let end_label = format!("L{}", label_counter);
            *label_counter += 1;

            // Condition
            let cond_value = generate_tac_for_rec_expr(
                condition,
                instructions,
                temp_counter,
                function_env,
                variable_env,
            )?;

            // Jump to the next statement (either end of if-sequence or else branch) if condition is false
            instructions.push(TacInstruction::If(cond_value, next_label.clone()));

            // If body
            for body_expr in body {
                generate_tac_for_base_expr(
                    body_expr,
                    instructions,
                    temp_counter,
                    label_counter,
                    function_env,
                    variable_env,
                )?;
            }

            instructions.push(TacInstruction::Goto(end_label.clone()));

            match else_statement {
                Some(else_expression) => {
                    instructions.push(TacInstruction::Label(next_label));
                    generate_tac_for_base_expr(
                        else_expression,
                        instructions,
                        temp_counter,
                        label_counter,
                        function_env,
                        variable_env,
                    )?;
                }
                None => {
                    // No else branch, just label the next statement
                    instructions.push(TacInstruction::Label(next_label));
                }
            }
            instructions.push(TacInstruction::Label(end_label));
        }
        BaseExprData::ElseIfStatement {
            condition,
            body,
            else_statement,
        } => {
            let next_label = format!("L{}", label_counter);
            *label_counter += 1;
            let end_label = format!("L{}", label_counter);
            *label_counter += 1;

            // Condition
            let cond_value = generate_tac_for_rec_expr(
                condition,
                instructions,
                temp_counter,
                function_env,
                variable_env,
            )?;

            // Jump to the next statement (either end of if-sequence or else branch) if condition is false
            instructions.push(TacInstruction::If(cond_value, next_label.clone()));

            // If body
            for body_expr in body {
                generate_tac_for_base_expr(
                    body_expr,
                    instructions,
                    temp_counter,
                    label_counter,
                    function_env,
                    variable_env,
                )?;
            }

            instructions.push(TacInstruction::Goto(end_label.clone()));

            match else_statement {
                Some(else_expression) => {
                    // No else branch, just label the next statement
                    instructions.push(TacInstruction::Label(next_label));
                    generate_tac_for_base_expr(
                        else_expression,
                        instructions,
                        temp_counter,
                        label_counter,
                        function_env,
                        variable_env,
                    )?;
                }
                None => {
                    // No else branch, just label the next statement
                    instructions.push(TacInstruction::Label(next_label));
                }
            }
            instructions.push(TacInstruction::Label(end_label));
        }
        BaseExprData::ElseStatement { body } => {
            // Else body
            for body_expr in body {
                generate_tac_for_base_expr(
                    body_expr,
                    instructions,
                    temp_counter,
                    label_counter,
                    function_env,
                    variable_env,
                )?;
            }
        }
        _ => {
            // For other base expressions, we can ignore them or handle as needed
        }
    }
    Ok(())
}

fn generate_tac_for_rec_expr(
    expr: &RecExpr<Type>,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    function_env: &mut TacFunctionEnvironment,
    variable_env: &mut TacVariableEnvironment,
) -> Result<TacValue, Error> {
    match &expr.data {
        RecExprData::Number { number } => Ok(TacValue::Constant(*number)),
        RecExprData::String { value } => Ok(TacValue::StringLiteral(value.clone())),
        RecExprData::Variable { name } => Ok(TacValue::Variable(name.clone())),
        RecExprData::Boolean { value } => Ok(TacValue::Constant(if *value { 1 } else { 0 })),
        RecExprData::Add { left, right } => generate_binary_op_tac(
            &BinOp::Add,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::Subtract { left, right } => generate_binary_op_tac(
            &BinOp::Sub,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::Multiply { left, right } => generate_binary_op_tac(
            &BinOp::Mul,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::Divide { left, right } => generate_binary_op_tac(
            &BinOp::Div,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::Equals { left, right } => generate_binary_op_tac(
            &BinOp::Eq,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::NotEquals { left, right } => generate_binary_op_tac(
            &BinOp::Ne,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::LessThan { left, right } => generate_binary_op_tac(
            &BinOp::Lt,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::LessThanOrEqual { left, right } => generate_binary_op_tac(
            &BinOp::Le,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::GreaterThan { left, right } => generate_binary_op_tac(
            &BinOp::Gt,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::GreaterThanOrEqual { left, right } => generate_binary_op_tac(
            &BinOp::Ge,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::Or { left, right } => generate_binary_op_tac(
            &BinOp::Or,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),
        RecExprData::And { left, right } => generate_binary_op_tac(
            &BinOp::And,
            left,
            right,
            instructions,
            temp_counter,
            function_env,
            variable_env,
        ),

        RecExprData::Not { right } => {
            let operand = generate_tac_for_rec_expr(
                right,
                instructions,
                temp_counter,
                function_env,
                variable_env,
            )?;
            let temp_var = format!("t{}", temp_counter);
            *temp_counter += 1;
            instructions.push(TacInstruction::UnaryOp(
                temp_var.clone(),
                UnOp::Neg,
                operand,
            ));
            Ok(TacValue::Variable(temp_var))
        }

        RecExprData::List { elements } => Err(Error::SimpleError {
            message: "List expressions are not supported in TAC generation".to_string(),
        }),

        RecExprData::FunctionCall {
            function_name,
            args,
        } => {
            let mut arg_values = Vec::new();
            let mut arg_types = Vec::new();
            for arg in args {
                let arg_value = generate_tac_for_rec_expr(
                    arg,
                    instructions,
                    temp_counter,
                    function_env,
                    variable_env,
                )?;
                arg_values.push(arg_value);
                arg_types.push(arg.generic_data.clone());
            }
            let (function_label, return_type) =
                match find_function(function_env, function_name, arg_types) {
                    Ok((label, return_type)) => (label, return_type),
                    Err(e) => return Err(e),
                };

            match return_type {
                Type::Undefined => {
                    instructions.push(TacInstruction::Call(function_label, arg_values, None));
                    return Ok(TacValue::Variable(
                        "TODO fix undefined return type".to_string(),
                    )); // or some other placeholder for void
                }
                _ => {
                    // The return value will be stored in temp_var
                    let temp_var = format!("t{}", temp_counter);
                    *temp_counter += 1;
                    instructions.push(TacInstruction::Call(
                        function_label,
                        arg_values,
                        Some(temp_var.clone()),
                    ));
                    Ok(TacValue::Variable(temp_var))
                }
            }
        }

        _ => Err(Error::SimpleError {
            message: format!("Unsupported expression type {:?}", expr.data),
        }),
    }
}

fn generate_binary_op_tac(
    operator: &BinOp,
    left: &RecExpr<Type>,
    right: &RecExpr<Type>,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    function_env: &mut TacFunctionEnvironment,
    variable_env: &mut TacVariableEnvironment,
) -> Result<TacValue, Error> {
    let left_val =
        generate_tac_for_rec_expr(left, instructions, temp_counter, function_env, variable_env)?;
    let right_val = generate_tac_for_rec_expr(
        right,
        instructions,
        temp_counter,
        function_env,
        variable_env,
    )?;

    let temp_var = format!("t{}", temp_counter);
    *temp_counter += 1;
    instructions.push(TacInstruction::BinOp(
        temp_var.clone(),
        left_val,
        *operator,
        right_val,
    ));
    Ok(TacValue::Variable(temp_var))
}

fn print_instructions(instructions: &Vec<TacInstruction>) {
    for instr in instructions {
        print_instruction(instr);
    }
}

fn print_instruction(instr: &TacInstruction) {
    match instr {
        TacInstruction::Assign(var, value) => {
            println!("{} = {:?}", var, value);
        }
        TacInstruction::BinOp(result, left, op, right) => {
            let op_str = match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                BinOp::Eq => "==",
                BinOp::Ne => "!=",
                BinOp::Lt => "<",
                BinOp::Le => "<=",
                BinOp::Gt => ">",
                BinOp::Ge => ">=",
                BinOp::And => "&",
                BinOp::Or => "|",
                _ => unreachable!(),
            };
            println!("{} = {:?} {} {:?}", result, left, op_str, right);
        }
        TacInstruction::UnaryOp(result, op, operand) => {
            let op_str = match op {
                UnOp::Neg => "-",
                UnOp::Not => "!",
            };
            println!("{} = {}{:?}", result, op_str, operand);
        }
        TacInstruction::Goto(label) => {
            println!("goto {}", label);
        }
        TacInstruction::If(cond, label) => {
            println!("if not {:?} goto {}", cond, label);
        }
        TacInstruction::Label(label) => {
            println!("{}:", label);
        }
        TacInstruction::FunctionLabel(name, params) => {
            println!("function {}({})", name, params.join(", "));
        }
        TacInstruction::Call(func, args, ret) => {
            if let Some(ret_var) = ret {
                println!("{} = call {}({:?})", ret_var, func, args);
            } else {
                println!("call {}({:?})", func, args);
            }
        }
        TacInstruction::Return(ret) => {
            if let Some(ret_var) = ret {
                println!("return {:?}", ret_var);
            } else {
                println!("return");
            }
            println!("end");
        }
    }
}
