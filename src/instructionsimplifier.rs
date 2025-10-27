use crate::codegenerator::*;

#[derive(PartialEq, Debug, Clone)]
pub enum AssemblyInstruction {
	Mov(Argument, Argument), // dest, src
	Add(Argument, Argument), // dest, src
	Sub(Argument, Argument), // dest, src
	Mul(Argument, Argument), // dest, src
	Div(Argument), // idivq SRC; quotient in RAX, remainder in RDX
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
	ProgramStart,
	Comment(String),
}

pub fn to_assembly_instructions(instructions: &Vec<Instruction>) -> Vec<AssemblyInstruction> {
	let mut assembly_instructions = Vec::new();
	for instr in instructions {
		match instr {
			Instruction::PostCallStackAlign(_) | Instruction::PreCallStackAlign(_) => {
				// These are handled in ensure_stack_alignment and do not translate to assembly instructions
			}
			Instruction::Mov(dest, src) => assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src.clone())),
			Instruction::Add(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can add directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src2.clone()));
						}
						// If r2 is the destination register, we can add directly but need to swap the order
						else if r2 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src1.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(_), Argument::Immediate(_), Argument::Immediate(_)) => {
						assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
						assembly_instructions.push(AssemblyInstruction::Add(dest.clone(), src2.clone()));
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Add instruction conversion to assembly, {} <- {} + {}", dest, src1, src2);
					}
				}
			}
			Instruction::Sub(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can subtract directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Sub(dest.clone(), src2.clone()));
						}
						else if r2 == dest_reg {
							// If r2 is the destination register, we can subtract directly but need to rearrange
							// So we first move r1 into rax
							assembly_instructions.push(AssemblyInstruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), src1.clone()));
							// Then subtract r2 from rax
							assembly_instructions.push(AssemblyInstruction::Sub(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), src2.clone()));
							// Finally move rax into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Sub(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Sub(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Sub(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Immediate(_), Argument::Register(r1)) => {
						if r1 == dest_reg {
							// We must use RAX as an intermediary
							assembly_instructions.push(AssemblyInstruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Sub(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), src2.clone()));
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));
						} else {
							// Since r1 is not the destination, we can safely move the immediate into dest and subtract r1
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Sub(dest.clone(), src2.clone()));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Sub instruction conversion to assembly");
					}
				}
			}
			Instruction::Mul(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can multiply directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Mul(dest.clone(), src2.clone()));
						}
						// If r2 is the destination register, we can multiply directly but need to swap the order
						else if r2 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Mul(dest.clone(), src1.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Mul(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Mul(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Mul(dest.clone(), src2.clone()));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Mul instruction conversion to assembly");
					}
				}
			}
			Instruction::Div(quot_dest, rem_dest, val_src, div_src) => {
				match (val_src, div_src, quot_dest, rem_dest) {
					(_, _, _, Argument::Register(r_remainder)) => {
						// Save RDX if necessary
						let save_rdx = *r_remainder != Register::General(RegisterType::RDX, RegisterSize::QuadWord);

						if save_rdx {
							assembly_instructions.push(AssemblyInstruction::Push(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
						}

						// First move the value into rax
						assembly_instructions.push(AssemblyInstruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), val_src.clone()));
						// Clear RDX
						assembly_instructions.push(AssemblyInstruction::Xor(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
						// Then perform the division
						assembly_instructions.push(AssemblyInstruction::Div(div_src.clone()));
						// Move the quotient to the destination
						assembly_instructions.push(AssemblyInstruction::Mov(quot_dest.clone(), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));
						// Move the remainder to the remainder destination
						assembly_instructions.push(AssemblyInstruction::Mov(rem_dest.clone(), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
						// Restore RDX if necessary
						if save_rdx {
							assembly_instructions.push(AssemblyInstruction::Pop(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Div instruction conversion to assembly");
					}
				}
			}
			Instruction::Xor(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can xor directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Xor(dest.clone(), src2.clone()));
						}
						// If r2 is the destination register, we can xor directly but need to swap the order
						else if r2 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Xor(dest.clone(), src1.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Xor(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Xor(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Xor(dest.clone(), src2.clone()));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Xor instruction conversion to assembly");
					}
				}
			}
			Instruction::And(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can and directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::And(dest.clone(), src2.clone()));
						}
						// If r2 is the destination register, we can and directly but need to swap the order
						else if r2 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::And(dest.clone(), src1.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::And(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::And(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::And(dest.clone(), src2.clone()));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in And instruction conversion to assembly");
					}
				}
			}
			Instruction::Or(dest, src1, src2) => {
				match (dest, src1, src2) {
					(Argument::Register(dest_reg), Argument::Register(r1), Argument::Register(r2)) => {
						// If r1 is the destination register, we can or directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Or(dest.clone(), src2.clone()));
						}
						// If r2 is the destination register, we can or directly but need to swap the order
						else if r2 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Or(dest.clone(), src1.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Or(dest.clone(), src2.clone()));
						}
					}
					(Argument::Register(dest_reg), Argument::Register(r1), _) => {
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Or(dest.clone(), src2.clone()));
						} else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src1.clone()));
							assembly_instructions.push(AssemblyInstruction::Or(dest.clone(), src2.clone()));
						}
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Or instruction conversion to assembly");
					}
				}
			}
			Instruction::Not(dest, src) => {
				match (dest, src) {
					(Argument::Register(dest_reg), Argument::Register(r1)) => {
						// If r1 is the destination register, we can not directly
						if r1 == dest_reg {
							assembly_instructions.push(AssemblyInstruction::Not(dest.clone()));
						}
						else {
							// If the destination is not r1, we must not overwrite r1 and thus first move r1 into dest
							assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src.clone()));
							assembly_instructions.push(AssemblyInstruction::Not(dest.clone()));
						}
					}
					(Argument::Register(dest_reg), _) => {
						assembly_instructions.push(AssemblyInstruction::Mov(dest.clone(), src.clone()));
						assembly_instructions.push(AssemblyInstruction::Not(dest.clone()));
					}
					_ => {
						unimplemented!("Only register arguments are implemented in Not instruction conversion to assembly");
					}
				}
			}
			Instruction::Cmp(op1, op2) => {
				match (op1, op2) {
					(Argument::Immediate(v1), Argument::Immediate(v2)) => {
						// To compare two immediates, we can move one into a register and then compare
						assembly_instructions.push(AssemblyInstruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*v1)));
						assembly_instructions.push(AssemblyInstruction::Cmp(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(*v2)));
					}
					(Argument::Immediate(v), Argument::Register(r)) => {
						assembly_instructions.push(AssemblyInstruction::Cmp(op2.clone(), op1.clone()));
					}
					_ => {
						assembly_instructions.push(AssemblyInstruction::Cmp(op1.clone(), op2.clone()));
					}
				}
			}
			Instruction::Jmp(label) => assembly_instructions.push(AssemblyInstruction::Jmp(label.clone())),
			Instruction::Je(label) => assembly_instructions.push(AssemblyInstruction::Je(label.clone())),
			Instruction::Jne(label) => assembly_instructions.push(AssemblyInstruction::Jne(label.clone())),
			Instruction::Jg(label) => assembly_instructions.push(AssemblyInstruction::Jg(label.clone())),
			Instruction::Jge(label) => assembly_instructions.push(AssemblyInstruction::Jge(label.clone())),
			Instruction::Jl(label) => assembly_instructions.push(AssemblyInstruction::Jl(label.clone())),
			Instruction::Jle(label) => assembly_instructions.push(AssemblyInstruction::Jle(label.clone())),
			Instruction::Ret => assembly_instructions.push(AssemblyInstruction::Ret),
			Instruction::Push(arg) => assembly_instructions.push(AssemblyInstruction::Push(arg.clone())),
			Instruction::Pop(arg) => assembly_instructions.push(AssemblyInstruction::Pop(arg.clone())),
			Instruction::Label(name, _) => assembly_instructions.push(AssemblyInstruction::Label(name.clone())),
			Instruction::ExternCall(name) => assembly_instructions.push(AssemblyInstruction::ExternCall(name.clone())),
			Instruction::Call(name) => assembly_instructions.push(AssemblyInstruction::Call(name.clone())),
			Instruction::Nop => assembly_instructions.push(AssemblyInstruction::Nop),
			Instruction::ProgramStart => assembly_instructions.push(AssemblyInstruction::ProgramStart),
			Instruction::Comment(comment) => assembly_instructions.push(AssemblyInstruction::Comment(comment.to_string())),
		}
	}
	assembly_instructions
}

pub fn print_assembly_instructions(instructions: &Vec<AssemblyInstruction>) {
	for instr in instructions {
		match instr {
			AssemblyInstruction::Mov(dest, src) => {
				println!("MOV {} {}", dest, src);
			}
			AssemblyInstruction::Add(dest, src) => {
				println!("ADD {} {}", dest, src);
			}
			AssemblyInstruction::Sub(dest, src) => {
				println!("SUB {} {}", dest, src);
			}
			AssemblyInstruction::Mul(dest, src) => {
				println!("MUL {} {}", dest, src);
			}
			AssemblyInstruction::Div(src) => {
				println!("DIV {}", src);
			}
			AssemblyInstruction::Xor(dest, src) => {
				println!("XOR {} {}", dest, src);
			}
			AssemblyInstruction::And(dest, src) => {
				println!("AND {} {}", dest, src);
			}
			AssemblyInstruction::Or(dest, src) => {
				println!("OR {} {}", dest, src);
			}
			AssemblyInstruction::Not(src) => {
				println!("NOT {}", src);
			}
			AssemblyInstruction::Cmp(op1, op2) => {
				println!("CMP {} {}", op1, op2);
			}
			AssemblyInstruction::Jmp(label) => {
				println!("JMP {}", label);
			}
			AssemblyInstruction::Je(label) => {
				println!("JE {}", label);
			}
			AssemblyInstruction::Jne(label) => {
				println!("JNE {}", label);
			}
			AssemblyInstruction::Jg(label) => {
				println!("JG {}", label);
			}
			AssemblyInstruction::Jge(label) => {
				println!("JGE {}", label);
			}
			AssemblyInstruction::Jl(label) => {
				println!("JL {}", label);
			}
			AssemblyInstruction::Jle(label) => {
				println!("JLE {}", label);
			}
			AssemblyInstruction::Ret => {
				println!("RET");
			}
			AssemblyInstruction::Push(arg) => {
				println!("PUSH {}", arg);
			}
			AssemblyInstruction::Pop(arg) => {
				println!("POP {}", arg);
			}
			AssemblyInstruction::Label(name) => {
				println!("{}:", name);
			}
			AssemblyInstruction::ExternCall(name) => {
				println!("EXTERN_CALL {}", name);
			}
			AssemblyInstruction::Call(name) => {
				println!("CALL {}", name);
			}
			AssemblyInstruction::Nop => {
				println!("NOP");
			}
			AssemblyInstruction::ProgramStart => {
				println!("; Program Start");
			}
			AssemblyInstruction::Comment(comment) => {
				println!("; {}", comment);
			}
		}
	}
}