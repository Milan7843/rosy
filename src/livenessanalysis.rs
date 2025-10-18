use std::collections::HashSet;

use crate::tac::TacInstruction;
use crate::tac::TacValue;
use crate::tac::VariableValue;

pub fn analyze_liveness(instructions: &Vec<TacInstruction>) -> Vec<HashSet<VariableValue>> {
     // Initialize liveness vector with empty sets
    let mut liveness: Vec<HashSet<VariableValue>> = Vec::with_capacity(instructions.len() + 1);

    // Initialize the last liveness set
    liveness.push(HashSet::new());

    for (index, instruction) in instructions.iter().rev().enumerate() {

        let liveness_after = &mut liveness[index];
        let mut liveness_before = liveness_after.clone();

        match instruction {
            TacInstruction::Label(_) => {
                // Labels do not affect liveness
            }
            TacInstruction::MovRSPTo(var) => {
                liveness_before.remove(var);
            }
            TacInstruction::Push(value) => {
                match value {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
            }
            TacInstruction::Pop(var) => {
                liveness_before.remove(var);
            }
            TacInstruction::FunctionLabel(name, params) => {
                // Function entry point: parameters are live
                for param in params {
                    liveness_before.remove(&VariableValue::Variable(param.clone()));
                }
            }
            TacInstruction::Assign(dest, value) => {
                // Remove the destination variable from the liveness set
                liveness_before.remove(dest);
                match value {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
            }
            TacInstruction::BinOp(dest, left, op, right) => {
                // Remove the destination variable from the liveness set
                liveness_before.remove(dest);
                match left {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
                match right {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
            }
            TacInstruction::UnaryOp(dest, op, value) => {
                // Remove the destination variable from the liveness set
                liveness_before.remove(dest);
                match value {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
            }
            TacInstruction::CompareAndGoto(left, right, _, _) => {
                match left {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
                match right {
                    TacValue::Variable(var) => {
                        liveness_before.insert(VariableValue::Variable(var.clone()));
                    }
                    _ => {}
                }
            }
            TacInstruction::Goto(label) => {
                // Note: The label itself does not affect liveness
            }
            TacInstruction::Return(value) => {
                if let Some(val) = value {
                    match val {
                        TacValue::Variable(var) => {
                            liveness_before.insert(VariableValue::Variable(var.clone()));
                        }
                        _ => {}
                    }
                }
            }
            TacInstruction::Call(function_name, args, return_var) => {
                if let Some(ret_var) = return_var {
                    liveness_before.remove(ret_var);
                }
                for (arg_index, arg) in args.iter().enumerate() {
                    liveness_before.insert(arg.clone());
                }
            }
            TacInstruction::ExternCall(function_name, args, return_var) => {
                if let Some(ret_var) = return_var {
                    liveness_before.remove(ret_var);
                }
                for (arg_index, arg) in args.iter().enumerate() {
                    liveness_before.insert(arg.clone());
                }
            }
            TacInstruction::ProgramStart() => {
                // Program start does not affect liveness
            }
            TacInstruction::DirectInstruction(_) => {
                // Direct instructions may affect liveness, but we don't analyze them here
            }
        }

        liveness.push(liveness_before);
    }

    liveness.reverse();
    return liveness;
}

pub fn print_code_with_liveness(instructions: &Vec<TacInstruction>, liveness: &Vec<HashSet<VariableValue>>) {
    for (index, instruction) in instructions.iter().enumerate() {
        let live_vars_before = &liveness[index];
        let live_vars_after = &liveness[index+1];
        println!("{:3}: {:?} \t Live vars: {:?} -> {:?}", index, instruction, live_vars_before, live_vars_after);
    }
}