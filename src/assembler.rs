use std::collections::HashMap;

use crate::codegenerator::*;
use crate::instructionsimplifier::AssemblyInstruction;
use crate::exewriter::write_at_bytes;

fn get_rex_byte(w: bool, r: bool, x: bool, b: bool) -> u8 {
	0x40 | ((w as u8) << 3) | ((r as u8) << 2) | ((x as u8) << 1) | (b as u8)
}

fn get_rm(reg: &Register) -> u8 {
	// 7 6 | 5 4 3 | 2 1 0
	// MOD |  REG  |  R/M
	match reg {
		Register::General(register_type, _) => match register_type {
			RegisterType::RAX => 0,
			RegisterType::RCX => 1,
			RegisterType::RDX => 2,
			RegisterType::RBX => 3,
			RegisterType::RSP => 4,
			RegisterType::RBP => 5,
			RegisterType::RSI => 6,
			RegisterType::RDI => 7,
		},
		Register::Extended(num, _) => {
			// r8 through r15
			if *num >= 8 && *num <= 15 {
				return (*num - 8) as u8;
			} else {
				panic!("Invalid extended register number (assembler): {}", *num);
			}
		}
	}
}

fn get_register_is_extended(reg: &Register) -> bool {
	match reg {
		Register::General(_, _) => false,
		Register::Extended(_, _) => true,
	}
}

fn resolve_addresses(label_addresses: &HashMap<String, usize>, jumps_to_resolve: &mut Vec<(String, usize)>, machine_code: &mut Vec<u8>) {
	for (label, pos) in jumps_to_resolve.iter() {
		if let Some(&label_addr) = label_addresses.get(label) {
			let jump_from = pos + 4; // After the 4-byte relative address
			let relative_addr = (label_addr as isize - jump_from as isize) as u32;
			let bytes = relative_addr.to_le_bytes();
			write_at_bytes(machine_code, *pos, &bytes);
		} else {
			panic!("Undefined label: {}", label);
		}
	}
}

pub fn assemble_program(instructions: Vec<AssemblyInstruction>) -> (Vec<u8>, Vec<(String, usize)>, usize) {
	let mut machine_code = Vec::new();

	let mut label_addresses: HashMap<String, usize> = std::collections::HashMap::new();
	let mut jumps_to_resolve: Vec<(String, usize)> = Vec::new();
	let mut syscalls_to_resolve: Vec<(String, usize)> = Vec::new();
	let mut starting_point: usize = 0;

	assemble(instructions, &mut machine_code, &mut label_addresses, &mut jumps_to_resolve, &mut syscalls_to_resolve, &mut starting_point);

	// Resolve all jumps to their correct addresses
	resolve_addresses(&label_addresses, &mut jumps_to_resolve, &mut machine_code);

	(machine_code, syscalls_to_resolve, starting_point)
} 

fn assemble(instructions: Vec<AssemblyInstruction>, machine_code: &mut Vec<u8>, label_addresses: &mut HashMap<String, usize>, jumps_to_resolve: &mut Vec<(String, usize)>, syscalls_to_resolve: &mut Vec<(String, usize)>, starting_point: &mut usize) {
	for instr in instructions {
		match instr {
			AssemblyInstruction::ProgramStart => {
				*starting_point = machine_code.len();
			}
			AssemblyInstruction::Label(label) => {
				// The address of the label is the current length of machine_code
				label_addresses.insert(label, machine_code.len());
			}
			AssemblyInstruction::Mov(dest, src) => {
				match (dest, src) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r1);
						let rex_r = get_register_is_extended(&r2);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x89); // MOV r/m64, r64

						let mod_rm = 0b11_000_000 | (get_rm(&r2) << 3) | get_rm(&r1);
						write_u8(machine_code, mod_rm);
					}
					(Argument::Register(r), Argument::Immediate(imm)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0xB8 | get_rm(&r)); // MOV r64, imm64
						write_u64(machine_code, imm as u64);
					}
					(Argument::Register(r), Argument::MemoryAddressDirect(m)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x8B); // MOV r64, r/m64

						let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, m as u32); // 32-bit displacement
					}
					(Argument::MemoryAddressDirect(m), Argument::Register(r)) => {
						let rex_r = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, false)); // REX.W prefix
						write_u8(machine_code, 0x89); // MOV r/m64, r64
						let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, m as u32); // 32-bit displacement
					}
					(Argument::MemoryAddressRegister(mar), Argument::Register(r)) => {
						let size = match mar.clone() {
							Register::General(_, s) | Register::Extended(_, s) => s,
						};

						match size {
							RegisterSize::QuadWord => {
								// MOV [reg], r64
								let rex_r = get_register_is_extended(&r);
								let rex_b = get_register_is_extended(&mar);

								// REX.W = 1 for 64-bit operand
								write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b));
								write_u8(machine_code, 0x89); // MOV r/m64, r64

								// MOD = 00 (no displacement), REG = r (source), R/M = m (base register)
								let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | get_rm(&mar);
								write_u8(machine_code, mod_rm);

								// Special case: if m == RBP or m == R13, MOD=00 with R/M=101 or 1001 is interpreted as disp32.
								// You must emit a disp8 = 0 in that case to indicate [RBP + 0] or [R13 + 0].
								if matches!(mar, Register::General(RegisterType::RBP, _) | Register::Extended(13, _)) {
									write_u8(machine_code, 0x00);
								}
							}
							RegisterSize::Byte => {
								// MOV [reg], r8
								let rex_r = get_register_is_extended(&r);
								let rex_b = get_register_is_extended(&mar);
								write_u8(machine_code, get_rex_byte(false, rex_r, false, rex_b)); // REX prefix without W
								write_u8(machine_code, 0x88); // MOV r/m8, r
								// MOD = 00 (no displacement), REG = r (source), R/M = m (base register)
								let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | get_rm(&mar);
								write_u8(machine_code, mod_rm);
							}
							_ => {
								unimplemented!("Only 64-bit MOV is implemented for MemoryAddressRegister");
							}
						}
					}
					(Argument::MemoryAddressRegister(mar), Argument::Immediate(imm)) => {
						let size = match mar.clone() {
							Register::General(_, s) | Register::Extended(_, s) => s,
						};

						match size {
							RegisterSize::QuadWord => {
								// MOV [reg], imm32
								let rex_b = get_register_is_extended(&mar);

								write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
								write_u8(machine_code, 0xC7); // MOV r/m64, imm32

								// MOD = 00 (no displacement), REG = 000 (for MOV), R/M = m (base register)
								let mod_rm = 0b00_000_000 | (0b000 << 3) | get_rm(&mar);
								write_u8(machine_code, mod_rm);

								// 32-bit immediate
								write_u32(machine_code, imm as u32);
							}
							RegisterSize::Byte => {
								// MOV [reg], imm8
								let rex_b = get_register_is_extended(&mar);

								write_u8(machine_code, get_rex_byte(false, false, false, rex_b)); // REX prefix without W
								write_u8(machine_code, 0xC6); // MOV r/m8, imm8

								// MOD = 00 (no displacement), REG = 000 (for MOV), R/M = m (base register)
								let mod_rm = 0b00_000_000 | (0b000 << 3) | get_rm(&mar);
								write_u8(machine_code, mod_rm);

								// 8-bit immediate
								write_u8(machine_code, imm as u8);
							}
							_ => {
								unimplemented!("Only 64-bit and 8-bit MOV is implemented for MemoryAddressRegister");
							}
						}
					}
					_ => {
						// Placeholder for other MOV cases
						unimplemented!("MOV not implemented yet");
					}
				}
			}
			AssemblyInstruction::Add(dest, src) => {
				match (dest, src) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r1);
						let rex_r = get_register_is_extended(&r2);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x01); // ADD r/m64, r64

						let mod_rm = 0b11_000_000 | (get_rm(&r2) << 3) | get_rm(&r1);
						write_u8(machine_code, mod_rm);
					}
					(Argument::Register(r), Argument::Immediate(imm)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x81); // ADD r/m64, imm32
						let mod_rm = 0b11_000_000 | (0b000 << 3) | get_rm(&r); // MOD=11, REG=000 (ADD), R/M=r
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, imm as u32); // 32-bit immediate
					}
					(Argument::MemoryAddressDirect(m), Argument::Register(r)) => {
						let rex_r = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, false)); // REX.W prefix
						write_u8(machine_code, 0x01); // ADD r/m64, r64

						let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, m as u32); // 32-bit displacement
					}
					(Argument::Register(r), Argument::MemoryAddressDirect(m)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x03); // ADD r64, r/m64

						let mod_rm = 0b00_000_000 | (get_rm(&r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, m as u32); // 32-bit displacement
					}
					(Argument::MemoryAddressDirect(m), Argument::Immediate(imm)) => {
						write_u8(machine_code, get_rex_byte(true, false, false, false)); // REX.W prefix
						write_u8(machine_code, 0x81); // ADD r/m64, imm32
						let mod_rm = 0b00_000_000 | (0b000 << 3) | 0b101; // MOD=00, REG=000 (ADD), R/M=101
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, imm as u32); // 32-bit immediate
					}
					_ => {
						// Placeholder for other ADD cases
						unimplemented!("ADD not implemented yet");
					}
				}
			}
			AssemblyInstruction::Mul(dest, src) => {
				match (dest, src) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r2);
						let rex_r = get_register_is_extended(&r1);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x0F); // Two-byte opcode prefix
						write_u8(machine_code, 0xAF); // IMUL r64, r/m64

						let mod_rm = 0b11_000_000 | (get_rm(&r1) << 3) | get_rm(&r2);
						write_u8(machine_code, mod_rm);
					}
					(Argument::Register(r), Argument::Immediate(imm)) => {
						let rex_b = get_register_is_extended(&r);

						// REX.W + 0x69 /r id
						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x69); // IMUL r64, r/m64, imm32

						// MOD=11 (register-direct), REG=r (dest), R/M=r (source)
						let mod_rm = 0b11_000_000 | (get_rm(&r) << 3) | get_rm(&r);
						write_u8(machine_code, mod_rm);

						// 32-bit immediate (sign-extended)
						write_u32(machine_code, imm as u32);
					}
					_ => {
						// Placeholder for other MUL cases
						unimplemented!("MUL not implemented yet");
					}
				}
			}
			AssemblyInstruction::Sub(dest, src) => {
				match (dest, src) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r1);
						let rex_r = get_register_is_extended(&r2);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x29); // SUB r/m64, r64

						let mod_rm = 0b11_000_000 | (get_rm(&r2) << 3) | get_rm(&r1);
						write_u8(machine_code, mod_rm);
					}
					(Argument::Register(r), Argument::Immediate(imm)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x81); // SUB r/m64, imm32
						let mod_rm = 0b11_000_000 | (0b101 << 3) | get_rm(&r); // MOD=11, REG=101 (SUB), R/M=r
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, imm as u32); // 32-bit immediate
					}
					_ => {
						// Placeholder for other SUB cases
						unimplemented!("SUB not implemented yet");
					}
				}
			}
			AssemblyInstruction::Div(source) => {
				match source {
					Argument::Register(r_value) => {
						// DIV r/m64
						let rex_b = get_register_is_extended(&r_value);
						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0xF7); // DIV r/m64
						let mod_rm = 0b11_000_000 | (0b110 << 3) | get_rm(&r_value); // MOD=11, REG=110 (DIV), R/M=r_value
						write_u8(machine_code, mod_rm);
					}
					_ => {
						// Placeholder for other DIV cases
						unimplemented!("DIV not implemented yet");
					}
				}
			}
			AssemblyInstruction::Xor(dest, src) => {
				match (dest, src) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r1);
						let rex_r = get_register_is_extended(&r2);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x31); // XOR r/m64, r64

						let mod_rm = 0b11_000_000 | (get_rm(&r2) << 3) | get_rm(&r1);
						write_u8(machine_code, mod_rm);
					}
					_ => {
						// Placeholder for other XOR cases
						unimplemented!("XOR not implemented yet");
					}
				}
			}
			AssemblyInstruction::Cmp(arg1, arg2) => {
				match (arg1, arg2) {
					(Argument::Register(r1), Argument::Register(r2)) => {
						let rex_b = get_register_is_extended(&r1);
						let rex_r = get_register_is_extended(&r2);

						write_u8(machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x39); // CMP r/m64, r64

						let mod_rm = 0b11_000_000 | (get_rm(&r2) << 3) | get_rm(&r1);
						write_u8(machine_code, mod_rm);
					}
					(Argument::Register(r), Argument::Immediate(imm)) => {
						let rex_b = get_register_is_extended(&r);

						write_u8(machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
						write_u8(machine_code, 0x81); // CMP r/m64, imm32
						let mod_rm = 0b11_000_000 | (0b111 << 3) | get_rm(&r); // MOD=11, REG=111 (CMP), R/M=r
						write_u8(machine_code, mod_rm);
						write_u32(machine_code, imm as u32); // 32-bit immediate
					}
					_ => {
						// Placeholder for other CMP cases
						unimplemented!("CMP not implemented yet");
					}
				}
			}
			AssemblyInstruction::Jmp(to_label) => {
				// Placeholder for JMP instruction
				write_u8(machine_code, 0xE9); // JMP opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Jne(to_label) => {
				// Placeholder for JNE instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x85); // JNE opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Je(to_label) => {
				// Placeholder for JE instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x84); // JE opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Jg(to_label) => {
				// Placeholder for JG instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x8F); // JG opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Jge(to_label) => {
				// Placeholder for JGE instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x8D); // JGE opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Jle(to_label) => {
				// Placeholder for JLE instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x8E); // JLE opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Jl(to_label) => {
				// Placeholder for JL instruction
				write_u8(machine_code, 0x0F); // Two-byte opcode prefix
				write_u8(machine_code, 0x8C); // JL opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This jump still needs its address resolved
				jumps_to_resolve.push((to_label, pos));
			}
			AssemblyInstruction::Push(argument) => {
				match argument {
					Argument::Register(r) => {
						let rex_b = get_register_is_extended(&r);
						if rex_b {
							write_u8(machine_code, get_rex_byte(true, false, false, rex_b));
						}
						write_u8(machine_code, 0x50 | get_rm(&r)); // PUSH r64
					}
					Argument::Immediate(imm) => {
						if imm >= -128 && imm <= 127 {
							write_u8(machine_code, 0x6A); // PUSH imm8
							write_u8(machine_code, imm as u8);
						} else {
							write_u8(machine_code, 0x68); // PUSH imm32
							write_u32(machine_code, imm as u32);
						}
					}
					_ => {
						unimplemented!("PUSH not implemented for this argument type");
					}
				}
			}
			AssemblyInstruction::Pop(argument) => {
				match argument {
					Argument::Register(r) => {
						let rex_b = get_register_is_extended(&r);
						if rex_b {
							write_u8(machine_code, get_rex_byte(true, false, false, rex_b));
						}
						write_u8(machine_code, 0x58 | get_rm(&r)); // POP r64
					}
					_ => {
						unimplemented!("POP not implemented for this argument type");
					}
				}
			}
			AssemblyInstruction::Ret => {
				write_u8(machine_code, 0xC3); // RET opcode
			}
			AssemblyInstruction::ExternCall(syscall_function) => {

				// call rax
				write_u8(machine_code, 0xFF);
				write_u8(machine_code, 0x15);

				let pos = machine_code.len();
				write_u32(machine_code, 0); // 8 bytes of zero for now

				// record where to patch the 8-byte immediate and which function to resolve
				syscalls_to_resolve.push((syscall_function, pos));
			}
			AssemblyInstruction::Nop => {
				machine_code.push(0x90); // NOP opcode
			}
			AssemblyInstruction::Call(func_name) => {
				// Placeholder for CALL instruction
				write_u8(machine_code, 0xE8); // CALL opcode
				let pos = machine_code.len();

				// Placeholder for relative address
				write_u32(machine_code, 0);

				// This call still needs its address resolved
				jumps_to_resolve.push((func_name, pos));
			}
			_ => {
				// Placeholder for other instructions
				unimplemented!("Instruction {:?} not implemented yet", instr);
			}
		}
	}
}

fn write_u8(buf: &mut Vec<u8>, value: u8) -> usize {
	let index = buf.len();
	buf.push(value);
	index
}

fn write_u16(buf: &mut Vec<u8>, value: u16) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_u32(buf: &mut Vec<u8>, value: u32) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_u64(buf: &mut Vec<u8>, value: u64) -> usize {
	let index = buf.len();
	buf.extend_from_slice(&value.to_le_bytes());
	index
}

fn write_bytes(buf: &mut Vec<u8>, data: &[u8]) -> usize {
	let index = buf.len();
	buf.extend_from_slice(data);
	index
}

fn write_zeroes(buf: &mut Vec<u8>, count: usize) -> usize {
	let index = buf.len();
	buf.resize(buf.len() + count, 0);
	index
}
