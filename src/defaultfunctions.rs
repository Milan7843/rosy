use core::num;

use crate::tac::BinOp;
use crate::typechecker::FunctionType;
use crate::typechecker::Type;
use crate::tac::TacFunctionEnvironment;
use crate::tac::TacInstruction;
use crate::tac::TacFunction;
use crate::tac::TacValue;
use crate::tac::VariableValue;
use crate::codegenerator::from_register;
use crate::codegenerator::Instruction;
use crate::codegenerator::Argument;
use crate::codegenerator::Register;
use crate::codegenerator::RegisterType;
use crate::codegenerator::RegisterSize;

pub fn is_default_function(name: &str) -> bool {
	match name {
		"print" | "println" => true,
		_ => false,
	}
}

pub fn add_default_functions(
    functions: Vec<FunctionType>,
    function_env: &mut TacFunctionEnvironment,
    instructions: &mut Vec<TacInstruction>,
    temp_counter: &mut i64,
    label_counter: &mut i64)
{
	for func in functions {
		if !func.is_used {
			continue;
		}
		if is_default_function(&func.name) {
			match func.name.as_str() {
				"print" => {
					default_print_int_function(func, function_env, instructions, temp_counter, label_counter);
				}
				"println" => {
					default_println_int_function(func, function_env, instructions, temp_counter, label_counter);
				}
				_ => {}
			}
		}
	}
}

fn new_variable_name(base: &str, counter: &mut i64) -> String {
	let name = format!("{}{}", base, counter);
	*counter += 1;
	name
}

fn add_direct(
	instructions: &mut Vec<TacInstruction>,
	instruction: Instruction)
{
	instructions.push(TacInstruction::DirectInstruction(instruction));
}

fn default_print_int_function(
	func: FunctionType,
	function_env: &mut TacFunctionEnvironment,
	instructions: &mut Vec<TacInstruction>,
	temp_counter: &mut i64,
	label_counter: &mut i64)
{
    let label = format!("func_print_int");

	let param_name = func.param_names[0].clone();
	function_env.functions.push(TacFunction {
		name: "print".to_string(),
		params: vec![Type::Integer],
		return_type: Type::Undefined,
		label: label.clone(),
	});

	instructions.push(TacInstruction::FunctionLabel(label.clone(), vec![param_name.clone()]));

	// Push RBX
	add_direct(instructions, Instruction::Push(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Push RDI
	add_direct(instructions, Instruction::Push(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Allocate stack space for the buffer
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Get pointer to buffer
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord))));
	// Add 32 to get the end of the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Save the start pointer
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Find the end of the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Immediate(16)));
	// Copy the integer to print to RAX
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord))));
	// Move the divider into RCX
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Immediate(10)));

	let loop_start_label = format!("print_int_loop_start_{}", label_counter);

	add_direct(instructions, Instruction::Label(loop_start_label.clone()));
	*label_counter += 1;
	// Clear RDX
	add_direct(instructions, Instruction::Xor(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
	// Divide RAX by 10
	add_direct(instructions, Instruction::Div(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord))));
	// Get the digit of the remainder: add 30 to RDX
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Immediate(48)));
	// Decrease RBX to point to the next position: sub rbx, 1
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Immediate(1)));
	// Store the digit: mov [rbx], dl
	add_direct(instructions, Instruction::Mov(Argument::MemoryAddressRegister(Register::General(RegisterType::RBX, RegisterSize::Byte)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::Byte))));
	// Check if RAX is zero
	add_direct(instructions, Instruction::Cmp(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(0)));
	add_direct(instructions, Instruction::Jne(loop_start_label));

	// GetStdHandle routine
	// mov rcx, -11
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Immediate(-11)));
	// setup for call: sub rsp, 40
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// call GetStdHandle
	add_direct(instructions, Instruction::ExternCall("GetStdHandle".to_string()));
	// add rsp, 40
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// Move the buffer start to r8
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(8, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Move the handle to rsi
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RSI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));

	// Compute the string length
	// move rdx, rdi
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// add rdx, 16
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Immediate(16)));
	// sub rdx, rbx (length = end - start)
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Now rdx has the length, move it to r8 as the function argument
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(8, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
	// Move the handle into rcx
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSI, RegisterSize::QuadWord))));
	// Move the pointer to the buffer into rdx
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Move 0 into r9 (lpNumberOfBytesWritten)
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(9, RegisterSize::QuadWord)), Argument::Immediate(0)));

	// WriteFile call
	// setup for call: sub rsp, 40
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// call WriteFile
	add_direct(instructions, Instruction::ExternCall("WriteFile".to_string()));
	// add rsp, 40
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	
	// Cleanup
	// Deallocate stack space for the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Pop RDI
	add_direct(instructions, Instruction::Pop(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Pop RBX
	add_direct(instructions, Instruction::Pop(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Return
	add_direct(instructions, Instruction::Ret);

	return;
}

fn default_println_int_function(
	func: FunctionType,
	function_env: &mut TacFunctionEnvironment,
	instructions: &mut Vec<TacInstruction>,
	temp_counter: &mut i64,
	label_counter: &mut i64)
{
    let label = format!("func_println_int");

	let param_name = func.param_names[0].clone();
	function_env.functions.push(TacFunction {
		name: "println".to_string(),
		params: vec![Type::Integer],
		return_type: Type::Undefined,
		label: label.clone(),
	});

	instructions.push(TacInstruction::FunctionLabel(label.clone(), vec![param_name.clone()]));

	// Push RBX
	add_direct(instructions, Instruction::Push(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Push RDI
	add_direct(instructions, Instruction::Push(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Allocate stack space for the buffer
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Get pointer to buffer
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord))));
	// Add 32 to get the end of the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Save the start pointer
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Find the end of the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Immediate(16)));
	// Copy the integer to print to RAX
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord))));
	// Move the divider into RCX
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Immediate(10)));

	let loop_start_label = format!("print_int_loop_start_{}", label_counter);

	add_direct(instructions, Instruction::Label(loop_start_label.clone()));
	*label_counter += 1;
	// Clear RDX
	add_direct(instructions, Instruction::Xor(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
	// Divide RAX by 10
	add_direct(instructions, Instruction::Div(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord))));
	// Get the digit of the remainder: add 30 to RDX
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Immediate(48)));
	// Decrease RBX to point to the next position: sub rbx, 1
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Immediate(1)));
	// Store the digit: mov [rbx], dl
	add_direct(instructions, Instruction::Mov(Argument::MemoryAddressRegister(Register::General(RegisterType::RBX, RegisterSize::Byte)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::Byte))));
	// Check if RAX is zero
	add_direct(instructions, Instruction::Cmp(Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord)), Argument::Immediate(0)));
	add_direct(instructions, Instruction::Jne(loop_start_label));

	// Write the newline character
	// Decrease RBX to point to the next position: sub rbx, 1
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord)), Argument::Immediate(1)));
	// Store the newline character: mov [rbx], 0xA
	add_direct(instructions, Instruction::Mov(Argument::MemoryAddressRegister(Register::General(RegisterType::RBX, RegisterSize::Byte)), Argument::Immediate(0xA)));

	// GetStdHandle routine
	// mov rcx, -11
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Immediate(-11)));
	// setup for call: sub rsp, 40
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// call GetStdHandle
	add_direct(instructions, Instruction::ExternCall("GetStdHandle".to_string()));
	// add rsp, 40
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// Move the buffer start to r8
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(8, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Move the handle to rsi
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RSI, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RAX, RegisterSize::QuadWord))));

	// Compute the string length
	// move rdx, rdi
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// add rdx, 16
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Immediate(16)));
	// sub rdx, rbx (length = end - start)
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Now rdx has the length, move it to r8 as the function argument
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(8, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord))));
	// Move the handle into rcx
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RCX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSI, RegisterSize::QuadWord))));
	// Move the pointer to the buffer into rdx
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::General(RegisterType::RDX, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Move 0 into r9 (lpNumberOfBytesWritten)
	add_direct(instructions, Instruction::Mov(Argument::Register(Register::Extended(9, RegisterSize::QuadWord)), Argument::Immediate(0)));

	// WriteFile call
	// setup for call: sub rsp, 40
	add_direct(instructions, Instruction::Sub(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	// call WriteFile
	add_direct(instructions, Instruction::ExternCall("WriteFile".to_string()));
	// add rsp, 40
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(40)));
	
	// Cleanup
	// Deallocate stack space for the buffer
	add_direct(instructions, Instruction::Add(Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Register(Register::General(RegisterType::RSP, RegisterSize::QuadWord)), Argument::Immediate(32)));
	// Pop RDI
	add_direct(instructions, Instruction::Pop(Argument::Register(Register::General(RegisterType::RDI, RegisterSize::QuadWord))));
	// Pop RBX
	add_direct(instructions, Instruction::Pop(Argument::Register(Register::General(RegisterType::RBX, RegisterSize::QuadWord))));
	// Return
	add_direct(instructions, Instruction::Ret);

	return;
}
