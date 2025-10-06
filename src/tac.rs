use std::fmt::format;

use crate::parser;
use crate::parser::BaseExpr;
use crate::parser::BaseExprData;
use crate::parser::RecExpr;
use crate::parser::RecExprData;
use crate::tokenizer::Error;
use crate::typechecker::Type;

#[derive(Debug, Clone)]
pub enum TacInstruction {
    Assign(String, TacValue),
    BinOp(String, TacValue, BinOp, TacValue),
    UnaryOp(String, UnOp, TacValue),
    Goto(String),
    If(String, String),
    Label(String),
    Call(String, Vec<String>, Option<String>),
    Return(Option<String>),
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

pub fn generate_tac(program: Vec<BaseExpr<Type>>) -> Result<Vec<TacInstruction>, Error> {
    let mut instructions = Vec::new();
    let mut temp_counter = 0;
    let mut label_counter = 0;

    for expr in program
    {
        generate_tac_for_base_expr(
            &expr,
            &mut instructions,
            &mut temp_counter,
            &mut label_counter,
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
) -> Result<(), Error> {
    match &expr.data
    {
        BaseExprData::VariableAssignment { var_name, expr } =>
        {
            let value = generate_tac_for_rec_expr(expr, instructions, temp_counter)?;
            instructions.push(TacInstruction::Assign(var_name.clone(), value));
        }
        BaseExprData::ForLoop {
            var_name,
            until,
            body,
        } =>
        {
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
            let until_value = generate_tac_for_rec_expr(until, instructions, temp_counter)?;
            let cond_temp = format!("t{}", temp_counter);
            *temp_counter += 1;
            instructions.push(TacInstruction::BinOp(
                cond_temp.clone(),
                TacValue::Variable(var_name.clone()),
                BinOp::Ge,
                until_value,
            ));
            instructions.push(TacInstruction::If(cond_temp, end_label.clone()));
            // Loop body
            for body_expr in body
            {
                generate_tac_for_base_expr(body_expr, instructions, temp_counter, label_counter)?;
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
        _ =>
        {
            // For other base expressions, we can ignore them or handle as needed
        }
    }
    Ok(())
}

fn generate_tac_for_rec_expr(
    expr: &RecExpr<Type>,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
) -> Result<TacValue, Error> {
    match &expr.data
    {
        RecExprData::Number { number } => Ok(TacValue::Constant(*number)),
        RecExprData::String { value } => Ok(TacValue::StringLiteral(value.clone())),
        RecExprData::Variable { name } => Ok(TacValue::Variable(name.clone())),
        RecExprData::Add { left, right } =>
        {
            generate_binary_op_tac(&BinOp::Add, left, right, instructions, temp_counter)
        }
        RecExprData::Subtract { left, right } =>
        {
            generate_binary_op_tac(&BinOp::Sub, left, right, instructions, temp_counter)
        }
        RecExprData::Multiply { left, right } =>
        {
            generate_binary_op_tac(&BinOp::Mul, left, right, instructions, temp_counter)
        }
        RecExprData::Divide { left, right } =>
        {
            generate_binary_op_tac(&BinOp::Div, left, right, instructions, temp_counter)
        }

        _ => Err(Error::SimpleError {
            message: "Unsupported expression type".to_string(),
        }),
    }
}

fn generate_binary_op_tac(
    operator: &BinOp,
    left: &RecExpr<Type>,
    right: &RecExpr<Type>,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
) -> Result<TacValue, Error> {
    let left_val = generate_tac_for_rec_expr(left, instructions, temp_counter)?;
    let right_val = generate_tac_for_rec_expr(right, instructions, temp_counter)?;

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
    for instr in instructions
    {
        print_instruction(instr);
    }
}

fn print_instruction(instr: &TacInstruction) {
    match instr
    {
        TacInstruction::Assign(var, value) =>
        {
            println!("{} = {:?}", var, value);
        }
        TacInstruction::BinOp(result, left, op, right) =>
        {
            let op_str = match op
            {
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
                _ => unreachable!(),
            };
            println!("{} = {:?} {} {:?}", result, left, op_str, right);
        }
        TacInstruction::UnaryOp(result, op, operand) =>
        {
            let op_str = match op
            {
                UnOp::Neg => "-",
                UnOp::Not => "!",
            };
            println!("{} = {}{:?}", result, op_str, operand);
        }
        TacInstruction::Goto(label) =>
        {
            println!("goto {}", label);
        }
        TacInstruction::If(cond, label) =>
        {
            println!("if {} goto {}", cond, label);
        }
        TacInstruction::Label(label) =>
        {
            println!("{}:", label);
        }
        TacInstruction::Call(func, args, ret) =>
        {
            if let Some(ret_var) = ret
            {
                println!("{} = call {}({})", ret_var, func, args.join(", "));
            }
            else
            {
                println!("call {}({})", func, args.join(", "));
            }
        }
        TacInstruction::Return(ret) =>
        {
            if let Some(ret_var) = ret
            {
                println!("return {}", ret_var);
            }
            else
            {
                println!("return");
            }
            println!("end");
        }
    }
}
