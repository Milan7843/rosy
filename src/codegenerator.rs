use std::collections::HashMap;
use std::collections::HashSet;

use crate::tac;
use crate::tac::BinOp;
use crate::tac::UnOp;
use crate::tac::TacInstruction;
use crate::tac::TacValue;
use crate::tokenizer::Error;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum RegisterType {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum Register {
    General(RegisterType),
    Extended(u8),
}

fn to_register(reg_num: isize) -> Register {
    match reg_num {
        0 => Register::General(RegisterType::RCX),
        1 => Register::General(RegisterType::RDX),
        2 => Register::General(RegisterType::RSI),
        3 => Register::General(RegisterType::RDI),
        4 => Register::Extended(8),  // r8
        5 => Register::Extended(9),  // r9
        6 => Register::Extended(10), // r10
        7 => Register::General(RegisterType::RBX),
        8 => Register::Extended(12), // r12
        9 => Register::Extended(13), // r13
        10 => Register::Extended(14), // r14
        -1 => Register::General(RegisterType::RAX),
        -2 => Register::General(RegisterType::RSP),
        -3 => Register::General(RegisterType::RBP),
        -4 => Register::Extended(11), // r11
        -5 => Register::Extended(15), // r15
        //_ if reg_num > 10 => Register::Extended((reg_num - 10) as u8), // TODO Stack location [rsp + (reg_num - 10) * 8] 
        _ => panic!("Invalid register number: {}", reg_num),
    }
}

fn is_caller_saved(register: &Register) -> bool {
    match register {
        Register::General(reg_type) => match reg_type {
            RegisterType::RAX | RegisterType::RCX | RegisterType::RDX | RegisterType::RSI | RegisterType::RDI => true,
            RegisterType::RSP | RegisterType::RBP | RegisterType::RBX => false,
        },
        Register::Extended(num) => match num {
            8 | 9 | 10 | 11 => true,  // r8, r9, r10, r11
            12 | 13 | 14 | 15 => false, // r12, r13, r14, r15
            _ => panic!("Invalid extended register number (code gen): {}", num),
        },
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Argument {
    Register(Register),
    Immediate(i64),
    Label(String),
    MemoryAddress(u64), // e.g., [rax], [rbx + 4]
}

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    Mov(Argument, Argument), // dest, src
    Add(Argument, Argument), // dest, src
    Sub(Argument, Argument), // dest, src
    Mul(Argument, Argument), // dest, src
    Div(Argument, Argument), // dest, src
    Xor(Argument, Argument), // dest, src
    And(Argument, Argument), // dest, src
    Or(Argument, Argument),  // dest, src
    Not(Argument),           // src
    Cmp(Argument, Argument), // op1, op2
    Jmp(String),           // label
    Je(String),            // label
    Jne(String),           // label
    Jg(String),            // label
    Jge(String),           // label
    Jl(String),            // label
    Jle(String),           // label
    Ret,
    Push(Argument), // value or register
    Pop(Argument),  // register
    Label(String),  // label name
    Syscall(String), // name of the syscall function (used for IAT)
    Nop,
}

fn get_register(variable_name: &String, register_allocation: &HashMap<String, isize>) -> Result<Register, Error> {
    if let Some(&reg_num) = register_allocation.get(variable_name) {
        Ok(to_register(reg_num))
    } else {
        Err(Error::SimpleError{message: format!("Variable {} not found in register allocation", variable_name)})
    }
}

pub fn generate_code(tac: &Vec<TacInstruction>, register_allocation: &HashMap<String, isize>, liveness: &Vec<HashSet<String>>) -> Result<Vec<Instruction>, Error> {
    let mut instructions = Vec::new();

    for (instruction_index, tac_inst) in tac.iter().enumerate() {
        match tac_inst {
            TacInstruction::Label(name) => {
                instructions.push(Instruction::Label(name.clone()));
            }
            TacInstruction::FunctionLabel(name, params) => {
                instructions.push(Instruction::Label(name.clone()));
                // Now we need to move the parameters from their argument registers to their allocated registers
                let arg_registers = [
                    Register::General(RegisterType::RCX),
                    Register::General(RegisterType::RDX),
                    Register::Extended(8),  // r8
                    Register::Extended(9),  // r9
                ];
                for (i, param) in params.iter().enumerate() {
                    if let Some(&reg_num) = register_allocation.get(param) {
                        let dest_reg = to_register(reg_num);
                        if i < arg_registers.len() {
                            // Move from argument register to allocated register
                            instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Register(arg_registers[i].clone())));
                        } else {
                            // Parameter passed on stack, need to load from [rsp + offset]
                            let stack_offset = ((i - arg_registers.len()) * 8) as u64;
                            instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::MemoryAddress(stack_offset)));
                        }
                    } else {
                        return Err(Error::SimpleError{message: format!("Function parameter {} not found in register allocation", param)});
                    }
                }
            }
            TacInstruction::Assign(dest, value) => {
                let dest_reg = get_register(dest, register_allocation)?;
                match value {
                    TacValue::Constant(imm) => {
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Immediate(*imm as i64)));
                    }
                    TacValue::Variable(var) => {
                        let src_reg = get_register(var, register_allocation)?;
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Register(src_reg)));
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported TacValue in Assign: {:?}", value)});
                    }
                }
            }
            TacInstruction::BinOp(dest, left, op, right) => {
                let dest_reg = get_register(dest, register_allocation)?;
                match (left, right) {
                    // If we want to add two variables into a third, we first move one into the dest register, then add the other
                    (TacValue::Variable(var_left), TacValue::Variable(var_right)) => {
                        let left_reg = get_register(var_left, register_allocation)?;
                        let right_reg = get_register(var_right, register_allocation)?;
                        // Move left operand to dest register
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(left_reg)));
                        // Perform operation with right operand
                        match op {
                            BinOp::Add => instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            _ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
                        };
                    }
                    (TacValue::Variable(var_left), TacValue::Constant(imm_right)) => {
                        let left_reg = get_register(var_left, register_allocation)?;
                        // Move left operand to dest register
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(left_reg)));
                        // Perform operation with immediate right operand
                        match op {
                            BinOp::Add => instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            _ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
                        };
                    }
                    (TacValue::Constant(imm_left), TacValue::Variable(var_right)) => {
                        let right_reg = get_register(var_right, register_allocation)?;
                        // Move left immediate to dest register
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Immediate(*imm_left as i64)));
                        // Perform operation with right operand
                        match op {
                            BinOp::Add => instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            _ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
                        };
                    }
                    (TacValue::Constant(imm_left), TacValue::Constant(imm_right)) => {
                        // Move left immediate to dest register
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Immediate(*imm_left as i64)));
                        // Perform operation with right immediate
                        match op {
                            BinOp::Add => instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
                            _ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
                        };
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported TacValue combination in BinOp: {:?}, {:?}", left, right)});
                    }
                };
            }
            TacInstruction::Return(value) => {
                match value {
                    Some(TacValue::Variable(var)) => {
                        let ret_reg = get_register(var, register_allocation)?;
                        instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX)), Argument::Register(ret_reg)));
                    }
                    Some(TacValue::Constant(imm)) => {
                        instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX)), Argument::Immediate(*imm as i64)));
                    }
                    None => {
                        // No return value
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported TacValue in Return: {:?}", value)});
                    }
                }
                instructions.push(Instruction::Ret);
            }
            TacInstruction::UnaryOp(name, operator, operand) => {
                let dest_reg = get_register(name, register_allocation)?;

                match operand {
                    TacValue::Variable(var) => {
                        let src_reg = get_register(var, register_allocation)?;
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(src_reg)));
                    }
                    TacValue::Constant(imm) => {
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Immediate(*imm as i64)));
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported TacValue in UnaryOp: {:?}", operand)});
                    }
                }
                match operator {
                    UnOp::Neg => {
                        instructions.push(Instruction::Sub(Argument::Register(dest_reg.clone()), Argument::Immediate(0)));
                    }
                    UnOp::Not => {
                        instructions.push(Instruction::Not(Argument::Register(dest_reg.clone())));
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported unary operation: {:?}", operator)});
                    }
                }
            }
            TacInstruction::CompareAndGoto(left, right, comparison, label) => {
                match (left, right) {
                    // Two registers
                    (TacValue::Variable(var_left), TacValue::Variable(var_right)) => {
                        let left_reg = get_register(var_left, register_allocation)?;
                        let right_reg = get_register(var_right, register_allocation)?;
                        instructions.push(Instruction::Cmp(Argument::Register(left_reg), Argument::Register(right_reg)));
                    }
                    // One register, one immediate
                    (TacValue::Variable(var_left), TacValue::Constant(imm_right)) => {
                        let left_reg = get_register(var_left, register_allocation)?;
                        instructions.push(Instruction::Cmp(Argument::Register(left_reg), Argument::Immediate(*imm_right as i64)));
                    }
                    // One immediate, one register
                    (TacValue::Constant(imm_left), TacValue::Variable(var_right)) => {
                        // Only reg, imm is supported in x86 cmp, so we need to swap the order and adjust the logic accordingly
                        let right_reg = get_register(var_right, register_allocation)?;
                        instructions.push(Instruction::Cmp(Argument::Register(right_reg), Argument::Immediate(*imm_left as i64)));
                    }
                    (TacValue::Constant(imm_left), TacValue::Constant(imm_right)) => {
                        instructions.push(Instruction::Cmp(Argument::Immediate(*imm_left as i64), Argument::Immediate(*imm_right as i64)));
                    }
                    _ => {
                        return Err(Error::SimpleError{message: format!("Unsupported TacValue combination in Compare: {:?}, {:?}", left, right)});
                    }
                }
                match comparison {
                    tac::ComparisonOp::Eq => instructions.push(Instruction::Je(label.clone())),
                    tac::ComparisonOp::Ne => instructions.push(Instruction::Jne(label.clone())),
                    tac::ComparisonOp::Lt => instructions.push(Instruction::Jl(label.clone())),
                    tac::ComparisonOp::Le => instructions.push(Instruction::Jle(label.clone())),
                    tac::ComparisonOp::Gt => instructions.push(Instruction::Jg(label.clone())),
                    tac::ComparisonOp::Ge => instructions.push(Instruction::Jge(label.clone())),
                }
            }
            TacInstruction::Goto(label) => {
                instructions.push(Instruction::Jmp(label.clone()));
            }
            TacInstruction::Call(func_name, args, return_var) => {
                generate_code_for_call(&mut instructions, func_name, args, return_var, register_allocation, liveness, instruction_index, false)?;
            }
            TacInstruction::SysCall(func_name, args, return_var) => {
                generate_code_for_call(&mut instructions, func_name, args, return_var, register_allocation, liveness, instruction_index, true)?;
            }

        }
    }
    Ok(instructions)
}

fn generate_code_for_call(
    instructions: &mut Vec<Instruction>,
    func_name: &String,
    args: &Vec<TacValue>,
    return_var: &Option<String>,
    register_allocation: &HashMap<String, isize>,
    liveness: &Vec<HashSet<String>>,
    instruction_index: usize,
    is_syscall: bool,
) -> Result<(), Error> {
    // Check for active caller-saved registers and save them
    let mut saved_registers = Vec::new();
    if instruction_index < liveness.len() {
        // Check which variables are live at this point
        let live_vars = &liveness[instruction_index];

        let mut live_registers = HashSet::new();
        for var in live_vars {
            if let Some(&reg_num) = register_allocation.get(var) {
                let reg = to_register(reg_num);
                if is_caller_saved(&reg) {
                    live_registers.insert(reg);
                }
            }
        }

        // Push the contents of these registers onto the stack
        for reg in live_registers.iter() {
            instructions.push(Instruction::Push(Argument::Register(reg.clone())));
            saved_registers.push(reg.clone());
        }
    }

    // Move arguments into appropriate registers
    let arg_registers = [
        Register::General(RegisterType::RCX),
        Register::General(RegisterType::RDX),
        Register::Extended(8),  // r8
        Register::Extended(9),  // r9
    ];

    for (i, arg) in args.iter().enumerate() {
        if i < arg_registers.len() {
            match arg {
                TacValue::Variable(var) => {
                    let src_reg = get_register(var, register_allocation)?;
                    instructions.push(Instruction::Mov(Argument::Register(arg_registers[i].clone()), Argument::Register(src_reg)));
                }
                TacValue::Constant(imm) => {
                    instructions.push(Instruction::Mov(Argument::Register(arg_registers[i].clone()), Argument::Immediate(*imm as i64)));
                }
                _ => {
                    return Err(Error::SimpleError{message: format!("Unsupported TacValue in Call argument: {:?}", arg)});
                }
            }
        } else {
            // Push the rest of the arguments onto the stack (right to left)
            match arg {
                TacValue::Variable(var) => {
                    let src_reg = get_register(var, register_allocation)?;
                    instructions.push(Instruction::Push(Argument::Register(src_reg)));
                }
                TacValue::Constant(imm) => {
                    instructions.push(Instruction::Push(Argument::Immediate(*imm as i64)));
                }
                _ => {
                    return Err(Error::SimpleError{message: format!("Unsupported TacValue in Call argument: {:?}", arg)});
                }
            }
        }
    }

    // Call the function
    if is_syscall {
        // System call, we need to know the name of the syscall function to find it in the IAT
        instructions.push(Instruction::Syscall(func_name.clone()));
    } else {
        // Regular function call
        instructions.push(Instruction::Jmp(func_name.clone()));
    }

    // Move return value from rax to the appropriate variable, if needed
    if let Some(ret_var) = return_var {
        let ret_reg = get_register(ret_var, register_allocation)?;
        instructions.push(Instruction::Mov(Argument::Register(ret_reg), Argument::Register(Register::General(RegisterType::RAX))));
    }

    // Restore saved caller-saved registers
    for reg in saved_registers.iter().rev() {
        instructions.push(Instruction::Pop(Argument::Register(reg.clone())));
    }
    Ok(())
}

pub fn print_instructions(instructions: &Vec<Instruction>) {
    for instr in instructions {
        print_instruction(instr);
    }
}

fn print_instruction(instruction: &Instruction) {
    match instruction {
        Instruction::Mov(dest, src) => {
            println!("MOV {:?}, {:?}", dest, src);
        }
        Instruction::Add(dest, src) => {
            println!("ADD {:?}, {:?}", dest, src);
        }
        Instruction::Sub(dest, src) => {
            println!("SUB {:?}, {:?}", dest, src);
        }
        Instruction::Mul(dest, src) => {
            println!("MUL {:?}, {:?}", dest, src);
        }
        Instruction::Div(dest, src) => {
            println!("DIV {:?}, {:?}", dest, src);
        }
        Instruction::Xor(dest, src) => {
            println!("XOR {:?}, {:?}", dest, src);
        }
        Instruction::And(dest, src) => {
            println!("AND {:?}, {:?}", dest, src);
        }
        Instruction::Or(dest, src) => {
            println!("OR {:?}, {:?}", dest, src);
        }
        Instruction::Not(src) => {
            println!("NOT {:?}", src);
        }
        Instruction::Cmp(op1, op2) => {
            println!("CMP {:?}, {:?}", op1, op2);
        }
        Instruction::Jmp(label) => {
            println!("JMP {:?}", label);
        }
        Instruction::Je(label) => {
            println!("JE {:?}", label);
        }
        Instruction::Jne(label) => {
            println!("JNE {:?}", label);
        }
        Instruction::Jg(label) => {
            println!("JG {:?}", label);
        }
        Instruction::Jge(label) => {
            println!("JGE {:?}", label);
        }
        Instruction::Jl(label) => {
            println!("JL {:?}", label);
        }
        Instruction::Jle(label) => {
            println!("JLE {:?}", label);
        }
        Instruction::Ret => {
            println!("RET");
        }
        Instruction::Push(arg) => {
            println!("PUSH {:?}", arg);
        }
        Instruction::Pop(arg) => {
            println!("POP {:?}", arg);
        }
        Instruction::Label(name) => {
            println!("{}:", name);
        }
        Instruction::Syscall(num) => {
            println!("SYSCALL {}", num);
        }
        Instruction::Nop => {
            println!("NOP");
        }
    }
}
