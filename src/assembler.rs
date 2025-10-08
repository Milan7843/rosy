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

fn get_register_is_extended(reg: &Register) -> bool {
    match reg {
        Register::General(_) => false,
        Register::Extended(_) => true,
    }
}

fn get_rex_byte(w: bool, r: bool, x: bool, b: bool) -> u8 {
    0x40 | ((w as u8) << 3) | ((r as u8) << 2) | ((x as u8) << 1) | (b as u8)
}

fn get_rm(reg: Register) -> u8 {
    // 7 6 | 5 4 3 | 2 1 0
    // MOD |  REG  |  R/M
    match reg {
        Register::General(register_type) => match register_type {
            RegisterType::RAX => 0,
            RegisterType::RCX => 1,
            RegisterType::RDX => 2,
            RegisterType::RBX => 3,
            RegisterType::RSP => 4,
            RegisterType::RBP => 5,
            RegisterType::RSI => 6,
            RegisterType::RDI => 7,
        },
        Register::Extended(num) => {
            if num < 8 {
                num
            } else {
                panic!("Register number out of range for R/M field");
            }
        }
    }
}

pub fn assemble(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut machine_code = Vec::new();

    for instr in instructions {
        match instr {
            Instruction::Mov(dest, src) => {
                match (dest, src) {
                    (Argument::Register(r1), Argument::Register(r2)) => {
                        let rex_b = get_register_is_extended(&r1);
                        let rex_r = get_register_is_extended(&r2);

                        write_u8(&mut machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0x89); // MOV r/m64, r64

                        let mod_rm = 0b11_000_000 | (get_rm(r2) << 3) | get_rm(r1);
                        write_u8(&mut machine_code, mod_rm);
                    }
                    (Argument::Register(r), Argument::Immediate(imm)) => {
                        let rex_b = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0xB8 | get_rm(r)); // MOV r64, imm64
                        write_u64(&mut machine_code, imm as u64);
                    }
                    (Argument::Register(r), Argument::MemoryAddress(m)) => {
                        let rex_b = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0x8B); // MOV r64, r/m64

                        let mod_rm = 0b00_000_000 | (get_rm(r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, m as u32); // 32-bit displacement
                    }
                    (Argument::MemoryAddress(m), Argument::Register(r)) => {
                        let rex_r = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, rex_r, false, false)); // REX.W prefix
                        write_u8(&mut machine_code, 0x89); // MOV r/m64, r64
                        let mod_rm = 0b00_000_000 | (get_rm(r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, m as u32); // 32-bit displacement
                    }
                    _ => {
                        // Placeholder for other MOV cases
                        unimplemented!("MOV not implemented yet");
                    }
                }
            }
            Instruction::Add(dest, src) => {
                match (dest, src) {
                    (Argument::Register(r1), Argument::Register(r2)) => {
                        let rex_b = get_register_is_extended(&r1);
                        let rex_r = get_register_is_extended(&r2);

                        write_u8(&mut machine_code, get_rex_byte(true, rex_r, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0x01); // ADD r/m64, r64

                        let mod_rm = 0b11_000_000 | (get_rm(r2) << 3) | get_rm(r1);
                        write_u8(&mut machine_code, mod_rm);
                    }
                    (Argument::Register(r), Argument::Immediate(imm)) => {
                        let rex_b = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0x81); // ADD r/m64, imm32
                        let mod_rm = 0b11_000_000 | (0b000 << 3) | get_rm(r); // MOD=11, REG=000 (ADD), R/M=r
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, imm as u32); // 32-bit immediate
                    }
                    (Argument::MemoryAddress(m), Argument::Register(r)) => {
                        let rex_r = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, rex_r, false, false)); // REX.W prefix
                        write_u8(&mut machine_code, 0x01); // ADD r/m64, r64

                        let mod_rm = 0b00_000_000 | (get_rm(r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, m as u32); // 32-bit displacement
                    }
                    (Argument::Register(r), Argument::MemoryAddress(m)) => {
                        let rex_b = get_register_is_extended(&r);

                        write_u8(&mut machine_code, get_rex_byte(true, false, false, rex_b)); // REX.W prefix
                        write_u8(&mut machine_code, 0x03); // ADD r64, r/m64

                        let mod_rm = 0b00_000_000 | (get_rm(r) << 3) | 0b101; // MOD=00, R/M=101 for disp32
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, m as u32); // 32-bit displacement
                    }
                    (Argument::MemoryAddress(m), Argument::Immediate(imm)) => {
                        write_u8(&mut machine_code, get_rex_byte(true, false, false, false)); // REX.W prefix
                        write_u8(&mut machine_code, 0x81); // ADD r/m64, imm32
                        let mod_rm = 0b00_000_000 | (0b000 << 3) | 0b101; // MOD=00, REG=000 (ADD), R/M=101
                        write_u8(&mut machine_code, mod_rm);
                        write_u32(&mut machine_code, imm as u32); // 32-bit immediate
                    }
                    _ => {
                        // Placeholder for other ADD cases
                        unimplemented!("ADD not implemented yet");
                    }
                }
            }
            Instruction::Nop => {
                machine_code.push(0x90); // NOP opcode
            }
            _ => {
                // Placeholder for other instructions
                unimplemented!("Instruction {:?} not implemented yet", instr);
            }
        }
    }

    machine_code
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
