use std::collections::HashMap;
use std::collections::HashSet;

use crate::tac;
use crate::tac::BinOp;
use crate::tac::UnOp;
use crate::tac::TacInstruction;
use crate::tac::TacValue;
use crate::tac::VariableValue;
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

#[derive(Debug, Clone, Eq, Hash)]
pub enum Register {
	General(RegisterType, RegisterSize),
	Extended(u8, RegisterSize),
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum RegisterSize {
	Byte, // 8 bit registers like AL, BL
	Word, // 16 bit registers like AX, BX
	DoubleWord, // 32 bit registers like EAX, EBX
	QuadWord, // 64 bit registers like RAX, RBX
}

impl std::fmt::Display for Register {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Register::General(reg_type, reg_size) => {
				match reg_size {
					RegisterSize::Byte => {
						let reg_str = match reg_type {
							RegisterType::RAX => "al",
							RegisterType::RBX => "bl",
							RegisterType::RCX => "cl",
							RegisterType::RDX => "dl",
							RegisterType::RSI => "sil",
							RegisterType::RDI => "dil",
							RegisterType::RSP => "spl",
							RegisterType::RBP => "bpl",
						};
						write!(f, "{}", reg_str)
					}
					RegisterSize::Word => {
						let reg_str = match reg_type {
							RegisterType::RAX => "ax",
							RegisterType::RBX => "bx",
							RegisterType::RCX => "cx",
							RegisterType::RDX => "dx",
							RegisterType::RSI => "si",
							RegisterType::RDI => "di",
							RegisterType::RSP => "sp",
							RegisterType::RBP => "bp",
						};
						write!(f, "{}", reg_str)
					}
					RegisterSize::DoubleWord => {
						let reg_str = match reg_type {
							RegisterType::RAX => "eax",
							RegisterType::RBX => "ebx",
							RegisterType::RCX => "ecx",
							RegisterType::RDX => "edx",
							RegisterType::RSI => "esi",
							RegisterType::RDI => "edi",
							RegisterType::RSP => "esp",
							RegisterType::RBP => "ebp",
						};
						write!(f, "{}", reg_str)
					}
					RegisterSize::QuadWord => {
						let reg_str = match reg_type {
							RegisterType::RAX => "rax",
							RegisterType::RBX => "rbx",
							RegisterType::RCX => "rcx",
							RegisterType::RDX => "rdx",
							RegisterType::RSI => "rsi",
							RegisterType::RDI => "rdi",
							RegisterType::RSP => "rsp",
							RegisterType::RBP => "rbp",
						};
						write!(f, "{}", reg_str)
					}
				}
			}
			Register::Extended(num, reg_size) => {
				match reg_size {
					RegisterSize::Byte => write!(f, "r{}b", num),
					RegisterSize::Word => write!(f, "r{}w", num),
					RegisterSize::DoubleWord => write!(f, "r{}d", num),
					RegisterSize::QuadWord => write!(f, "r{}", num),
				}
			}
		}
	}
}

// Implement equality for register ignoring size
impl PartialEq for Register {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Register::General(reg_type1, _), Register::General(reg_type2, _)) => reg_type1 == reg_type2,
			(Register::Extended(num1, _), Register::Extended(num2, _)) => num1 == num2,
			_ => false,
		}
	}
}

fn to_register(reg_num: isize) -> Register {
	match reg_num {
		0 => Register::General(RegisterType::RCX, RegisterSize::QuadWord),
		1 => Register::General(RegisterType::RDX, RegisterSize::QuadWord),
		2 => Register::General(RegisterType::RSI, RegisterSize::QuadWord),
		3 => Register::General(RegisterType::RDI, RegisterSize::QuadWord),
		4 => Register::Extended(8, RegisterSize::QuadWord),  // r8
		5 => Register::Extended(9, RegisterSize::QuadWord),  // r9
		6 => Register::Extended(10, RegisterSize::QuadWord), // r10
		7 => Register::General(RegisterType::RBX, RegisterSize::QuadWord),
		8 => Register::Extended(12, RegisterSize::QuadWord), // r12
		9 => Register::Extended(13, RegisterSize::QuadWord), // r13
		10 => Register::Extended(14, RegisterSize::QuadWord), // r14
		-1 => Register::General(RegisterType::RAX, RegisterSize::QuadWord),
		-2 => Register::General(RegisterType::RSP, RegisterSize::QuadWord),
		-3 => Register::General(RegisterType::RBP, RegisterSize::QuadWord),
		-4 => Register::Extended(11, RegisterSize::QuadWord), // r11
		-5 => Register::Extended(15, RegisterSize::QuadWord), // r15
		//_ if reg_num > 10 => Register::Extended((reg_num - 10) as u8), // TODO Stack location [rsp + (reg_num - 10) * 8] 
		_ => panic!("Invalid register number: {}", reg_num),
	}
}

pub fn from_register(register: &Register) -> isize {
	match register {
		Register::General(reg_type, _) => match reg_type {
			RegisterType::RCX => 0,
			RegisterType::RDX => 1,
			RegisterType::RSI => 2,
			RegisterType::RDI => 3,
			RegisterType::RBX => 7,
			RegisterType::RAX => -1,
			RegisterType::RSP => -2,
			RegisterType::RBP => -3,
		},
		Register::Extended(num, _) => match num {
			8 => 4,  // r8
			9 => 5,  // r9
			10 => 6, // r10
			11 => -4, // r11
			12 => 8, // r12
			13 => 9, // r13
			14 => 10, // r14
			15 => -5, // r15
			_ => panic!("Invalid extended register number (from_register): {}", num),
		},
	}
}

fn is_caller_saved(register: &Register) -> bool {
	match register {
		Register::General(reg_type, _) => match reg_type {
			RegisterType::RAX | RegisterType::RCX | RegisterType::RDX | RegisterType::RSI | RegisterType::RDI => true,
			RegisterType::RSP | RegisterType::RBP | RegisterType::RBX => false,
		},
		Register::Extended(num, _) => match num {
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
	MemoryAddressDirect(u64), // e.g., [rax], [rbx + 4]
	MemoryAddressRegister(Register)
}

impl std::fmt::Display for Argument {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Argument::Register(reg) => write!(f, "{}", reg),
			Argument::Immediate(value) => write!(f, "{}", value),
			Argument::Label(name) => write!(f, "{}", name),
			Argument::MemoryAddressDirect(addr) => write!(f, "[{}]", addr),
			Argument::MemoryAddressRegister(reg) => write!(f, "[{}]", reg),
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
	Mov(Argument, Argument), // dest, src
	Add(Argument, Argument), // dest, src
	Sub(Argument, Argument), // dest, src
	Mul(Argument, Argument), // dest, src
	Div(Argument, Argument, Argument, Argument), // quotient dest, remainder dest, value src, div src
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
	ExternCall(String), // name of the syscall function (used for IAT)
	Call(String),   // function name
	Nop,
	ProgramStart
}

fn get_register(variable_name: &String, register_allocation: &HashMap<String, isize>) -> Result<Register, Error> {
	if let Some(&reg_num) = register_allocation.get(variable_name) {
		Ok(to_register(reg_num))
	} else {
		Err(Error::SimpleError{message: format!("Variable {} not found in register allocation", variable_name)})
	}
}

pub fn generate_code(tac: &Vec<TacInstruction>, register_allocation: &HashMap<String, isize>, liveness: &Vec<HashSet<VariableValue>>) -> Result<Vec<Instruction>, Error> {
	let mut instructions = Vec::new();

	for (instruction_index, tac_inst) in tac.iter().enumerate() {
		match tac_inst {
			TacInstruction::DirectInstruction(inner) => {
				instructions.push(inner.clone());
			}
			TacInstruction::ProgramStart() => {
				instructions.push(Instruction::ProgramStart); // Placeholder for program start
			}
			TacInstruction::Label(name) => {
				instructions.push(Instruction::Label(name.clone()));
			}
			TacInstruction::FunctionLabel(name, params) => {
				instructions.push(Instruction::Label(name.clone()));
				// Now we need to move the parameters from their argument registers to their allocated registers
				let arg_registers = [
					Register::General(RegisterType::RCX, RegisterSize::QuadWord),
					Register::General(RegisterType::RDX, RegisterSize::QuadWord),
					Register::Extended(8, RegisterSize::QuadWord),  // r8
					Register::Extended(9, RegisterSize::QuadWord),  // r9
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
							// TODO can also just pop this
							instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::MemoryAddressDirect(stack_offset)));
						}
					} else {
						return Err(Error::SimpleError{message: format!("Function parameter {} not found in register allocation", param)});
					}
				}
			}
			TacInstruction::Assign(dest, value) => {
				let dest_name = match dest {
					// If dest has a requested register, use that
					tac::VariableValue::Variable(name) => name,
					tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
				};
				let dest_reg = get_register(dest_name, register_allocation)?;
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
				let dest_name = match dest {
					// If dest has a requested register, use that
					tac::VariableValue::Variable(name) => name,
					tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
				};
				let dest_reg = get_register(dest_name, register_allocation)?;
				match (left, right) {
					// If we want to add two variables into a third, we first move one into the dest register, then add the other
					(TacValue::Variable(var_left), TacValue::Variable(var_right)) => {
						let left_reg = get_register(var_left, register_allocation)?;
						let right_reg = get_register(var_right, register_allocation)?;
						// Move left operand to dest register
						//instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(left_reg.clone())));
						// Perform operation with right operand
						match op {
							BinOp::Add => instructions.push(Instruction::Add(Argument::Register(left_reg.clone()), Argument::Register(right_reg))),
							BinOp::Sub => instructions.push(Instruction::Sub(Argument::Register(left_reg.clone()), Argument::Register(right_reg))),
							BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(left_reg.clone()), Argument::Register(right_reg))),
							BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg.clone()), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(left_reg.clone()), Argument::Register(right_reg))),
							_ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
						};
						// Move the left operand to dest register
						instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Register(left_reg)));
					}
					(TacValue::Variable(var_left), TacValue::Constant(imm_right)) => {
						let left_reg = get_register(var_left, register_allocation)?;
						// Perform operation with immediate right operand
						match op {
							BinOp::Add => {
								// Move left operand to dest register
								instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(left_reg.clone())));
								instructions.push(Instruction::Add(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64)));
							}
							BinOp::Sub => {
								// Move left operand to dest register
								instructions.push(Instruction::Mov(Argument::Register(dest_reg.clone()), Argument::Register(left_reg.clone())));
								instructions.push(Instruction::Sub(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64)));
							}
							BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Immediate(*imm_right as i64))),
							BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg.clone()), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(left_reg),Argument::Register(dest_reg))),
							_ => return Err(Error::SimpleError{message: format!("Unsupported binary operation: {:?}", op)}),
						};
					}
					(TacValue::Constant(imm_left), TacValue::Variable(var_right)) => {
						let right_reg = get_register(var_right, register_allocation)?;
						// Perform operation with right operand
						match op {
							BinOp::Add => {
								// Move left immediate to dest register
								instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*imm_left as i64)));
								instructions.push(Instruction::Add(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(right_reg)));
							}
							BinOp::Sub => {
								// Move left immediate to dest register
								instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*imm_left as i64)));
								instructions.push(Instruction::Sub(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(right_reg)));
							}
							BinOp::Mul => instructions.push(Instruction::Mul(Argument::Register(dest_reg), Argument::Immediate(*imm_left as i64))),
							BinOp::Div => instructions.push(Instruction::Div(Argument::Register(dest_reg.clone()), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(dest_reg), Argument::Register(right_reg))),
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
							BinOp::Div => {
								// TODO for division maybe rdx should be saved?
								// For division, we need to move the left immediate into RAX first
								instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*imm_left as i64)));
								instructions.push(Instruction::Div(Argument::Register(dest_reg), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*imm_right as i64)));
							}
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
						instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(ret_reg)));
					}
					Some(TacValue::Constant(imm)) => {
						instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*imm as i64)));
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
				let dest_name = match name {
					// If dest has a requested register, use that
					tac::VariableValue::Variable(name) => name,
					tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
				};
				let dest_reg = get_register(dest_name, register_allocation)?;

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
			TacInstruction::ExternCall(func_name, args, return_var) => {
				generate_code_for_call(&mut instructions, func_name, args, return_var, register_allocation, liveness, instruction_index, true)?;
			}
			TacInstruction::MovRSPTo(var_name) => {
				let dest_name = match var_name {
					// If dest has a requested register, use that
					tac::VariableValue::Variable(name) => name,
					tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
				};
				let dest_reg = get_register(dest_name, register_allocation)?;
				instructions.push(Instruction::Mov(Argument::Register(dest_reg), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord))));
			}
			TacInstruction::Push(value) => {
				match value {
					TacValue::Variable(var) => {
						let src_reg = get_register(var, register_allocation)?;
						instructions.push(Instruction::Push(Argument::Register(src_reg)));
					}
					TacValue::Constant(imm) => {
						instructions.push(Instruction::Push(Argument::Immediate(*imm as i64)));
					}
					_ => {
						return Err(Error::SimpleError{message: format!("Unsupported TacValue in Push: {:?}", value)});
					}
				}
			}
			TacInstruction::Pop(var_name) => {
				let dest_name = match var_name {
					// If dest has a requested register, use that
					tac::VariableValue::Variable(name) => name,
					tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
				};
				let dest_reg = get_register(dest_name, register_allocation)?;
				instructions.push(Instruction::Pop(Argument::Register(dest_reg)));
			}
		}
	}

	// Adding the ExitProcess call
	instructions.push(Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Immediate(0))); // Exit code 0
	instructions.push(Instruction::ExternCall("ExitProcess".into()));

	ensure_stack_alignment(&mut instructions);

	Ok(instructions)
}

fn generate_code_for_call(
	instructions: &mut Vec<Instruction>,
	func_name: &String,
	args: &Vec<VariableValue>,
	return_var: &Option<tac::VariableValue>,
	register_allocation: &HashMap<String, isize>,
	liveness: &Vec<HashSet<VariableValue>>,
	instruction_index: usize,
	is_syscall: bool,
) -> Result<(), Error> {
	// Check for active caller-saved registers and save them
	let mut saved_registers = Vec::new();
	if instruction_index < liveness.len() && instruction_index > 0 {
		// Check which variables are live right before this instruction
		let live_vars = &liveness[instruction_index];

		let mut live_registers = HashSet::new();
		for var in live_vars {
			let var_name = match var {
				VariableValue::Variable(name) => name,
				VariableValue::VariableWithRequestedRegister(name, _) => name,
			};
			if let Some(&reg_num) = register_allocation.get(var_name) {
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
	
	// reserve 32-byte shadow + 8 for alignment
	instructions.push(Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));

	// Call the function
	if is_syscall {
		// System call, we need to know the name of the syscall function to find it in the IAT
		instructions.push(Instruction::ExternCall(func_name.clone()));
	} else {
		// Regular function call
		instructions.push(Instruction::Call(func_name.clone()));
	}

	// Restore the stack pointer
	instructions.push(Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));


	// Restore saved caller-saved registers
	for reg in saved_registers.iter().rev() {
		instructions.push(Instruction::Pop(Argument::Register(reg.clone())));
	}

	// Move return value from rax to the appropriate variable, if needed
	if let Some(ret_var) = return_var {
		let ret_name = match ret_var {
			// If dest has a requested register, use that
			tac::VariableValue::Variable(name) => name,
			tac::VariableValue::VariableWithRequestedRegister(name, _) => name,
		};
		let ret_reg = get_register(ret_name, register_allocation)?;
		instructions.push(Instruction::Mov(Argument::Register(ret_reg), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));
	}
	Ok(())
}

fn ensure_stack_alignment(instructions: &mut Vec<Instruction>) {
	// Count pushes and pops to determine current stack alignment
	let mut stack_offset: i64 = 0;
	let mut instruction_index_offset: usize = 0;
	let instructions_copy = instructions.clone();
	for (index, instr) in instructions_copy.iter().enumerate() {
		match instr {
			Instruction::Push(_) => stack_offset -= 8,
			Instruction::Pop(_) => stack_offset += 8,
			Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(imm)) => {
				stack_offset += *imm;
			}
			Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(imm)) => {
				stack_offset -= *imm;
			}
			Instruction::Call(_) => {
				if stack_offset % 16 == 0 {
					continue; // Already aligned
				}
				print!("Aligning stack before call at instruction {}. Current offset: {}\n", index + instruction_index_offset, stack_offset);
				let adjustment = (16 - stack_offset.rem_euclid(16)) % 16;
				instructions.insert(index + instruction_index_offset, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(adjustment)));
				instructions.insert(index + 2 + instruction_index_offset, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(adjustment)));
				instruction_index_offset += 2; // Account for the two new instructions
			}
			Instruction::ExternCall(_) => {
				if stack_offset % 16 == 0 {
					continue; // Already aligned
				}
				print!("Aligning stack before extern call at instruction {}. Current offset: {}\n", index + instruction_index_offset, stack_offset);
				let adjustment = (16 - stack_offset.rem_euclid(16)) % 16;
				instructions.insert(index + instruction_index_offset, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(adjustment)));
				instructions.insert(index + 2 + instruction_index_offset, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(adjustment)));
				instruction_index_offset += 2; // Account for the two new instructions
			}
			_ => {}
		}
	}
}

pub fn print_instructions(instructions: &Vec<Instruction>) {
	for instr in instructions {
		print_instruction(instr);
	}
}

fn print_instruction(instruction: &Instruction) {
	match instruction {
		Instruction::Mov(dest, src) => {
			println!("MOV {} <- {}", dest, src);
		}
		Instruction::Add(dest, src) => {
			println!("ADD {} + {}", dest, src);
		}
		Instruction::Sub(dest, src) => {
			println!("SUB {} - {}", dest, src);
		}
		Instruction::Mul(dest, src) => {
			println!("MUL {} * {}", dest, src);
		}
		Instruction::Div(dest, remainder_dest, value, div) => {
			println!("DIV {} / {} -> {}, {}", value, div, dest, remainder_dest);
		}
		Instruction::Xor(dest, src) => {
			println!("XOR {}, {}", dest, src);
		}
		Instruction::And(dest, src) => {
			println!("AND {} & {}", dest, src);
		}
		Instruction::Or(dest, src) => {
			println!("OR {} | {}", dest, src);
		}
		Instruction::Not(src) => {
			println!("NOT {}", src);
		}
		Instruction::Cmp(op1, op2) => {
			println!("CMP {} =? {}", op1, op2);
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
			println!("PUSH {}", arg);
		}
		Instruction::Pop(arg) => {
			println!("POP {}", arg);
		}
		Instruction::Label(name) => {
			println!("{}:", name);
		}
		Instruction::ExternCall(num) => {
			println!("EXTERN_CALL {} (call rax)", num);
		}
		Instruction::Call(func_name) => {
			println!("CALL {}", func_name);
		}
		Instruction::Nop => {
			println!("NOP");
		}
		Instruction::ProgramStart => {
			println!("; Program Start");
		}
	}
}
