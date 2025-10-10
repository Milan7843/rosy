use std::collections::HashMap;

use crate::tac;
use crate::tac::BinOp;
use crate::tac::TacInstruction;
use crate::tac::TacValue;
use crate::tokenizer::Error;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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
        _ if reg_num > 10 => Register::Extended((reg_num - 10) as u8), // TODO Stack location [rsp + (reg_num - 10) * 8] 
        _ => panic!("Invalid register number: {}", reg_num),
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Argument {
    Register(Register),
    Immediate(u64),
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
    Cmp(Argument, Argument), // op1, op2
    Jmp(Argument),           // label
    Je(Argument),            // label
    Jne(Argument),           // label
    Jg(Argument),            // label
    Jge(Argument),           // label
    Jl(Argument),            // label
    Jle(Argument),           // label
    Call(Argument),          // label or function name
    Ret,
    Push(Argument), // value or register
    Pop(Argument),  // register
    Label(String),  // label name
    Syscall(u32),   // syscall number
    Nop,
}

fn get_register(variable_name: &String, register_allocation: &HashMap<String, isize>) -> Result<Register, Error> {
    if let Some(&reg_num) = register_allocation.get(variable_name) {
        Ok(to_register(reg_num))
    } else {
        Err(Error::SimpleError{message: format!("Variable {} not found in register allocation", variable_name)})
    }
}

pub fn generate_code(tac: &Vec<TacInstruction>, register_allocation: &HashMap<String, isize>) -> Result<Vec<Instruction>, Error> {
    let mut instructions = Vec::new();

    for tac_inst in tac {
        match tac_inst {
            TacInstruction::Label(name) => {
                instructions.push(Instruction::Label(*name));
            }
            TacInstruction::FunctionLabel(name, params) => {
                instructions.push(Instruction::Label(*name));
                // Function parameters are assumed to be in the correct registers
            }
            TacInstruction::Assign(dest, value) => {
                let dest_reg = get_register(dest, register_allocation)?;
                match value {
                    TacValue::Constant(imm) => {
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Immediate(*imm as u64)));
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
                        instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Register(left_reg)));
                        // Perform operation with right operand
                        match op {
                            BinOp::Add => instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Register(right_reg))),
                            BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Register(right_reg))),
                        };
                    }
                };
            }
        }
    }
    Ok(instructions)
}
